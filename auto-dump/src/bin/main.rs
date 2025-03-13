use std::{fmt, path::PathBuf, time::Duration};

use anyhow::{anyhow, Context};
use bincode::deserialize;
use clap::{arg, Parser};
use dotenv::dotenv;
use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::get_epoch;
use jito_vault_auto_dump::{metrics::emit_vault_metrics, vault_handler::VaultHandler};
use jito_vault_core::{vault::Vault, vault_operator_delegation::VaultOperatorDelegation};
use jupiter_swap_api_client::{
    quote::QuoteRequest, swap::SwapRequest, transaction_config::TransactionConfig,
    JupiterSwapApiClient,
};
use log::{error, info};
use solana_rpc_client::{nonblocking::rpc_client::RpcClient, rpc_client::SerializableTransaction};
use solana_sdk::{pubkey::Pubkey, signature::{Keypair, read_keypair_file, Signer}, transaction::{Transaction, VersionedTransaction}, instruction::Instruction, address_lookup_table::{AddressLookupTableAccount, state::AddressLookupTable}, compute_budget::{ComputeBudgetInstruction}};
use spl_associated_token_account::get_associated_token_address;
use spl_token::instruction::transfer;
use std::str::FromStr;

#[derive(Parser)]
struct Args {
    /// RPC URL for the cluster
    #[arg(short, long, env, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Path to keypair used to pay
    #[arg(short, long, env)]
    keypair_path: PathBuf,

    /// Vault program ID (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "Vau1t6sLNxnzB7ZDsef8TLbPLfyZMYXH8WTNqUdm9g8"
    )]
    vault_program_id: Pubkey,

    /// kySOL Vault ATA (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "CQpvXgoaaawDCLh8FwMZEwQqnPakRUZ5BnzhjnEBPJv"
    )]
    kysol_vault_ata: Pubkey,

    /// kyJTO Vault ATA (Pubkey as base58 string)
    #[arg(
        long,
        env,
        default_value = "ABsoYTwRPBJEf55G7N8hVw7tQnDKBA6GkZCKBVrjTTcf"
    )]
    kyjto_vault_ata: Pubkey,

    /// Interval in seconds between cranking attempts (default: 50 minutes)
    #[arg(long, env, default_value = "3000")]
    crank_interval: u64,

    /// Interval in seconds between metrics emission (default: 5 minutes)
    #[arg(long, env, default_value = "300")]
    metrics_interval: u64,

    /// Priority fees (in microlamports per compute unit)
    #[arg(long, env, default_value = "10000")]
    priority_fees: u64,
}

impl fmt::Display for Args {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Jito Vault Auto Dumpooor Configuration:\n\
            -------------------------------\n\
            RPC URL: {}\n\
            Keypair Path: {:?}\n\
            Vault Program ID: {}\n\
            kySOL Vault ATA: {}\n\
            kyJTO Vault ATA: {}\n\
            Crank Interval: {} seconds\n\
            Metrics Interval: {} seconds\n\
            Priority Fees: {} microlamports\n\
            -------------------------------",
            self.rpc_url,
            self.keypair_path,
            self.vault_program_id,
            self.kysol_vault_ata,
            self.kyjto_vault_ata,
            self.crank_interval,
            self.metrics_interval,
            self.priority_fees,
        )
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<(), anyhow::Error> {
    let accounts_to_dump = [
        (
            Pubkey::from_str("J1toso1uCk3RLmjorhTtrVwY9HJ7X8V9yYac6Y7kGCPn").unwrap(),
            Pubkey::from_str("jtojtomepa8beP8AuQc6eXt5FriJwfFMwQx2v2f9mCL").unwrap(),
            Pubkey::from_str("BcGSCZckRdHHCuK4HV181KcM64nr9WrmFf7BK1wWwRhf").unwrap(),
            Pubkey::from_str("HzwDsHJBtuSTRx3VV6bz1R8yrLywxKgfGte7FASXU8Gd").unwrap(),
        )
    ];

    dotenv().ok();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();

    info!("{}", args);

    let rpc_client = RpcClient::new_with_timeout(args.rpc_url.clone(), Duration::from_secs(60));
    let payer = read_keypair_file(&args.keypair_path)
        .map_err(|e| anyhow!("Failed to read keypair file: {}", e))?;

    let config_address =
        jito_vault_core::config::Config::find_program_address(&args.vault_program_id).0;

    let account = rpc_client
        .get_account(&config_address)
        .await
        .context("Failed to read Jito vault config address")?;
    let config = jito_vault_core::config::Config::try_from_slice_unchecked(&account.data)
        .context("Failed to deserialize Jito vault config")?;

    let vault_handler = VaultHandler::new(
        &args.rpc_url,
        &payer,
        args.vault_program_id,
        config_address,
        args.priority_fees,
    );

    loop {
        let slot = rpc_client.get_slot().await.context("get slot")?;
        let epoch = get_epoch(slot, config.epoch_length()).unwrap();

        let kyjto_vault_ata_account = rpc_client.get_account(&args.kyjto_vault_ata).await?;
        let kyjto_vault = Vault::try_from_slice_unchecked(&kyjto_vault_ata_account.data)?;

        let kysol_vault_ata_account = rpc_client.get_account(&args.kysol_vault_ata).await?;
        let kysol_vault = Vault::try_from_slice_unchecked(&kysol_vault_ata_account.data)?;

        let compute_budget_instruction = ComputeBudgetInstruction::set_compute_unit_limit(
            100_000,
        );

        let compute_unit_price_instruction = ComputeBudgetInstruction::set_compute_unit_price(
            args.priority_fees,
        );

        for (from_token, to_token, vault_from_ata, vault_to_ata) in accounts_to_dump {
            match (|| async {
                info!("/////////////////////// START ///////////////////////");
                info!("Pulling {} tokens from {} to {}", from_token, vault_from_ata, payer.pubkey());

                let from_token_account = rpc_client
                    .get_token_account_balance(&vault_from_ata)
                    .await
                    .context("Failed to get token account balance")?;

                if from_token_account.amount.parse::<u64>().unwrap() < 100000000 {
                    // 0.1 JitoSOL minimum
                    info!("Not enough funds to pull.");
                } else {
                    let payer_from_token_account = get_associated_token_address(&payer.pubkey(), &from_token);
                    info!("Payer's associated token account for {}: {}", from_token, payer_from_token_account);
        
                    let transfer_ix = transfer(
                        &spl_token::id(),
                        &vault_from_ata,
                        &payer_from_token_account,
                        &payer.pubkey(),
                        &[],
                        from_token_account.amount.parse::<u64>().unwrap(),
                    )?;
        
                    let blockhash = rpc_client.get_latest_blockhash().await?;
                    let tx = Transaction::new_signed_with_payer(
                        &[compute_budget_instruction.clone(), compute_unit_price_instruction.clone(), transfer_ix],
                        Some(&payer.pubkey()),
                        &[&payer],
                        blockhash,
                    );
                    info!("> Delegating token transfer: {:?}", tx.get_signature());
                    let result = rpc_client.send_and_confirm_transaction(&tx).await;
                    if result.is_err() {
                        return Err(anyhow::anyhow!("Transaction failed: {:?}", result.err()));
                    }
                    info!("> Transaction confirmed: {:?}", tx.get_signature());
                }

                tokio::time::sleep(Duration::from_secs(10)).await;

                let payer_from_token_account = get_associated_token_address(&payer.pubkey(), &from_token);
                let payer_from_token_account_balance = rpc_client
                    .get_token_account_balance(&payer_from_token_account)
                    .await
                    .context("Failed to get token account balance")?;

                if payer_from_token_account_balance.amount.parse::<u64>().unwrap() < 100000000 {
                    info!("Not enough funds to swap.");
                } else {
                    info!("Dumping the funds to {}", to_token);
                    let jupiter_swap_api_client = JupiterSwapApiClient::new("https://quote-api.jup.ag/v6".to_string());

                    let quote_request = QuoteRequest {
                        amount: (payer_from_token_account_balance.amount.parse::<u64>().unwrap() as f64 * 0.95) as u64,
                        input_mint: from_token,
                        output_mint: to_token,
                        slippage_bps: 50,
                        ..QuoteRequest::default()
                    };
        
                    let quote_response = jupiter_swap_api_client.quote(&quote_request).await.unwrap();
        
                    let swap_response = jupiter_swap_api_client
                        .swap(&SwapRequest {
                            user_public_key: payer.pubkey(),
                            quote_response,
                            config: TransactionConfig::default(),
                        }, None)
                        .await
                        .map_err(|e| anyhow::Error::new(e))?;
        
                    let versioned_transaction: VersionedTransaction = bincode::deserialize(&swap_response.swap_transaction).unwrap();
                    let signed_versioned_transaction = VersionedTransaction::try_new(versioned_transaction.message, &[&payer]).unwrap();
        
                    // Send the raw transaction
                    let result = rpc_client.send_transaction(&signed_versioned_transaction).await;
                    if result.is_err() {
                        return Err(anyhow::anyhow!("Transaction failed: {:?}", result.err()));
                    }
                    info!("> Transaction confirmed");
                }

                tokio::time::sleep(Duration::from_secs(30)).await;

                let payer_to_token_account = get_associated_token_address(&payer.pubkey(), &to_token);

                let to_balance = rpc_client
                    .get_token_account_balance(&payer_to_token_account)
                    .await
                    .context("Failed to get token account balance")?;
                info!(" > Token balance of {} on wallet: {}", to_token, to_balance.amount);

                let fee_balance = rpc_client
                    .get_token_account_balance(&payer_from_token_account)
                    .await
                    .context("Failed to get token account balance")?;

                let fee_wallet_ata = get_associated_token_address(&Pubkey::from_str("42iznAJXXefUPmnYz6N6GCzFvXG42o3oTd2D1ymH4UmX").unwrap(), &from_token);

                if to_balance.amount.parse::<u64>().unwrap() < 100000 {
                    info!("Not enough funds to send.");
                    Ok(())
                } else {
                    info!("Sending the funds to {}", vault_to_ata);
                    let transfer_ix = transfer(
                        &spl_token::id(),
                        &payer_to_token_account,
                        &vault_to_ata,
                        &payer.pubkey(),
                        &[],
                        to_balance.amount.parse::<u64>().unwrap(),
                    )?;

                    let fee_transfer_ix = transfer(
                        &spl_token::id(),
                        &payer_from_token_account,
                        &fee_wallet_ata,
                        &payer.pubkey(),
                        &[],
                        fee_balance.amount.parse::<u64>().unwrap(),
                    )?;
        
                    let blockhash = rpc_client.get_latest_blockhash().await?;
                    let tx = Transaction::new_signed_with_payer(
                        &[compute_budget_instruction.clone(), compute_unit_price_instruction.clone(), transfer_ix, fee_transfer_ix],
                        Some(&payer.pubkey()),
                        &[&payer],
                        blockhash,
                    );
                    info!("> Sending token transfer to vault tx: {:?}", tx.get_signature());
                    let result = rpc_client.send_and_confirm_transaction(&tx).await;
                    if result.is_err() {
                        return Err(anyhow::anyhow!("Transaction failed: {:?}", result.err()));
                    }

                    info!("> Transaction confirmed: {:?}", tx.get_signature());
                    Ok(())
                }
            })().await {
                Ok(_) => info!("Execution completed."),
                Err(e) => error!("Error processing tokens {} -> {}: {}", from_token, to_token, e),
            }
        }
        info!("Sleeping for {} seconds", args.crank_interval);

        // ---------- SLEEP (crank_interval)----------
        tokio::time::sleep(Duration::from_secs(args.crank_interval)).await;
    }
}

use std::{collections::HashMap, fmt, path::PathBuf, time::Duration};

use anyhow::{anyhow, Context};
use clap::{arg, Parser};
use dotenv::dotenv;
use jito_bytemuck::AccountDeserialize;
use jito_jsm_core::get_epoch;
use jito_vault_client::instructions::AddDelegationBuilder;
use jito_vault_core::{vault::Vault, vault_operator_delegation::VaultOperatorDelegation};
use jito_vault_auto_delegate::{metrics::emit_vault_metrics, vault_handler::VaultHandler};
use log::{error, info};
use solana_rpc_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
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

    /// Interval in seconds between cranking attempts (default: 5 minutes)
    #[arg(long, env, default_value = "300")]
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
            "Jito Vault Auto Delegatoor Configuration:\n\
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

    let kyjto_vault_ata_account = rpc_client.get_account(&args.kyjto_vault_ata).await?;
    let kyjto_vault = Vault::try_from_slice_unchecked(&kyjto_vault_ata_account.data)?;

    let kysol_vault_ata_account = rpc_client.get_account(&args.kysol_vault_ata).await?;
    let kysol_vault = Vault::try_from_slice_unchecked(&kysol_vault_ata_account.data)?;

    let operators_with_amounts = [
        (Pubkey::from_str("29rxXT5zbTR1ctiooHtb1Sa1TD4odzhQHsrLz3D78G5w").unwrap(), "Kiln", 0),
        (Pubkey::from_str("LKFpfXtBkH5b7D9mo8dPcjCLZCZpmLQC9ELkbkyVdah").unwrap(), "Luganodes", 0),
        (Pubkey::from_str("BFEsrxFPsBcY2hR5kgyfKnpwgEc8wYQdngvRukLQXwG2").unwrap(), "Helius", 0),
        (Pubkey::from_str("CA8PaNSoFWzvbCJ2oK3QxBEutgyHSTT5omEptpj8YHPY").unwrap(), "Temporal", 0),
        (Pubkey::from_str("859tThorUu4uskw4yXSHkW9xqeCJmG5vm4KXhjWirLwL").unwrap(), "Laine", 0),
        (Pubkey::from_str("GZxp4e2Tm3Pw9GyAaxuF6odT3XkRM96jpZkp3nxhoK4Y").unwrap(), "PierTwo", 0),
        (Pubkey::from_str("FzZ9EXmHv7ANCXijpALUBzCza6wYNprnsfaEHuoNx9sE").unwrap(), "Everstake", 0),
    ];

    info!("/////////////////////// jitoSOL ///////////////////////");

    info!("jitoSOL Deposited : {:?}", kysol_vault.tokens_deposited());
    info!("kySOL staked amount : {:?}", kysol_vault.delegation_state.staked_amount());
    info!("kySOL VRT vrt_enqueued_for_cooldown_amount : {:?}", kysol_vault.vrt_enqueued_for_cooldown_amount());
    info!("kySOL VRT vrt_cooling_down_amount : {:?}", kysol_vault.vrt_cooling_down_amount());
    info!("kySOL VRT vrt_ready_to_claim_amount : {:?}", kysol_vault.vrt_ready_to_claim_amount());

    let total_stake_available = kysol_vault.tokens_deposited() - kysol_vault.delegation_state.staked_amount() - kysol_vault.vrt_enqueued_for_cooldown_amount() - kysol_vault.vrt_cooling_down_amount() - kysol_vault.vrt_ready_to_claim_amount();
    info!("Total stake available for delegation: {:?}", total_stake_available);

    let total_refered_amounts = operators_with_amounts
        .iter()
        .map(|(_, _, amount)| (amount * 75) / 100) // 75% for each
        .sum::<u64>();
    info!("Total jitoSOL refered amounts (75%): {:?}", total_refered_amounts);

    let total_amount_to_delegate = total_stake_available - total_refered_amounts;
    info!("Total jitoSOL amount to delegate: {:?}", total_amount_to_delegate);

    let amount_to_delegate_per_operator = total_amount_to_delegate / operators_with_amounts.len() as u64;
    info!("Amount jitoSOL to delegate per operator: {:?}", amount_to_delegate_per_operator);

    info!("/////////////////////// JTO ///////////////////////");

    info!("kyJTO staked amount : {:?}", kyjto_vault.delegation_state.staked_amount());
    info!("kyJTO VRT vrt_enqueued_for_cooldown_amount : {:?}", kyjto_vault.vrt_enqueued_for_cooldown_amount());
    info!("kyJTO VRT vrt_cooling_down_amount : {:?}", kyjto_vault.vrt_cooling_down_amount());
    info!("kyJTO VRT vrt_ready_to_claim_amount : {:?}", kyjto_vault.vrt_ready_to_claim_amount());

    let total_stake_available_kyjto = kyjto_vault.tokens_deposited() - kyjto_vault.delegation_state.staked_amount() - kyjto_vault.vrt_enqueued_for_cooldown_amount() - kyjto_vault.vrt_cooling_down_amount() - kyjto_vault.vrt_ready_to_claim_amount();
    info!("Total JTO stake available for delegation: {:?}", total_stake_available_kyjto);

    let total_refered_amounts_kyjto = operators_with_amounts
        .iter()
        .map(|(_, _, amount)| (amount * 75) / 100) // 75% for each
        .sum::<u64>();
    info!("Total JTO refered amounts (75%): {:?}", total_refered_amounts_kyjto);

    let total_amount_to_delegate_kyjto = total_stake_available_kyjto - total_refered_amounts_kyjto;
    info!("Total JTO amount to delegate: {:?}", total_amount_to_delegate_kyjto);

    let amount_to_delegate_per_operator_kyjto = total_amount_to_delegate_kyjto / operators_with_amounts.len() as u64;
    info!("Amount JTO to delegate per operator: {:?}", amount_to_delegate_per_operator_kyjto);

    /* 
    // Track vault metrics in separate thread
    tokio::spawn({
        let epoch_length = config.epoch_length();
        async move {
            let metrics_client = RpcClient::new_with_timeout(args.rpc_url, Duration::from_secs(60));
            loop {
                if let Err(e) = emit_vault_metrics(&metrics_client, epoch_length).await {
                    error!("Failed to emit metrics: {}", e);
                }
                tokio::time::sleep(Duration::from_secs(args.metrics_interval)).await;
            }
        }
    }); */
    let mut instructions = vec![];

    //loop {
        let slot = rpc_client.get_slot().await.context("get slot")?;
        let epoch = get_epoch(slot, config.epoch_length()).unwrap();

        info!("Checking for kySOL delegation to make to Operators. Slot: {slot}, Current Epoch: {epoch}");

        for (operator_key, operator_name, amount) in operators_with_amounts {
            info!("Processing operator: {}, key: {} with a total of self stake of : {}", operator_name, operator_key, amount);
            
            let vault_operator_delegation = VaultOperatorDelegation::find_program_address(
                &args.vault_program_id,
                &args.kysol_vault_ata,
                &operator_key,
            )
            .0;

            info!("Vault operator delegation: {}", vault_operator_delegation);
            let amount_to_delegate_local = amount_to_delegate_per_operator + ((amount * 75) / 100);
            info!("Delegating {} tokens to {}", amount_to_delegate_local, operator_name);
            let delegate_ix = vault_handler.delegate_to_operator_instruction(
                &args.kysol_vault_ata,
                &operator_key,
                &vault_operator_delegation,
                amount_to_delegate_local
            ).await?;
            instructions.push(delegate_ix);
        }

        info!("Sending batch transaction with {} delegation instructions", instructions.len());
        //vault_handler.send_and_confirm_transaction_with_retry(instructions).await?;
        info!("Successfully processed all delegations");


        info!("/////////////////////// JTO ///////////////////////");

        info!("Checking for JTO delegation to make to Operators. Slot: {slot}, Current Epoch: {epoch}");

        let mut instructions_kyjto = vec![];

        for (operator_key, operator_name, amount) in operators_with_amounts {
            info!("Processing operator: {}, key: {} with a total of self stake of : {}", operator_name, operator_key, amount);

            let vault_operator_delegation = VaultOperatorDelegation::find_program_address(
                &args.vault_program_id,
                &args.kyjto_vault_ata,
                &operator_key,
            )
            .0;

            info!("Vault operator delegation: {}", vault_operator_delegation);
            let amount_to_delegate_local = amount_to_delegate_per_operator_kyjto + ((amount * 75) / 100);
            info!("Delegating {} JTO tokens to {}", amount_to_delegate_local, operator_name);
            
            let delegate_ix = vault_handler.delegate_to_operator_instruction(
                &args.kyjto_vault_ata,
                &operator_key,
                &vault_operator_delegation,
                amount_to_delegate_local
            ).await?;
            instructions_kyjto.push(delegate_ix);
        }

        info!("Sending batch transaction with {} delegation instructions", instructions_kyjto.len());
        //vault_handler.send_and_confirm_transaction_with_retry(instructions_kyjto).await?;
        info!("Successfully processed all JTO delegations");


        //info!("Sleeping for {} seconds", args.crank_interval);
        // ---------- SLEEP (crank_interval)----------
        //tokio::time::sleep(Duration::from_secs(args.crank_interval)).await;
    // }
    
    Ok(())
}

//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>

use borsh::{BorshDeserialize, BorshSerialize};

/// Accounts.
pub struct ChangeWithdrawalTicketOwner {
    pub config: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub vault_staker_withdrawal_ticket: solana_program::pubkey::Pubkey,

    pub old_owner: solana_program::pubkey::Pubkey,

    pub new_owner: solana_program::pubkey::Pubkey,
}

impl ChangeWithdrawalTicketOwner {
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(&[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.config,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.vault_staker_withdrawal_ticket,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.old_owner,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.new_owner,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let data = ChangeWithdrawalTicketOwnerInstructionData::new()
            .try_to_vec()
            .unwrap();

        solana_program::instruction::Instruction {
            program_id: crate::JITO_VAULT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ChangeWithdrawalTicketOwnerInstructionData {
    discriminator: u8,
}

impl ChangeWithdrawalTicketOwnerInstructionData {
    pub fn new() -> Self {
        Self { discriminator: 14 }
    }
}

impl Default for ChangeWithdrawalTicketOwnerInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

/// Instruction builder for `ChangeWithdrawalTicketOwner`.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[]` vault
///   2. `[writable]` vault_staker_withdrawal_ticket
///   3. `[signer]` old_owner
///   4. `[]` new_owner
#[derive(Clone, Debug, Default)]
pub struct ChangeWithdrawalTicketOwnerBuilder {
    config: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    vault_staker_withdrawal_ticket: Option<solana_program::pubkey::Pubkey>,
    old_owner: Option<solana_program::pubkey::Pubkey>,
    new_owner: Option<solana_program::pubkey::Pubkey>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl ChangeWithdrawalTicketOwnerBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn config(&mut self, config: solana_program::pubkey::Pubkey) -> &mut Self {
        self.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn vault_staker_withdrawal_ticket(
        &mut self,
        vault_staker_withdrawal_ticket: solana_program::pubkey::Pubkey,
    ) -> &mut Self {
        self.vault_staker_withdrawal_ticket = Some(vault_staker_withdrawal_ticket);
        self
    }
    #[inline(always)]
    pub fn old_owner(&mut self, old_owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.old_owner = Some(old_owner);
        self
    }
    #[inline(always)]
    pub fn new_owner(&mut self, new_owner: solana_program::pubkey::Pubkey) -> &mut Self {
        self.new_owner = Some(new_owner);
        self
    }
    /// Add an aditional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = ChangeWithdrawalTicketOwner {
            config: self.config.expect("config is not set"),
            vault: self.vault.expect("vault is not set"),
            vault_staker_withdrawal_ticket: self
                .vault_staker_withdrawal_ticket
                .expect("vault_staker_withdrawal_ticket is not set"),
            old_owner: self.old_owner.expect("old_owner is not set"),
            new_owner: self.new_owner.expect("new_owner is not set"),
        };

        accounts.instruction_with_remaining_accounts(&self.__remaining_accounts)
    }
}

/// `change_withdrawal_ticket_owner` CPI accounts.
pub struct ChangeWithdrawalTicketOwnerCpiAccounts<'a, 'b> {
    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_staker_withdrawal_ticket: &'b solana_program::account_info::AccountInfo<'a>,

    pub old_owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_owner: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `change_withdrawal_ticket_owner` CPI instruction.
pub struct ChangeWithdrawalTicketOwnerCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub config: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault_staker_withdrawal_ticket: &'b solana_program::account_info::AccountInfo<'a>,

    pub old_owner: &'b solana_program::account_info::AccountInfo<'a>,

    pub new_owner: &'b solana_program::account_info::AccountInfo<'a>,
}

impl<'a, 'b> ChangeWithdrawalTicketOwnerCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: ChangeWithdrawalTicketOwnerCpiAccounts<'a, 'b>,
    ) -> Self {
        Self {
            __program: program,
            config: accounts.config,
            vault: accounts.vault,
            vault_staker_withdrawal_ticket: accounts.vault_staker_withdrawal_ticket,
            old_owner: accounts.old_owner,
            new_owner: accounts.new_owner,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(5 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.config.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.vault_staker_withdrawal_ticket.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.old_owner.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.new_owner.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let data = ChangeWithdrawalTicketOwnerInstructionData::new()
            .try_to_vec()
            .unwrap();

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::JITO_VAULT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(5 + 1 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.config.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.vault_staker_withdrawal_ticket.clone());
        account_infos.push(self.old_owner.clone());
        account_infos.push(self.new_owner.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `ChangeWithdrawalTicketOwner` via CPI.
///
/// ### Accounts:
///
///   0. `[]` config
///   1. `[]` vault
///   2. `[writable]` vault_staker_withdrawal_ticket
///   3. `[signer]` old_owner
///   4. `[]` new_owner
#[derive(Clone, Debug)]
pub struct ChangeWithdrawalTicketOwnerCpiBuilder<'a, 'b> {
    instruction: Box<ChangeWithdrawalTicketOwnerCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> ChangeWithdrawalTicketOwnerCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(ChangeWithdrawalTicketOwnerCpiBuilderInstruction {
            __program: program,
            config: None,
            vault: None,
            vault_staker_withdrawal_ticket: None,
            old_owner: None,
            new_owner: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn config(
        &mut self,
        config: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.config = Some(config);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn vault_staker_withdrawal_ticket(
        &mut self,
        vault_staker_withdrawal_ticket: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.vault_staker_withdrawal_ticket = Some(vault_staker_withdrawal_ticket);
        self
    }
    #[inline(always)]
    pub fn old_owner(
        &mut self,
        old_owner: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.old_owner = Some(old_owner);
        self
    }
    #[inline(always)]
    pub fn new_owner(
        &mut self,
        new_owner: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.new_owner = Some(new_owner);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let instruction = ChangeWithdrawalTicketOwnerCpi {
            __program: self.instruction.__program,

            config: self.instruction.config.expect("config is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            vault_staker_withdrawal_ticket: self
                .instruction
                .vault_staker_withdrawal_ticket
                .expect("vault_staker_withdrawal_ticket is not set"),

            old_owner: self.instruction.old_owner.expect("old_owner is not set"),

            new_owner: self.instruction.new_owner.expect("new_owner is not set"),
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct ChangeWithdrawalTicketOwnerCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    config: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault_staker_withdrawal_ticket: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    old_owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    new_owner: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
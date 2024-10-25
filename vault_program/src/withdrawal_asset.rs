use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub const fn process_withdrawal_asset(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _amount: u64,
) -> ProgramResult {
    Ok(())
}

// program logic
use solana_program:: {
  msg,
  account_info::{ AccountInfo, next_account_info },
  entrypoint::ProgramResult,
  program_error::ProgramError,
  pubkey::Pubkey,
  sysvar::{ rent::Rent, Sysvar},
  program_pack::{ IsInitialized, Pack },
  program::invoke,
};

use crate::{
  instruction::EscrowInstruction,
  error::EscrowError,
  state::Escrow,
};

pub struct Processor;
impl Processor {
  pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    let instruction = EscrowInstruction::unpack(instruction_data)?;

    match instruction {
      EscrowInstruction::InitEscrow { amount } => {
        Self::process_init_escrow(accounts, amount, program_id)
      }
    }
  }

  fn process_init_escrow(
    accounts: &[AccountInfo],
    amount: u64,
    program_id: &Pubkey
  ) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    // 1st account
    let initializer = next_account_info(account_info_iter)?;

    // check that account owner who initializes the transfer
    // is the signer of the given account
    if !initializer.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }

    // 2nd account
    let temp_token_account = next_account_info(account_info_iter)?;

    // 3rd account
    let token_to_receive_account = next_account_info(account_info_iter)?;

    if *token_to_receive_account.owner != spl_token::id() {
      return Err(ProgramError::IncorrectProgramId);
    }

    // 4th account
    let escrow_account = next_account_info(account_info_iter)?;

    // 5th account
    let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

    if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
      return Err(EscrowError::NotRentExempt.into());
    }

    let mut escrow_info = Escrow::unpack(&escrow_account.try_borrow_data()?)?;
    if !escrow_info.is_initialized() {
      return Err(ProgramError::AccountAlreadyInitialized);
    }

    // initialize the Escrow
    escrow_info.is_initialized = true;
    escrow_info.initializer_pubkey = *initializer.key;
    escrow_info.temp_token_account_pubkey = *temp_token_account.key;
    escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
    escrow_info.expected_amount = amount;

    Escrow::pack(escrow_info, &mut escrow_account.try_borrow_mut_data()?)?;

    // get the PDA
    let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

    // 6th account
    let token_program = next_account_info(account_info_iter)?;
    let change_ownership_ix = spl_token::instruction::set_authority(
      token_program.key, // the token program
      temp_token_account.key, // account to make the change
      Some(&pda), // new authority
      spl_token::instruction::AuthorityType::AccountOwner, // authority type to change
      initializer.key, // who owns the account
      &[&initializer.key] // who signs the instruction
    )?;

    msg!("Calling the token program to transfer token account ownership...");
    invoke(
      &change_ownership_ix,
      &[
        temp_token_account.clone(), // authority type to change
        initializer.clone(), // new authority
        token_program.clone()
      ]
    )?;

    Ok(())
  }
}
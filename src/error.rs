// program specific errors
use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
  #[error("Invalid Instruction")]
  InvalidInstruction,
  #[error("Not enough lamports to exempt rent")]
  NotRentExempt,
}

impl From<EscrowError> for ProgramError {
  fn from(err: EscrowError) -> Self {
    ProgramError::Custom(err as u32)
  }
}
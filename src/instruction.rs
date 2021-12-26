// program API, (deserialize instruction data)
use std::convert::TryInto;
use solana_program::program_error::ProgramError;
use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {

  /// start the trade by creating and populating an escrow account,
  /// and transferring ownership of the given temp account to the PDA.__rust_force_expr!
  /// 
  /// Accounts expected:
  /// 
  /// 0. `[signer]` The account of the person initializing the escrow
  /// 1. `[writable]` Temporary token account that should be created prior to this instruction
  ///   and owned by the initializer
  /// 2. `[]` The initializer's token account for the token they will receive should the trade go through
  /// 3. `[writable]` The escrow account, it will hold all necessary information about the trade
  /// 4. `[]` The rent sysvar
  /// 5. `[]` The token program
  InitEscrow {
    /// The amount party A expects to receive of token Y
    amount: u64
  }
}

impl EscrowInstruction {
  /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html)
  pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
    let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

    match tag {
      0 => Ok(Self::InitEscrow {
        amount: Self::unpack_amount(rest)?
      }),
      _ => Err(InvalidInstruction.into())
    }
  }

  pub fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
    let amount = input
      .get(..8)
      .and_then(|slice| slice.try_into().ok())
      .map(u64::from_le_bytes)
      .ok_or(InvalidInstruction)?;
    Ok(amount)
  }
}
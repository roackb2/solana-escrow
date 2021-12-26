// program API, (deserialize instruction data)

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
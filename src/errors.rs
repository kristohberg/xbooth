use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, program_error::PrintProgramError, program_error::ProgramError,
};
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, Copy, FromPrimitive, PartialEq)]
pub enum XBoothError {
    #[error("Invalid Account address.")]
    InvalidAccountAddress,
    #[error("Invalid Vault Account")]
    InvalidVaultAccount,
    #[error("Exchange booth is not writable")]
    ExchangeBoothNotWritable,
    #[error("Account is not writable")]
    AccountIsNotWritable,
    #[error("Account is not signer")]
    AccountIsNotSigner,
    #[error("Not correct owner")]
    InvalidOwner,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Account is not initialized")]
    AccountNotInitialized,
    #[error("Invalid SPL token account")]
    InvalidSPLTokenAccount,
    #[error("Invalid mint key")]
    InvalidMint,
    #[error("Accounts cannot have the same mint")]
    UniqueMintAccounts,
}

impl From<XBoothError> for ProgramError {
    fn from(e: XBoothError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for XBoothError {
    fn type_of() -> &'static str {
        "Exchange booth error"
    }
}

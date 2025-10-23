use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("Invalid amount provided.")]
    InvalidAmount,
    #[msg("Agreed upon amount not reached yet")]
    InsufficientTokensReceived
}
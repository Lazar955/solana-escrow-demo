use anchor_lang::prelude::*;

#[error_code]
pub enum EscrowError {
    #[msg("That is not the token mint the maker offered!")]
    MintError,
    #[msg("You are not authorized to accept this escrow!")]
    UnauthorizationError,
}

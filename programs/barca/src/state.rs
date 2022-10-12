use anchor_lang::prelude::*;

#[account]
pub struct Offer {
    pub initializer: Pubkey,
    pub offered_amount: u64,
    pub wanted_token_mint: Pubkey,
    pub wanted_amount: u64,
}

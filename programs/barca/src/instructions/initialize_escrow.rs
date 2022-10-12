use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, self};
use crate::state::Offer;
use std::mem::size_of;
use crate::error::*;


#[derive(Accounts)]
#[instruction()]
pub struct InitializeEscrow<'info> {
    #[account(
        init_if_needed, 
        seeds = [initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = 8 + size_of::<Offer>(),
    )]
    pub offer: Box<Account<'info, Offer>>,
    #[account(
        mut, 
        constraint = 
            initializer_offered_tokens_account.mint == 
            offered_tokens_mint.key() @ EscrowError::MintError
    )]
    pub initializer_offered_tokens_account: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        seeds = [offer.key().as_ref()],
        bump,
        token::mint = offered_tokens_mint,
        token::authority = escrowed_offered_tokens_account_pda
    )]
    pub escrowed_offered_tokens_account_pda: Account<'info, TokenAccount>,
    
    // Mint account of offered tokens
    pub offered_tokens_mint: Account<'info, Mint>,
    // Mint account of wanted tokens
    pub wanted_tokens_mint: Account<'info, Mint>,
    #[account(mut)]
    pub initializer: Signer<'info>,

    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_escrow(ctx: Context<InitializeEscrow>, offered_amount: u64,
    wanted_amount: u64,) -> Result<()> {
    let offer = &mut ctx.accounts.offer;
    offer.initializer = ctx.accounts.initializer.key();
    offer.offered_amount = offered_amount;
    offer.wanted_token_mint = ctx.accounts.wanted_tokens_mint.key();
    offer.wanted_amount = wanted_amount;
    
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.initializer_offered_tokens_account.to_account_info(),
                to: ctx.accounts.escrowed_offered_tokens_account_pda.to_account_info(),
                authority: ctx.accounts.initializer.to_account_info(),
            },
        ),
        offered_amount,
    )?;

    Ok(())
}




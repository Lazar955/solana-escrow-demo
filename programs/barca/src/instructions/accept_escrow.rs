use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, self};
use crate::state::Offer;
use crate::error::*;


#[derive(Accounts)]
pub struct AcceptEscrow<'info> {
    #[account(
        mut,
        constraint = offer.initializer == initializer.key(),
        // initializer is the one receiving lamports after closing
        close = initializer,
    )]
    pub offer: Box<Account<'info, Offer>>,
    // PDA of escrowed offered tokens
    #[account(
        mut,
        seeds = [offer.key().as_ref()],
        bump,
    )]
    pub escrowed_offered_tokens_account_pda: Account<'info, TokenAccount>,
    ///CHECK: Offer initializer's account
    #[account(
        mut,
        address = offer.initializer
    )]
    pub initializer: AccountInfo<'info>,
    // Offer taker's account
    #[account(mut)]
    pub taker: Signer<'info>,

    // initializer's wanted token account in which wanted tokens will land
    #[account(
        mut,
        constraint = 
            initializers_wanted_token_account.mint == 
            wanted_token_mint.key() @ EscrowError::MintError,
    )]
    pub initializers_wanted_token_account: Account<'info, TokenAccount>,
    // Takers wanted token account from which wanted tokens will be sent
    #[account(
        mut,
        constraint = 
            taker_wanted_token_account.mint == 
            offer.wanted_token_mint           
    )]
    pub taker_wanted_token_account: Account<'info, TokenAccount>,
    // Takers offered token account in which offered tokens will land
    #[account(
        mut,
        constraint = 
            takers_offered_tokens_account.mint ==
            escrowed_offered_tokens_account_pda.mint,    
    )]
    pub takers_offered_tokens_account: Account<'info, TokenAccount>,
    #[account(address = offer.wanted_token_mint.key() @ EscrowError::MintError)]
    pub wanted_token_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
}


pub fn accept_escrow(ctx: Context<AcceptEscrow>) -> Result<()> {
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.taker_wanted_token_account.to_account_info(),
                to: ctx.accounts.initializers_wanted_token_account.to_account_info(),
                authority: ctx.accounts.taker.to_account_info(),
            },
        ),
        ctx.accounts.offer.wanted_amount,
    )?;
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx
                    .accounts
                    .escrowed_offered_tokens_account_pda
                    .to_account_info(),
                to: ctx.accounts.takers_offered_tokens_account.to_account_info(),
                authority: ctx
                    .accounts
                    .escrowed_offered_tokens_account_pda
                    .to_account_info(),
            },
            &[&[
                ctx.accounts.offer.key().as_ref(),
                &[*ctx
                    .bumps
                    .get(&"escrowed_offered_tokens_account_pda".to_string())
                    .unwrap()],
            ]],
        ),
        ctx.accounts.escrowed_offered_tokens_account_pda.amount,
    )?;
    token::close_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::CloseAccount {
            account: ctx
                .accounts
                .escrowed_offered_tokens_account_pda
                .to_account_info(),
            destination: ctx.accounts.initializer.to_account_info(),
            authority: ctx
                .accounts
                .escrowed_offered_tokens_account_pda
                .to_account_info(),
        },
        &[&[
            ctx.accounts.offer.key().as_ref(),
            &[*ctx
                    .bumps
                    .get(&"escrowed_offered_tokens_account_pda".to_string())
                    .unwrap()],
        ]],
    ))
}

use crate::state::Offer;
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount};

#[derive(Accounts)]
pub struct CancelEscrow<'info> {
    #[account(
        mut,
        constraint = offer.initializer == *initializer.key,
        close = initializer,
    )]
    pub offer: Box<Account<'info, Offer>>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut)]
    pub initializers_offered_tokens_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [offer.key().as_ref()],
        bump,
    )]
    pub escrowed_offered_tokens_account_pda: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            token::Transfer {
                from: ctx
                    .accounts
                    .escrowed_offered_tokens_account_pda
                    .to_account_info(),
                to: ctx
                    .accounts
                    .initializers_offered_tokens_account
                    .to_account_info(),
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
            destination: ctx
                .accounts
                .initializers_offered_tokens_account
                .to_account_info(),
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

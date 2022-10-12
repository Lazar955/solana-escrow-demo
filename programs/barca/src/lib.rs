use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod barca {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrow>,
        offered_amount: u64,
        wanted_amount: u64,
    ) -> Result<()> {
        instructions::initialize_escrow(ctx, offered_amount, wanted_amount)
    }

    pub fn cancel_escrow(ctx: Context<CancelEscrow>) -> Result<()> {
        instructions::cancel_escrow(ctx)
    }

    pub fn accept_escrow(ctx: Context<AcceptEscrow>) -> Result<()> {
        instructions::accept_escrow(ctx)
    }
}

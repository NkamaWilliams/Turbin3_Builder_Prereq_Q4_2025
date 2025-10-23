use anchor_lang::prelude::*;

declare_id!("Ga72Tw8DFUVcAMpTc9PMCE18bezPy3Xao8vXdtdE6cZS");

pub mod state;
pub mod instructions;
pub mod error;

pub use instructions::*;

#[program]
pub mod escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        // msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)
    }

    pub fn take(ctx: Context<Take>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        ctx.accounts.receive(seed)
    }
}
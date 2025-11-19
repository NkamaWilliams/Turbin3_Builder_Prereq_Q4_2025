use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{errors::TruthLieError, state::PlayerStats};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        seeds = [b"stats", payer.key().as_ref()],
        bump = player_stats.bump
    )]
    pub player_stats: Account<'info, PlayerStats>,
    #[account(
        mut,
        seeds = [b"vault", payer.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        require!(self.vault.lamports() >= amount, TruthLieError::InsufficientFunds);
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.payer.to_account_info()
        };
        let signer_seeds: &[&[&[u8]]] = &[&[b"vault", self.payer.to_account_info().key.as_ref(), &[self.player_stats.vault_bump]]];
        let ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(ctx, amount)
    }
}
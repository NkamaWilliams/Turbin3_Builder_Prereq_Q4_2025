use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::state::PlayerStats;

#[derive(Accounts)] 
pub struct ClosePlayer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"stats", payer.key().as_ref()],
        bump = player_stats.bump,
        close = payer
    )]
    pub player_stats: Account<'info, PlayerStats>,
    #[account(
        mut,
        seeds = [b"vault", player_stats.wallet.as_ref()],
        bump = player_stats.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    
    pub system_program: Program<'info, System>
}

impl<'info> ClosePlayer<'info> {
    pub fn close_player(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"vault", 
            self.player_stats.to_account_info().key.as_ref(), 
            &[self.player_stats.vault_bump]
        ]];
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.payer.to_account_info()
        };
        let ctx = CpiContext::new_with_signer(self.system_program.to_account_info(), accounts, signer_seeds);
        transfer(ctx, self.vault.lamports())
    }
}
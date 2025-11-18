use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::state::PlayerStats;

#[derive(Accounts)]
pub struct CreatePlayer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        space = PlayerStats::DISCRIMINATOR.len() + PlayerStats::INIT_SPACE,
        seeds = [b"stats", payer.key().as_ref()],
        payer = payer,
        bump
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

impl<'info> CreatePlayer<'info> {
    pub fn create_player(&mut self, bumps: CreatePlayerBumps) -> Result<()> {
        self.player_stats.set_inner(PlayerStats {
            wallet: self.payer.key(),
            vault: self.vault.key(),
            bump: bumps.player_stats,
            vault_bump: bumps.vault,
            achievements: vec![],
            minted_badges: vec![0; 32],
            correct_guesses: 0u32,
            games_played: 0u32,
            wrong_guesses: 0u32
        });

        let rent_exempt = Rent::get()?.minimum_balance(self.vault.data_len());
        let accounts = Transfer {
            from: self.payer.to_account_info(),
            to: self.vault.to_account_info()
        };
        let ctx = CpiContext::new(
            self.system_program.to_account_info(), 
            accounts
        );

        transfer(ctx, rent_exempt)
    }
}
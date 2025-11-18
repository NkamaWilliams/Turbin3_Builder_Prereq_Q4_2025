use anchor_lang::prelude::*;

use crate::{state::PlayerStats, utils::Achievement, program::Truthlie};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct PlayerStatsInput {
    pub games_played: Option<u32>,
    pub correct_guesses: Option<u32>,
    pub wrong_guesses: Option<u32>
}

#[derive(Accounts)]
pub struct UpdatePlayer<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK this account will be validated when used to derive player_stats seeds
    pub player: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"stats", player.key().as_ref()],
        bump = player_stats.bump
    )]
    pub player_stats: Account<'info, PlayerStats>,

    pub system_program: Program<'info, System>,
    #[account(constraint = truthlie_program.programdata_address()? == Some(program_data.key()))]
    pub truthlie_program: Program<'info, Truthlie>,
    #[account(constraint = program_data.upgrade_authority_address == Some(payer.key()))]
    pub program_data: Account<'info, ProgramData>
}

impl<'info> UpdatePlayer<'info> {
    pub fn update_player(&mut self, update: PlayerStatsInput) -> Result<()> {
        self.player_stats.games_played = update.games_played.unwrap_or(self.player_stats.games_played);
        self.player_stats.correct_guesses = update.correct_guesses.unwrap_or(self.player_stats.correct_guesses);
        self.player_stats.wrong_guesses = update.wrong_guesses.unwrap_or(self.player_stats.wrong_guesses);

        Ok(())
    }

    pub fn unlock_achievements(&mut self, unlocked_achievements: Vec<Achievement>) -> Result<()> {
        for achievement in unlocked_achievements {
            self.player_stats.add_achievement(achievement);
        }

        Ok(())
    }
}
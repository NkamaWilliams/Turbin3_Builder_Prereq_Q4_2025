use anchor_lang::prelude::*;

use crate::utils::Achievement;

#[account]
#[derive(InitSpace)]
pub struct PlayerStats {
    pub wallet: Pubkey,
    pub vault: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
    #[max_len(32)]
    pub achievements: Vec<Achievement>,
    #[max_len(32)]
    pub minted_badges: Vec<u8>,
    pub games_played: u32,
    pub correct_guesses: u32,
    pub wrong_guesses: u32
}

impl PlayerStats {
    pub fn contains_achievement(&self, achievement: Achievement) -> bool {
        self.achievements.contains(&achievement)
    }

    pub fn add_achievement(&mut self, achievement: Achievement) {
        if !self.contains_achievement(achievement) {
            self.achievements.push(achievement);
        }
    }
}
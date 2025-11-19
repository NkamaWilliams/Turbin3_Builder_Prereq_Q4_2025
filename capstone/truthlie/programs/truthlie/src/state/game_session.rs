use anchor_lang::prelude::*;

use crate::utils::MAX_PLAYERS_PER_GAME;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct Player {
    pub pubkey: Pubkey,
    pub score: u8
}

#[account]
#[derive(InitSpace)]
pub struct GameSession {
    #[max_len(MAX_PLAYERS_PER_GAME)]
    pub players: Vec<Player>,
    pub stake_amount: u64,
    pub seed: u64,
    pub max_players: u8,
    pub bump: u8,
    pub rounds: u8,
    pub vault_bump: u8,
    pub is_started: bool
}

impl GameSession {
    pub fn contains_player(&self, player: &Pubkey) -> bool {
        return self.players.iter().any(|p| p.pubkey == *player);
    }

    /// Adds a player to a game session
    /// 
    /// NOTE: This assumes that you are checking to ensure the number of players
    /// in a game doesn't exceed the maximum amount allowed
    pub fn add_player(&mut self, player: Pubkey) {
        self.players.push(Player { pubkey: player, score: 0 });
    }

    /// Updates the score of a player in a game session
    /// 
    /// Returns 'true' if player was found and score updated successfully
    /// else returns 'false'
    /// 
    /// NOTE should only be called to store final score
    pub fn update_player(&mut self, player: &Pubkey, score: u8) -> bool {
        for old_player in self.players.iter_mut() {
            if old_player.pubkey == *player {
                old_player.score = score;
                return true;
            }
        }

        false
    }
}
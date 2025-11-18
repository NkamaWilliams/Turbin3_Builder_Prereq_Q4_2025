use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{errors::TruthLieError, state::GameSession};

#[derive(Accounts)]
pub struct JoinGameSession<'info> {
    #[account(
        mut,
        constraint = !game_session.contains_player(player.key) @ TruthLieError::PlayerAlreadyInGame
    )]
    pub player: Signer<'info>,
    #[account(
        mut,
        seeds = [b"session", game_session.seed.to_le_bytes().as_ref()],
        bump = game_session.bump
    )]
    pub game_session: Account<'info, GameSession>,
    #[account(
        mut,
        seeds = [b"vault", game_session.key().as_ref()],
        bump = game_session.vault_bump
    )]
    pub game_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> JoinGameSession<'info> {
    pub fn join_session(&mut self) -> Result<()> {
        require!(self.player.lamports() >= self.game_session.stake_amount, TruthLieError::InsufficientBalanceForStake);
        require!(self.game_session.players.len() < self.game_session.max_players as usize, TruthLieError::GameLobbyFull);

        self.game_session.add_player(self.player.key());

        let accounts = Transfer {
            from: self.player.to_account_info(),
            to: self.game_vault.to_account_info()
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        
        transfer(ctx, self.game_session.stake_amount)
    }
}
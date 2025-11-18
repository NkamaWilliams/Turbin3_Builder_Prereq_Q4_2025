use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{errors::TruthLieError, state::GameSession, utils::{MAX_PLAYERS_PER_GAME, MIN_STAKE_AMOUNT}};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct GameSessionInput {
    pub max_players: u8,
    pub rounds: u8,
    pub stake_amount: u64,
}

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateGameSession<'info> {
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(
        init,
        payer = host,
        seeds = [b"session", seed.to_le_bytes().as_ref()],
        space = GameSession::DISCRIMINATOR.len() + GameSession::INIT_SPACE,
        bump
    )]
    pub game_session: Account<'info, GameSession>,
    #[account(
        mut,
        seeds = [b"vault", game_session.key().as_ref()],
        bump
    )]
    pub game_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateGameSession<'info> {
    pub fn create_session(&mut self, seed: u64, input: GameSessionInput, bumps: CreateGameSessionBumps) -> Result<()> {
        require!(input.stake_amount >= MIN_STAKE_AMOUNT, TruthLieError::StakeAmountTooSmall);
        require!(self.host.lamports() >= input.stake_amount, TruthLieError::InsufficientBalanceForStake);
        require!(input.max_players <= MAX_PLAYERS_PER_GAME, TruthLieError::MaximumPlayersExceeded);
        require!(input.max_players >= 2, TruthLieError::InvalidPlayerCount);

        self.game_session.set_inner(GameSession {
            players: vec![],
            stake_amount: input.stake_amount,
            rounds: input.rounds,
            seed,
            max_players: input.max_players,
            bump: bumps.game_session,
            vault_bump: bumps.game_vault,
        });

        self.game_session.add_player(self.host.key());

        let accounts = Transfer {
            from: self.host.to_account_info(),
            to: self.game_vault.to_account_info()
        };

        let ctx = CpiContext::new(self.system_program.to_account_info(), accounts);
        
        transfer(ctx, input.stake_amount)
    }
}

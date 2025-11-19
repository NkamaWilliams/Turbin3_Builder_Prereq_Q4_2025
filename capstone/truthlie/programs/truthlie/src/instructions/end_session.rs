use anchor_lang::prelude::{program::invoke_signed, *};

use crate::{errors::TruthLieError, program::Truthlie, state::{GameSession, Player}};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct EndSessionInput {
    rounds_completed: u8,
    player_scores: Vec<Player>
}

#[derive(Accounts)]
pub struct EndGameSession<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
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
    #[account(constraint = truthlie_program.programdata_address()? == Some(program_data.key()))]
    pub truthlie_program: Program<'info, Truthlie>,
    #[account(constraint = program_data.upgrade_authority_address == Some(payer.key()) @TruthLieError::NotAuthorized)]
    pub program_data: Account<'info, ProgramData>
}

impl<'info> EndGameSession<'info> {
    pub fn update_scores(&mut self, input: EndSessionInput) -> Result<()> {
        require!(input.rounds_completed == self.game_session.rounds, TruthLieError::GameNotOverYet);

        for player in input.player_scores {
            let success = self.game_session.update_player(&player.pubkey, player.score);
            if !success {
                return Err(TruthLieError::PlayerNotPartOfGame.into());
            }
        }

        Ok(())
    }

    pub fn end_session(&mut self, vaults: &'info [AccountInfo<'info>]) -> Result<()> {
        let mut winners = vec![];
        let mut highest_score: u8 = 0;

        for player in self.game_session.players.iter() {
            if player.score > highest_score {
                winners.clear();
                highest_score = player.score;
            }
            if player.score == highest_score {
                winners.push(player.pubkey);
            }
        }

        require!(winners.len() == vaults.len(), TruthLieError::IncorrectAmountOfWinners);

        let winnings = self.game_vault.lamports() / winners.len() as u64;

        let vault_seeds: &[&[&[u8]]] = &[&[
            b"vault",
            self.game_session.to_account_info().key.as_ref(),
            &[self.game_session.vault_bump]
        ]];

        let winner_vaults = winners.into_iter().map(|winner| {
            let (vault, _bump) = Pubkey::find_program_address(
                &[
                    b"vault",
                    winner.as_ref()
                ], 
                &crate::ID
            );
            vault
        }).collect::<Vec<_>>();

        for vault in vaults {
            if !winner_vaults.contains(&vault.key()) {
                return Err(TruthLieError::InvalidWinner.into());
            }
            let transfer_ix = system_instruction::transfer(&self.game_vault.key(), &vault.key(), winnings);
            
            invoke_signed(
                &transfer_ix, 
                &[self.game_vault.to_account_info(), vault.clone()], 
                vault_seeds)?;
        }

        Ok(())
    }
}
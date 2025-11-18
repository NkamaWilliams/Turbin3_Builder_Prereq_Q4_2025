use anchor_lang::prelude::*;

declare_id!("CncSQYC79gXJzmsRo93NZ5XrCb1VrjQzrwnQKNMD58T6");

pub mod state;
pub mod utils;
pub mod instructions;
pub mod errors;

pub use instructions::*;
use crate::utils::Achievement;

#[program]
pub mod truthlie {

    use super::*;

    pub fn create_player(ctx: Context<CreatePlayer>) -> Result<()> {
        ctx.accounts.create_player(ctx.bumps)
    }

    pub fn update_player(ctx: Context<UpdatePlayer>, update: PlayerStatsInput) -> Result<()> {
        ctx.accounts.update_player(update)
    }

    pub fn unlock_achievements(ctx: Context<UpdatePlayer>, unlocked_achievements: Vec<Achievement>) -> Result<()> {
        ctx.accounts.unlock_achievements(unlocked_achievements)
    }

    pub fn close_player(ctx: Context<ClosePlayer>) -> Result<()> {
        ctx.accounts.close_player()
    }

    pub fn create_session(ctx: Context<CreateGameSession>, seed: u64, input: GameSessionInput) -> Result<()> {
        ctx.accounts.create_session(seed, input, ctx.bumps)
    }

    pub fn join_session(ctx: Context<JoinGameSession>) -> Result<()> {
        ctx.accounts.join_session()
    }

    pub fn end_session<'info>(ctx: Context<'_, '_, 'info, 'info, EndGameSession<'info>>, input: EndSessionInput) -> Result<()> {
        ctx.accounts.update_scores(input)?;
        
        let winning_vaults = ctx.remaining_accounts;
        ctx.accounts.end_session(winning_vaults)
    }

    pub fn whitelist_creator(ctx: Context<WhitelistCreator>) -> Result<()> {
        ctx.accounts.whitelist_creator(&ctx.bumps)
    }

    pub fn create_collection(ctx: Context<CreateCollection>, args: CreateCollectionArgs) -> Result<()> {
        ctx.accounts.create_collection(args, &ctx.bumps)
    }

    pub fn mint_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        ctx.accounts.mint_nft(args)
    }
}
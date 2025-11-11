use anchor_lang::prelude::*;

declare_id!("72sncXoP5FUSmoBUTQvJwMQBYFR6br9aBdsBNJQEmjGs");

pub mod errors;
pub mod state;
pub mod instructions;

use crate::instructions::*;
#[program]
pub mod mplxcore {

    use super::*;

    pub fn whitelist_creator(ctx: Context<WhitelistCreator>) -> Result<()> {
        ctx.accounts.whitelist_creator(&ctx.bumps)
    }

    pub fn create_collection(ctx: Context<CreateCollection>, args: CreateCollectionArgs) -> Result<()> {
        ctx.accounts.create_collection(args, &ctx.bumps)
    }

    pub fn mint_nft(ctx: Context<MintNft>) -> Result<()> {
        ctx.accounts.mint_nft()
    }

    pub fn freeze_nft(ctx: Context<FreezeNft>) -> Result<()> {
        ctx.accounts.freeze_nft()
    }

    pub fn thaw_nft(ctx: Context<ThawNft>) -> Result<()> {
        ctx.accounts.thaw_nft()
    }

    // pub fn update_nft(ctx: Context<UpdateNft>, new_name: String) -> Result<()> {
    //     ctx.accounts.update_nft(new_name)
    // }
}
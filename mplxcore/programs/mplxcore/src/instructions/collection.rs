use anchor_lang::prelude::*;
use mpl_core::{instructions::CreateCollectionV2CpiBuilder};

use crate::{errors::MplxcoreError, state::{CollectionAuthority, WhitelistedCreators}};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct CreateCollectionArgs {
    pub name: String,
    pub uri: String,
    pub nft_name: String,
    pub nft_uri: String
}

#[derive(Accounts)]
pub struct CreateCollection<'info> {
    #[account(
        mut,
        constraint = whitelisted_creators.contains(&creator) @MplxcoreError::NotAuthorized
    )]
    pub creator: Signer<'info>,
    #[account(
        mut,
        constraint = collection.data_is_empty() @MplxcoreError::CollectionAlreadyInitialized
    )]
    pub collection: Signer<'info>,
    #[account(
        seeds = [b"whitelist"],
        bump = whitelisted_creators.bump
    )]
    pub whitelisted_creators: Account<'info, WhitelistedCreators>,
    #[account(
        init,
        payer = creator,
        seeds = [b"collection_authority", collection.key().as_ref()],
        space = CollectionAuthority::DISCRIMINATOR.len() + CollectionAuthority::INIT_SPACE,
        bump
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,

    /// CHECK: This will also be checked by core
    pub core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>
}

impl<'info> CreateCollection<'info>{
    pub fn create_collection(
        &mut self,
        args: CreateCollectionArgs,
        bumps: &CreateCollectionBumps
    ) -> Result<()> {
        self.collection_authority.set_inner(CollectionAuthority {
            creator: self.creator.key(),
            collection: self.collection.key(),
            nft_name: args.nft_name,
            nft_uri: args.nft_uri,
            bump: bumps.collection_authority
        });

        let collection_key = self.collection.key();
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_authority".as_ref(), 
            &collection_key.to_bytes(), 
            &[bumps.collection_authority]
         ]];

        CreateCollectionV2CpiBuilder::new(&self.core_program.to_account_info())
            .collection(&self.collection.to_account_info())
            .payer(&self.creator.to_account_info())
            .update_authority(Some(&self.collection_authority.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(args.name)
            .uri(args.uri)
            .plugins(vec![])
            .invoke_signed(signer_seeds)?;

        Ok(())
    }
} 
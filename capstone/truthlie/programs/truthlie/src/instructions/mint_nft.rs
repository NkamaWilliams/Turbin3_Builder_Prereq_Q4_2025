use crate::{errors::TruthLieError, state::{CollectionAuthority, PlayerStats}, utils::Achievement};
use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateV2CpiBuilder,
    types::{
        Attribute, Attributes, FreezeDelegate, Plugin, PluginAuthority,
        PluginAuthorityPair,
    },
    ID as CORE_PROGRAM_ID,
};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct MintNftArgs {
    nft_name: String,
    nft_uri: String,
    achievement: Achievement
}

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,
    #[account(mut, constraint = asset.data_is_empty() @TruthLieError::AssetAlreadyInitialized)]
    pub asset: Signer<'info>,
    #[account(
        mut,
        constraint = collection.owner == &CORE_PROGRAM_ID @TruthLieError::InvalidCollection,
        constraint = !collection.data_is_empty() @TruthLieError::CollectionNotInitialized
    )]
    /// CHECK: This will be validated by the constraints and further in the logic
    pub collection: UncheckedAccount<'info>,
    #[account(
        seeds = [b"collection_authority", collection.key().as_ref()],
        bump = collection_authority.bump
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,
    #[account(
        mut,
        seeds = [b"stats", minter.key().as_ref()],
        bump = player_stats.bump
    )]
    pub player_stats: Account<'info, PlayerStats>,

    pub system_program: Program<'info, System>,
    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: This will also be checked by core
    pub core_program: UncheckedAccount<'info>,
}

impl<'info> MintNft<'info> {
    pub fn mint_nft(&mut self, args: MintNftArgs) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_authority".as_ref(),
            &self.collection.key().to_bytes(),
            &[self.collection_authority.bump],
        ]];

        if self.player_stats.minted_badges[args.achievement as usize] == 1 {
            return Err(TruthLieError::AchievementAlreadyMinted.into());
        }

        let plugins = vec![
            PluginAuthorityPair {
                plugin: Plugin::Attributes(Attributes {
                    attribute_list: vec![
                        Attribute {
                            key: "Creator".to_string(),
                            value: self.collection_authority.creator.to_string(),
                        },
                        Attribute {
                            key: "Minter".to_string(),
                            value: self.minter.key().to_string(),
                        },
                    ],
                }),
                authority: None,
            },
            PluginAuthorityPair {
                plugin: Plugin::FreezeDelegate(FreezeDelegate { frozen: true }),
                authority: Some(PluginAuthority::Address {
                    address: self.collection_authority.key(),
                }),
            },
        ];

        CreateV2CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .authority(Some(&self.collection_authority.to_account_info()))
            .payer(&self.minter.to_account_info())
            .owner(Some(&self.minter.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(args.nft_name)
            .uri(args.nft_uri)
            .plugins(plugins)
            .external_plugin_adapters(vec![])
            .invoke_signed(signer_seeds)?;
        
        self.player_stats.minted_badges[args.achievement as usize] = 1;
        Ok(())
    }
}

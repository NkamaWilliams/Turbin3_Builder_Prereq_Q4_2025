use anchor_lang::prelude::*;
use mpl_core::{ID as CORE_PROGRAM_ID, instructions::CreateV2CpiBuilder, types::{Attribute, Attributes, BurnDelegate, FreezeDelegate, Plugin, PluginAuthority, PluginAuthorityPair}};
use crate::{errors::MplxcoreError, state::CollectionAuthority};

#[derive(Accounts)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub minter: Signer<'info>,
    #[account(mut, constraint = asset.data_is_empty() @MplxcoreError::AssetAlreadyInitialized)]
    pub asset: Signer<'info>,
    #[account(
        mut,
        constraint = collection.owner == &CORE_PROGRAM_ID @MplxcoreError::InvalidCollection,
        constraint = !collection.data_is_empty() @MplxcoreError::CollectionNotInitialized
    )]
    /// CHECK: This will be validated by the constraints and further in the logic
    pub collection: UncheckedAccount<'info>,
    #[account(
        seeds = [b"collection_authority", collection.key().as_ref()],
        bump = collection_authority.bump
    )]
    pub collection_authority: Account<'info, CollectionAuthority>,

    pub system_program: Program<'info, System>,
    #[account(address = CORE_PROGRAM_ID)]
    /// CHECK: This will also be checked by core
    pub core_program: UncheckedAccount<'info>
}

impl<'info> MintNft<'info> {
    pub fn mint_nft(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"collection_authority".as_ref(), 
            &self.collection.key().to_bytes(), 
            &[self.collection_authority.bump ]
        ]];

        let plugins = vec![
            PluginAuthorityPair {
                plugin: Plugin::Attributes(
                    Attributes {attribute_list:  vec![
                        Attribute {
                            key: "Creator".to_string(),
                            value: self.collection_authority.creator.to_string() 
                        },
                        Attribute {
                            key: "Minter".to_string(),
                            value: self.minter.key().to_string()
                        }
                    ]}
                ),
                authority: None
            },
            PluginAuthorityPair {
                plugin: Plugin::FreezeDelegate(
                    FreezeDelegate {
                        frozen: false
                    }
                ),
                authority: Some(PluginAuthority::Address { address: self.collection_authority.key() })
            },
            PluginAuthorityPair {
                plugin: Plugin::BurnDelegate(BurnDelegate {}),
                authority: Some(PluginAuthority::Address { address: self.collection_authority.key() })
            }
        ];

        CreateV2CpiBuilder::new(&self.core_program.to_account_info())
            .asset(&self.asset.to_account_info())
            .collection(Some(&self.collection.to_account_info()))
            .authority(Some(&self.collection_authority.to_account_info()))
            .payer(&self.minter.to_account_info())
            .owner(Some(&self.minter.to_account_info()))
            .system_program(&self.system_program.to_account_info())
            .name(self.collection_authority.nft_name.clone())
            .uri(self.collection_authority.nft_uri.clone())
            .plugins(plugins)
            .external_plugin_adapters(vec![])
            .invoke_signed(signer_seeds)?;
            

        Ok(())
    }
}
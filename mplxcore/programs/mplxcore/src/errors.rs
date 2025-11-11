use anchor_lang::prelude::*;

#[error_code]
pub enum MplxcoreError {
    #[msg("Missing Signer")]
    MissingSigner,
    #[msg("Creator List is already full")]
    CreatorListAlreadyFull,
    #[msg("Creator already in white list")]
    CreatorAlreadyInWhitelist,
    #[msg("The payer is not the program's upgrade authority")]
    NotAuthorized,
    #[msg("The collection has already been initialized")]
    CollectionAlreadyInitialized,
     #[msg("The collection has not been initialized")]
    CollectionNotInitialized,
    #[msg("The asset has already been initialized")]
    AssetAlreadyInitialized,
    #[msg("The collection account provided is invalid")]
    InvalidCollection,
    #[msg("The asset account provided is invalid")]
    InvalidAsset
}
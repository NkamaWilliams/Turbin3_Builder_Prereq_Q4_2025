use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct CollectionAuthority {
    pub creator: Pubkey,
    pub collection: Pubkey,
    pub bump: u8
}
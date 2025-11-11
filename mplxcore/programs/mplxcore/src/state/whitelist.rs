use anchor_lang::prelude::*;
use crate::errors::MplxcoreError;

#[account]
#[derive(InitSpace)]
pub struct WhitelistedCreators {
    pub creators: [Pubkey; 10],
    pub num_creators: u8,
    pub bump: u8
}
impl WhitelistedCreators {
    pub fn contains(&self, creator: &AccountInfo) -> bool {
        self.creators[..self.num_creators as usize].contains(creator.key)
    }

    pub fn whitelist_creator(&mut self, creator: &AccountInfo) -> Result<()> {
        if self.num_creators as usize >= self.creators.len() {
            return err!(MplxcoreError::CreatorListAlreadyFull);
        }

        if self.contains(creator) {
            return err!(MplxcoreError::CreatorAlreadyInWhitelist);
        }

        self.creators[self.num_creators as usize] = creator.key();
        self.num_creators += 1;
        Ok(())
    }
}
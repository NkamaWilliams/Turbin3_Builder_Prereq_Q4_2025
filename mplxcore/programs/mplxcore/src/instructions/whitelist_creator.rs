use anchor_lang::prelude::*;

use crate::{
    program::Mplxcore, state::WhitelistedCreators
};

#[derive(Accounts)]
pub struct WhitelistCreator<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    /// CHECK should be a keypair
    pub creator: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        payer = admin,
        space = WhitelistedCreators::DISCRIMINATOR.len() + WhitelistedCreators::INIT_SPACE,
        seeds = [b"whitelist"],
        bump
    )]
    pub whitelisted_creators: Account<'info, WhitelistedCreators>,
    pub system_program: Program<'info, System>,
    #[account(constraint = this_program.programdata_address()? == Some(program_data.key()))]
    pub this_program: Program<'info, Mplxcore>,
    #[account(constraint = program_data.upgrade_authority_address == Some(admin.key()))]
    pub program_data: Account<'info, ProgramData>
}

impl<'info> WhitelistCreator<'info> {
    pub fn whitelist_creator(&mut self, bumps: &WhitelistCreatorBumps) -> Result<()> {
        self.whitelisted_creators.bump = bumps.whitelisted_creators;
        self.whitelisted_creators.whitelist_creator(&self.creator)
    }
}
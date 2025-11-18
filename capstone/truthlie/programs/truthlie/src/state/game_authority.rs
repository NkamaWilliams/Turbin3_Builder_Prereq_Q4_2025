use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GameAuthority {
    #[max_len(10)]
    admins: Vec<Pubkey>,
    bump: u8
}
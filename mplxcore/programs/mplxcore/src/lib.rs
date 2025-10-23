use anchor_lang::prelude::*;

declare_id!("7s527xEE6GkjUJsWLapDVSEWi2vSJp5kUt5iutag6M2R");

#[program]
pub mod mplxcore {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

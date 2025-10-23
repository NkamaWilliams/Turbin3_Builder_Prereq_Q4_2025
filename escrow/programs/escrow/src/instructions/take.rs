use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked}};

use crate::{state::Escrow, error::EscrowError};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Take<'info>{
    #[account(mut)]
    pub taker: Signer<'info>,

    /// CHECK: fails if maker fails escrow's has_one constraint
    pub maker: UncheckedAccount<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = escrow.mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut, // Not init_if_needed since it ain't fair for the taker to pay for creating the maker's ata address
        associated_token::mint = escrow.mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        close = maker
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = escrow.mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Take<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        if amount != self.escrow.receive {
            msg!("Wrong amount provided: {}, expected: {}", amount, self.escrow.receive);
            return Err(EscrowError::InvalidAmount.into());
        }
        let accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            authority: self.taker.to_account_info()
        };
        let ctx = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer_checked(ctx, amount, self.mint_b.decimals)
    }

    pub fn receive(&mut self, seed: u64) -> Result<()> {
        if self.maker_ata_b.amount < self.escrow.receive {
            msg!("Maker has yet to receive agreed upon amount");
            return Err(EscrowError::InsufficientTokensReceived.into());
        }

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            mint: self.mint_a.to_account_info(),
            authority: self.escrow.to_account_info()
        };
        
        let maker = self.maker.key();
        let seed_bytes = seed.to_le_bytes();
        let signer_seeds: &[&[&[u8]]] = &[&[b"escrow", maker.as_ref(), seed_bytes.as_ref(), &[self.escrow.bump]]];
        let ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);

        transfer_checked(ctx, self.vault.amount, self.mint_a.decimals)
    }
}   
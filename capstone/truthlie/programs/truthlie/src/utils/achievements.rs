use anchor_lang::prelude::*;
use anchor_lang::{AnchorDeserialize, AnchorSerialize};

#[derive(Clone, Copy, PartialEq, AnchorDeserialize, AnchorSerialize, InitSpace)]
pub enum Achievement {
    Believer,
    Unbeliever,
    Minimalist,
    ExistentialCrisis,
    SeasonedTrickster,
    Observer
}
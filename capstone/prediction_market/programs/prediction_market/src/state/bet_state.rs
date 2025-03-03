use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace, Debug)]
pub enum Bet {
    Positive,
    Negative,
}

#[account]
#[derive(InitSpace)]
pub struct BetState {
    pub owner: Pubkey,
    pub market: Pubkey,
    pub bet: Bet,
    pub amount: u64,
    pub bump: u8,
}

impl BetState {
    pub const SEED: &'static [u8] = b"bet";
}

use crate::state::bet_state::Bet;
use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct MarketState {
    pub creator: Pubkey,
    #[max_len(120)]
    pub question: String,
    pub expires_at: i64,
    pub yes_amount: u64,
    pub no_amount: u64,
    pub positive_bets: u64,
    pub negative_bets: u64,
    pub outcome: Option<Bet>,
    pub bump: u8,
}

impl MarketState {
    pub const SEED: &'static [u8] = b"market";
    pub const QUESTION_MAX_LEN: usize = 120;
}

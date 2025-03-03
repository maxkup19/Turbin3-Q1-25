use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)]
pub struct EscrowState {
    pub seed: u64,
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amount: u64,
    pub bump: u8,
}

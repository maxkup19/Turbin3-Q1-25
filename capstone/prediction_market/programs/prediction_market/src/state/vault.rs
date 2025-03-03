use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct Vault {
    pub authority: Pubkey,
    pub bump: u8,
}

impl Vault {
    pub const SEED: &'static [u8] = b"vault";
}

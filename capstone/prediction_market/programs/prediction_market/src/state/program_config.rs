use anchor_lang::prelude::*;

#[account]
#[derive(Default, InitSpace)]
pub struct ProgramConfig {
    pub authority: Pubkey,
    pub bump: u8,
}

impl ProgramConfig {
    pub const SEED: &'static [u8] = b"config";
}

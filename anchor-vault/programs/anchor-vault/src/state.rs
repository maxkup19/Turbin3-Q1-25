use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[account]
#[derive(InitSpace, Debug)]
pub struct VaultState {
    pub vault_bump: u8,
    pub state_bump: u8,
}

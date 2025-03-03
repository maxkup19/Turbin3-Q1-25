use anchor_lang::prelude::*;

declare_id!("9994KSXhz2KwD7sfejdobQNXXHo8uGLBsgMc5wVD1PBa");

mod context;
mod error;
mod state;

use context::*;

#[program]
pub mod anchor_marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String, fee: u16) -> Result<()> {
        ctx.accounts.init(name, fee, &ctx.bumps)
    }

    pub fn list(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.create_listing(price, &ctx.bumps)?;
        ctx.accounts.deposit_nft()
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.withdraw_nft()?;
        ctx.accounts.close_listing()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.pay()?;
        ctx.accounts.transfer_nft()?;
        ctx.accounts.close_vault_account()?;
        ctx.accounts.reward_buyer()

    }
}

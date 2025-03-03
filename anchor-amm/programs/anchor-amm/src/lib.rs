use anchor_lang::prelude::*;

pub mod contexts;
pub mod error;
pub mod state;

pub use contexts::*;

declare_id!("EhKKYBrEHPkvXaqx7uWFm3dm4o4Ts8rFy2oiVPwipGbq");

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.init(seed, fee, authority, &ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, lp_amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(lp_amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, lp_amount: u64, min_x: u64, min_y: u64) -> Result<()> {
        ctx.accounts.withdraw(lp_amount, min_x, min_y)
    }

    pub fn swap(ctx: Context<Swap>, args: SwapArgs) -> Result<()> {
        ctx.accounts.swap(args)
    }
}

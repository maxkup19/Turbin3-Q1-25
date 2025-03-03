pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;

declare_id!("547ruAjAwphPUYjqLFVLTBsrPwJ89ezJCPwau3DQNdRE");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.make(seed, receive_amount, &ctx.bumps)?;
        ctx.accounts.deposit(receive_amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund()?;
        ctx.accounts.close()
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.release()?;
        ctx.accounts.close()
    }
}

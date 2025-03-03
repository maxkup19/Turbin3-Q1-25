mod context;
mod error;
mod state;
mod utility;

use anchor_lang::prelude::*;
use context::*;
use state::bet_state::Bet;
use state::*;

declare_id!("63aLZdtnc6ShpwYR2jEzEh78cjKUF1Tn2LK9DZhTnu4R");

#[program]
pub mod prediction_market {
    use super::*;

    pub fn init_program_config(ctx: Context<InitConfig>) -> Result<()> {
        msg!("Initializing program config");
        ctx.accounts.init_config(&ctx.bumps)
    }

    pub fn init_market(
        ctx: Context<InitMarket>,
        question: String,
        expires_at: i64,
        question_hash: Vec<u8>,
    ) -> Result<()> {
        msg!("Initializing market with question: {}", question);
        msg!("Expires at: {}", expires_at);

        ctx.accounts
            .init_market(question, expires_at, question_hash, &ctx.bumps)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, bet: Bet, amount: u64) -> Result<()> {
        msg!("Place bet");
        ctx.accounts.place_bet(bet, amount, &ctx.bumps)
    }

    pub fn resolve_market(ctx: Context<ResolveMarket>, winner: Bet) -> Result<()> {
        msg!("Resolving market");
        ctx.accounts.resolve_market(winner)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        msg!("Withdrawing");
        ctx.accounts.withdraw()
    }
}

use crate::error::MarketError;
use crate::state::bet_state::Bet;
use crate::state::market_state::MarketState;
use crate::state::program_config::ProgramConfig;
use crate::state::vault::Vault;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ResolveMarket<'info> {
    #[account(
        mut,
        address = program_config.authority,
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub market_state: Account<'info, MarketState>,

    #[account(
        mut,
        seeds = [ProgramConfig::SEED],
        bump = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> ResolveMarket<'info> {
    pub fn resolve_market(&mut self, winner: Bet) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            self.market_state.expires_at < current_time,
            MarketError::MarketNotExpired
        );

        require!(self.market_state.outcome.is_none(), MarketError::MarketAlreadyResolved);

        self.market_state.outcome = Some(winner);

        Ok(())
    }
}

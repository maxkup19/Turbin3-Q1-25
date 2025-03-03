use anchor_lang::prelude::*;

use crate::error::MarketError;
use crate::state::bet_state::{Bet, BetState};
use crate::state::market_state::MarketState;
use crate::state::program_config::ProgramConfig;
use crate::state::vault::{self, Vault};
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub market_state: Account<'info, MarketState>,

    #[account(
        mut,
        seeds = [BetState::SEED, user.key().as_ref(), market_state.key().as_ref()],
        bump = bet_state.bump,
        close = user
    )]
    pub bet_state: Account<'info, BetState>,

    #[account(
        mut,
        seeds = [Vault::SEED, program_config.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        mut,
        seeds = [ProgramConfig::SEED],
        bump = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        require!(
            self.market_state.outcome.is_some(),
            MarketError::MarketNotResolved
        );

        let winner = self.market_state.outcome.unwrap() == self.bet_state.bet;

        if winner {
            let total_amount = self
                .market_state
                .yes_amount
                .checked_add(self.market_state.no_amount)
                .ok_or(MarketError::MathOverflow)?;
            let user_share = if self.market_state.outcome.unwrap() == Bet::Positive {
                self.bet_state.amount as f64 / self.market_state.yes_amount as f64
            } else {
                self.bet_state.amount as f64 / self.market_state.no_amount as f64
            };

            let profit = (user_share * total_amount as f64) as u64 - self.bet_state.amount;
            let winner_amount = self.bet_state.amount + if profit > 0 { profit * 95 / 100 } else { 0 };


            **self.vault.to_account_info().try_borrow_mut_lamports()? = self
                 .vault
                 .to_account_info()
                 .lamports()
                 .checked_sub(winner_amount)
                 .ok_or(MarketError::MathOverflow)?;

            **self.user.to_account_info().try_borrow_mut_lamports()? = self
                .user
                .to_account_info()
                .lamports()
                .checked_add(winner_amount)
                 .ok_or(MarketError::MathOverflow)?;

             msg!("Transferred {} lamports from vault to user", winner_amount);
        }
        Ok(())
    }
}

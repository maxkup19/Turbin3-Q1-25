use crate::error::MarketError;
use crate::state::bet_state::{Bet, BetState};
use crate::state::market_state::MarketState;
use crate::state::program_config::ProgramConfig;
use crate::state::vault::Vault;
use crate::utility::ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(mut)]
    pub market_state: Account<'info, MarketState>,

    #[account(
        init,
        payer = signer,
        space = ACCOUNT_DISCRIMINATOR + BetState::INIT_SPACE,
        seeds = [BetState::SEED, signer.key().as_ref(), market_state.key().as_ref()],
        bump
    )]
    pub bet_state: Account<'info, BetState>,

    #[account(
        seeds = [ProgramConfig::SEED],
        bump = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    #[account(
        mut,
        seeds = [Vault::SEED, program_config.key().as_ref()],
        bump = vault.bump,
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn place_bet(&mut self, bet: Bet, amount: u64, bumps: &PlaceBetBumps) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        require!(
            current_time < self.market_state.expires_at,
            MarketError::MarketExpired
        );

        self.bet_state.set_inner(BetState {
            owner: self.signer.key(),
            market: self.market_state.key(),
            bet,
            amount,
            bump: bumps.bet_state,
        });
        match bet {
            Bet::Positive => {
                self.market_state.yes_amount = self
                    .market_state
                    .yes_amount
                    .checked_add(amount)
                    .ok_or(MarketError::MathOverflow)?;

                self.market_state.positive_bets = self
                    .market_state
                    .positive_bets
                    .checked_add(1)
                    .ok_or(MarketError::MathOverflow)?;
            }
            Bet::Negative => {
                self.market_state.no_amount = self
                    .market_state
                    .no_amount
                    .checked_add(amount)
                    .ok_or(MarketError::MathOverflow)?;

                self.market_state.positive_bets = self
                    .market_state
                    .positive_bets
                    .checked_add(1)
                    .ok_or(MarketError::MathOverflow)?;
            }
        }

        let transfer_instruction = Transfer {
            from: self.signer.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), transfer_instruction);
        transfer(cpi_ctx, amount)
    }
}

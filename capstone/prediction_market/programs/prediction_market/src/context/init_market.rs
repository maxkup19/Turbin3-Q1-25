use crate::error::MarketError;
use crate::state::market_state::*;
use crate::state::program_config::ProgramConfig;
use crate::utility::ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;
use solana_program::hash::hash;

#[derive(Accounts)]
#[instruction(question: String, expires_at: i64, question_hash: Vec<u8>)]
pub struct InitMarket<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = ACCOUNT_DISCRIMINATOR + MarketState::INIT_SPACE,
        seeds = [ MarketState::SEED, creator.key().as_ref(), &question_hash],
        bump
    )]
    pub market_state: Account<'info, MarketState>,

    #[account(
        seeds = [ProgramConfig::SEED],
        bump = program_config.bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitMarket<'info> {
    pub fn init_market(
        &mut self,
        question: String,
        expires_at: i64,
        question_hash: Vec<u8>,
        bumps: &InitMarketBumps,
    ) -> Result<()> {
        // Calculate the expected hash from the question
        let expected_hash = hash(question.as_bytes()).to_bytes();

        // Verify the seed matches what we expect
        require!(question_hash == expected_hash, MarketError::InvalidSeed);

        require!(
            !question.is_empty() && question.len() < MarketState::QUESTION_MAX_LEN,
            MarketError::InvalidQuestionLength
        );

        let current_timestamp = Clock::get()?.unix_timestamp;
        require!(
            expires_at > current_timestamp,
            MarketError::InvalidExpireDate
        );

        self.market_state.set_inner(MarketState {
            creator: *self.creator.key,
            question,
            expires_at,
            yes_amount: 0,
            no_amount: 0,
            positive_bets: 0,
            negative_bets: 0,
            outcome: None,
            bump: bumps.market_state,
        });

        Ok(())
    }
}

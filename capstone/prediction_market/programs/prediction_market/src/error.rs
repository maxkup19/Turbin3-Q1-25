use anchor_lang::error_code;

#[error_code]
pub enum MarketError {
    #[msg("Invalid question length")]
    InvalidQuestionLength,
    #[msg("Math overflow error")]
    MathOverflow,
    #[msg("Invalid expire date. Expire date should be in future")]
    InvalidExpireDate,
    #[msg("Invalid seed provided")]
    InvalidSeed,
    #[msg("Market expired")]
    MarketExpired,
    #[msg("Market not expired")]
    MarketNotExpired,
    #[msg("Market not resolved")]
    MarketNotResolved,
    #[msg("Market already resolved")]
    MarketAlreadyResolved,
}

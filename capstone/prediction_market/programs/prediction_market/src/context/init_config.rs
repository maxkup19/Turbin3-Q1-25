use crate::state::program_config::ProgramConfig;
use crate::state::vault::Vault;
use crate::utility::ACCOUNT_DISCRIMINATOR;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitConfig<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = ACCOUNT_DISCRIMINATOR + ProgramConfig::INIT_SPACE,
        seeds = [ProgramConfig::SEED],
        bump,
    )]
    pub program_config: Account<'info, ProgramConfig>,

    #[account(
        init,
        payer = authority,
        space = ACCOUNT_DISCRIMINATOR + Vault::INIT_SPACE,
        seeds = [Vault::SEED, program_config.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitConfig<'info> {
    pub fn init_config(&mut self, bumps: &InitConfigBumps) -> Result<()> {
        self.program_config.set_inner(ProgramConfig {
            authority: self.authority.key(),
            bump: bumps.program_config,
        });

        self.vault.set_inner(Vault {
            authority: self.authority.key(),
            bump: bumps.vault,
        });

        Ok(())
    }
}

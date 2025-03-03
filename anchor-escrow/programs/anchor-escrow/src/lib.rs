pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;

declare_id!("547ruAjAwphPUYjqLFVLTBsrPwJ89ezJCPwau3DQNdRE");

#[program]
pub mod anchor_escrow {
    use super::*;

    // TODO: Implment calling the instructions
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

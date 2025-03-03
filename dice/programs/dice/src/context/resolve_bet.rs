use anchor_instruction_sysvar::Ed25519InstructionSignatures;
use anchor_lang::prelude::*;

use anchor_lang::system_program::{transfer, Transfer};
use solana_program::{
    ed25519_program, hash::hash, sysvar::instructions::load_instruction_at_checked,
};

use crate::error::DiceError;
use crate::state::Bet;

#[derive(Accounts)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    pub house: Signer<'info>,

    #[account(mut)]
    pub player: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"vault", house.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub bet: Account<'info, Bet>,
    #[account(
        address = solana_program::sysvar::instructions::ID
    )]
    pub instructions_sysvar: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        let ix = load_instruction_at_checked(0, &self.instructions_sysvar.to_account_info())?;

        require_keys_eq!(
            ix.program_id,
            ed25519_program::ID,
            DiceError::InvalidProgramID
        );

        require_eq!(ix.accounts.len(), 0, DiceError::InvalidAccountCount);

        let signatures = Ed25519InstructionSignatures::unpack(&ix.data)?.0;

        require_eq!(signatures.len(), 1, DiceError::CustomError);

        let signature = &signatures[0];

        require!(signature.is_verifiable, DiceError::UnverifiableSignature);

        require!(
            signature.public_key == Some(self.house.key()),
            DiceError::UnverifiableSignature
        );

        require!(
            signature.signature.unwrap() == sig,
            DiceError::UnverifiableSignature
        );

        require!(
            signature.message.as_ref().unwrap().eq(&self.bet.to_slice()),
            DiceError::CustomError
        );

        Ok(())
    }

    pub fn resolve_bet(&mut self, sig: &[u8], bumps: &ResolveBetBumps) -> Result<()> {
        // in order to make working the sig less painful, we hash it to get a 32 byte array
        // think of the hash as a public in the case here (basically to make life easier)
        let hash = hash(sig).to_bytes();

        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);

        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll > roll {
            let payout = (self.bet.amount as u128)
                .checked_mul(10000 - 150 as u128)
                .unwrap()
                .checked_div(self.bet.roll as u128 - 1)
                .unwrap()
                .checked_div(100)
                .unwrap() as u64;

            let cpi_program = self.system_program.to_account_info();

            let cpi_accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };

            let seeds = [
                b"vault",
                self.house.to_account_info().key.as_ref(),
                &[bumps.vault],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

            transfer(cpi_ctx, payout)?;
        }

        Ok(())
    }
}

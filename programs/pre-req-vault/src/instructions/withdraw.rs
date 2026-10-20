use crate::state::VaultState;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

declare_program!(registration);

/// GitHub handle registered with the Turbin3 application account.
const GITHUB_ID: &str = "Swinecoder101";

use registration::cpi::{accounts::Initialize, initialize};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    mut,
    seeds = [b"vault", vault_state.key().as_ref()],
    bump = vault_state.vault_bump,
  )]
    pub vault: SystemAccount<'info>,

    #[account(
    seeds = [b"state", user.key().as_ref()],
    bump = vault_state.state_bump
  )]
    pub vault_state: Account<'info, VaultState>,

    /// CHECK: application account will be initialized by the cpi call to the application program
    #[account(
    mut,
    seeds = [b"prereqs", user.key().as_ref()],
    seeds::program = application_program.key(),
    bump
    )]
    pub application_account: UncheckedAccount<'info>,

    pub application_program: Program<'info, registration::program::Q3PreReqsRs>,

    system_program: Program<'info, System>,
}

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.user.to_account_info(),
        };

        let seeds = &[
            b"vault",
            self.vault_state.to_account_info().key.as_ref(),
            &[self.vault_state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(System::id(), cpi_accounts, signer_seeds);

        transfer(cpi_ctx, amount)?;

        // CPI to the Turbin3 registration program to initialize the application account.
        // `user` is a signer on the outer instruction, so its signature is forwarded to the CPI
        // automatically; the registration program signs for the `prereqs` PDA itself on `init`.
        let cpi_accounts = Initialize {
            user: self.user.to_account_info(),
            account: self.application_account.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.application_program.key(), cpi_accounts);

        initialize(cpi_ctx, GITHUB_ID.to_string())?;

        Ok(())
    }
}

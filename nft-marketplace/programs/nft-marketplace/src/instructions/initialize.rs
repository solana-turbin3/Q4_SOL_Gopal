use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::{error::Errors, Marketplace};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace"],
        space = Marketplace::INIT_SPACE,
        bump
    )]
    pub marketplace: Account<'info, Marketplace>,
    pub system_program: Program<'info, System>,

    #[account(
        seeds = [b"treasury", marketplace.key().as_ref()],
        bump,
    )]
    pub treasury: SystemAccount<'info>,

    #[account(
        init, payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump, mint::decimals = 6, mint::authority = treasury,
    )]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        require!(name.len() > 0 && name.len() < 4 + 33, Errors::InvalidName);

        self.marketplace.set_inner(Marketplace {
            amdin: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
            reward_bump: bumps.rewards_mint,
            name,
        });

        Ok(())
    }
}

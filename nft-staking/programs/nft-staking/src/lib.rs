pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BL3AaJBavZShHGXaoJFf3Q25WAUjboNaZamFoTfKnzRE");

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_config(
        ctx: Context<InitStakeConfigs>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init(&ctx.bumps, points_per_stake, max_stake, freeze_period)
    }

    pub fn init_acocunt(ctx: Context<InitUserAccount>) -> Result<()> {
        ctx.accounts.init_account(&ctx.bumps)
    }

    pub fn init_stake_account(ctx: Context<InitStakeAccount>) -> Result<()> {
        ctx.accounts.init_stake(&ctx.bumps)
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}

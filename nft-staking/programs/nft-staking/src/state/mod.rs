use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub staked_at: i64,
    pub mint: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct StakeConfig {
    pub max_stake: u8,
    pub freeze_period: u32,
    pub bump: u8,
    pub points_per_stake: u8,
    pub reward_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub point: u32,
    pub amount_staked: u8,
    pub bump: u8,
}

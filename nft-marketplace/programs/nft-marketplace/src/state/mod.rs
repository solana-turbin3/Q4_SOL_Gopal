use anchor_lang::prelude::*;

#[account]
pub struct Marketplace {
    pub amdin: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
    pub reward_bump: u8,
    pub name: String,
}

impl Space for Marketplace {
    const INIT_SPACE: usize = 8 + 32 + 2 + 3 * 1 + (4 + 32);
}

#[account]
pub struct Listing {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Listing {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}

#[account]
pub struct Offer {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Offer {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}

#[account]
pub struct Reward {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Reward {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}

#[account]
pub struct Treasury {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for Treasury {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}

use anchor_lang::prelude::*;
use mpl_core::types::OracleValidation;

// Setup State
#[account]
pub struct Protocol {
    pub validation: OracleValidation,
    pub bump: u8,
}

impl Space for Protocol {
    const INIT_SPACE: usize = 8 + 5 + 1;
}

#[account]
pub struct Manager {
    pub bump: u8,
}

impl Space for Manager {
    const INIT_SPACE: usize = 8 + 1;
}

#[account]
pub struct AdminProfile {
    pub address: Pubkey,
    pub username: String,
    pub creation_time: i64,
    pub bump: u8,
}

impl Space for AdminProfile {
    const INIT_SPACE: usize = 8 + 32 + 4 +  8 + 1;
}

#[account]
pub struct Profile {
    pub username: String,
    pub spending: u64,
    pub membership: Membership,
    pub is_verified: bool,
    pub bump: u8,
}

impl Space for Profile {
    const INIT_SPACE: usize = 8 + 4 + 8 + Membership::INIT_SPACE + 1 + 1 + 1;
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum Membership {
    Platinum, // 0
    Gold, // 1
    Basic, // 2
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub enum ObjectType {
    Apartment, // 0
    House, // 1
    Land, // 2
    Commercial, // 3
}

#[account]
pub struct FractionalizedCompletedListing {
    pub id: u64, // 8
    pub object_type: ObjectType, // 2
    pub object: Pubkey, // 32 -- offset 18
    pub share: u16,
    pub price: u64,
    pub bump: u8,
}

impl Space for FractionalizedCompletedListing {
    const INIT_SPACE: usize = 8 + 8 + ObjectType::INIT_SPACE + 32 + 2 + 8 + 1;
}
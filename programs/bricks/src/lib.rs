use anchor_lang::prelude::*;

pub mod state;
pub mod errors;
pub mod constant;

pub mod instructions;
pub use instructions::*;

declare_id!("DdoHZCPsMoQmRu7LtkvrVfu6TDR9PYJafKxps5cuh4uU");

#[program]
pub mod bricks {
    use super::*;

    // Setup Config
    pub fn initialize_protocol(ctx: Context<ProtocolInit>) -> Result<()> {
        instructions::initialize_protocol::handler(ctx)
    }

    pub fn update_protocol(ctx: Context<UpdateProtocol>) -> Result<()> {
        instructions::update_protocol::handler(ctx)
    }

    pub fn initialize_admin(ctx: Context<AdminInit>, args: AdminInitArgs) -> Result<()> {
        instructions::initialize_admin::handler(ctx, args)
    }

    pub fn claim_fractionalized_listing_revenue(ctx: Context<ClaimFractionalizedListingRevenue>) -> Result<()> {
        instructions::claim_fractionalized_listing_revenue::handler(ctx)
    }

    // // Profile Config
    pub fn initialize_profile(ctx: Context<ProfileInit>, args: ProfileInitArgs) -> Result<()> {
        instructions::initialize_profile::handler(ctx, args)
    }

    // // Objects Initialization
    // pub fn create_object(ctx: Context<CreateObject>, args: CreateObjectArgs) -> Result<()> {
    //     instructions::create_object::handler(ctx, args)
    // }
    
    // // Actions
    // pub fn create_fractionalized_listing(ctx: Context<CreateFractionalizedListing>, args: CreateFractionalizedListingArgs) -> Result<()> {
    //     instructions::create_fractionalized_listing::handler(ctx, args)
    // }

    // pub fn buy_fractionalized_listing(ctx: Context<BuyFractionalizedListing>, uri: String) -> Result<()> {
    //     instructions::buy_fractionalized_listing::handler(ctx, uri)
    // }

}
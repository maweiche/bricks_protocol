use anchor_lang::prelude::*;
use mpl_core::accounts::BaseCollectionV1;
use crate::{state::{AdminProfile, Manager, FractionalizedListing, ObjectType}, errors::SetupError};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateFractionalizedListingArgs {
    id: u64,
    object_type: u8,
    share: u16,
    price: u64,
    starting_time: i64,
}

#[derive(Accounts)]
#[instruction(args: CreateFractionalizedListingArgs)]
pub struct CreateFractionalizedListing<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        seeds = [b"admin", admin.key().as_ref()],
        bump
    )]
    pub admin_profile: Account<'info, AdminProfile>,
    #[account(
        seeds = [b"manager"],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,
    
    #[account(constraint = object.update_authority == manager.key())] 
    pub object: Account<'info, BaseCollectionV1>,
    #[account(
        init,
        payer = admin,
        space = FractionalizedListing::INIT_SPACE,
        seeds = [b"listing", args.id.to_le_bytes().as_ref()],
        bump,
    )] 
    pub listing: Account<'info, FractionalizedListing>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateFractionalizedListing<'info> {
    pub fn create_listing(&mut self, id: u64, object_type: ObjectType, share: u16, price: u64, starting_time: i64, bump: u8) -> Result<()> {

        self.listing.set_inner(
            FractionalizedListing {
                id,
                object_type,
                object: self.object.key(),
                share,
                share_sold: 0, 
                price,
                starting_time,
                bump,
            }
        );

       Ok(())
    }
}

pub fn handler(ctx: Context<CreateFractionalizedListing>, args: CreateFractionalizedListingArgs) -> Result<()> {
    let bump = ctx.bumps.listing;

    let object_type = match args.object_type {
        0 => ObjectType::Apartment,
        1 => ObjectType::House,
        2 => ObjectType::Land,
        3 => ObjectType::Commercial,
        _ => return Err(SetupError::InvalidObjectType.into()),
    };

    ctx.accounts.create_listing(args.id, object_type, args.share, args.price, args.starting_time, bump)?;

    Ok(())
}
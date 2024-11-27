use anchor_lang::{
    prelude::*, 
    solana_program::{
        program_memory::sol_memcpy,
        sysvar::instructions::{
            self,
            load_current_index_checked,
            load_instruction_at_checked
        }
    }
};

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Mint, TokenAccount, Token, transfer, Transfer}
};

use mpl_core::{
    ID as MPL_CORE_PROGRAM_ID,
    accounts::BaseCollectionV1,
    instructions::CreateV1CpiBuilder
};
use std::str::FromStr;
use crate::{
    state::{FractionalizedListing, Profile, Manager, FractionalizedCompletedListing, Membership, Protocol},
    constant::{
        // protocol_currency, 
        signing_authority, 
        ED25519_PROGRAM_ID
    },
    errors::BuyingError
};

#[derive(Accounts)]
pub struct BuyFractionalizedListing<'info> {
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,

    // #[account(address = protocol_currency::id())]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = buyer,
    )]
    pub buyer_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = listing,
    )]
    pub listing_ata: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"manager"],
        bump = manager.bump,
    )]
    pub manager: Account<'info, Manager>,
    #[account(
        mut,
        seeds = [b"profile", buyer.key().as_ref()],
        bump
    )]
    pub buyer_profile: Account<'info, Profile>,
    #[account(
        mut,
        seeds = [b"listing", listing.id.to_le_bytes().as_ref()],
        bump,
    )] 
    pub listing: Account<'info, FractionalizedListing>,

    #[account(
        mut,
        constraint = object.update_authority == manager.key()
    )] 
    pub object: Account<'info, BaseCollectionV1>,
    #[account(mut)] 
    pub fraction: Signer<'info>,
    #[account(
        seeds = [b"protocol"],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(address = instructions::ID)]
    /// CHECK: InstructionsSysvar account
    instructions: UncheckedAccount<'info>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This account will be checked by the constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyFractionalizedListing<'info> {
    pub fn create_fraction(&mut self, uri: String) -> Result<()> {
        // create seeds to use later on for the CPI calls
        let signer_seeds: &[&[u8]; 2] = &[b"manager", &[self.manager.bump]];

        CreateV1CpiBuilder::new(&self.mpl_core_program.to_account_info())
        .asset(&self.fraction.to_account_info())
        .collection(Some(&self.object.to_account_info()))
        .authority(Some(&self.manager.to_account_info()))
        .payer(&self.payer.to_account_info())
        .owner(Some(&self.buyer.to_account_info()))
        .system_program(&self.system_program.to_account_info())
        .name(format!("{} - {}", self.object.name, self.listing.id))
        .uri(uri)
        .add_remaining_account(&self.protocol.to_account_info(), false, false)
        .invoke_signed(&[signer_seeds])?;

        if self.listing.share_sold + 1 == self.listing.share {
            let info = self.listing.to_account_info(); 
            let mut data = info.try_borrow_mut_data()?;

            // Transform to CompletedListing
            let completed_listing = FractionalizedCompletedListing {
                id: self.listing.id,
                object_type: self.listing.object_type.clone(),
                object: self.listing.object,
                share: self.listing.share,
                price: self.listing.price,
                bump: self.listing.bump,
            };

            // Serialize
            let mut writer: Vec<u8> = vec![];
            completed_listing.try_serialize(&mut writer)?;
            writer.truncate(FractionalizedCompletedListing::INIT_SPACE);

            sol_memcpy(&mut data, &writer, writer.len());
        } else {
            self.listing.share_sold += 1;
        }

        Ok(())
    }

    pub fn pay_fraction(&mut self) -> Result<()> {
        let membership = match self.buyer_profile.membership {
            Membership::Platinum => 2,
            Membership::Gold => 1,
            Membership::Basic => 0,
        };

        require!(
            self.listing.starting_time < Clock::get()?.unix_timestamp
                || (self.listing.starting_time < Clock::get()?.unix_timestamp - 6 * 3600 && membership == 1)
                || (self.listing.starting_time < Clock::get()?.unix_timestamp - 24 * 3600 && membership == 2),
            BuyingError::NotTimeYet
        );

        transfer(
            CpiContext::new(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.buyer_ata.to_account_info(),
                    to: self.listing_ata.to_account_info(),
                    authority: self.buyer.to_account_info(),
                }
            ),
            self.listing.price * 10u64.pow(self.mint.decimals as u32),
        )?;

        Ok(())
    }

    pub fn stripe_payment(&mut self, current_index: usize) -> Result<()> {
        let ixs = self.instructions.to_account_info();

        if let Ok(signature_ix) = load_instruction_at_checked(current_index - 1, &ixs) {
            if Pubkey::from_str(ED25519_PROGRAM_ID).unwrap() == signature_ix.program_id {
                require!(
                    signing_authority::ID.to_bytes().eq(&signature_ix.data[16..48]),
                    BuyingError::SignatureAuthorityMismatch
                );

                let mut message_data: [u8; 4] = [0u8; 4];
                message_data.copy_from_slice(&signature_ix.data[112..116]);
                let amount = u32::from_le_bytes(message_data);

                msg!("The message from Signature instruction is: {}", amount);

                let amount_paid = amount as u64;

                require!(
                    amount_paid <= self.listing.price * 10u64.pow(self.mint.decimals as u32),
                    BuyingError::PriceMismatch
                );
            } else {
                return Err(BuyingError::InvalidInstruction.into());
            }
        } else {
            return Err(BuyingError::InvalidInstruction.into());
        }

        Ok(())
    }
    
}

pub fn handler(ctx: Context<BuyFractionalizedListing>, uri: String) -> Result<()> {
    // Profile Checks
    require!(
        ctx.accounts.buyer_profile.is_verified || ctx.accounts.buyer_profile.spending < 500,
        BuyingError::NotVerified
    );

    ctx.accounts.create_fraction(uri)?;

    // Instruction Check
    let ixs = ctx.accounts.instructions.to_account_info();
    let current_index = load_current_index_checked(&ixs)? as usize;

    // If the current index is 0, then the buyer must pay the fraction via the listing currency, else it's a stripe payment
    match current_index {
        0 => ctx.accounts.pay_fraction()?,
        _ => ctx.accounts.stripe_payment(current_index)?
    }
   
   
    // match ctx.accounts.stripe_payment(current_index) {
    //     Ok(_) => {}
    //     Err(_) => ctx.accounts.pay_fraction()?
    // }
                    
    Ok(())
}
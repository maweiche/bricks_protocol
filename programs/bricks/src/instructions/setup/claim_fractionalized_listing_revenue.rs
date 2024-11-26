use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken, 
    token::{Mint, TokenAccount, Token, transfer, Transfer, close_account, CloseAccount}
};

use crate::{
    state::FractionalizedCompletedListing,
    // constant::{
        // protocol_currency, 
        // admin_wallet::ID as ADMIN_WALLET,
    // },
};

#[derive(Accounts)]
pub struct ClaimFractionalizedListingRevenue<'info> {
    #[account(
        mut,
        // address = ADMIN_WALLET,
    )]
    pub owner: Signer<'info>,

    // #[account(address = protocol_currency::id())]
    pub mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    pub owner_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = listing,
    )]
    pub listing_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"listing", listing.id.to_le_bytes().as_ref()],
        bump,
    )] 
    pub listing: Account<'info, FractionalizedCompletedListing>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimFractionalizedListingRevenue<'info> {
    pub fn withdraw_fractionalized_listing_revenue(&self) -> Result<()> {
        
        let amount = self.listing_ata.amount;

        let listing_id = self.listing.id.to_le_bytes();
        let signer_seeds = &[b"listing", listing_id.as_ref(), &[self.listing.bump]];

        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.listing_ata.to_account_info(),
                    to: self.owner_ata.to_account_info(),
                    authority: self.listing.to_account_info(),
                },
                &[signer_seeds],
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn close_listing_token_account(&mut self) -> Result<()> {

        let listing_id = self.listing.id.to_le_bytes();
        let signer_seeds = &[b"listing", listing_id.as_ref(), &[self.listing.bump]];
        
        close_account(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                CloseAccount {
                    account: self.listing_ata.to_account_info(),
                    destination: self.owner.to_account_info(),
                    authority: self.owner.to_account_info(),
                },
                &[signer_seeds],
            )
        )?;

        Ok(())
    }
    
}

pub fn handler(ctx: Context<ClaimFractionalizedListingRevenue>) -> Result<()> {
    
    ctx.accounts.withdraw_fractionalized_listing_revenue()?;

    ctx.accounts.close_listing_token_account()?;
    
    Ok(())
}
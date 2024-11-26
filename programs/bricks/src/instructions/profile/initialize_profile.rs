use anchor_lang::prelude::*;
use crate::state::{Profile, Membership};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ProfileInitArgs {
    username: String,
}

#[derive(Accounts)]
#[instruction(args: ProfileInitArgs)]
pub struct ProfileInit<'info> {
    /// CHECK: This account will be created on the user's behalf
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = Profile::INIT_SPACE + args.username.len(),
        seeds = [b"profile", user.key().as_ref()],
        bump
    )]
    pub profile: Account<'info, Profile>,
    pub system_program: Program<'info, System>,
}

impl<'info> ProfileInit<'info> {
    pub fn initialize_profile(&mut self, username: String, bump: u8) -> Result<()> {

        self.profile.set_inner(
            Profile {
                username,
                spending: 0,
                membership: Membership::Basic,
                is_verified: false,
                bump,
            }
        );

        Ok(())
    }
}

pub fn handler(ctx: Context<ProfileInit>, args: ProfileInitArgs) -> Result<()> {
    // Generate the bumps
    let bump = ctx.bumps.profile;

    // Initialize the new profile
    ctx.accounts.initialize_profile(args.username,  bump)?;

    Ok(())
}


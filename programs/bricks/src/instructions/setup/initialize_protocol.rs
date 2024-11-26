use anchor_lang::prelude::*;
use crate::state::{Protocol, Manager};
use mpl_core::types::{OracleValidation, ExternalValidationResult};

#[derive(Accounts)]
pub struct ProtocolInit<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = Protocol::INIT_SPACE,
        seeds = [b"protocol"],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        init,
        payer = owner,
        space = Manager::INIT_SPACE,
        seeds = [b"manager"],
        bump,
    )]
    pub manager: Account<'info, Manager>,
    pub system_program: Program<'info, System>,
}

/*
    
    Initialize Protocol Ix:

    Some security check:
    - Check if the account that is interacting with this instruction is the mutlisig account 
    of the team that is the highest security clearance for the enitre protocol.

    What these Instructions do:
    - Initialize the Protocol account with the new settings.

*/

impl<'info> ProtocolInit<'info> {
    pub fn initialize_protocol(&mut self, bump: u8) -> Result<()> { 
        self.protocol.set_inner(
            Protocol {
                validation: OracleValidation::V1 {
                    create: ExternalValidationResult::Approved,
                    transfer: ExternalValidationResult::Approved,
                    burn: ExternalValidationResult::Approved,
                    update: ExternalValidationResult::Approved,
                },
                bump,
            }
        );

        Ok(())
    }

    pub fn initialize_manager(&mut self, bump: u8) -> Result<()> {
        self.manager.set_inner(
            Manager {
                bump,
            }
        );

        Ok(())
    }
}

pub fn handler(ctx: Context<ProtocolInit>) -> Result<()> {
    // Make sure it's the admin of the protocol that is initializing the new admin and that the new admin is not the admin of the protocol
    // require!(ctx.accounts.owner.key() == ADMIN::id(), SetupError::Unauthorized);

    // Generate the bumps
    let bumps = ctx.bumps;

    // Initialize the new protocol
    ctx.accounts.initialize_protocol(bumps.protocol)?;

    // Initialize the new manager
    ctx.accounts.initialize_manager(bumps.manager)?;

    Ok(())
}

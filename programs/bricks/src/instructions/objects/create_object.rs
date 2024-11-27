use anchor_lang::prelude::*;
use mpl_core::{
    instructions::CreateCollectionV2CpiBuilder, 
    types::{
        Attribute, Attributes, Creator, ExternalCheckResult, ExternalPluginAdapterInitInfo, HookableLifecycleEvent, OracleInitInfo, PermanentBurnDelegate, PermanentFreezeDelegate, PermanentTransferDelegate, Plugin, PluginAuthority, PluginAuthorityPair, Royalties, RuleSet, ValidationResultsOffset
    }, 
    ID as MPL_CORE_PROGRAM_ID
};

use crate::state::{AdminProfile, Manager, Protocol};

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateObjectArgs {
    pub name: String,
    pub uri: String,
    pub reference: String,
    pub attributes: Vec<Attributes>,
}

#[derive(Accounts)]
#[instruction(args: CreateObjectArgs)]
pub struct CreateObject<'info> {
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
    #[account(
        seeds = [b"protocol"],
        bump = protocol.bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        mut,
        seeds = [b"object", args.reference.as_bytes().as_ref()],
        bump
    )] 
    /// CHECK: This account will be checked by the constraint
    pub object: UncheckedAccount<'info>,

    #[account(address = MPL_CORE_PROGRAM_ID)]
    /// CHECK: This account will be checked by the constraint
    pub mpl_core_program: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateObject<'info> {
    pub fn create_object(&mut self, args: CreateObjectArgs, bump: u8) -> Result<()> {
        let reference = args.reference.clone();
        
        // Add an Attribute Plugin that will hold the object details
        let mut collection_plugin: Vec<PluginAuthorityPair> = vec![];

        let attribute_list: Vec<Attribute> = args.attributes
            .into_iter()
            .flat_map(|attributes| attributes.attribute_list)
            .collect();

        collection_plugin.push(PluginAuthorityPair { plugin: Plugin::Attributes(Attributes { attribute_list }), authority: Some(PluginAuthority::UpdateAuthority) });
        collection_plugin.push(PluginAuthorityPair { plugin: Plugin::PermanentBurnDelegate( PermanentBurnDelegate {}), authority: Some(PluginAuthority::UpdateAuthority) });
        collection_plugin.push(PluginAuthorityPair { plugin: Plugin::PermanentFreezeDelegate( PermanentFreezeDelegate { frozen: false }), authority: Some(PluginAuthority::UpdateAuthority) });
        collection_plugin.push(PluginAuthorityPair { plugin: Plugin::PermanentTransferDelegate( PermanentTransferDelegate {}), authority: Some(PluginAuthority::UpdateAuthority) });
        collection_plugin.push(PluginAuthorityPair { plugin: Plugin::Royalties( Royalties { basis_points: 200, creators: vec![Creator {address: self.manager.key(), percentage: 100 }], rule_set: RuleSet::None}), authority: Some(PluginAuthority::UpdateAuthority) });
        
        // Add an External Plugin Adapter that will hold the event details
        let mut collection_external_plugin: Vec<ExternalPluginAdapterInitInfo> = vec![];
        
        collection_external_plugin.push(ExternalPluginAdapterInitInfo::Oracle(
            OracleInitInfo {
                base_address: self.protocol.key(),
                base_address_config: None,
                results_offset: Some(ValidationResultsOffset::Anchor),
                lifecycle_checks: vec![
                    (HookableLifecycleEvent::Transfer, ExternalCheckResult { flags: 4 }),
                    (HookableLifecycleEvent::Burn, ExternalCheckResult { flags: 4 }),
                    (HookableLifecycleEvent::Update, ExternalCheckResult { flags: 4 }),
                    (HookableLifecycleEvent::Create, ExternalCheckResult { flags: 4 }),
                ],
                init_plugin_authority: Some(PluginAuthority::UpdateAuthority),
            }
        ));

        let manager_seed: &[&[u8]; 2] = &[b"manager", &[self.manager.bump]];
        let object_seed = &[b"object", reference.as_bytes().as_ref(), &[bump]];

        // Create the Collection that will hold the object 
        CreateCollectionV2CpiBuilder::new(&self.mpl_core_program.to_account_info())
        .collection(&self.object.to_account_info())
        .update_authority(Some(&self.manager.to_account_info()))
        .payer(&self.admin.to_account_info())
        .system_program(&self.system_program.to_account_info())
        .name(args.name)
        .uri(args.uri)
        .plugins(collection_plugin)
        .external_plugin_adapters(collection_external_plugin)
        .add_remaining_account(&self.protocol.to_account_info(), false, false)
        .invoke_signed(&[manager_seed, object_seed])?;

        Ok(())
    }
}

pub fn handler(ctx: Context<CreateObject>, args: CreateObjectArgs) -> Result<()> {
    ctx.accounts.create_object(args, ctx.bumps.object)?;

    Ok(())
}


use anchor_lang::prelude::*;


use libreplex_fair_launch::DeploymentV2;
use nifty_asset::{extensions::{ExtensionBuilder, MetadataBuilder}, instructions::{UpdateCpiBuilder, RemoveCpiBuilder, ResizeCpiBuilder}, types::{ExtensionInput, ExtensionType, Strategy}};




use crate::NiftyHybrid;

#[derive(Accounts)]
pub struct UpdateMetadataCtx<'info> {
    
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, 
        constraint = creator.key() == nifty_hybrid.creator)]
    pub creator: Signer<'info>,

    #[account(mut,
        constraint = nifty_hybrid.deployment == deployment.key())]
    pub nifty_hybrid: Box<Account<'info, NiftyHybrid>>,
   

    #[account(mut)]
    pub deployment: Box<Account<'info, DeploymentV2>>,

    /// CHECK: Checked in logic
    #[account(mut)]
    pub non_fungible_mint: UncheckedAccount<'info>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = nifty_program.key().eq(&nifty_asset::ID)
    )]
    pub nifty_program: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

}

pub fn update_metadata_handler<'info>(ctx: Context<'_, '_, '_, 'info, UpdateMetadataCtx<'info>>) -> Result<()> {
    let nifty_hybrid = &mut ctx.accounts.nifty_hybrid;
    
    let nifty_program = &ctx.accounts.nifty_program;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let deployment = &ctx.accounts.deployment;
    let payer = &ctx.accounts.payer;
    let seeds = &[
        b"nifty_hybrid",
        nifty_hybrid.seed.as_ref(),
        &[nifty_hybrid.bump],
    ];

    let system_program = &ctx.accounts.system_program;

    RemoveCpiBuilder::new(nifty_program)
    .asset(&non_fungible_mint.to_account_info())
    .recipient(&payer.to_account_info())
    .authority(&nifty_hybrid.to_account_info())
    .extension_type(ExtensionType::Metadata).invoke_signed(&[seeds])?;

    let mut metadata_builder = MetadataBuilder::default();
    metadata_builder.set(
        Some(&deployment.ticker.to_string()), 
        None, 
        Some(&deployment.offchain_url.to_string()));


    let data = metadata_builder.data();

    ResizeCpiBuilder::new(nifty_program)
    .asset(&ctx.accounts.non_fungible_mint)
    .authority(&nifty_hybrid.to_account_info())
    .payer(&payer.to_account_info(), false)
    .strategy(Strategy::Trim)
    .system_program(Some(system_program)).invoke_signed(&[seeds])?;

    UpdateCpiBuilder::new(nifty_program)
    .asset(&ctx.accounts.non_fungible_mint.to_account_info())
    .authority(&nifty_hybrid.to_account_info())
    .payer(Some(&payer.to_account_info()))
    .system_program(Some(system_program))
    .mutable(true)
     .extension(ExtensionInput {
        extension_type: ExtensionType::Metadata,
        length: data.len() as u32,
        data: Some(data),
    })
    .invoke_signed(&[seeds])?;

    Ok(())
}

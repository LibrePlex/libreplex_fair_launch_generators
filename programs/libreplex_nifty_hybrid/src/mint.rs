use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::Mint;
use libreplex_monoswap_client::instructions::CreateSwapInstructionArgs;


use nifty_asset::extensions::{ExtensionBuilder, MetadataBuilder};
use nifty_asset::instructions::{AllocateCpiBuilder, CreateCpiBuilder};

use nifty_asset::types::{ExtensionInput, ExtensionType};
use solana_program::program::{invoke, invoke_signed};


use libreplex_monoswap_client::programs::MONOSWAP_ID;

use crate::NiftyHybrid;
use libreplex_fair_launch::cpi::accounts::JoinRawCtx;
use libreplex_fair_launch::{program::LibreplexFairLaunch, DeploymentV2, MintInput};

#[derive(Accounts)]
pub struct MintCtx<'info> {
    /// CHECK: CAn be anyone
    #[account(mut)]
    pub receiver: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, has_one = deployment)]
    pub nifty_hybrid: Box<Account<'info, NiftyHybrid>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: Account<'info, DeploymentV2>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program
    #[account(mut)]
    pub swap_marker: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program, owned by the swap marker
    #[account(mut)]
    pub fungible_mint_target_ata: UncheckedAccount<'info>,


    /// CHECK: ATA of the recipient (minter) -> we swap nifties into SPL on mint
    #[account(mut)]
    pub fungible_mint_minter_ata: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program, owned by nifty hybrid
    #[account(mut)]
    pub fungible_mint_source_ata: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program  - this is the non fungible
    #[account(mut)]
    pub non_fungible_mint: Signer<'info>,

    /// CHECK: Passed into monoswap program  - this is the non fungible
    #[account(mut, 
        constraint = nifty_hybrid.group_mint == group_mint.key())]
    pub group_mint: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi
    #[account(mut)]
    pub incoming_asset_program: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut, 
        constraint = deployment.fungible_mint == fungible_mint.key())]
    pub fungible_mint: InterfaceAccount<'info, Mint>,

    pub fair_launch: Program<'info, LibreplexFairLaunch>,

    #[account()]
    pub associated_token_program: Program<'info, AssociatedToken>,

   
    /// CHECK: Checked in constraint
    #[account(
        constraint = nifty_program.key().eq(&nifty_asset::ID)
    )]
    pub nifty_program: UncheckedAccount<'info>,

    /// CHECK: Checked in constraint
    #[account(
        constraint = monoswap_program.key().eq(&MONOSWAP_ID)
    )]
    pub monoswap_program: UncheckedAccount<'info>,
}

pub fn mint_handler<'info>(ctx: Context<'_, '_, '_, 'info, MintCtx<'info>>) -> Result<()> {
    let fair_launch = &ctx.accounts.fair_launch;
    let deployment = &ctx.accounts.deployment;
    let nifty_hybrid = &mut ctx.accounts.nifty_hybrid;
    let receiver = &ctx.accounts.receiver;
    let payer = &ctx.accounts.payer;
    let swap_marker = &ctx.accounts.swap_marker;
    let fungible_mint_target_ata = &ctx.accounts.fungible_mint_target_ata;
    let fungible_mint_source_ata = &ctx.accounts.fungible_mint_source_ata;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let incoming_asset_program = &ctx.accounts.incoming_asset_program;
    let system_program = &ctx.accounts.system_program;
    let associated_token_program = &ctx.accounts.associated_token_program;
    let fungible_mint = &ctx.accounts.fungible_mint;
    let group_mint = &ctx.accounts.group_mint;
    let nifty_program = &ctx.accounts.nifty_program;

    let seeds = &[
        b"nifty_hybrid",
        nifty_hybrid.seed.as_ref(),
        &[nifty_hybrid.bump],
    ];


    let mut metadata_builder = MetadataBuilder::default();
    metadata_builder.set(
        Some(&deployment.ticker.to_string()), 
        None, 
        Some(&deployment.offchain_url.to_string()));

    let data = metadata_builder.data();

    AllocateCpiBuilder::new(nifty_program)
    .asset(&ctx.accounts.non_fungible_mint.to_account_info())
    .payer(Some(payer))
    .system_program(Some(system_program))
    .extension(ExtensionInput {
        extension_type: ExtensionType::Metadata,
        length: data.len() as u32,
        data: Some(data),
    })
    .invoke_signed(&[seeds])?;

    CreateCpiBuilder::new(nifty_program)
    .asset(&ctx.accounts.non_fungible_mint.to_account_info())
    .authority(&nifty_hybrid.to_account_info(), false)
    .group(Some(&group_mint.to_account_info()))
    .group_authority(Some(&nifty_hybrid.to_account_info()))
    .owner(&receiver.to_account_info())
    .payer(Some(&payer.to_account_info()))
    .system_program(Some(system_program))
    .name(deployment.ticker.clone())
    .standard(nifty_asset::types::Standard::NonFungible)
    .mutable(true)
    .invoke_signed(&[seeds])?;
    
    invoke_signed(
        &libreplex_monoswap_client::instructions::CreateSwap {
            payer: payer.key(),
            // use raw deployment as the namespace - this allows us to narrow things
            // down when filtering on target options
            namespace: nifty_hybrid.key(),
            authority: nifty_hybrid.key(),
            swap_marker: swap_marker.key(),
            swap_marker_ata: Some(fungible_mint_target_ata.key()),
            incoming_asset: fungible_mint.key(),
            nifty_asset_group: Some(group_mint.key()),
            authority_ata: Some(fungible_mint_source_ata.key()),
            external_asset: non_fungible_mint.key(),
            incoming_asset_program: incoming_asset_program.key(),
            associated_token_program: Some(associated_token_program.key()),
            system_program: system_program.key(),
        }
        .instruction(CreateSwapInstructionArgs {
            incoming_amount: deployment.get_base_amount_per_mint(fungible_mint), // one NFT in
            external_amount: 1,
        }),
        &[
            payer.to_account_info(),
            nifty_hybrid.to_account_info(),
            swap_marker.to_account_info(),
            group_mint.to_account_info(),
            fungible_mint_target_ata.to_account_info(),
            fungible_mint.to_account_info(),
            fungible_mint_source_ata.to_account_info(),
            non_fungible_mint.to_account_info(),
            incoming_asset_program.to_account_info(),
            associated_token_program.to_account_info(),
            system_program.to_account_info(),
        ],
        &[seeds]
    )?;

    let fungible_mint_minter_ata = &ctx.accounts.fungible_mint_minter_ata;


    libreplex_fair_launch::cpi::joinraw(
        CpiContext::new_with_signer(
            fair_launch.to_account_info(),
            JoinRawCtx {
                deployment: ctx.accounts.deployment.to_account_info(),
                hashlist: ctx.accounts.hashlist.to_account_info(),
                hashlist_marker: ctx.accounts.hashlist_marker.to_account_info(),
                payer: ctx.accounts.payer.to_account_info(),
                signer: nifty_hybrid.to_account_info(),
                non_fungible_mint: ctx.accounts.non_fungible_mint.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            &[seeds],
        )
        .with_remaining_accounts(vec![]),
        MintInput {
            multiplier_denominator: 1,
            multiplier_numerator: 1,
        },
    )?;


    // swap straight into a fungible (for now until nifties are more established)
    invoke(
        &libreplex_monoswap_client::instructions::SwapNiftySPL {
            payer: payer.key(),
            // use raw deployment as the namespace - this allows us to narrow things
            // down when filtering on target options
            authority: receiver.key(),
            swap_marker: swap_marker.key(),
            swap_marker_ata: fungible_mint_target_ata.key(),
            incoming_asset: non_fungible_mint.key(),
            nifty_asset_group: Some(group_mint.key()),
            authority_ata: fungible_mint_minter_ata.key(),
            escrowed_asset: fungible_mint.key(),
            escrowed_asset_program: incoming_asset_program.key(),
            incoming_asset_program: nifty_program.key(),
            associated_token_program: Some(associated_token_program.key()),
            system_program: system_program.key(),
        }
        .instruction(),
        &[
            payer.to_account_info(),
            nifty_hybrid.to_account_info(),
            swap_marker.to_account_info(),
            group_mint.to_account_info(),
            fungible_mint_target_ata.to_account_info(),
            fungible_mint.to_account_info(),
            fungible_mint_source_ata.to_account_info(),
            fungible_mint_minter_ata.to_account_info(),
            non_fungible_mint.to_account_info(),
            incoming_asset_program.to_account_info(),
            associated_token_program.to_account_info(),
            system_program.to_account_info(),
        ]
    )?;



    Ok(())
}

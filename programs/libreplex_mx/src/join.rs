use anchor_lang::prelude::*;

use libreplex_fair_launch::{Deployment, DeploymentConfig};
use mpl_token_metadata::accounts::Metadata;

use libreplex_fair_launch::{program::LibreplexFairLaunch, MintInput};

use crate::{LibreplexMxErrors, MetaplexJoiner, FILTER_TYPE_COLLECTION, FILTER_TYPE_CREATOR};

#[derive(Accounts)]
pub struct JoinCtx<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut, has_one = deployment)]
    pub metaplex_joiner: Box<Account<'info, MetaplexJoiner>>,

    pub system_program: Program<'info, System>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    /// CHECK: Checked in cpi.
    #[account(mut,
        constraint = deployment_config.deployment == deployment.key()
     )
    ]
    pub deployment_config: Account<'info, DeploymentConfig>,

    /// CHECK: Checked in cpi.
    #[account(mut,
        constraint = deployment_config.creator_fee_treasury == creator_fee_treasury.key()
    )]
    pub creator_fee_treasury: UncheckedAccount<'info>,

    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub hashlist: UncheckedAccount<'info>,

    /// CHECK: Passed into monoswap program  - this is the non fungible
    #[account(mut)]
    pub non_fungible_mint: UncheckedAccount<'info>,

    /// CHECK: Checked by deserializing in logic
    #[account(mut,
    seeds=[
        "metadata".as_bytes(),
        &mpl_token_metadata::ID.as_ref(),
        non_fungible_mint.key().as_ref(),
    ],
    seeds::program = mpl_token_metadata::ID,
    bump)]
    pub non_fungible_metadata: UncheckedAccount<'info>,

    /// CHECK: Checked and created via a cpi.
    #[account(mut)]
    pub hashlist_marker: UncheckedAccount<'info>,

    pub fair_launch: Program<'info, LibreplexFairLaunch>,
}

pub fn join_handler<'info>(ctx: Context<'_, '_, '_, 'info, JoinCtx<'info>>) -> Result<()> {
    let fair_launch = &ctx.accounts.fair_launch;
    let deployment = &ctx.accounts.deployment;
    let metaplex_joiner = &mut ctx.accounts.metaplex_joiner;
    let payer = &ctx.accounts.payer;
    let non_fungible_mint = &ctx.accounts.non_fungible_mint;
    let non_fungible_metadata = &ctx.accounts.non_fungible_metadata;
    let system_program = &ctx.accounts.system_program;
    let hashlist = &ctx.accounts.hashlist;
    let hashlist_marker = &ctx.accounts.hashlist_marker;
    let deployment_config = &ctx.accounts.deployment_config;
    let creator_fee_treasury = &ctx.accounts.creator_fee_treasury;
    let metadata_obj = Metadata::try_from(&non_fungible_metadata.to_account_info())?;

    // check that metadata matches the filter
    if metaplex_joiner.filter_type == FILTER_TYPE_COLLECTION {
        if let Some(x) = metadata_obj.collection {
            if !x.key.eq(&metaplex_joiner.filter_value) {
                panic!(
                    "Metadata has invalid collection. Expected {}",
                    metaplex_joiner.filter_value
                );
            } else if !x.verified {
                panic!(
                    "Metadata collection {} not verified.",
                    metaplex_joiner.filter_value
                );
            } else {
                msg!("Collection matches filter");
            };
        } else {
            panic!(
                "Metadata object has no collection. Filter expected {}",
                metaplex_joiner.filter_value
            );
        }
    } else if metaplex_joiner.filter_type == FILTER_TYPE_CREATOR {
        if let Some(x) = metadata_obj.creators {
            let creator_match = x
                .iter()
                .find(|x| x.address.eq(&metaplex_joiner.filter_value));
            match creator_match {
                None => {
                    return Err(LibreplexMxErrors::NoCreatorMatchingFilter.into());
                }
                Some(x) => {
                    if !x.verified {
                        return Err(LibreplexMxErrors::CreatorNotVerified.into());
                    } else {
                        msg!("Creator matches filter");
                    }
                }
            }
        } else {
            panic!(
                "Metadata object has no creators. Filter expected {}",
                metaplex_joiner.filter_value
            );
        }
    } else {
        panic!("Unexpected filter type {}", metaplex_joiner.filter_type);
    }
    let seeds = &[
        b"metaplex_joiner",
        metaplex_joiner.seed.as_ref(),
        &[metaplex_joiner.bump],
    ];

    msg!("Joining (hashlist marker {})", {hashlist_marker.key()});
    libreplex_fair_launch::cpi::join(
        CpiContext::new_with_signer(
            fair_launch.to_account_info(),
            libreplex_fair_launch::cpi::accounts::JoinCtx {
                deployment: deployment.to_account_info(),
                hashlist: hashlist.to_account_info(),
                hashlist_marker: hashlist_marker.to_account_info(),
                payer: payer.to_account_info(),
                signer: metaplex_joiner.to_account_info(),
                non_fungible_mint: non_fungible_mint.to_account_info(),
                system_program: system_program.to_account_info(),
                deployment_config: deployment_config.to_account_info(),
                creator_fee_treasury: creator_fee_treasury.to_account_info(),
            },
            &[seeds],
        ),
        MintInput {
            multiplier_denominator: 1,
            multiplier_numerator: 1,
        },
    )?;

    Ok(())
}

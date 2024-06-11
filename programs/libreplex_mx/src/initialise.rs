use anchor_lang::{prelude::*, system_program};
use libreplex_fair_launch::{program::LibreplexFairLaunch, Deployment, ToggleSwapCosignerInput};

use crate::{events::MetaplexJoinerCreate, MetaplexJoiner};

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct InitialiseInput {
    pub seed: Pubkey,
    pub cosigner: Option<Pubkey>,
    pub cosigner_program_id: Option<Pubkey>,
    pub filter_value: Pubkey,
    pub filter_type: u8
}

#[derive(Accounts)]
#[instruction(input: InitialiseInput)]
pub struct InitialiseCtx<'info> {
    /// CHECK: Checked in cpi.
    #[account(mut)]
    pub deployment: Account<'info, Deployment>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(init, payer = payer, space = 8 + MetaplexJoiner::INIT_SPACE,
         seeds = [b"metaplex_joiner", input.seed.as_ref()], bump)]
    pub metaplex_joiner: Box<Account<'info, MetaplexJoiner>>,

    pub system_program: Program<'info, System>,

    pub fair_launch: Program<'info, LibreplexFairLaunch>,

   
}

pub fn init_handler(ctx: Context<InitialiseCtx>, input: InitialiseInput) -> Result<()> {
    let InitialiseInput {
        seed,
        cosigner,
        cosigner_program_id,
        filter_type,
        filter_value
    } = input;

    let fair_launch = &ctx.accounts.fair_launch;
    let deployment = &ctx.accounts.deployment;
    let metaplex_joiner = &mut ctx.accounts.metaplex_joiner;
    let payer = &ctx.accounts.payer;
    metaplex_joiner.set_inner(MetaplexJoiner {
        seed,
        creator: ctx.accounts.creator.key(),
        bump: ctx.bumps.metaplex_joiner,
        deployment: ctx.accounts.deployment.key(),
        cosigner: match &cosigner {
            Some(x) => *x,
            _ => system_program::ID,
        },
        cosigner_program_id: match cosigner_program_id {
            Some(x) => x,
            _ => system_program::ID,
        },
      
        filter_value,
        filter_type,
        padding: [0; 100],
    });

    let seeds = &[
        b"metaplex_joiner",
        metaplex_joiner.seed.as_ref(),
        &[metaplex_joiner.bump],
    ];


    libreplex_fair_launch::cpi::toggle_swap_cosigner(
        CpiContext::new_with_signer(
            fair_launch.to_account_info(),
            libreplex_fair_launch::cpi::accounts::ToggleSwapCosignerCtx {
                deployment: deployment.to_account_info(),
                payer: payer.to_account_info(),
                creator: metaplex_joiner.to_account_info()
            },
            &[seeds],
        ),
        ToggleSwapCosignerInput {
            disable_swap_cosigner: true
        },
    )?;

    // // mint the required amount of fungible
    emit_create(&ctx.accounts.metaplex_joiner);

    Ok(())
}

// Avoid blowing up the stack.
fn emit_create(metaplex_joiner: &Account<MetaplexJoiner>) {
    let metaplex_joiner_ref: &MetaplexJoiner = metaplex_joiner.as_ref();
    emit!(MetaplexJoinerCreate {
        metaplex_joiner: metaplex_joiner_ref.clone(),
        id: metaplex_joiner.key()
    });
}

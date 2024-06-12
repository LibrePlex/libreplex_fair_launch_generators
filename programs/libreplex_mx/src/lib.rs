use anchor_lang::prelude::*;

pub mod state;
pub use state::*;

pub mod errors;
pub use errors::*;

pub mod join;
pub use join::*;

pub mod initialise;
pub use initialise::*;


declare_id!("METANohMwuVeNCJBhWDu4biTd74YAKyEruWdPUF1TDe");

#[program]
pub mod libreplex_mx {

   
    use super::*;

    // pub fn bla(ctx: Context<BlaCtx>) -> Result<()> {
    //     bla_handler(ctx)
    // }

    pub fn join<'info>(ctx: Context<'_, '_, '_, 'info, JoinCtx<'info>>) -> Result<()> {
        join_handler(ctx)
    }

    pub fn initialise<'info>(ctx: Context<'_,'_,'_,'info, InitialiseCtx<'info>>
        , input: InitialiseInput
    ) -> Result<()> {
        init_handler(ctx, input)
    }

    // no deploy method here.  deploy_raw can be called directly
}

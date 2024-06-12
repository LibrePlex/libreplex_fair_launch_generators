

use anchor_lang::prelude::*;

use anchor_lang::{system_program, InstructionData, ToAccountMetas};


use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::*;

use solana_sdk::{signer::Signer, transaction::Transaction};

use crate::state::Pnft;


pub async fn join_to_fair_launch(
    context: &mut ProgramTestContext,
    ticker: &str,
    pnft_mint: &Pnft,
    creator_fee_treasury: Pubkey,
    metaplex_joiner: Pubkey,
) -> Option<BanksClientError> {
    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist_marker = Pubkey::find_program_address(
        &[
            b"hashlist_marker",
            deployment.key().as_ref(),
            pnft_mint.mint.as_ref(),
        ],
        &libreplex_fair_launch::ID,
    )
    .0;

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let non_fungible_metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &mpl_token_metadata::ID.as_ref(),
            pnft_mint.mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let accounts = ::libreplex_mx::accounts::JoinCtx {
        deployment,
        deployment_config,
        creator_fee_treasury,
        payer: context.payer.pubkey(),
        system_program: system_program::ID,
        hashlist,
        hashlist_marker,
        non_fungible_mint: pnft_mint.mint,
        metaplex_joiner,
        non_fungible_metadata,
        fair_launch: libreplex_fair_launch::ID,
    }
    .to_account_metas(None);

    context.banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: ::libreplex_mx::id(),
                data: ::libreplex_mx::instruction::Join {}.data(),
                accounts,
            }],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .err()

    
}

use anchor_lang::prelude::*;

use anchor_lang::{system_program, InstructionData, ToAccountMetas};

use anchor_spl::token::{spl_token, Mint};
use libreplex_fair_launch::Deployment;
use libreplex_shared::sysvar_instructions_program;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::*;

use crate::{DECIMALS, INITIAL_ESCROW_TOKENS};
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

pub async fn initialise_mx_joiner<'info>(
    context: &mut ProgramTestContext,
    filter_value: Pubkey,
    filter_type: u8,
    deployment: &Account<'info, Deployment>,
    metaplex_joiner: Pubkey,
    metaplex_joiner_seed: Pubkey,
) -> Result<(Pubkey, Pubkey, Pubkey)> {
    let _hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let fungible_mint = Keypair::new();
    // create a metaplex fungible mint

    let rent = context.banks_client.get_rent().await.unwrap();

    let mint_rent = rent.minimum_balance(Mint::LEN);
    let create_mint_account_ix = solana_program::system_instruction::create_account(
        &context.payer.pubkey(),
        &fungible_mint.pubkey(),
        mint_rent,
        Mint::LEN as u64,
        &spl_token::ID,
    );

    let initialize_mint_account_ix = spl_token::instruction::initialize_mint(
        &spl_token::ID,
        &fungible_mint.pubkey(),
        &context.payer.pubkey(),
        None,
        DECIMALS,
    )
    .unwrap();

    let fungible_escrow_token_account =
        anchor_spl::associated_token::get_associated_token_address_with_program_id(
            &deployment.key(),
            &fungible_mint.pubkey(),
            &spl_token::ID,
        );

    let create_ata_ix = spl_associated_token_account::instruction::create_associated_token_account(
        &context.payer.pubkey(),
        &deployment.key(),
        &fungible_mint.pubkey(),
        &spl_token::ID,
    );

    // mint some tokens into the escrow
    let mint_ix = spl_token::instruction::mint_to(
        &spl_token::ID,
        &fungible_mint.pubkey(),
        &fungible_escrow_token_account,
        &context.payer.pubkey(),
        &[&context.payer.pubkey()],
        INITIAL_ESCROW_TOKENS,
    )
    .unwrap();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[
                create_mint_account_ix,
                initialize_mint_account_ix,
                create_ata_ix,
                // initialize_ata_ix,
                mint_ix,
            ], //create_fungible_mint_ix],
            Some(&context.payer.pubkey()),
            &[&context.payer, &fungible_mint],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    println!("Minted fungible to escrow");

    let hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::DeployExtFungible {}.data(),
                accounts: libreplex_fair_launch::accounts::DeployWithExternalFungibleCtx {
                    deployment: deployment.key(),
                    hashlist,
                    payer: context.payer.pubkey(),
                    fungible_mint: fungible_mint.pubkey(),
                    system_program: system_program::ID,
                    sysvar_instructions: sysvar_instructions_program::ID,
                    // these will be ignored for hybrid w/ deployment type TOKEN2022
                }
                .to_account_metas(None),
            }],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    println!("Deployed");

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: ::libreplex_mx::id(),
                data: ::libreplex_mx::instruction::Initialise {
                    input: ::libreplex_mx::InitialiseInput {
                        seed: metaplex_joiner_seed,
                        cosigner: None,
                        cosigner_program_id: None,
                        filter_value,
                        filter_type,
                    },
                }
                .data(),
                accounts: ::libreplex_mx::accounts::InitialiseCtx {
                    fair_launch: libreplex_fair_launch::ID,
                    deployment: deployment.key(),
                    payer: context.payer.pubkey(),
                    creator: context.payer.pubkey(),
                    system_program: system_program::ID,
                    metaplex_joiner,
                }
                .to_account_metas(None),
            }],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    // check that the fungible token has been minted out into the escrow
    // and that authorities have been removed
    Ok((
        fungible_mint.pubkey(),
        fungible_escrow_token_account,
        metaplex_joiner,
    ))
}

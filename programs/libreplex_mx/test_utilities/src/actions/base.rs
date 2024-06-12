use std::str::FromStr;

use anchor_lang::prelude::*;

use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use anchor_spl::associated_token::{self, get_associated_token_address_with_program_id};

use anchor_spl::token::spl_token;
use libreplex_fair_launch::{MultiplierLimits, TOKEN2022_DEPLOYMENT_TYPE};
use libreplex_shared::operations::auth_rules_program;
use libreplex_shared::sysvar_instructions_program;
use solana_program::hash::Hash;

use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::constants::AUTH_RULES;
use crate::state::Pnft;
use crate::{CREATOR_FEE_IN_LAMPORTS, DECIMALS, LIMIT_PER_MINT, MAX_NUMBER_OF_TOKENS, TICKER};

pub async fn swap_to_fungible_2022(
    context: &mut ProgramTestContext,
    ticker: &str,
    non_fungible_mint: &Pnft,
    minter_wallet: Option<&Keypair>,
    fungible_mint: Pubkey,
) -> Result<Pubkey> {
    let mut signing_keypairs: Vec<&Keypair> = vec![];

    let minter_wallet_key = match &minter_wallet {
        Some(x) => {
            signing_keypairs.push(x);
            x.pubkey()
        }
        _ => {
            signing_keypairs.push(&context.payer);
            context.payer.pubkey()
        }
    };

    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist_marker = Pubkey::find_program_address(
        &[
            b"hashlist_marker",
            deployment.key().as_ref(),
            non_fungible_mint.mint.as_ref(),
        ],
        &libreplex_fair_launch::ID,
    )
    .0;

    let non_fungible_source_token_account = non_fungible_mint.token_account;

    let token_record_source = non_fungible_mint.token_record;

    let non_fungible_target_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &non_fungible_mint.mint,
        &spl_token::ID,
    );

    let token_record_target = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            non_fungible_mint.mint.as_ref(),
            b"token_record",
            non_fungible_target_token_account.key().as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let fungible_source_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &fungible_mint,
        // fungible is still old skool at this point
        &anchor_spl::token::ID,
    );

    let fungible_target_token_account = get_associated_token_address_with_program_id(
        &minter_wallet_key,
        &fungible_mint,
        &anchor_spl::token::ID,
    );

    let metaplex_metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &mpl_token_metadata::ID.as_ref(),
            non_fungible_mint.mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let mut accounts = libreplex_fair_launch::accounts::SwapToFungible2022Ctx {
        deployment,
        payer: minter_wallet_key,
        fungible_target_token_account_owner: minter_wallet_key,
        signer: minter_wallet_key,
        non_fungible_source_account_owner: minter_wallet_key,
        system_program: system_program::ID,
        hashlist_marker,
        non_fungible_mint: non_fungible_mint.mint,
        token_program: anchor_spl::token::ID,
        token_program_22: spl_token_2022::ID,
        associated_token_program: associated_token::ID,
        sysvar_instructions: sysvar_instructions_program::ID,
        fungible_mint,
        fungible_source_token_account,
        fungible_target_token_account,
        non_fungible_source_token_account,
        non_fungible_target_token_account,
    }
    .to_account_metas(None);

    let master_edition = Pubkey::find_program_address(
        &[
            b"metadata",
            &mpl_token_metadata::ID.as_ref(),
            &non_fungible_mint.mint.as_ref(),
            b"edition",
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let mut additional_accounts = vec![
        AccountMeta {
            pubkey: metaplex_metadata,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(AUTH_RULES).unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: master_edition,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: token_record_source,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: token_record_target,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: sysvar_instructions_program::ID,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: auth_rules_program::ID,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: mpl_token_metadata::ID,
            is_signer: false,
            is_writable: false,
        },
    ];

    println!("{:?}", additional_accounts);
    accounts.append(&mut additional_accounts);

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::SwapToFungible22 {}.data(),
                accounts,
            }],
            Some(&minter_wallet_key),
            &signing_keypairs,
            context.last_blockhash,
        ))
        .await
        .unwrap();

    Ok(fungible_target_token_account)
}

pub async fn swap_to_non_fungible_2022(
    context: &mut ProgramTestContext,
    ticker: &str,
    non_fungible_mint: &Pnft,
    minter_wallet: Option<&Keypair>,
    fungible_mint: Pubkey,
) -> Result<()> {
    let mut signing_keypairs: Vec<&Keypair> = vec![];

    let minter_wallet_key = match &minter_wallet {
        Some(x) => {
            signing_keypairs.push(x);
            x.pubkey()
        }
        _ => {
            signing_keypairs.push(&context.payer);
            context.payer.pubkey()
        }
    };

    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let hashlist_marker = Pubkey::find_program_address(
        &[
            b"hashlist_marker",
            deployment.key().as_ref(),
            non_fungible_mint.mint.as_ref(),
        ],
        &libreplex_fair_launch::ID,
    )
    .0;

    let non_fungible_source_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &non_fungible_mint.mint,
        &anchor_spl::token::ID,
    );

    let fungible_source_token_account = get_associated_token_address_with_program_id(
        &minter_wallet_key,
        &fungible_mint,
        &anchor_spl::token::ID,
    );

    let fungible_target_token_account = get_associated_token_address_with_program_id(
        &deployment.key(),
        &fungible_mint,
        &anchor_spl::token::ID,
    );

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let master_edition = Pubkey::find_program_address(
        &[
            b"metadata",
            &mpl_token_metadata::ID.as_ref(),
            &non_fungible_mint.mint.as_ref(),
            b"edition",
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let mut accounts = libreplex_fair_launch::accounts::SwapToNonFungible2022Ctx {
        deployment,
        deployment_config,
        payer: minter_wallet_key,
        system_program: system_program::ID,
        hashlist_marker,
        non_fungible_mint: non_fungible_mint.mint,
        token_program: anchor_spl::token::ID,
        token_program_22: spl_token_2022::ID,
        associated_token_program: associated_token::ID,
        sysvar_instructions: sysvar_instructions_program::ID,
        fungible_mint,
        fungible_source_token_account,
        fungible_target_token_account,
        non_fungible_source_token_account,
        non_fungible_target_token_account: non_fungible_mint.token_account,
    }
    .to_account_metas(None);

    let metaplex_metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &mpl_token_metadata::ID.as_ref(),
            non_fungible_mint.mint.as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let token_record_source = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            non_fungible_mint.mint.as_ref(),
            b"token_record",
            non_fungible_source_token_account.key().as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let token_record_target = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            non_fungible_mint.mint.as_ref(),
            b"token_record",
            non_fungible_mint.token_account.key().as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let mut additional_accounts = vec![
        AccountMeta {
            pubkey: metaplex_metadata,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: Pubkey::from_str(AUTH_RULES).unwrap(),
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: master_edition,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: token_record_source,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: token_record_target,
            is_signer: false,
            is_writable: true,
        },
        AccountMeta {
            pubkey: sysvar_instructions_program::ID,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: auth_rules_program::ID,
            is_signer: false,
            is_writable: false,
        },
        AccountMeta {
            pubkey: mpl_token_metadata::ID,
            is_signer: false,
            is_writable: false,
        },
    ];

    accounts.append(&mut additional_accounts);

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::SwapToNonfungible22 {}.data(),
                accounts,
            }],
            Some(&minter_wallet_key),
            &signing_keypairs,
            context.last_blockhash,
        ))
        .await
        .unwrap();

    Ok(())
}

pub async fn initialise_deployment(
    context: &mut ProgramTestContext,
    creator: Pubkey,
    ticker: &str,
) -> Result<(Pubkey, Pubkey, Pubkey)> {
    let deployment = Pubkey::find_program_address(
        &[b"deployment", ticker.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let deploy_template = format!("{}-deploy", ticker);
    let mint_template = format!("{}-mint", ticker);
    let offchain_url = format!("https://dummy.com/{}.jpg", ticker);

    let creator_fee_treasury = Keypair::new().pubkey();

    context
        .banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::InitialiseV3 {
                    input: libreplex_fair_launch::InitialiseInputV3 {
                        limit_per_mint: LIMIT_PER_MINT,
                        max_number_of_tokens: MAX_NUMBER_OF_TOKENS,
                        decimals: DECIMALS,
                        ticker: TICKER.to_owned(),
                        deployment_template: deploy_template.clone(),
                        mint_template: mint_template.clone(),
                        offchain_url: offchain_url.clone(),
                        creator_cosign_program_id: Some(system_program::ID),
                        use_inscriptions: true,
                        deployment_type: TOKEN2022_DEPLOYMENT_TYPE,
                        creator_fee_per_mint_in_lamports: CREATOR_FEE_IN_LAMPORTS,
                        creator_fee_treasury,
                        transfer_fee_config: None,
                        multiplier_limits: MultiplierLimits {
                            max_numerator: 1,
                            min_denominator: 1,
                        },
                    },
                }
                .data(),
                accounts: libreplex_fair_launch::accounts::InitialiseV3Ctx {
                    deployment,
                    deployment_config,

                    payer: context.payer.pubkey(),
                    creator,
                    system_program: system_program::ID,
                }
                .to_account_metas(None),
            }],
            Some(&context.payer.pubkey()),
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    Ok((deployment, deployment_config, creator_fee_treasury))
}

pub async fn refresh_blockhash_and_warp(context: &mut ProgramTestContext, slot: &mut u64) -> Hash {
    // context.warp_to_slot(*slot).unwrap();
    context.warp_forward_force_reward_interval_end().unwrap();
    context.warp_to_epoch(*slot).unwrap();
    *slot += 1;

    context
        .banks_client
        .get_new_latest_blockhash(&context.last_blockhash)
        .await
        .unwrap()
}

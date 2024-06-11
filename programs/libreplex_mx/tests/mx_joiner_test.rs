use std::str::FromStr;

use anchor_lang::prelude::*;

use anchor_lang::{system_program, InstructionData, ToAccountMetas};
use anchor_spl::associated_token::{self, get_associated_token_address_with_program_id};

use anchor_spl::token::{spl_token, Mint};
use libreplex_fair_launch::{
    Deployment, DeploymentConfig, MintInput, MultiplierLimits, TOKEN2022_DEPLOYMENT_TYPE,
};
use libreplex_mx::FILTER_TYPE_CREATOR;
use libreplex_shared::operations::auth_rules_program;
use libreplex_shared::sysvar_instructions_program;
use mpl_token_metadata::instructions::{CreateV1Builder, MintBuilder, VerifyCreatorV1Builder};
use mpl_token_metadata::types::{Creator, PrintSupply, TokenStandard};
use solana_program::hash::Hash;
use solana_program::program_pack::Pack;
use solana_program::{instruction::Instruction, pubkey::Pubkey};
use solana_program_test::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};
use spl_token_2022::extension::StateWithExtensions;

const TICKER: &str = "hedgehog";
const CREATOR_FEE_IN_LAMPORTS: u64 = 10_000_000;
const LIMIT_PER_MINT: u64 = 1000;
const MAX_NUMBER_OF_TOKENS: u64 = 2;
// pick a silly number to make sure we haven't hard coded a 9 in there
const DECIMALS: u8 = 5;

const INITIAL_ESCROW_TOKENS: u64 = 1_000_000_000;

const NAME: &str = "NAME";
const SYMBOL: &str = "SYMBOL";
const URI: &str = "URI";

const AUTH_RULES: &str = "AdH2Utn6Fus15ZhtenW4hZBQnvtLgM1YCW2MfVp7pYS5";


pub struct Pnft {
    mint: Pubkey,
    token_account: Pubkey,
    token_record: Pubkey,
}

pub async fn refresh_blockhash_and_warp(context: &mut ProgramTestContext, slot: &mut u64) -> Hash {
    println!("Current slot {}", *slot);
    // context.warp_to_slot(*slot).unwrap();
    context.warp_forward_force_reward_interval_end().unwrap();
    context.warp_to_epoch(*slot).unwrap();
    *slot += 1;

    let last_blockhash = context
        .banks_client
        .get_new_latest_blockhash(&context.last_blockhash)
        .await
        .unwrap();
    return last_blockhash;
}

mod mx_joiner_tests {

    use anchor_lang::prelude::Account;
    use libreplex_fair_launch::{Deployment, TOKEN2022_DEPLOYMENT_TYPE};

    use solana_program::account_info::AccountInfo;
    use solana_program::pubkey::Pubkey;

    use solana_sdk::system_instruction::transfer;
    use spl_token_2022::processor::Processor;

    use super::*;

    #[tokio::test]
    async fn mx_joiner_test() {
        let mut program = ProgramTest::new(
            "libreplex_mx",
            ::libreplex_mx::ID,
            None, // processor!(libreplex_inscriptions::entry),
        );

        program.add_program(
            "libreplex_fair_launch",
            libreplex_fair_launch::ID,
            None, // processor!(libreplex_inscriptions::entry),
        );

        program.set_compute_max_units(5_000_000);

        program.add_program(
            "token_metadata",
            Pubkey::from_str("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s").unwrap(),
            None,
        );
        program.prefer_bpf(false);
        program.add_program(
            "spl_token_2022",
            spl_token_2022::id(),
            processor!(Processor::process),
        );
        let mut context: ProgramTestContext = program.start_with_context().await;
        let mut slot: u64 = 1;

        let libreplex_mx_joiner_seed = Keypair::new().pubkey();

        let metaplex_joiner = Pubkey::find_program_address(
            &[b"metaplex_joiner", libreplex_mx_joiner_seed.as_ref()],
            &::libreplex_mx::ID,
        )
        .0;

        refresh_blockhash_and_warp(&mut context, &mut slot).await;
        let royalty_creator = Keypair::new();
        let (deployment, _deployment_config, creator_fee_treasury) =
            initialise_deployment(&mut context, metaplex_joiner, TICKER)
                .await
                .unwrap();

        let mut deployment_account = context
            .banks_client
            .get_account(deployment)
            .await
            .unwrap()
            .unwrap();

        let deployment_account_info = AccountInfo::new(
            &deployment,
            false,
            false,
            &mut deployment_account.lamports,
            &mut deployment_account.data,
            &deployment_account.owner,
            deployment_account.executable,
            deployment_account.rent_epoch,
        );

        let deployment_account_obj: Account<Deployment> =
            Account::try_from(&deployment_account_info).unwrap();

        let deploy_template = format!("{}-deploy", TICKER);

        let mint_template = format!("{}-mint", TICKER);

        let offchain_url = format!("https://dummy.com/{}.jpg", TICKER);

        assert_eq!(deployment_account_obj.deployment_template, deploy_template);

        assert_eq!(deployment_account_obj.mint_template, mint_template);

        assert_eq!(deployment_account_obj.offchain_url, offchain_url);

        assert_eq!(
            deployment_account_obj.deployment_type,
            TOKEN2022_DEPLOYMENT_TYPE
        );

        assert_eq!(deployment_account_obj.limit_per_mint, LIMIT_PER_MINT);

        refresh_blockhash_and_warp(&mut context, &mut slot).await;

        let (fungible_mint, _fungible_mint_escrow, metaplex_joiner) = initialise_mx_joiner(
            &mut context,
            royalty_creator.pubkey(),
            &deployment_account_obj,
            metaplex_joiner,
            libreplex_mx_joiner_seed,
            &mut slot,
        )
        .await
        .unwrap();

        let mut fungible_mint_account = context
            .banks_client
            .get_account(fungible_mint)
            .await
            .unwrap()
            .unwrap();

        let fungible_mint_account_info = AccountInfo::new(
            &fungible_mint,
            false,
            false,
            &mut fungible_mint_account.lamports,
            &mut fungible_mint_account.data,
            &fungible_mint_account.owner,
            fungible_mint_account.executable,
            fungible_mint_account.rent_epoch,
        );

        let fungible_mint_account_obj = spl_token_2022::state::Mint::unpack_from_slice(
            &fungible_mint_account_info.try_borrow_data().unwrap(),
        )
        .unwrap();

        // check total supply is as expected
        assert_eq!(fungible_mint_account_obj.supply, INITIAL_ESCROW_TOKENS);

        assert_eq!(fungible_mint_account_obj.decimals, DECIMALS);

        let mut banks_client = &mut context.banks_client;

        check_deployment_account_state(
            banks_client,
            deployment,
            fungible_mint,
            deployment_account_obj.creator,
            0,
            0,
            DECIMALS,
        )
        .await;

        // let minter_wallet = Keypair::new();

        // let minter_wallet_key = minter_wallet.pubkey();

        // banks_client
        //     .process_transaction(Transaction::new_signed_with_payer(
        //         &[transfer(
        //             &context.payer.pubkey(),
        //             &minter_wallet.pubkey(),
        //             1_000_000_000,
        //         )],
        //         Some(&context.payer.pubkey()),
        //         &[&context.payer],
        //         context.last_blockhash,
        //     ))
        //     .await
        //     .unwrap();

        
        let mut non_fungible_mints: Vec<Pnft> = vec![];

        // context.warp_to_slot(100);

        let (mint_1, non_fungible_token_account, token_record, banks_client_error) =
            mint_pnft_and_join_to_fair_launch(
                banks_client,
                &deployment_account_obj.ticker,
                creator_fee_treasury,
                &context.payer,
                context.last_blockhash,
                metaplex_joiner,
                royalty_creator,
                true,
            )
            .await;

        assert_eq!(banks_client_error.is_none(), true);

        non_fungible_mints.push(Pnft {
            mint: mint_1,
            token_account: non_fungible_token_account,
            token_record,
        });

        check_deployment_account_state(
            banks_client,
            deployment,
            fungible_mint,
            metaplex_joiner,
            0,
            1,
            DECIMALS,
        )
        .await;


        // // see if we can swap
        let minter_fungible_token_account = swap_to_fungible_2022(
            banks_client,
            &deployment_account_obj.ticker,
            &non_fungible_mints[0],
            Some(&context.payer),
            fungible_mint,
            context.payer.pubkey(),
            context.last_blockhash,
        )
        .await
        .unwrap();

        
        // assert_eq!(banks_client_error.is_none(), true);

        check_deployment_account_state(
            banks_client,
            deployment,
            fungible_mint,
            metaplex_joiner,
            1,
            1,
            DECIMALS,
        )
        .await;

        // non_fungible_mints.push(mint_1);

        // let (mint_2, _, banks_client_error) = mint_token_2022(
        //     banks_client,
        //     &deployment_account_obj.ticker,
        //     Some(&minter_wallet),
        //     creator_fee_treasury,
        //     fungible_mint,
        //     &context.payer,
        //     context.last_blockhash,
        // )
        // .await;

        // check_deployment_account_state(
        //     banks_client,
        //     deployment,
        //     fungible_mint,
        //     context.payer.pubkey(),
        //     0,
        //     2,
        //     DECIMALS,
        // )
        // .await;

        // assert_eq!(banks_client_error.is_none(), true);

        // non_fungible_mints.push(mint_2);

        // // 3rd mint should throwan error
        // let (_, _, banks_client_error) = mint_token_2022(
        //     banks_client,
        //     &deployment_account_obj.ticker,
        //     Some(&minter_wallet),
        //     creator_fee_treasury,
        //     fungible_mint,
        //     &context.payer,
        //     context.last_blockhash,
        // )
        // .await;

        // // deployment should be unchanged
        // check_deployment_account_state(
        //     banks_client,
        //     deployment,
        //     fungible_mint,
        //     context.payer.pubkey(),
        //     0,
        //     2,
        //     DECIMALS,
        // )
        // .await;

        // assert_eq!(banks_client_error.unwrap().to_string(), "transport transaction error: Error processing Instruction 0: custom program error: 0x1775");

        // for m in non_fungible_mints.iter() {
        //     check_mint_state(
        //         banks_client,
        //         m.clone(),
        //         None,                   // mint auth must be None
        //         Some(deployment.key()), // mint auth must be None
        //         // check that this mint belongs to the fungible mint group
        //         Some(fungible_mint.clone()),
        //     )
        //     .await;
        // }

        // check_token_account_state(
        //     banks_client,
        //     fungible_mint_escrow,
        //     fungible_mint,
        //     deployment.key(),
        //     deployment_account_obj.max_number_of_tokens
        //         * deployment_account_obj.limit_per_mint
        //         * (10_u64.pow(deployment_account_obj.decimals as u32)),
        // )
        // .await;

        // check_token_account_state(
        //     banks_client,
        //     fungible_mint_escrow,
        //     fungible_mint,
        //     deployment.key(),
        //     (deployment_account_obj.max_number_of_tokens - 1)
        //         * deployment_account_obj.limit_per_mint
        //         * (10_u64.pow(deployment_account_obj.decimals as u32)),
        // )
        // .await;

        check_token_account_state(
            banks_client,
            minter_fungible_token_account,
            fungible_mint,
            context.payer.pubkey(),
            deployment_account_obj.limit_per_mint
                * (10_u64.pow(deployment_account_obj.decimals as u32)),
        )
        .await;

        // // deployment should be unchanged
        // check_deployment_account_state(
        //     &mut banks_client,
        //     deployment,
        //     fungible_mint,
        //     context.payer.pubkey(),
        //     1,
        //     2,
        //     DECIMALS,
        // )
        // .await;

        // see if we can swap back
        swap_to_non_fungible_2022(
            banks_client,
            &deployment_account_obj.ticker,
            &non_fungible_mints[0],
            Some(&context.payer),
            fungible_mint,
            context.payer.pubkey(),
            context.last_blockhash,
        )
        .await
        .unwrap();

        // // deployment should be unchanged
        // check_deployment_account_state(
        //     &mut banks_client,
        //     deployment,
        //     fungible_mint,
        //     context.payer.pubkey(),
        //     0,
        //     2,
        //     DECIMALS,
        // )
        // .await;

        check_token_account_state(
            banks_client,
            minter_fungible_token_account,
            fungible_mint,
            context.payer.pubkey(),
            0
        )
        .await;

        // check_deployment_config_account_state(
        //     &mut banks_client,
        //     deployment.key(),
        //     creator_fee_treasury,
        //     10_000_000,
        //     0,
        // )
        // .await;
    }
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

pub async fn initialise_mx_joiner<'info>(
    context: &mut ProgramTestContext,
    royalty_creator: Pubkey,
    deployment: &Account<'info, Deployment>,
    metaplex_joiner: Pubkey,
    metaplex_joiner_seed: Pubkey,
    slot: &mut u64,
) -> Result<(Pubkey, Pubkey, Pubkey)> {
    let _hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let fungible_mint = Keypair::new();
    // create a metaplex fungible mint

    refresh_blockhash_and_warp(context, slot).await;
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

    let blockhash = refresh_blockhash_and_warp(context, slot).await;

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
            blockhash,
        ))
        .await
        .unwrap();

    println!("Minted fungible to escrow");

    let hashlist = Pubkey::find_program_address(
        &[b"hashlist", deployment.key().as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let blockhash = refresh_blockhash_and_warp(context, slot).await;
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
            blockhash,
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
                        filter_value: royalty_creator,
                        filter_type: FILTER_TYPE_CREATOR,
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
            blockhash,
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

pub async fn mint_pnft_and_join_to_fair_launch(
    banks_client: &mut BanksClient,
    ticker: &str,
    creator_fee_treasury: Pubkey,
    context_payer: &Keypair,
    recent_blockhash: Hash,
    metaplex_joiner: Pubkey,
    royalty_creator: Keypair,
    verify_creator: bool,
) -> (Pubkey, Pubkey, Pubkey, Option<BanksClientError>) {
    let pnft_mint = Keypair::new();

    let metaplex_metadata = Pubkey::find_program_address(
        &[
            "metadata".as_bytes(),
            &mpl_token_metadata::ID.as_ref(),
            pnft_mint.pubkey().as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let master_edition = Pubkey::find_program_address(
        &[
            b"metadata",
            &mpl_token_metadata::ID.as_ref(),
            &pnft_mint.pubkey().as_ref(),
            b"edition",
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    print!("about to create");

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[CreateV1Builder::new()
                .metadata(metaplex_metadata.key())
                .mint(pnft_mint.pubkey(), true)
                .authority(context_payer.pubkey())
                .payer(context_payer.pubkey())
                .update_authority(context_payer.pubkey(), true)
                .master_edition(Some(master_edition))
                .seller_fee_basis_points(0)
                // .rule_set(Pubkey::from_str(AUTH_RULES).unwrap())
                .token_standard(TokenStandard::ProgrammableNonFungible)
                .is_mutable(true) // starts off as mutable so we can do an update later
                .name(NAME.to_string())
                .creators(vec![Creator {
                    address: royalty_creator.pubkey(),
                    verified: false,
                    share: 100,
                }])
                .symbol(SYMBOL.to_string())
                .uri(URI.to_string())
                .decimals(0)
                .print_supply(PrintSupply::Zero)
                .instruction()],
            Some(&context_payer.pubkey()),
            &[context_payer, &pnft_mint],
            recent_blockhash,
        ))
        .await
        .unwrap();

    let non_fungible_token_account = get_associated_token_address_with_program_id(
        &context_payer.pubkey(),
        &pnft_mint.pubkey(),
        &anchor_spl::token::ID,
    );

    let token_record = Pubkey::find_program_address(
        &[
            b"metadata",
            mpl_token_metadata::ID.as_ref(),
            pnft_mint.pubkey().as_ref(),
            b"token_record",
            non_fungible_token_account.key().as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[MintBuilder::new()
                .authority(context_payer.pubkey())
                .token(non_fungible_token_account)
                .token_owner(Some(context_payer.pubkey()))
                .mint(pnft_mint.pubkey())
                .payer(context_payer.pubkey())
                .metadata(metaplex_metadata)
                .token_record(Some(token_record))
                .authorization_rules(Some(Pubkey::from_str(AUTH_RULES).unwrap()))
                .master_edition(Some(master_edition))
                .mint_args(mpl_token_metadata::types::MintArgs::V1 {
                    amount: 1,
                    authorization_data: None,
                })
                .instruction()],
            None,
            &[context_payer],
            recent_blockhash,
        ))
        .await
        .unwrap();

    if verify_creator {
        println!("Verifying creator");
        banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[VerifyCreatorV1Builder::new()
                    .metadata(metaplex_metadata)
                    .authority(royalty_creator.pubkey())
                    .instruction()],
                Some(&context_payer.pubkey()),
                &[&royalty_creator, context_payer],
                recent_blockhash,
            ))
            .await
            .unwrap();
        println!("Creator verified");
    }

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
            pnft_mint.pubkey().as_ref(),
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
            pnft_mint.pubkey().as_ref(),
        ],
        &mpl_token_metadata::ID,
    )
    .0;

    let accounts = ::libreplex_mx::accounts::JoinCtx {
        deployment,
        deployment_config,
        creator_fee_treasury,
        payer: context_payer.pubkey(),
        system_program: system_program::ID,
        hashlist,
        hashlist_marker,
        non_fungible_mint: pnft_mint.pubkey(),
        metaplex_joiner,
        non_fungible_metadata,
        fair_launch: libreplex_fair_launch::ID,
    }
    .to_account_metas(None);

    println!("executing join");

    let err_result = banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: ::libreplex_mx::id(),
                data: ::libreplex_mx::instruction::Join {}.data(),
                accounts,
            }],
            Some(&context_payer.pubkey()),
            &[&context_payer],
            recent_blockhash,
        ))
        .await
        .err();

    print!("Join {:?}", err_result);

    (
        pnft_mint.pubkey(),
        non_fungible_token_account,
        token_record,
        err_result,
    )
}

pub async fn swap_to_fungible_2022(
    banks_client: &mut BanksClient,
    ticker: &str,
    non_fungible_mint: &Pnft,
    minter_wallet: Option<&Keypair>,
    fungible_mint: Pubkey,
    payer: Pubkey,
    last_blockhash: Hash,
) -> Result<Pubkey> {
    let mut signing_keypairs: Vec<&Keypair> = vec![];

    let minter_wallet_key = match &minter_wallet {
        Some(x) => {
            signing_keypairs.push(&x);
            x.pubkey()
        }
        _ => payer,
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
    // let non_fungible_source_token_account = get_associated_token_address_with_program_id(
    //     &minter_wallet_key,
    //     &non_fungible_mint.mint,
    //     &spl_token::ID,
    // );
    let token_record_source = non_fungible_mint.token_record;

    // let token_record_source = Pubkey::find_program_address(
    //     &[
    //         b"metadata",
    //         mpl_token_metadata::ID.as_ref(),
    //         non_fungible_mint.mint.as_ref(),
    //         b"token_record",
    //         non_fungible_source_token_account.key().as_ref(),
    //     ],
    //     &mpl_token_metadata::ID,
    // )
    // .0;

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

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::SwapToFungible22 {}.data(),
                accounts,
            }],
            Some(&minter_wallet_key),
            &signing_keypairs,
            last_blockhash,
        ))
        .await
        .unwrap();

    Ok(fungible_target_token_account)
}

pub async fn swap_to_non_fungible_2022(
    banks_client: &mut BanksClient,
    ticker: &str,
    non_fungible_mint: &Pnft,
    minter_wallet: Option<&Keypair>,
    fungible_mint: Pubkey,
    payer: Pubkey,
    last_blockhash: Hash,
) -> Result<()> {
    let mut signing_keypairs: Vec<&Keypair> = vec![];

    let minter_wallet_key = match &minter_wallet {
        Some(x) => {
            signing_keypairs.push(&x);
            x.pubkey()
        }
        _ => payer,
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

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[Instruction {
                program_id: libreplex_fair_launch::id(),
                data: libreplex_fair_launch::instruction::SwapToNonfungible22 {}.data(),
                accounts,
            }],
            Some(&minter_wallet_key),
            &signing_keypairs,
            last_blockhash,
        ))
        .await
        .unwrap();

    Ok(())
}

pub async fn check_token_account_state(
    banks_client: &mut BanksClient,
    token_account: Pubkey,
    expected_mint: Pubkey,
    expected_owner: Pubkey,
    expected_amount: u64,
) {
    let mut token_account_data = banks_client
        .get_account(token_account)
        .await
        .unwrap()
        .unwrap();

    let token_account_info = AccountInfo::new(
        &token_account,
        false,
        false,
        &mut token_account_data.lamports,
        &mut token_account_data.data,
        &token_account_data.owner,
        token_account_data.executable,
        token_account_data.rent_epoch,
    );

    let token_account_obj = spl_token_2022::state::Account::unpack_from_slice(
        &token_account_info.try_borrow_data().unwrap(),
    )
    .unwrap();

    assert_eq!(token_account_obj.mint, expected_mint);
    assert_eq!(token_account_obj.owner, expected_owner);

    assert_eq!(token_account_obj.amount, expected_amount);
}

pub async fn check_deployment_account_state(
    banks_client: &mut BanksClient,
    deployment: Pubkey,
    expected_fungible_mint: Pubkey,
    expected_creator: Pubkey,
    expected_escrow_non_fungible_count: u64,
    expected_tokens_issued: u64,
    expected_decimals: u8,
) {
    let mut deployment_account = banks_client.get_account(deployment).await.unwrap().unwrap();

    let deployment_account_info = AccountInfo::new(
        &deployment,
        false,
        false,
        &mut deployment_account.lamports,
        &mut deployment_account.data,
        &deployment_account.owner,
        deployment_account.executable,
        deployment_account.rent_epoch,
    );

    let deployment_account_obj: Account<Deployment> =
        Account::try_from(&deployment_account_info).unwrap();

    assert_eq!(
        deployment_account_obj.fungible_mint,
        expected_fungible_mint.key()
    );

    assert_eq!(deployment_account_obj.creator, expected_creator);

    assert_eq!(
        deployment_account_obj.number_of_tokens_issued,
        expected_tokens_issued
    );

    assert_eq!(
        deployment_account_obj.escrow_non_fungible_count,
        expected_escrow_non_fungible_count
    );

    assert_eq!(deployment_account_obj.decimals, expected_decimals);
}

pub async fn check_deployment_config_account_state(
    banks_client: &mut BanksClient,
    deployment: Pubkey,
    creator_fee_treasury: Pubkey,
    creator_fee_per_mint_lamports: u64,
    expected_excess_spl: u64,
) {
    let deployment_config = Pubkey::find_program_address(
        &[b"deployment_config", deployment.as_ref()],
        &libreplex_fair_launch::ID,
    )
    .0;

    let mut deployment_config_account = banks_client
        .get_account(deployment_config)
        .await
        .unwrap()
        .unwrap();

    let deployment_config_account_info = AccountInfo::new(
        &deployment_config,
        false,
        false,
        &mut deployment_config_account.lamports,
        &mut deployment_config_account.data,
        &deployment_config_account.owner,
        deployment_config_account.executable,
        deployment_config_account.rent_epoch,
    );

    let deployment_account_obj: Account<DeploymentConfig> =
        Account::try_from(&deployment_config_account_info).unwrap();

    assert_eq!(
        deployment_account_obj.creator_fee_per_mint_lamports,
        creator_fee_per_mint_lamports
    );

    assert_eq!(
        deployment_account_obj.creator_fee_treasury,
        creator_fee_treasury
    );

    assert_eq!(
        deployment_account_obj.spl_excess_in_escrow,
        expected_excess_spl
    );
}

pub async fn check_mint_state(
    banks_client: &mut BanksClient,
    mint: Pubkey,
    expected_mint_authority: Option<Pubkey>,
    expected_freeze_authority: Option<Pubkey>,
    _expected_group: Option<Pubkey>, // to enable once groups are enabled
) {
    println!("Check mint state");
    let mut mint_account = banks_client.get_account(mint).await.unwrap().unwrap();

    let mint_account_info = AccountInfo::new(
        &mint,
        false,
        false,
        &mut mint_account.lamports,
        &mut mint_account.data,
        &mint_account.owner,
        mint_account.executable,
        mint_account.rent_epoch,
    );

    let input_data = &mint_account_info.try_borrow_data().unwrap();
    let mint_account_obj =
        StateWithExtensions::<spl_token_2022::state::Mint>::unpack(input_data).unwrap();

    match &expected_freeze_authority {
        None => {
            assert_eq!(mint_account_obj.base.freeze_authority.is_none(), true);
        }
        Some(x) => {
            assert_eq!(mint_account_obj.base.freeze_authority.unwrap(), x.clone());
        }
    }

    match &expected_mint_authority {
        None => {
            assert_eq!(mint_account_obj.base.mint_authority.is_none(), true);
        }
        Some(x) => {
            assert_eq!(mint_account_obj.base.mint_authority.unwrap(), x.clone());
        }
    }

    // Renable once groups have rolled out
    // match &expected_group {
    //     None => {}
    //     Some(x) => {
    //         let token_group_member_extension = mint_account_obj.get_extension::<TokenGroupMember>();
    //         println!("Group is not none");
    //         let extension = token_group_member_extension.unwrap();
    //         assert_eq!(extension.mint, mint);

    //         assert_eq!(&extension.group, x);
    //     }
    // }
}

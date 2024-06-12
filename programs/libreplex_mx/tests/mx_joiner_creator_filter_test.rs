use std::str::FromStr;

use anchor_lang::prelude::*;


use libreplex_fair_launch::{
    Deployment, DeploymentConfig,
};
use solana_program::hash::Hash;
use solana_program::program_pack::Pack;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;

use solana_sdk::{signature::Keypair, signer::Signer};
use spl_token_2022::extension::StateWithExtensions;
use utilities::Pnft;

pub async fn refresh_blockhash_and_warp(context: &mut ProgramTestContext, slot: &mut u64) -> Hash {
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

mod mx_joiner_creator_filter_tests {

    use anchor_lang::prelude::Account;
    use libreplex_fair_launch::{Deployment, TOKEN2022_DEPLOYMENT_TYPE};

    use libreplex_mx::FILTER_TYPE_CREATOR;
    use solana_program::account_info::AccountInfo;
    use solana_program::pubkey::Pubkey;

    use solana_sdk::transaction::TransactionError;
    use spl_token_2022::processor::Processor;
    use utilities::{
        initialise_deployment, initialise_mx_joiner, join_to_fair_launch, mint_pnft, swap_to_fungible_2022, swap_to_non_fungible_2022, DECIMALS, INITIAL_ESCROW_TOKENS, LIMIT_PER_MINT, TICKER
    };

    use super::*;

    #[tokio::test]
    async fn mx_joiner_creator_filter_test() {
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

        let (fungible_mint, _fungible_mint_escrow, metaplex_joiner) = initialise_mx_joiner(
            &mut context,
            royalty_creator.pubkey(),
            FILTER_TYPE_CREATOR,
            &deployment_account_obj,
            metaplex_joiner,
            libreplex_mx_joiner_seed
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

        let banks_client = &mut context.banks_client;

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

        let mut non_fungible_mints: Vec<Pnft> = vec![];

        let mint_1 = mint_pnft(
            banks_client,
            &context.payer,
            context.last_blockhash,
            &royalty_creator,
            true,
            None,
        )
        .await;

        // add a good pnft with the correct creator
        let banks_client_error = join_to_fair_launch(
            banks_client,
            &deployment_account_obj.ticker,
            &mint_1,
            creator_fee_treasury,
            &context.payer,
            context.last_blockhash,
            metaplex_joiner,
        )
        .await;

        assert_eq!(banks_client_error.is_none(), true);

        let bad_creator = Keypair::new();
        // add a bad pnft with an incorrect creator
        let mint_2 = mint_pnft(
            banks_client,
            &context.payer,
            context.last_blockhash,
            &bad_creator,
            true,
            None,
        )
        .await;

        let banks_client_error = join_to_fair_launch(
            banks_client,
            &deployment_account_obj.ticker,
            &mint_2,
            creator_fee_treasury,
            &context.payer,
            context.last_blockhash,
            metaplex_joiner,
        )
        .await;

        match &banks_client_error {
            Some(z) => match &z {
                BanksClientError::TransactionError(x) => match &x {
                    TransactionError::InstructionError(_, instruction_error) => {
                        assert_eq!(
                            instruction_error.to_string(),
                            "custom program error: 0x1770"
                        );
                    }
                    _ => {
                        panic!("Unexpected result. Expected TransactionError:InstructionError")
                    }
                },
                _ => {
                    panic!("Unexpected result. Expected BanksClientError::TransactionError")
                }
            },
            _ => {
                panic!("Bad creator joined successfully. This should not happen.")
            }
        }

        let mint_3 = mint_pnft(
            banks_client,
            &context.payer,
            context.last_blockhash,
            &royalty_creator,
            false,
            None,
        )
        .await;

        let banks_client_error = join_to_fair_launch(
            banks_client,
            &deployment_account_obj.ticker,
            &mint_3,
            creator_fee_treasury,
            &context.payer,
            context.last_blockhash,
            metaplex_joiner
        )
        .await;

        match &banks_client_error {
            Some(z) => match &z {
                BanksClientError::TransactionError(x) => match &x {
                    TransactionError::InstructionError(_, instruction_error) => {
                        assert_eq!(
                            instruction_error.to_string(),
                            "custom program error: 0x1771"
                        );
                    }
                    _ => {
                        panic!("Unexpected result. Expected TransactionError:InstructionError")
                    }
                },
                _ => {
                    panic!("Unexpected result. Expected BanksClientError::TransactionError")
                }
            },
            _ => {
                panic!("Unverified creator joined successfully. This should not happen.")
            }
        }

        non_fungible_mints.push(mint_1);

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

        check_token_account_state(
            banks_client,
            minter_fungible_token_account,
            fungible_mint,
            context.payer.pubkey(),
            deployment_account_obj.limit_per_mint
                * (10_u64.pow(deployment_account_obj.decimals as u32)),
        )
        .await;

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

        check_token_account_state(
            banks_client,
            minter_fungible_token_account,
            fungible_mint,
            context.payer.pubkey(),
            0,
        )
        .await;
    }
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

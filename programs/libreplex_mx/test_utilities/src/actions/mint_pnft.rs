use std::str::FromStr;

use anchor_lang::prelude::*;

use anchor_spl::associated_token::get_associated_token_address_with_program_id;

use mpl_token_metadata::instructions::{
    CreateV1Builder, MintBuilder, VerifyCollectionV1Builder,
    VerifyCreatorV1Builder,
};
use mpl_token_metadata::types::{Collection, Creator, PrintSupply, TokenStandard};

use solana_program::pubkey::Pubkey;
use solana_program_test::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::{Pnft, AUTH_RULES, NAME, SYMBOL, URI};

#[derive(Copy, Clone)]
pub struct CollectionInput {
    pub key: Pubkey,
    pub verify: bool,
}

pub async fn mint_pnft(
    context: &mut ProgramTestContext,
    royalty_creator: &Keypair,
    verify_creator: bool,
    collection_mint: Option<CollectionInput>,
) -> Pnft {
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

    let mut builder = CreateV1Builder::new();
    builder
        .metadata(metaplex_metadata.key())
        .mint(pnft_mint.pubkey(), true)
        .authority(context.payer.pubkey())
        .payer(context.payer.pubkey())
        .update_authority(context.payer.pubkey(), true)
        .master_edition(Some(master_edition))
        .seller_fee_basis_points(0)
        .creators(vec![Creator {
            address: royalty_creator.pubkey(),
            verified: false,
            share: 100,
        }])
        // .rule_set(Pubkey::from_str(AUTH_RULES).unwrap())
        .token_standard(TokenStandard::ProgrammableNonFungible)
        .is_mutable(true) // starts off as mutable so we can do an update later
        .name(NAME.to_string())
        .symbol(SYMBOL.to_string())
        .uri(URI.to_string())
        .decimals(0)
        .print_supply(PrintSupply::Zero);

    if let Some(x) = collection_mint {
        builder.collection(Collection {
            verified: false,
            key: x.key,
        });
    }

    context.banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[builder.instruction()],
            Some(&context.payer.pubkey()),
            &[&context.payer, &pnft_mint],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    let non_fungible_token_account = get_associated_token_address_with_program_id(
        &context.payer.pubkey(),
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

    context.banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[MintBuilder::new()
                .authority(context.payer.pubkey())
                .token(non_fungible_token_account)
                .token_owner(Some(context.payer.pubkey()))
                .mint(pnft_mint.pubkey())
                .payer(context.payer.pubkey())
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
            &[&context.payer],
            context.last_blockhash,
        ))
        .await
        .unwrap();

    if verify_creator {
        context.banks_client
            .process_transaction(Transaction::new_signed_with_payer(
                &[VerifyCreatorV1Builder::new()
                    .metadata(metaplex_metadata)
                    .authority(royalty_creator.pubkey())
                    .instruction()],
                Some(&context.payer.pubkey()),
                &[&royalty_creator, &context.payer],
                context.last_blockhash,
            ))
            .await
            .unwrap();
    }

    if let Some(y) = collection_mint {
        if y.verify {
            let metaplex_metadata_collection = Pubkey::find_program_address(
                &[
                    "metadata".as_bytes(),
                    &mpl_token_metadata::ID.as_ref(),
                    y.key.as_ref(),
                ],
                &mpl_token_metadata::ID,
            )
            .0;

            let master_edition_collection = Pubkey::find_program_address(
                &[
                    b"metadata",
                    &mpl_token_metadata::ID.as_ref(),
                    &y.key.as_ref(),
                    b"edition",
                ],
                &mpl_token_metadata::ID,
            )
            .0;

            context.banks_client
                .process_transaction(Transaction::new_signed_with_payer(
                    &[VerifyCollectionV1Builder::new()
                        .metadata(metaplex_metadata)
                        .collection_metadata(Some(metaplex_metadata_collection))
                        .collection_mint(y.key)
                        .collection_master_edition(Some(master_edition_collection))
                        .authority(context.payer.pubkey())
                        .instruction()],
                    Some(&context.payer.pubkey()),
                    &[&context.payer],
                    context.last_blockhash,
                ))
                .await
                .unwrap();
        }
    }

    Pnft {
        mint: pnft_mint.pubkey(),
        token_account: non_fungible_token_account,
        token_record,
    }
}

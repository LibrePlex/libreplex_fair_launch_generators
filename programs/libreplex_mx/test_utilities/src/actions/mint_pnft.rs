use std::str::FromStr;

use anchor_lang::prelude::*;

use anchor_spl::associated_token::get_associated_token_address_with_program_id;

use mpl_token_metadata::instructions::{CreateV1Builder, MintBuilder, VerifyCollectionV1Builder, VerifyCreatorV1Builder};
use mpl_token_metadata::types::{Collection, Creator, PrintSupply, TokenStandard};
use solana_program::hash::Hash;
use solana_program::pubkey::Pubkey;
use solana_program_test::*;

use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

use crate::{Pnft, AUTH_RULES, NAME, SYMBOL, URI};

#[derive(Copy, Clone)]
pub struct CollectionInput {
    pub key: Pubkey,
    pub verify: bool
}

pub async fn mint_pnft(
    banks_client: &mut BanksClient,
    context_payer: &Keypair,
    recent_blockhash: Hash,
    royalty_creator: &Keypair,
    verify_creator: bool,
    collection_mint: Option<CollectionInput>
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
    builder.metadata(metaplex_metadata.key())
        .mint(pnft_mint.pubkey(), true)
        .authority(context_payer.pubkey())
        .payer(context_payer.pubkey())
        .update_authority(context_payer.pubkey(), true)
        .master_edition(Some(master_edition))
        .seller_fee_basis_points(0)
        .creators(vec![Creator {
            address: royalty_creator.pubkey(),
            verified: false,
            share: 100
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

    banks_client
        .process_transaction(Transaction::new_signed_with_payer(
            &[builder.instruction()],
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

            banks_client
                .process_transaction(Transaction::new_signed_with_payer(
                    &[VerifyCollectionV1Builder::new()
                        .metadata(metaplex_metadata_collection)
                        .collection_mint(y.key)
                        .authority(context_payer.pubkey())
                        .instruction()],
                    Some(&context_payer.pubkey()),
                    &[&context_payer],
                    recent_blockhash,
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

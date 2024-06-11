use anchor_lang::prelude::*;

pub const FILTER_TYPE_COLLECTION: u8 = 0;
pub const FILTER_TYPE_CREATOR: u8 = 1; 

/*

    A flat-rarity (p))NFT joiner.

    Generates a fair launch where NFTs/pNFTs from a collection can be joined 
    to a fair launch. Each (p)NFT is swappable at the same swap rate.

    Multiple swap rates (typically to handle rarities) are handled in a separate
    generator contract.

*/
#[account]
#[derive(InitSpace)]
pub struct MetaplexJoiner {
    pub seed: Pubkey,
    pub bump: u8,
    pub creator: Pubkey,
    pub deployment: Pubkey,
    pub filter_value: Pubkey,

    /// 0 - collection
    /// 1 - creator 
    pub filter_type: u8,
    
    pub cosigner: Pubkey,

    pub cosigner_program_id: Pubkey, 

    pub padding: [u8; 100]
}


pub mod events {
    use super::*;

    #[event]
    pub struct MetaplexJoinerCreate {
        pub id: Pubkey,
        pub metaplex_joiner: MetaplexJoiner,
    }
}


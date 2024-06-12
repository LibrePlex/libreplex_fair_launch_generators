
use solana_program::pubkey::Pubkey;

pub struct Pnft {
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub token_record: Pubkey,
}
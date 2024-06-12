use anchor_lang::prelude::*;

#[error_code]
pub enum LibreplexMxErrors {
    
    /// 6000 / 0x1770
    #[msg("No creator matching filter")]
    NoCreatorMatchingFilter,

    /// 6001 / 0x1771
    #[msg("Creator not verified")]
    CreatorNotVerified,
    
}

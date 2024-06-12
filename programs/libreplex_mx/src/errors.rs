use anchor_lang::prelude::*;

#[error_code]
pub enum LibreplexMxErrors {
    
    /// 6000 / 0x1770
    #[msg("No creator matching filter")]
    NoCreatorMatchingFilter,

    /// 6001 / 0x1771
    #[msg("Creator not verified")]
    CreatorNotVerified,

    /// 6002 / 0x1772
    #[msg("Collection not verified")]
    MetadataCollectionNotVerified,

    /// 6003 / 0x1773
    #[msg("Collection not verified")]
    MetadataHasInvalidCollection
    
}

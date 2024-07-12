use anchor_lang::prelude::*;

#[error_code]
pub enum ServiceOfferingError {
    #[msg("Service is not active")]
    ServiceNotActive,

    #[msg("Sold out")]
    SoldOut,

    #[msg("Invalid OSS program")]
    InvalidOssProgram,
}

#[error_code]
pub enum ListingError {
    #[msg("Listing is not active")]
    ListingNotActive,

    #[msg("Invalid OSS program")]
    InvalidOssProgram,

    #[msg("Asset is soulbound")]
    AssetIsSoulbound,

    #[msg("Asset is locked")]
    AssetIsLocked,

    #[msg("Invalid group")]
    InvalidGroup,
}
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

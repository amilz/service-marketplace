use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use error::*;
pub use instructions::*;
pub use state::*;

declare_id!("gJ12Bk8QFGQjdhZqHZ2PZqqYx8yi5ryVnuosP9S9m7Z");

#[program]
pub mod service_marketplace {
    use super::*;

    pub fn create_service_offering(
        ctx: Context<CreateServiceOffering>,
        offering_name: String,
        max_quantity: u64,
        sol_price: u64,
        expires_at: Option<i64>,
        symbol: String,
        description: String,
        uri: String,
        image: String,
        royalty_basis_points: u64,
        terms_of_service_uri: String,
        is_transferrable: bool,
    ) -> Result<()> {
        create_service_offering::handler(
            ctx,
            offering_name,
            max_quantity,
            sol_price,
            expires_at,
            symbol,
            description,
            uri,
            image,
            royalty_basis_points,
            terms_of_service_uri,
            is_transferrable,
        )
    }

    pub fn buy_service(ctx: Context<BuyService>, offering_name: String) -> Result<()> {
        buy_service::handler(ctx, offering_name)
    }

    pub fn list_asset(ctx: Context<ListAsset>, price: u64, expires_at: Option<i64>) -> Result<()> {
        list_asset::handler(ctx, price, expires_at)
    }
}


pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;
pub use error::*;

declare_id!("gJ12Bk8QFGQjdhZqHZ2PZqqYx8yi5ryVnuosP9S9m7Z");

#[program]
pub mod service_marketplace {
    use super::*;

    pub fn create_service_offering(ctx: Context<CreateServiceOffering>, offering_name: String, max_quantity: u64, sol_price: u64, expires_at: Option<i64>) -> Result<()> {
        create_service_offering::handler(ctx, offering_name, max_quantity, sol_price, expires_at)
    }
}

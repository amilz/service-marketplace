use anchor_lang::prelude::*;

use crate::{ServiceOffering, SEED_SERVICE_OFFERING, SEED_SERVICE_OFFERING_GROUP};

#[derive(Accounts)]
#[instruction(offering_name: String)]
pub struct CreateServiceOffering<'info> {
    // The public key of the vendor offering the service
    #[account(mut)]
    pub vendor: Signer<'info>,

    #[account(
        init,
        payer = vendor,
        space = ServiceOffering::get_size(),
        seeds = [
            SEED_SERVICE_OFFERING.as_bytes(),
            vendor.key().as_ref(),
            offering_name.as_bytes()
        ],
        bump
    )]
    pub service_offering: Account<'info, ServiceOffering>,

    // The public key of the associated NFT group asset (1 group for each service offering)
    /// CHECK: OSS inits it as an Asset
    #[account(
        mut,
        seeds = [
            SEED_SERVICE_OFFERING_GROUP.as_bytes(),
            service_offering.key().as_ref(),
        ],
        bump,
    )]
    pub offering_group_asset: UncheckedAccount<'info>,

    /// CHECK: use address constraint
    #[account(
        // address = NiftyAssetID @ ServiceOfferingError::InvalidOssProgram
    )]
    pub oss_program: UncheckedAccount<'info>,

    // Solana program system account
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<CreateServiceOffering>, offering_name: String, max_quantity: u64, sol_price: u64, expires_at: Option<i64>) -> Result<()> {
    let service_offering = &mut ctx.accounts.service_offering;
    service_offering.create(
        ctx.accounts.vendor.key(),
        ctx.accounts.offering_group_asset.key(),
        max_quantity,
        sol_price,
        expires_at,
        ctx.bumps.service_offering,
    );
    initialize_group_asset(offering_name)?;
    Ok(())
}

fn initialize_group_asset(offering_name: String) -> Result<()> {
    msg!("Initializing group asset for offering: {}", offering_name);
    Ok(())
}


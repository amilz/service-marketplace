use anchor_lang::system_program::{transfer, Transfer};
use anchor_lang::{prelude::*, solana_program::program::invoke_signed};

use crate::{
    ServiceOffering, ServiceOfferingError, SEED_SERVICE_OFFERING, SEED_SERVICE_OFFERING_GROUP,
};
use nifty_asset::{instructions::CreateBuilder, types::Standard, ID as NIFTY_ASSET_PROGRAM_ID};

#[derive(Accounts)]
#[instruction(offering_name: String)]
pub struct BuyService<'info> {
    // The buyer
    #[account(mut)]
    pub buyer: Signer<'info>,

    // The public key of the vendor offering the service (receiving the payment)
    #[account(mut)]
    pub vendor: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [
            SEED_SERVICE_OFFERING.as_bytes(),
            vendor.key().as_ref(),
            offering_name.as_bytes()
        ],
        bump,
        has_one = vendor
    )]
    pub service_offering: Account<'info, ServiceOffering>,

    // The public key of the associated NFT group asset
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

    /// CHECK: New NFT Mint (will be init by OSS Program via CPI - address random keypair)
    #[account(mut)]
    pub new_asset: Signer<'info>,

    /// CHECK: use address constraint
    #[account(
        address = NIFTY_ASSET_PROGRAM_ID @ ServiceOfferingError::InvalidOssProgram
    )]
    pub oss_program: UncheckedAccount<'info>,

    // Solana program system account
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyService>, offering_name: String) -> Result<()> {
    let service_offering = &mut ctx.accounts.service_offering;
    let vendor_key = ctx.accounts.vendor.key();
    let service_offering_key = service_offering.key();

    let service_offering_seeds = &[
        SEED_SERVICE_OFFERING.as_bytes(),
        vendor_key.as_ref(),
        offering_name.as_bytes(),
        &[ctx.bumps.service_offering],
    ];

    let combined_signer_seeds = &[&service_offering_seeds[..]];

    let account_infos = vec![
        ctx.accounts.buyer.to_account_info(),
        ctx.accounts.offering_group_asset.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.oss_program.to_account_info(),
        ctx.accounts.new_asset.to_account_info(),
        service_offering.to_account_info(),
    ];

    create_asset(
        &ctx.accounts.new_asset.key(),
        &ctx.accounts.buyer.key(),
        &service_offering_key,
        &ctx.accounts.system_program.key(),
        &ctx.accounts.offering_group_asset.key(),
        &account_infos,
        combined_signer_seeds,
        offering_name.clone(),
        service_offering.is_transferrable,
    )?;

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.vendor.to_account_info(),
            },
        ),
        service_offering.sol_price,
    )?;

    service_offering.increment_sold()?;

    Ok(())
}

fn create_asset(
    asset_key: &Pubkey,
    payer_key: &Pubkey,
    authority_key: &Pubkey,
    system_program_key: &Pubkey,
    group_asset_key: &Pubkey,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]; 1],
    asset_name: String,
    is_transferrable: bool,
) -> Result<()> {
    let standard = if is_transferrable {
        Standard::NonFungible
    } else {
        Standard::Soulbound
    };

    let create_ix = CreateBuilder::new()
        .asset(*asset_key)
        .authority(*authority_key, true)
        .owner(*payer_key)
        .group(Some(*group_asset_key))
        .payer(Some(*payer_key))
        .system_program(Some(*system_program_key))
        .name(asset_name)
        .standard(standard)
        .mutable(true)
        .instruction();

    invoke_signed(&create_ix, account_infos, signer_seeds)?;

    Ok(())
}

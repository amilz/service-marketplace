use crate::{Listing, ListingError, SEED_LISTING};
use anchor_lang::{
    prelude::*,
    solana_program::program:: invoke_signed,
    system_program::{transfer, Transfer},
};

use nifty_asset::{
    accounts::Asset, instructions::{TransferBuilder, UnlockBuilder},  ID as NIFTY_ASSET_PROGRAM_ID
};

#[derive(Accounts)]
pub struct BuyListing<'info> {
    // The buyer
    #[account(mut)]
    pub buyer: Signer<'info>,

    // The public key of the seller
    #[account(mut)]
    pub seller: SystemAccount<'info>,

    // The Asset being purchased
    /// CHECK: we are doing some checks in the handler
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    /// CHECK: we are doing some checks in the handler
    #[account(mut)]
    pub group_asset: UncheckedAccount<'info>,

    // New PDA for the listing
    #[account(
        mut,
        close = seller,
        seeds = [
            SEED_LISTING.as_bytes(),
            asset.key().as_ref(),
            seller.key().as_ref(),
        ],
        bump
    )]
    pub listing: Account<'info, Listing>,

    /// CHECK: use address constraint
    #[account(
        address = NIFTY_ASSET_PROGRAM_ID @ ListingError::InvalidOssProgram
    )]
    pub oss_program: UncheckedAccount<'info>,

    // Solana program system account
    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<BuyListing>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    let asset_key = ctx.accounts.asset.key();
    let seller_key = ctx.accounts.seller.key();
    let asset: Asset = Asset::try_from(&ctx.accounts.asset.to_account_info())?;

    require!(listing.is_active(), ListingError::ListingNotActive);
    require!(asset.group.to_option() == Some(ctx.accounts.group_asset.key()), ListingError::InvalidGroup);

    // Process payment

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.buyer.to_account_info(),
                to: ctx.accounts.seller.to_account_info(),
            },
        ),
        listing.price,
    )?;

    // Unlock the asset

    let signer_seeds: &[&[&[u8]]; 1] = &[&[
        SEED_LISTING.as_bytes(),
        asset_key.as_ref(),
        seller_key.as_ref(),
        &[ctx.bumps.listing],
    ]];

    let unlock_account_infos = vec![
        ctx.accounts.asset.to_account_info(),
        listing.to_account_info(),
        ctx.accounts.oss_program.to_account_info(),
    ];

    let unlock_ix = UnlockBuilder::new()
        .asset(ctx.accounts.asset.key())
        .signer(listing.key())
        .instruction();

    invoke_signed(&unlock_ix, &unlock_account_infos, signer_seeds)?;

    // Transfer the asset to the buyer

    let transfer_account_infos = vec![
        ctx.accounts.asset.to_account_info(),
        listing.to_account_info(),
        ctx.accounts.buyer.to_account_info(),
        ctx.accounts.oss_program.to_account_info(),
        ctx.accounts.group_asset.to_account_info(),
    ];

    let transfer_ix = TransferBuilder::new()
        .asset(ctx.accounts.asset.key())
        .signer(listing.key())
        .recipient(ctx.accounts.buyer.key())
        .group(Some(ctx.accounts.group_asset.key()))
        .instruction();

    invoke_signed(&transfer_ix, &transfer_account_infos, signer_seeds)?;

    Ok(())
}

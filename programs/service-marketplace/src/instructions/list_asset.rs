use crate::{Listing, ListingError, SEED_LISTING};
use anchor_lang::{
    prelude::*,
    solana_program::program::{invoke, invoke_signed},
};
use nifty_asset::{
    accounts::Asset, instructions::{ApproveBuilder, LockBuilder}, types::{DelegateInput, DelegateRole, Standard, State}, ID as NIFTY_ASSET_PROGRAM_ID
};

#[derive(Accounts)]
pub struct ListAsset<'info> {
    // The seller
    #[account(mut)]
    pub seller: Signer<'info>,

    // The Asset being listed
    /// CHECK: we are doing some checks in the handler
    #[account(mut)]
    pub asset: UncheckedAccount<'info>,

    // New PDA for the listing
    #[account(
        init,
        space = Listing::get_size(),
        payer = seller,
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

pub fn handler(ctx: Context<ListAsset>, price: u64, expires_at: Option<i64>) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    let asset_key = ctx.accounts.asset.key();
    let seller_key = ctx.accounts.seller.key();

    let asset: Asset = Asset::try_from(&ctx.accounts.asset.to_account_info())?;
    let standard = asset.standard;
    let owner = asset.owner;

    require!(
        standard != Standard::Soulbound,
        ListingError::AssetIsSoulbound
    );
    require!(asset.state == State::Unlocked, ListingError::AssetIsLocked);
    require_keys_eq!(owner, ctx.accounts.seller.key());

    let approve_ix = ApproveBuilder::new()
        .asset(ctx.accounts.asset.key())
        .owner(ctx.accounts.seller.key())
        .delegate(listing.key())
        .delegate_input(DelegateInput::Some {
            roles: vec![DelegateRole::Transfer, DelegateRole::Lock],
        })
        .instruction();

    invoke(
        &approve_ix,
        &[
            ctx.accounts.asset.to_account_info(),
            ctx.accounts.seller.to_account_info(),
            listing.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    let signer_seeds: &[&[&[u8]]; 1] = &[&[
        SEED_LISTING.as_bytes(),
        asset_key.as_ref(),
        seller_key.as_ref(),
        &[ctx.bumps.listing],
    ]];

    let account_infos = vec![
        ctx.accounts.asset.to_account_info(),
        listing.to_account_info(),
        ctx.accounts.oss_program.to_account_info(),
    ];

    let lock_ix = LockBuilder::new()
        .asset(ctx.accounts.asset.key())
        .signer(listing.key())
        .instruction();
    invoke_signed(&lock_ix, &account_infos, signer_seeds)?;

    listing.create(
        ctx.accounts.seller.key(),
        ctx.accounts.asset.key(),
        price,
        expires_at,
        ctx.bumps.listing,
    );

    Ok(())
}

use anchor_lang::{
    prelude::*,
    solana_program::{instruction::Instruction, program::invoke_signed},
};

use crate::{
    ServiceOffering, ServiceOfferingError, SEED_SERVICE_OFFERING, SEED_SERVICE_OFFERING_GROUP,
};

use nifty_asset::{
    constraints::EmptyBuilder,
    extensions::{
        CreatorsBuilder, ExtensionBuilder, GroupingBuilder, LinksBuilder, MetadataBuilder,
        RoyaltiesBuilder,
    },
    instructions::{AllocateBuilder, CreateBuilder},
    types::{ExtensionInput, ExtensionType, Standard},
    ID as NIFTY_ASSET_PROGRAM_ID,
};

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
        address = NIFTY_ASSET_PROGRAM_ID @ ServiceOfferingError::InvalidOssProgram
    )]
    pub oss_program: UncheckedAccount<'info>,

    // Solana program system account
    pub system_program: Program<'info, System>,
}

pub fn handler(
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
    let service_offering = &mut ctx.accounts.service_offering;
    let vendor_key = ctx.accounts.vendor.key();
    let service_offering_key = service_offering.key();

    service_offering.create(
        vendor_key,
        ctx.accounts.offering_group_asset.key(),
        max_quantity,
        sol_price,
        expires_at,
        is_transferrable,
        ctx.bumps.service_offering,
    );

    let service_offering_seeds = &[
        SEED_SERVICE_OFFERING.as_bytes(),
        vendor_key.as_ref(),
        offering_name.as_bytes(),
        &[ctx.bumps.service_offering],
    ];
    let asset_seeds = &[
        SEED_SERVICE_OFFERING_GROUP.as_bytes(),
        service_offering_key.as_ref(),
        &[ctx.bumps.offering_group_asset],
    ];

    let combined_signer_seeds = &[&asset_seeds[..], &service_offering_seeds[..]];

    let account_infos = vec![
        ctx.accounts.vendor.to_account_info(),
        ctx.accounts.offering_group_asset.to_account_info(),
        ctx.accounts.system_program.to_account_info(),
        ctx.accounts.oss_program.to_account_info(),
        service_offering.to_account_info(),
    ];

    write_metadata(
        symbol.clone(),
        description.clone(),
        uri.clone(),
        image.clone(),
        &ctx.accounts.offering_group_asset.key(),
        &vendor_key,
        &ctx.accounts.system_program.key(),
        &account_infos,
        combined_signer_seeds,
    )?;

    add_royalties(
        &ctx.accounts.offering_group_asset.key(),
        &vendor_key,
        &ctx.accounts.system_program.key(),
        &account_infos,
        combined_signer_seeds,
        royalty_basis_points,
    )?;

    add_terms_of_service(
        &ctx.accounts.offering_group_asset.key(),
        &vendor_key,
        &ctx.accounts.system_program.key(),
        &account_infos,
        combined_signer_seeds,
        terms_of_service_uri.clone(),
    )?;

    create_group(
        &ctx.accounts.offering_group_asset.key(),
        &vendor_key,
        &ctx.accounts.system_program.key(),
        &account_infos,
        combined_signer_seeds,
    )?;

    create_asset(
        &ctx.accounts.offering_group_asset.key(),
        &vendor_key,
        &service_offering.key(),
        &ctx.accounts.system_program.key(),
        &account_infos,
        combined_signer_seeds,
        offering_name.clone(),
    )?;

    Ok(())
}

fn write_metadata(
    symbol: String,
    description: String,
    uri: String,
    image: String,
    asset_key: &Pubkey,
    payer_key: &Pubkey,
    system_program_key: &Pubkey,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]; 2],
) -> Result<()> {
    let mut metadata_builder = MetadataBuilder::default();
    metadata_builder.set(Some(&symbol), Some(&description), Some(&uri), Some(&image));

    let metadata: Vec<u8> = metadata_builder.data();

    let links_ix: Instruction = AllocateBuilder::new()
        .asset(*asset_key)
        .payer(Some(*payer_key))
        .system_program(Some(*system_program_key))
        .extension(ExtensionInput {
            extension_type: ExtensionType::Metadata,
            length: metadata.len() as u32,
            data: Some(metadata),
        })
        .instruction();

    invoke_signed(&links_ix, account_infos, signer_seeds)?;

    Ok(())
}

fn create_group(
    asset_key: &Pubkey,
    payer_key: &Pubkey,
    system_program_key: &Pubkey,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]; 2],
) -> Result<()> {
    let mut group_builder = GroupingBuilder::default();
    let group_data = group_builder.data();

    let group_ix: Instruction = AllocateBuilder::new()
        .asset(*asset_key)
        .payer(Some(*payer_key))
        .system_program(Some(*system_program_key))
        .extension(ExtensionInput {
            extension_type: ExtensionType::Grouping,
            length: group_data.len() as u32,
            data: Some(group_data),
        })
        .instruction();

    invoke_signed(&group_ix, account_infos, signer_seeds)?;

    Ok(())
}

fn add_royalties(
    asset_key: &Pubkey,
    payer_key: &Pubkey,
    system_program_key: &Pubkey,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]; 2],
    basis_points: u64,
) -> Result<()> {
    // Part 1: Define the royalty amounts

    let mut royalties_builder = RoyaltiesBuilder::default();
    royalties_builder.set(basis_points, &mut EmptyBuilder::default());
    let royalties_data: Vec<u8> = royalties_builder.data();

    let royalties_ix: Instruction = AllocateBuilder::new()
        .asset(*asset_key)
        .payer(Some(*payer_key))
        .system_program(Some(*system_program_key))
        .extension(ExtensionInput {
            extension_type: ExtensionType::Royalties,
            length: royalties_data.len() as u32,
            data: Some(royalties_data),
        })
        .instruction();

    invoke_signed(&royalties_ix, account_infos, signer_seeds)?;

    // Part 2: Define the creators

    let mut creators = CreatorsBuilder::default();
    creators.add(&payer_key, true, 100); // for now, limit to the creator

    let creators_data = creators.data();

    let creators_ix: Instruction = AllocateBuilder::new()
        .asset(*asset_key)
        .payer(Some(*payer_key))
        .system_program(Some(*system_program_key))
        .extension(ExtensionInput {
            extension_type: ExtensionType::Creators,
            length: creators_data.len() as u32,
            data: Some(creators_data),
        })
        .instruction();

    invoke_signed(&creators_ix, account_infos, signer_seeds)?;


    Ok(())
}


fn create_asset(
    asset_key: &Pubkey,
    payer_key: &Pubkey,
    authority_key: &Pubkey,
    system_program_key: &Pubkey,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]; 2],
    asset_name: String,
) -> Result<()> {
    let create_ix = CreateBuilder::new()
        .asset(*asset_key)
        .authority(*authority_key, false)
        .owner(*payer_key)
        .group(None)
        .payer(Some(*payer_key))
        .system_program(Some(*system_program_key))
        .name(asset_name)
        .standard(Standard::NonFungible)
        .mutable(true) // ALLOW change to royalties, metadata, etc.
        .instruction();

    invoke_signed(&create_ix, account_infos, signer_seeds)?;

    Ok(())
}

fn add_terms_of_service(
    asset_key: &Pubkey,
    payer_key: &Pubkey,
    system_program_key: &Pubkey,
    account_infos: &[AccountInfo],
    signer_seeds: &[&[&[u8]]; 2],
    uri: String,
) -> Result<()> {
    let mut links_builder = LinksBuilder::default();
    links_builder.add("Terms of Service", &uri);
    let links_data: Vec<u8> = links_builder.data();

    let links_ix: Instruction = AllocateBuilder::new()
        .asset(*asset_key)
        .payer(Some(*payer_key))
        .system_program(Some(*system_program_key))
        .extension(ExtensionInput {
            extension_type: ExtensionType::Links,
            length: links_data.len() as u32,
            data: Some(links_data),
        })
        .instruction();

    invoke_signed(&links_ix, account_infos, signer_seeds)?;

    Ok(())
}
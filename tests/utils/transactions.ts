import * as anchor from "@coral-xyz/anchor";
import { OSS_PROGRAM_ID } from "./keys";
import { ServiceMarketplace } from "../../target/types/service_marketplace";

export async function createServiceOffering(program, vendor, offeringDetails, serviceOffering, offeringGroupAsset) {
    return program.methods
        .createServiceOffering(
            offeringDetails.offeringName,
            new anchor.BN(offeringDetails.maxQuantity),
            new anchor.BN(offeringDetails.solPrice),
            offeringDetails.expiresAt,
            offeringDetails.symbol,
            offeringDetails.description,
            offeringDetails.uri,
            offeringDetails.image,
            offeringDetails.royaltyBasisPoints,
            offeringDetails.termsOfServiceUri,
            offeringDetails.isTransferrable,
        )
        .accountsPartial({
            vendor: vendor.publicKey,
            serviceOffering,
            offeringGroupAsset,
            ossProgram: OSS_PROGRAM_ID,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([vendor])
        .rpc({ skipPreflight: true, commitment: "processed" });
}

export async function fetchServiceOffering(program, serviceOffering) {
    return program.account.serviceOffering.fetch(serviceOffering);
}

export async function buyService(program, vendor, offeringDetails, serviceOffering, offeringGroupAsset, buyer, newAsset) {
    const accounts = {
        buyer: buyer.publicKey,
        vendor: vendor.publicKey,
        serviceOffering,
        offeringGroupAsset,
        newAsset: newAsset.publicKey,
        ossProgram: OSS_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
    }

    return program.methods
        .buyService(
            offeringDetails.offeringName,
        )
        .accountsPartial(accounts)
        .signers([buyer, newAsset])
        .rpc({ skipPreflight: true, commitment: "processed" });
}

export async function listAsset(program, listingDetails, seller, asset, listing) {
    const accounts = {
        seller: seller.publicKey,
        asset: asset.publicKey,
        listing,
        ossProgram: OSS_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
    }

    return program.methods
        .listAsset(
            new anchor.BN(listingDetails.solPrice),
            listingDetails.expiresAt,
        )
        .accountsPartial(accounts)
        .signers([seller])
        .rpc({ skipPreflight: true, commitment: "processed" });
}

export async function fetchListing(program: anchor.Program<ServiceMarketplace>, listing) {
    return program.account.listing.fetch(listing);
}


function logAccounts(accounts: Record<string, anchor.web3.PublicKey>) {
    console.log("Account Details:");
    Object.entries(accounts).forEach(([name, pubkey]) => {
        console.log(`${name}: ${pubkey.toString()}`);
    });
}
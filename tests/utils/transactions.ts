import * as anchor from "@coral-xyz/anchor";

export async function createServiceOffering(program, vendor, offeringDetails, serviceOffering, offeringGroupAsset, ossProgramId) {
    return program.methods
      .createServiceOffering(
        offeringDetails.offeringName,
        new anchor.BN(offeringDetails.maxQuantity),
        new anchor.BN(offeringDetails.solPrice),
        offeringDetails.expiresAt,
      )
      .accountsPartial({
        vendor: vendor.publicKey,
        serviceOffering,
        offeringGroupAsset,
        ossProgram: ossProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([vendor])
      .rpc();
  }
  
  export async function fetchServiceOffering(program, serviceOffering) {
    return program.account.serviceOffering.fetch(serviceOffering);
  }
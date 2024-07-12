// tests/utils/pda-utils.ts

import { PublicKey } from "@solana/web3.js";
import { SEED_LISTING, SEED_SERVICE_OFFERING, SEED_SERVICE_OFFERING_GROUP } from "./seeds";

export function findServiceOfferingPDA(
  vendor: PublicKey,
  offeringName: string,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(SEED_SERVICE_OFFERING),
      vendor.toBuffer(),
      Buffer.from(offeringName),
    ],
    programId
  );
}

export function findOfferingGroupAssetPDA(
  serviceOffering: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(SEED_SERVICE_OFFERING_GROUP),
      serviceOffering.toBuffer(),
    ],
    programId
  );
}

export function findListingPDA(
  asset: PublicKey,
  seller: PublicKey,
  programId: PublicKey
): [PublicKey, number] {
  return PublicKey.findProgramAddressSync(
    [
      Buffer.from(SEED_LISTING),
      asset.toBuffer(),
      seller.toBuffer(),
    ],
    programId
  );
}
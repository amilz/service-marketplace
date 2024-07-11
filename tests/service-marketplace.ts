import { setupTest, TestSetup } from "./utils/fixtures";
import { createServiceOffering, fetchServiceOffering } from "./utils/transactions";
import { findOfferingGroupAssetPDA, findServiceOfferingPDA } from "./utils/pdas";
import { assert, expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ServiceMarketplace } from "../target/types/service_marketplace";
import { Program } from "@coral-xyz/anchor";

describe("Service Marketplace", () => {
  let program: Program<ServiceMarketplace>;
  let vendor1: Keypair;

  before(async () => {
    const setup = await setupTest();
    program = setup.program;
    vendor1 = setup.vendor1;
  });
  
  describe("Service Offering Creation", () => {
    const offeringDetails = {
      offeringName: "Test Offering",
      maxQuantity: 10,
      solPrice: LAMPORTS_PER_SOL,
      expiresAt: null,
    };

    let serviceOffering, offeringGroupAsset;

    before(() => {
      [serviceOffering] = findServiceOfferingPDA(vendor1.publicKey, offeringDetails.offeringName, program.programId);
      [offeringGroupAsset] = findOfferingGroupAssetPDA(serviceOffering, program.programId);
    });

    it("should successfully create a new service offering", async () => {
      const tx = await createServiceOffering(program, vendor1, offeringDetails, serviceOffering, offeringGroupAsset);
      assert.ok(tx, "Transaction should be successful");

      const serviceOfferingAccount = await fetchServiceOffering(program, serviceOffering);

      assert.equal(serviceOfferingAccount.vendor.toBase58(), vendor1.publicKey.toBase58(), "Vendor pubkey doesn't match");
      assert.equal(serviceOfferingAccount.maxQuantity.toNumber(), offeringDetails.maxQuantity, "Max quantity doesn't match");
      assert.equal(serviceOfferingAccount.solPrice.toNumber(), offeringDetails.solPrice, "SOL price doesn't match");
      assert.isTrue(serviceOfferingAccount.active, "Service offering should be active");
      assert.equal(serviceOfferingAccount.numSold.toNumber(), 0, "Initial number sold should be 0");
    });

    it.skip("should fail to create a service offering with invalid max quantity", async () => {
      const invalidOfferingDetails = { ...offeringDetails, maxQuantity: -1 };
      // TODO: Add test for invalid inputs
    });

    it.skip("should fail to create a service offering with invalid price", async () => {
      const invalidOfferingDetails = { ...offeringDetails, solPrice: -1 };
      // TODO: Add test for invalid inputs
    });
  });

  // Add more describe blocks for other functionalities
});
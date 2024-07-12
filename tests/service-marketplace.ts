import { setupTest } from "./utils/fixtures";
import { createServiceOffering, fetchServiceOffering, buyService, listAsset, fetchListing, buyListing } from "./utils/transactions";
import { findListingPDA, findOfferingGroupAssetPDA, findServiceOfferingPDA } from "./utils/pdas";
import { assert, expect } from "chai";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ServiceMarketplace } from "../target/types/service_marketplace";
import { Program, BN } from "@coral-xyz/anchor";

describe("Service Marketplace", () => {
  let program: Program<ServiceMarketplace>;
  let vendor1: Keypair;
  let buyer1: Keypair;
  let buyer2: Keypair;

  before(async () => {
    const setup = await setupTest();
    program = setup.program;
    vendor1 = setup.vendor1;
    buyer1 = setup.buyer1;
    buyer2 = setup.buyer2;
  });

  const offeringDetails = {
    offeringName: "Test Offering",
    maxQuantity: 10,
    solPrice: LAMPORTS_PER_SOL,
    expiresAt: null,
    symbol: "TEST",
    description: "Test Offering Description",
    uri: "https://test.com",
    image: "https://test.com/image.png",
    royaltyBasisPoints: new BN(100),
    termsOfServiceUri: "https://test.com/tos.pdf",
    isTransferrable: true,
  };

  let serviceOffering, offeringGroupAsset, newAsset, listing;

  before(() => {
    [serviceOffering] = findServiceOfferingPDA(vendor1.publicKey, offeringDetails.offeringName, program.programId);
    [offeringGroupAsset] = findOfferingGroupAssetPDA(serviceOffering, program.programId);
    newAsset = Keypair.generate();
    [listing] = findListingPDA(newAsset.publicKey, buyer1.publicKey, program.programId);

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

    // TODO: add assertions for the group asset

  });

/*   it("should fail to create a service offering with invalid max quantity", async () => {
    const invalidOfferingDetails = { ...offeringDetails, maxQuantity: -1 };
    // TODO: Add test for invalid inputs
  });

  it("should fail to create a service offering with invalid price", async () => {
    const invalidOfferingDetails = { ...offeringDetails, solPrice: -1 };
    // TODO: Add test for invalid inputs
  });
 */

  it("should successfully buy a service offering", async () => {
    const tx = await buyService(program, vendor1, offeringDetails, serviceOffering, offeringGroupAsset, buyer1, newAsset);
    assert.ok(tx, "Transaction should be successful");

    const serviceOfferingAccount = await fetchServiceOffering(program, serviceOffering);

    assert.equal(serviceOfferingAccount.numSold.toNumber(), 1, "Number of sold services should be incremented");
  });

  it("should successfully list an asset", async () => {
    const listingDetails = {
      solPrice: 2 * LAMPORTS_PER_SOL,
      expiresAt: null,
    };

    const tx = await listAsset(program, listingDetails, buyer1, newAsset, listing);
    assert.ok(tx, "Transaction should be successful");

    const listingAccount = await fetchListing(program, listing);

    assert.equal(listingAccount.seller.toBase58(), buyer1.publicKey.toBase58(), "Seller pubkey doesn't match");
    assert.equal(listingAccount.assetId.toBase58(), newAsset.publicKey.toBase58(), "Asset pubkey doesn't match");
    assert.equal(listingAccount.price.toNumber(), listingDetails.solPrice, "Price doesn't match");
  });
  it("should successfully buy a listing", async () => {
    const tx = await buyListing(program, listing, buyer2, newAsset, offeringGroupAsset, buyer1, vendor1);
    assert.ok(tx, "Transaction should be successful");


  });

  // Add more describe blocks for other functionalities
});


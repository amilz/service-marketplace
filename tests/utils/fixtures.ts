
import * as anchor from "@coral-xyz/anchor";
import { ServiceMarketplace } from "../../target/types/service_marketplace";
import { PublicKey, Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { airdropToMultiple } from "../utils/utils";

export interface TestSetup {
  program: anchor.Program<ServiceMarketplace>;
  vendor1: Keypair;
  vendor2: Keypair;
}

export async function setupTest(): Promise<TestSetup> {
  anchor.setProvider(anchor.AnchorProvider.local());
  const program = anchor.workspace.ServiceMarketplace as anchor.Program<ServiceMarketplace>;

  const vendor1 = Keypair.generate();
  const vendor2 = Keypair.generate();

  await airdropToMultiple([vendor1.publicKey, vendor2.publicKey], program.provider.connection, 100 * LAMPORTS_PER_SOL);

  return { program, vendor1, vendor2 };
}
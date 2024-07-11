
import * as anchor from "@coral-xyz/anchor";
import { ServiceMarketplace } from "../../target/types/service_marketplace";
import { Keypair, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { airdropToMultiple } from "../utils/utils";

export async function setupTest() {
  anchor.setProvider(anchor.AnchorProvider.local());
  const program = anchor.workspace.ServiceMarketplace as anchor.Program<ServiceMarketplace>;

  const vendor1 = Keypair.generate();
  const vendor2 = Keypair.generate();
  const ossProgramId = Keypair.generate().publicKey;

  await airdropToMultiple([vendor1.publicKey, vendor2.publicKey], program.provider.connection, 100 * LAMPORTS_PER_SOL);

  return { program, vendor1, vendor2, ossProgramId };
}
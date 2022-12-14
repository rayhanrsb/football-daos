import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FootballDaos } from "../target/types/football_daos";

const main = async () => {
  console.log("🚀 Starting test...")
  // Configure the client to use the local cluster
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.FootballDaos as Program<FootballDaos>;
  // Check that the program is initialised
  const tx = await program.methods.initialize().rpc();
  console.log("📝 Your transaction signature", tx);

  // Step 1 - Create the community mint
  // Generate a keypair for the community mint
  const communityMint = anchor.web3.Keypair.generate();
  console.log("Mint is: " + communityMint.publicKey.toString());

};

const runMain = async () => {
  try {
    await main();
    process.exit(0);
  } catch (error) {
    console.error(error);
    process.exit(1);
  }
};

runMain();

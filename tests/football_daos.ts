import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { FootballDaos } from "../target/types/football_daos";
import {
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddress,
  createInitializeMintInstruction,
} from "@solana/spl-token"; 

const main = async () => {
  console.log("🚀 Starting test...")
  const provider = anchor.AnchorProvider.env()
  // Configure the client to use the local cluster
  anchor.setProvider(provider);
  const program = anchor.workspace.FootballDaos as Program<FootballDaos>;
  // Check that the program is initialised
  const tx = await program.methods.initialize().rpc();
  console.log("📝 Your transaction signature", tx);

  // Step 1 - Create the community mint
  // Generate a random keypair for the mint
  const mintKey: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  // AssociatedTokenAccount for anchor's workspace wallet
  let associatedTokenAccount = undefined;
  const key = anchor.AnchorProvider.env().wallet.publicKey;
  // Get the amount of SOL needed to pay rent for our Token Mint
  const lamports: number = await program.provider.connection.getMinimumBalanceForRentExemption(
    MINT_SIZE
  );

  // Get the ATA for a token and the account that we want to own the ATA (but it might not existing on the SOL network yet)
  associatedTokenAccount = await getAssociatedTokenAddress(
    mintKey.publicKey,
    key
  );

  // Fires a list of instructions
  const mint_tx = new anchor.web3.Transaction().add(
    // Use anchor to create an account from the mint key that we created
    anchor.web3.SystemProgram.createAccount({
      fromPubkey: key,
      newAccountPubkey: mintKey.publicKey,
      space: MINT_SIZE,
      programId: TOKEN_PROGRAM_ID,
      lamports,
    }),
    // Fire a transaction to create our mint account that is controlled by our anchor wallet
    createInitializeMintInstruction(
      mintKey.publicKey, 0, key, key
    ),
    // Create the ATA account that is associated with our mint on our anchor wallet
    createAssociatedTokenAccountInstruction(
      key, associatedTokenAccount, key, mintKey.publicKey
    )
  );

  // sends and create the transaction
  const res = await anchor.AnchorProvider.env().sendAndConfirm(mint_tx, [mintKey]);

  console.log(
    await program.provider.connection.getParsedAccountInfo(mintKey.publicKey)
  );

  console.log("Account: ", res);
  console.log("Mint key: ", mintKey.publicKey.toString());
  console.log("User: ", key.toString());

  // Executes our code to mint our token into our specified ATA
  await program.methods.mintToken().accounts({
    mint: mintKey.publicKey,
    tokenProgram: TOKEN_PROGRAM_ID,
    tokenAccount: associatedTokenAccount,
    authority: key,
  }).rpc();

  // Get minted token amount on the ATA for our anchor wallet
  const minted = (await program.provider.connection.getParsedAccountInfo(associatedTokenAccount)).value.data.parsed.info.tokenAmount.amount;
  console.log(minted + " should equal 10");


  // Step 2 - Create the realm
  const name = "first-football-dao-prototype";

  await program.methods.createRealm(name).accounts({
    mint: mintKey.publicKey,
    user: key,
  }).rpc();

  // Step 3 - Find the created realm address

  const [realmPubkey, _] = await anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode(name),
      program.programId.toBuffer(),
    ],
    program.programId
  );

  console.log(realmPubkey.toString());


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

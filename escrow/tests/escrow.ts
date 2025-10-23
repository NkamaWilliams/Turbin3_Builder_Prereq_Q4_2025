import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token"
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { assert } from "chai";

describe("escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.escrow as Program<Escrow>;

  const maker = provider.wallet;
  const taker = anchor.web3.Keypair.generate();
  
  let mintA: anchor.web3.PublicKey;
  let mintB: anchor.web3.PublicKey;
  let makerAtaA: anchor.web3.PublicKey;
  let takerAtaB: anchor.web3.PublicKey;
  let makerAtaB: anchor.web3.PublicKey;
  let takerAtaA: anchor.web3.PublicKey;

  const SEED = new anchor.BN(1234);
  // let escrowPda: anchor.web3.PublicKey;
  // let escrowBump: number;
  let vault: anchor.web3.PublicKey;

  const DEPOSIT_AMOUNT = 1;
  const RECEIVE_AMOUNT = 2;
  const DECIMALS = 9;
  const AMOUNT_DECIMALS = new anchor.BN(10).pow(new anchor.BN(DECIMALS));

  console.log("Maker:", maker.publicKey.toString());
  console.log("Taker:", taker.publicKey.toString());
  
  const [escrow] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("escrow"), maker.publicKey.toBuffer(), SEED.toBuffer("le", 8)], program.programId);
  before(async() => {
    // Airdrop to maker and taker
    const sig = await provider.connection.requestAirdrop(taker.publicKey, 10_000_000_000);

    // Create mints
    mintA = await createMint(provider.connection, provider.wallet.payer, maker.publicKey, null, DECIMALS);
    mintB = await createMint(provider.connection, provider.wallet.payer, taker.publicKey, null, DECIMALS);

    // Mint to makerAtaA and takerAtaB
    makerAtaA = (await getOrCreateAssociatedTokenAccount(provider.connection, maker.payer, mintA, maker.publicKey)).address;
    await mintTo(provider.connection, maker.payer, mintA, makerAtaA, maker.publicKey, 10_000_000_000);

    takerAtaB = (await getOrCreateAssociatedTokenAccount(provider.connection, taker, mintB, taker.publicKey)).address;
    await mintTo(provider.connection, taker, mintB, takerAtaB, taker.publicKey, 10_000_000_000);

    vault = getAssociatedTokenAddressSync(mintA, escrow, true);

    console.log("mintA:", mintA.toString());
    console.log("mintB:", mintB.toString());
    console.log("makerAtaA:", makerAtaA.toString());
    console.log("takerAtaB:", takerAtaB.toString());
    console.log("vault:", vault.toString());
  })
  

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods
      .make(SEED, new anchor.BN(DEPOSIT_AMOUNT).mul(AMOUNT_DECIMALS), new anchor.BN(RECEIVE_AMOUNT).mul(AMOUNT_DECIMALS))
      .accountsPartial({
        maker: maker.publicKey,
        mintA,
        mintB,
        makerAtaA,
        escrow,
        vault,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID
      })
      .rpc();
    
    const escrowAccount = await program.account.escrow.fetch(escrow);
    console.log("Your transaction signature", tx);
    console.log(escrowAccount);
  });
});

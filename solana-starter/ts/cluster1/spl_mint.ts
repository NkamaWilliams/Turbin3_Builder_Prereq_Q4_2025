import { findAssociatedTokenPda, getCreateAssociatedTokenInstructionAsync, getMintToInstruction, TOKEN_PROGRAM_ADDRESS } from "@solana-program/token";
import wallet from "../turbin3-wallet.json"
import { address, appendTransactionMessageInstructions, assertIsTransactionWithinSizeLimit, createKeyPairSignerFromBytes, createSolanaRpc, createSolanaRpcSubscriptions, createTransactionMessage, getSignatureFromTransaction, pipe, sendAndConfirmTransactionFactory, setTransactionMessageFeePayerSigner, setTransactionMessageLifetimeUsingBlockhash, signTransactionMessageWithSigners } from "@solana/kit";

// Import our keypair from the wallet file
// const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// //Create a Solana devnet connection
// const commitment: Commitment = "confirmed";
// const connection = new Connection("https://api.devnet.solana.com", commitment);

// const token_decimals = 1_000_000n;

const CLUSTER = "https://api.devnet.solana.com";
const CLUSTER_SUBSCRIPTION = "ws://api.devnet.solana.com";


(async () => {
    // Import keypair from wallet file
    const keypair = await createKeyPairSignerFromBytes(new Uint8Array(wallet));
    console.log(keypair.address);
    
    // Create a Solana devnet connection
    const rpc = createSolanaRpc(CLUSTER);
    const rpcSubscriptions = createSolanaRpcSubscriptions(CLUSTER_SUBSCRIPTION);
    
    // Mint address
    const mint = address("CqpdrB5y5Khk4AACcPeRQVGakfGTL4fuFtnZx8hAf5v7");
    try {
        // Create an ATA
        // const ata = ???
        // console.log(`Your ata is: ${ata.address.toBase58()}`);

        // Mint to ATA
        // const mintTx = ???
        // console.log(`Your mint txid: ${mintTx}`);

        // Create ATA
        const createAtaIx = await getCreateAssociatedTokenInstructionAsync({
            payer: keypair,
            mint,
            owner: keypair.address,
        });

        const {value: latestBlockhash} = await rpc.getLatestBlockhash().send();

        const txMessage = pipe(
            createTransactionMessage({version: 0}),
            tx => setTransactionMessageFeePayerSigner(keypair, tx),
            tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
            tx => appendTransactionMessageInstructions([createAtaIx], tx)
        )
        const signedTx = await signTransactionMessageWithSigners(txMessage);
        const txSignature = await getSignatureFromTransaction(signedTx);

        assertIsTransactionWithinSizeLimit(signedTx);

        const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({rpc, rpcSubscriptions});
        await sendAndConfirmTransaction(signedTx, {commitment: "confirmed"});
        console.log("Created ata:", txSignature);

        // Mint to ATA
        const [ata] = await findAssociatedTokenPda({
            mint,
            owner: keypair.address,
            tokenProgram: TOKEN_PROGRAM_ADDRESS
        });

        const mintToIx = getMintToInstruction({
            mint,
            token: ata,
            mintAuthority: keypair.address,
            amount: 100n
        });

        const {value: newLatestBlockhash} = await rpc.getLatestBlockhash().send();

        const mintTxMessage = pipe(
            createTransactionMessage({version: 0}),
            tx => setTransactionMessageFeePayerSigner(keypair, tx),
            tx => setTransactionMessageLifetimeUsingBlockhash(newLatestBlockhash, tx),
            tx => appendTransactionMessageInstructions([mintToIx], tx)
        )
        const signedMintTx = await signTransactionMessageWithSigners(mintTxMessage);
        const mintTxSignature = await getSignatureFromTransaction(signedMintTx);

        assertIsTransactionWithinSizeLimit(signedMintTx);

        await sendAndConfirmTransaction(signedMintTx, {commitment: "confirmed"});
        console.log("Minted to ata:", mintTxSignature);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()

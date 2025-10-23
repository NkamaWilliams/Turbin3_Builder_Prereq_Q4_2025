// import { Keypair, Connection, Commitment, sendAndConfirmTransaction } from '@solana/web3.js';
// import { createMint } from '@solana/spl-token';
import { 
    getInitializeMintInstruction, 
    getMintSize, 
    TOKEN_PROGRAM_ADDRESS 
} from "@solana-program/token";
import wallet from "../turbin3-wallet.json";
import { 
    appendTransactionMessageInstructions, 
    assertIsTransactionWithinSizeLimit, 
    createKeyPairSignerFromBytes, 
    createSolanaRpc, 
    createSolanaRpcSubscriptions, 
    createTransactionMessage, 
    generateKeyPairSigner, 
    getSignatureFromTransaction, 
    pipe, 
    sendAndConfirmTransactionFactory, 
    setTransactionMessageFeePayerSigner, 
    setTransactionMessageLifetimeUsingBlockhash, 
    signTransactionMessageWithSigners
} from "@solana/kit";
import { getCreateAccountInstruction } from "@solana-program/system";

// // Import our keypair from the wallet file
// const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// //Create a Solana devnet connection
// const commitment: Commitment = "confirmed";
// const connection = new Connection("https://api.devnet.solana.com", commitment);

const CLUSTER = "https://api.devnet.solana.com";
const CLUSTER_SUBSCRIPTION = "ws://api.devnet.solana.com";


(async () => {
    // Import keypair from wallet file
    const keypair = await createKeyPairSignerFromBytes(new Uint8Array(wallet));
    console.log(keypair.address);
    
    // Create a Solana devnet connection
    const rpc = createSolanaRpc(CLUSTER);
    const rpcSubscriptions = createSolanaRpcSubscriptions(CLUSTER_SUBSCRIPTION);
    try {
        // Start here
        const mint = await generateKeyPairSigner();
        const space = BigInt(getMintSize());
        const rent = await rpc.getMinimumBalanceForRentExemption(space).send();

        const createAccountIx = getCreateAccountInstruction({
            payer: keypair,
            newAccount: mint,
            lamports: rent,
            space,
            programAddress: TOKEN_PROGRAM_ADDRESS
        });

        const initMintIx = getInitializeMintInstruction({
            mint: mint.address,
            mintAuthority: keypair.address,
            decimals: 0
        });

        const {value: latestBlockhash} = await rpc.getLatestBlockhash().send();

        const txMessage = pipe(
            createTransactionMessage({version: 0}),
            tx => setTransactionMessageFeePayerSigner(keypair, tx),
            tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
            tx => appendTransactionMessageInstructions([createAccountIx, initMintIx], tx)
        )
        const signedTx = await signTransactionMessageWithSigners(txMessage);
        const txSignature = await getSignatureFromTransaction(signedTx);

        assertIsTransactionWithinSizeLimit(signedTx);

        const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({rpc, rpcSubscriptions});
        await sendAndConfirmTransaction(signedTx, {commitment: "confirmed"});

        console.log("Mint address:", mint.address);
        console.log("Transaction signature:", txSignature);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()

// CqpdrB5y5Khk4AACcPeRQVGakfGTL4fuFtnZx8hAf5v7 - Mint Address

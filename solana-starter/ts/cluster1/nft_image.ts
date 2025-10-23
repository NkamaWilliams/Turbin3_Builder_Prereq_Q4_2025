import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"
import { readFile } from "fs/promises"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

// https://arweave.net/<hash>
// https://devnet.irys.xyz/<hash>

(async () => {
    try {
        //1. Load image
        const file = await readFile("/home/liwlnakam/Documents/turbin3/q4_builders_2025/solana-starter/ts/cluster1/cga.png");

        //2. Convert image to generic file.
        const genericFile = createGenericFile(file, "andre");

        //3. Upload image

        const [myUri] = await umi.uploader.upload([genericFile]);

        // const [myUri] = image;
        console.log("Your image URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();

// umi.use(irysUploader({address: "https://devnet.irys.xyz/",}));

// https://gateway.irys.xyz/BJBsP5477mpr4tBwgzuxqiNuxysAdk92Y3eXQ7yeUv7x

// https://gateway.irys.xyz/Cv14H2mfprbtLR3joEYqbKBFMj9CADfb1pDszj5yonzg
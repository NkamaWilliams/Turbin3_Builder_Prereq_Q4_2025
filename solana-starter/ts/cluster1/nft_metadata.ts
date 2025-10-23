import wallet from "../turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://gateway.irys.xyz/Cv14H2mfprbtLR3joEYqbKBFMj9CADfb1pDszj5yonzg";
        const metadata = {
            name: "TheGoatAndre",
            symbol: "TGA",
            description: "No stress",
            image,
            attributes: [
                {trait_type: 'element', value: 'fire'},
                {trait_type: 'rarity', value: 'rare'},
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: []
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
// https://gateway.irys.xyz/8T3wp1DRbumexDU6siKZHtoqxiWiqHUL5comjnCwFyGV

// https://gateway.irys.xyz/2D9BbTzHQBupcPKQmv8V1APiLLwghaUpWmpdQpw3wHLZ
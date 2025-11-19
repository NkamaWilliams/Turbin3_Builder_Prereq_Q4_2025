import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Truthlie } from "../target/types/truthlie";
import { SYSTEM_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/native/system";
import { assert } from "chai";
import { fetchAsset, fetchCollectionV1, isAssetOwner, isFrozen, MPL_CORE_PROGRAM_ID, mplCore } from "@metaplex-foundation/mpl-core"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { publicKey } from "@metaplex-foundation/umi";

/// TODO:
/// Ensure only program upgrade authority that can update player_stats

describe("truthlie", () => {
  const umi = createUmi('http://127.0.0.1:8899');
  umi.use(mplCore());
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.truthlie as Program<Truthlie>;

  // Constants
  const SEED = new anchor.BN(Date.now());
  const MAX_PLAYERS = 2;

  // Generate accounts
  const player = anchor.web3.Keypair.generate();
  const player2 = anchor.web3.Keypair.generate();
  const invalidPlayer = anchor.web3.Keypair.generate();
  const creator = anchor.web3.Keypair.generate();
  const collection = anchor.web3.Keypair.generate();
  const nftAsset = anchor.web3.Keypair.generate();

  // Generate PDAs
  const [playerStatsPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("stats"), player.publicKey.toBuffer()], program.programId);
  const [playerVaultPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"), player.publicKey.toBuffer()], program.programId);
  const [player2StatsPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("stats"), player2.publicKey.toBuffer()], program.programId);
  const [player2VaultPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"), player2.publicKey.toBuffer()], program.programId);
  const [gameSessionPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("session"), SEED.toBuffer("le", 8)], program.programId);
  const [gameVaultPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("vault"), gameSessionPda.toBuffer()], program.programId);
  const [whitelistPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("whitelist")], program.programId);
  const [collectionAuthorityPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("collection_authority"), collection.publicKey.toBuffer()], program.programId);

  // Special accounts
  let programDataAccount: anchor.web3.PublicKey;

  console.log("player", player.publicKey.toString());
  console.log("player2", player2.publicKey.toString());
  console.log("invalidPlayer", invalidPlayer.publicKey.toString());
  console.log("playerStatsPda", playerStatsPda.toString());
  console.log("playerVaultPda", playerVaultPda.toString());
  console.log("gameSessionPda", gameSessionPda.toString());
  console.log("gameVaultPda", gameVaultPda.toString());
  console.log("nft asset", nftAsset.publicKey.toString());
  console.log("nft collection", collection.publicKey.toString());

  before("Fund Necessary Accounts", async () => {
    await provider.connection.requestAirdrop(player.publicKey, 2_000_000_000);
    await provider.connection.requestAirdrop(player2.publicKey, 2_000_000_000);
    await provider.connection.requestAirdrop(invalidPlayer.publicKey, 2_000_000_000);

    const BPF_LOADER_UPGRADEABLE_PROGRAM_ID = new anchor.web3.PublicKey("BPFLoaderUpgradeab1e11111111111111111111111");
    programDataAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [
        program.programId.toBuffer(),
      ],
      BPF_LOADER_UPGRADEABLE_PROGRAM_ID
    )[0];
    console.log(`programDataAccount ${programDataAccount.toString()}`);
    // Verify ProgramData exists after deployment
    const programData = await provider.connection.getAccountInfo(programDataAccount);
    assert.ok(programData, "ProgramData should exist after deployment");
  });

  describe("Whitelist NFT Creator and Create Collection", () => {
    it("Whitelist", async () => {
      const tx = await program.methods.whitelistCreator()
        .accountsStrict({
          admin: provider.publicKey,
          creator: provider.publicKey,
          whitelistedCreators: whitelistPda,
          programData: programDataAccount,
          systemProgram: SYSTEM_PROGRAM_ID,
          truthlieProgram: program.programId
        })
        .rpc();

      const whitelistedCreators = await program.account.whitelistedCreators.fetch(whitelistPda);
      assert(whitelistedCreators.creators.map(p => p.toString()).includes(provider.publicKey.toString()), "Failed to correctly whitelist creator");
    });

    it("Whitelist - Failed", async () => {
      try {
        const tx = await program.methods.whitelistCreator()
          .accountsStrict({
            admin: player.publicKey,
            creator: provider.publicKey,
            whitelistedCreators: whitelistPda,
            programData: programDataAccount,
            systemProgram: SYSTEM_PROGRAM_ID,
            truthlieProgram: program.programId
          })
          .signers([player])
          .rpc();
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          console.log(err.error.errorMessage);
          assert(err.error.errorCode.code == "NotAuthorized", )
        } else {
          console.log(err);
          assert.fail("Failed with unexpected error");
        }
      }
    });

    it("Create Achievements Collection", async () => {
      const tx = await program.methods.createCollection({
        name: "Truth Lie Achievements",
        uri: "https://truthlieAchievement.png"
      })
        .accountsStrict({
          coreProgram: MPL_CORE_PROGRAM_ID,
          creator: provider.publicKey,
          collection: collection.publicKey,
          whitelistedCreators: whitelistPda,
          systemProgram: SYSTEM_PROGRAM_ID,
          collectionAuthority: collectionAuthorityPda
        })
        .signers([collection])
        .rpc();
      
      const collectionAccount = await fetchCollectionV1(umi, publicKey(collection.publicKey.toString()));
      assert(collectionAccount.numMinted == 0);
    });
  });

  describe("Test Player Instructions", () => {
    it("Initialize Player Account", async () => {
      const tx = await program.methods.createPlayer()
        .accountsStrict({
          systemProgram: SYSTEM_PROGRAM_ID,
          payer: player.publicKey,
          vault: playerVaultPda,
          playerStats: playerStatsPda
        })
        .signers([player])
        .rpc();
      
      const stats = await program.account.playerStats.fetch(playerStatsPda);
      const vault = await provider.connection.getBalance(playerVaultPda);
      
      assert.equal(stats.achievements.length, 0, "There should be no achivements in a new account");
      assert(stats.correctGuesses == 0);
      assert(stats.wrongGuesses == 0);
      assert(stats.gamesPlayed == 0);
      assert(stats.wallet.toString() == player.publicKey.toString());
      assert(stats.vault.toString() == playerVaultPda.toString());
    });

    it("Update Player Stats", async () => {
      const tx = await program.methods.updatePlayer({gamesPlayed: 1, correctGuesses: 5, wrongGuesses: 3})
        .accountsStrict({
          systemProgram: SYSTEM_PROGRAM_ID,
          payer: provider.publicKey,
          player: player.publicKey,
          playerStats: playerStatsPda,
          truthlieProgram: program.programId,
          programData: programDataAccount
        })
        .rpc();
      
      const stats = await program.account.playerStats.fetch(playerStatsPda);
      
      assert(stats.gamesPlayed == 1);
      assert(stats.correctGuesses == 5);
      assert(stats.wrongGuesses == 3);
    });

    it("Update Player Achievements", async () => {
      const tx = await program.methods.unlockAchievements([
        {minimalist: {}}, 
        {existentialCrisis: {}}
      ])
        .accountsStrict({
          systemProgram: SYSTEM_PROGRAM_ID,
          payer: provider.publicKey,
          player: player.publicKey,
          playerStats: playerStatsPda,
          truthlieProgram: program.programId,
          programData: programDataAccount
        })
        .rpc();
      
      const stats = await program.account.playerStats.fetch(playerStatsPda);
      
      assert(stats.achievements.length == 2);
    });

    it("Update Player Achievements - Failed", async () => {
      try {
        const tx = await program.methods.unlockAchievements([
          {believer: {}}
        ])
          .accountsStrict({
            systemProgram: SYSTEM_PROGRAM_ID,
            payer: player.publicKey,
            player: player.publicKey,
            playerStats: playerStatsPda,
            truthlieProgram: program.programId,
            programData: programDataAccount
          })
          .signers([player])
          .rpc();
        
        const stats = await program.account.playerStats.fetch(playerStatsPda);
        
        assert(stats.achievements.length == 2);
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          console.log(err.error.errorMessage);
          assert(err.error.errorCode.code == "NotAuthorized")
        }
      }
    });

    it("Mint Minimalist NFT", async () => {
      const tx = await program.methods.mintNft({
        nftName: "Minimalist",
        nftUri: "https://minimalist.png",
        achievement: {minimalist:{}}
      })
      .accountsStrict({
        playerStats: playerStatsPda,
        asset: nftAsset.publicKey,
        minter: player.publicKey,
        collection: collection.publicKey,
        collectionAuthority: collectionAuthorityPda,
        coreProgram: MPL_CORE_PROGRAM_ID,
        systemProgram: SYSTEM_PROGRAM_ID
      })
      .signers([nftAsset, player])
      .rpc();
      
      const asset = await fetchAsset(umi, publicKey(nftAsset.publicKey.toString()));
      const nftCollection = await fetchCollectionV1(umi, publicKey(collection.publicKey.toString()));
      assert(isAssetOwner(player.publicKey.toString(), asset), "Player is not an owner of Minimalist NFT Asset");
      assert(isFrozen(asset, nftCollection));
    });

    it("Mint Minimalist NFT - Failed", async () => {
      await new Promise(resolve => setTimeout(resolve, 3000));
      const newAsset = anchor.web3.Keypair.generate();
      try {
        const tx = await program.methods.mintNft({
          nftName: "Minimalist",
          nftUri: "https://minimalist.png",
          achievement: {minimalist:{}}
        })
        .accountsStrict({
          playerStats: playerStatsPda,
          asset: newAsset.publicKey,
          minter: player.publicKey,
          collection: collection.publicKey,
          collectionAuthority: collectionAuthorityPda,
          coreProgram: MPL_CORE_PROGRAM_ID,
          systemProgram: SYSTEM_PROGRAM_ID
        })
        .signers([newAsset, player])
        .rpc();
        
        assert.fail("TX should fail since nft has been minted");
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          console.log(err.error.errorMessage);
          assert(err.error.errorCode.code == "AchievementAlreadyMinted", "This should have failed since the asset has already been minted");
        } else {
          console.log(err);
          assert.fail("Failed with unexpected error");
        }
      }
    });

    // it("Close Player Accounts", async () => {
    //   const tx = await program.methods.closePlayer()
    //     .accountsStrict({
    //       systemProgram: SYSTEM_PROGRAM_ID,
    //       payer: player.publicKey,
    //       vault: playerVaultPda,
    //       playerStats: playerStatsPda
    //     })
    //     .signers([player])
    //     .rpc();
    //   try {
    //     await program.account.playerStats.fetch(playerStatsPda);
        
    //   } catch (err) {}
      
    //   try {
    //     await provider.connection.getBalance(playerVaultPda);
    //   } catch (err) {}
    // });
  });

  describe("Test Lobby", () => {
    it("Initialize Game Session", async () => {
      const tx = await program.methods.createSession(SEED, {
        maxPlayers: MAX_PLAYERS,
        stakeAmount: new anchor.BN(10_000_000),
        rounds: 4
      })
        .accountsStrict({
          systemProgram: SYSTEM_PROGRAM_ID,
          host: player.publicKey,
          gameVault: gameVaultPda,
          gameSession: gameSessionPda
        })
        .signers([player])
        .rpc();
      
      const session = await program.account.gameSession.fetch(gameSessionPda);
      const vault = await provider.connection.getBalance(gameVaultPda);
      const players = session.players.map(player => player.pubkey.toString());

      console.log("Players Info", players);
      
      assert(session.maxPlayers == MAX_PLAYERS, "Max players does not match");
      assert(players.includes(player.publicKey.toString()), "Host not found in session players");
      assert(vault == 10_000_000);
    });

    it("Join Game Session - Fail", async () => {
      try {
        const tx = await program.methods.joinSession()
          .accountsStrict({
            systemProgram: SYSTEM_PROGRAM_ID,
            player: player.publicKey,
            gameVault: gameVaultPda,
            gameSession: gameSessionPda
          })
          .signers([player])
          .rpc();

        assert.fail("This test is supposed to fail since the host is already registered")
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          console.error(err.error.errorMessage);
          assert(err.error.errorCode.code ==  "PlayerAlreadyInGame");
        }
        else {
          console.error(err);
          assert.fail("Failed with unexpected error")
        }
      }
    });

    it("Join Game Session - Pass", async () => {
      const tx = await program.methods.joinSession()
        .accountsStrict({
          systemProgram: SYSTEM_PROGRAM_ID,
          player: player2.publicKey,
          gameVault: gameVaultPda,
          gameSession: gameSessionPda
        })
        .signers([player2])
        .rpc();
      
      const session = await program.account.gameSession.fetch(gameSessionPda);
      const vault = await provider.connection.getBalance(gameVaultPda);
      const players = session.players.map(player => player.pubkey.toString());

      console.log("Players Info 2", players);
      
      assert(players.includes(player2.publicKey.toString()), "Player2 not found in session players");
      assert(vault == 20_000_000);
    });

    it("Join Game Session - Fail", async () => {
      try {
        const tx = await program.methods.joinSession()
          .accountsStrict({
            systemProgram: SYSTEM_PROGRAM_ID,
            player: invalidPlayer.publicKey,
            gameVault: gameVaultPda,
            gameSession: gameSessionPda
          })
          .signers([invalidPlayer])
          .rpc();

        assert.fail("This test is supposed to fail since the session should only allow 2 players")
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          console.error(err.error.errorMessage);
          assert(err.error.errorCode.code ==  "GameLobbyFull");
        }
        else {
          console.error(err);
          assert.fail("Failed with unexpected error")
        }
      }
    });
  });

  describe("Test Game End", () => {
    before("Create player2 accounts", async () => {
      const tx = await program.methods.createPlayer()
        .accountsStrict({
          systemProgram: SYSTEM_PROGRAM_ID,
          payer: player2.publicKey,
          vault: player2VaultPda,
          playerStats: player2StatsPda
        })
        .signers([player2])
        .rpc();
      
      const stats = await program.account.playerStats.fetch(player2StatsPda);
      
      assert.equal(stats.achievements.length, 0, "There should be no achivements in a new account");
      assert(stats.correctGuesses == 0);
      assert(stats.wrongGuesses == 0);
      assert(stats.gamesPlayed == 0);
      assert(stats.wallet.toString() == player2.publicKey.toString());
      assert(stats.vault.toString() == player2VaultPda.toString());
    });

    it("End Session - Fail (No vaults provided)", async () => {
      const input = {
        roundsCompleted: 4,
        playerScores: [
          {pubkey: player.publicKey, score: 5},
          {pubkey: player2.publicKey, score: 3},
        ]
      };

      try {
        const tx = await program.methods.endSession(input)
          .accountsStrict({
            systemProgram: SYSTEM_PROGRAM_ID,
            payer: provider.publicKey,
            gameSession: gameSessionPda,
            gameVault: gameVaultPda,
            truthlieProgram: program.programId,
            programData: programDataAccount
          })
          .rpc();
          
        assert.fail("THis should fail since no winner vault were provided");
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          assert(err.error.errorCode.code == "IncorrectAmountOfWinners")
        } else
        assert.fail("Failed with unexpected error")
      }
    });

    it("End Session - Fail (Too many vaults provided)", async () => {
      const input = {
        roundsCompleted: 4,
        playerScores: [
          {pubkey: player.publicKey, score: 5},
          {pubkey: player2.publicKey, score: 3},
        ]
      };

      try {
        const tx = await program.methods.endSession(input)
          .accountsStrict({
            systemProgram: SYSTEM_PROGRAM_ID,
            payer: provider.publicKey,
            gameSession: gameSessionPda,
            gameVault: gameVaultPda,
            truthlieProgram: program.programId,
            programData: programDataAccount
          })
          .remainingAccounts([
            {
              pubkey: playerVaultPda,
              isSigner: false,
              isWritable: true
            },
            {
              pubkey: player2VaultPda,
              isSigner: false,
              isWritable: true
            }
          ])
          .rpc();
          
        assert.fail("THis should fail since too many winner vaults were provided");
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          assert(err.error.errorCode.code == "IncorrectAmountOfWinners")
        } else
        assert.fail("Failed with unexpected error")
      }
    });

    it("End Session - Fail (Wrong vault provided)", async () => {
      const input = {
        roundsCompleted: 4,
        playerScores: [
          {pubkey: player.publicKey, score: 5},
          {pubkey: player2.publicKey, score: 3},
        ]
      };

      try {
        const tx = await program.methods.endSession(input)
          .accountsStrict({
            systemProgram: SYSTEM_PROGRAM_ID,
            payer: provider.publicKey,
            gameSession: gameSessionPda,
            gameVault: gameVaultPda,
            truthlieProgram: program.programId,
            programData: programDataAccount
          })
          .remainingAccounts([
            {
              pubkey: player2VaultPda,
              isSigner: false,
              isWritable: true
            }
          ])
          .rpc();
          
        assert.fail("THis should fail since the wrong winner vault was provided");
      } catch (err) {
        if (err instanceof anchor.AnchorError) {
          assert(err.error.errorCode.code == "InvalidWinner")
        } else
        assert.fail("Failed with unexpected error")
      }
    });

    it("End Session - Pass", async () => {
      const input = {
        roundsCompleted: 4,
        playerScores: [
          {pubkey: player.publicKey, score: 5},
          {pubkey: player2.publicKey, score: 5},
        ]
      };

      const vaultInitial = await provider.connection.getBalance(playerVaultPda);
      const tx = await program.methods.endSession(input)
        .accountsStrict({
          systemProgram: SYSTEM_PROGRAM_ID,
          payer: provider.publicKey,
          gameSession: gameSessionPda,
          gameVault: gameVaultPda,
          truthlieProgram: program.programId,
          programData: programDataAccount
        })
        .remainingAccounts([
          {
            pubkey: playerVaultPda,
            isSigner: false,
            isWritable: true
          },
          {
            pubkey: player2VaultPda,
            isSigner: false,
            isWritable: true
          }
        ])
        .rpc();
      
      const vaultFinal = await provider.connection.getBalance(playerVaultPda);
      const sessionInfo = await program.account.gameSession.fetch(gameSessionPda);
      console.log("End Session Info:",sessionInfo);
      console.log("Inital Vault Balance", vaultInitial);
      console.log("Final Vault Balance", vaultFinal);
      assert(vaultFinal > vaultInitial, "Amount in winner's vault must increase");
    });
  });
});

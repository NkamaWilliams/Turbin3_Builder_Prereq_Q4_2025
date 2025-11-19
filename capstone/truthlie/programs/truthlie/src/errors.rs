use anchor_lang::prelude::*;

#[error_code]
pub enum TruthLieError {
    #[msg("Provided an achievement that does not exist")]
    NoSuchAchievement,
    #[msg("Insufficient balance to pay stake amount")]
    InsufficientBalanceForStake,
    #[msg("Maximum supported players exceeded")]
    MaximumPlayersExceeded,
    #[msg("Maximum players for game session reached")]
    GameLobbyFull,
    #[msg("Player already in game")]
    PlayerAlreadyInGame,
    #[msg("Game with the provided seeds is already initialized")]
    GameSessionAlreadyInitialized,
    #[msg("Stake amount below minimum")]
    StakeAmountTooSmall,
    #[msg("Games must have at least 2 players")]
    InvalidPlayerCount,
    #[msg("Game is still ongoing")]
    GameNotOverYet,
    #[msg("You provided a player that was not part of the target game")]
    PlayerNotPartOfGame,
    #[msg("Invalid winner provided")]
    InvalidWinner,
    #[msg("Failed to provide the correct amount of winners")]
    IncorrectAmountOfWinners,
    #[msg("The player has already minted the achievement")]
    AchievementAlreadyMinted,
    #[msg("Missing Signer")]
    MissingSigner,
    #[msg("Creator List is already full")]
    CreatorListAlreadyFull,
    #[msg("Creator already in white list")]
    CreatorAlreadyInWhitelist,
    #[msg("The payer is not the program's upgrade authority or not approved")]
    NotAuthorized,
    #[msg("The collection has already been initialized")]
    CollectionAlreadyInitialized,
     #[msg("The collection has not been initialized")]
    CollectionNotInitialized,
    #[msg("The asset has already been initialized")]
    AssetAlreadyInitialized,
    #[msg("The collection account provided is invalid")]
    InvalidCollection,
    #[msg("The asset account provided is invalid")]
    InvalidAsset
}
use serde_derive::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEventHubOfferCategory {
    Currencies,
    Tft,
    Loot,
    Borders,
    Skins,
    Chromas,
    Featured,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolCollectionsCollectionsSummonerBackdropType {
    SpecifiedSkin,
    HighestMastery,
    SummonerIcon,
    Default,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolFeaturedModesQueueAvailability {
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashRosterNotifyReason {
    GameRescheduled,
    GameStartFailedOpponent,
    GameStartFailedSummoners,
    GameStartFailed,
    GameStartRetryOpponent,
    GameStartRetrySummoners,
    GameStartRetry,
    TicketCouldNotBeCharged,
    TicketRefunded,
    TicketCharged,
    BannedSmurfOpponent,
    BannedSmurfTeammate,
    BannedSmurf,
    CannotFindMatch,
    BracketRosterReplaced,
    BracketRosterRemoved,
    TierChanged,
    NoShowPing,
    RoundComplete,
    Withdraw,
    VoteWithdrawDismiss,
    VoteWithdrawUpdate,
    OwnerTransfer,
    QueueDodge,
    GameEndError,
    GameStartedError,
    GameStarted,
    GameScheduled,
    GameCompleted,
    PeriodSplit,
    PeriodCancel,
    PhaseBackout,
    PhaseCheckin,
    PhaseReady,
    PhaseUnready,
    RestrictionAutoWin,
    Registered,
    EogPlayerUpdate,
    CheaterDetect,
    ChangePosition,
    BracketReady,
    ByeAutoWin,
    RosterRevokedTicket,
    RosterDeclineTicket,
    RosterAcceptTicket,
    RosterOfferTicket,
    RosterSetTicket,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLoginConfigReadinessEnum {
    Disabled,
    Ready,
    NotReady,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRankedNotificationDisplayType {
    Vignette,
    Modal,
    Toast,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolStoreLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolHonorV2LoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChatLeagueQueueType {
    #[serde(rename = "RANKED_TFT")]
    RankedTft,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTt,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RankedSolo5X5,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyMatchmakingDodgeState {
    StrangerDodged,
    PartyDodged,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMatchmakingGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSummonerLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InviteType {
    None,
    Selfjoin,
    Suggest,
    Friend,
    Freeagent,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolEventHubOfferPromotionType {
    KFeaturedHighlighted,
    KFeatured,
    KNone,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLeaverBusterLeaverBusterNotificationType {
    RankedRestrictedGames,
    OnLockoutWarning,
    PreLockoutWarning,
    Reforming,
    PunishedGamesRemaining,
    PunishmentIncurred,
    TaintedWarning,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolMissionsRewardStrategy {
    Selection,
    Random,
    All,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampSelectChampSelectSwapState {
    Accepted,
    Cancelled,
    Declined,
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSummonerProfilePrivacySetting {
    Public,
    Private,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLoyaltyLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolActiveBoostsItemOwnershipType {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampionsLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyLobbyMatchmakingSearchState {
    ServiceShutdown,
    ServiceError,
    Error,
    Found,
    Searching,
    Canceled,
    AbandonedLowPriorityQueue,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolYourshopItemOwnershipType {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PatcherComponentStateAction {
    Migrating,
    Repairing,
    Patching,
    CheckingForUpdates,
    Idle,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLoginLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderLobbyRemovedFromGameReason {
    ServiceShutdown,
    GameStartError,
    Timeout,
    Other,
    ServiceError,
    Left,
    Disbanded,
    Kicked,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolTftQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LcdsInviteeState {
    Banned,
    Kicked,
    Quit,
    Joined,
    AcceptFailed,
    Accepted,
    Declined,
    Pending,
    Creator,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChatGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampionMasteryLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLootGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TicketOfferState {
    Revoked,
    Rejected,
    Accepted,
    Active,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PlayerFinderEnum {
    Friend,
    Freeagent,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChatLeagueDivision {
    Na,
    V,
    Iv,
    Iii,
    Ii,
    I,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolHonorV2GameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEventHubEventHubType {
    HallOfLegends,
    EventShop,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolGeoinfoLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashTournamentState {
    Results,
    InGame,
    Scouting,
    LockIn,
    Idle,
    Upcoming,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderMatchmakingDodgeWarning {
    Penalty,
    Warning,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ElevationAction {
    FixBrokenPermissions,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentsPaymentsTelemetryState {
    #[serde(rename = "PMCClosed")]
    PmcClosed,
    #[serde(rename = "PMCComplete")]
    PmcComplete,
    #[serde(rename = "PMCOpen")]
    PmcOpen,
    Idle,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChallengesChallengeRequirementMappingName {
    Item,
    ChampionSkin,
    Champion,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubPurchaseOfferOrderStates {
    Success,
    Fail,
    InProgress,
    NotStarted,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolMissionsRewardStatus {
    Fulfilled,
    Pending,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChatConfigReadinessEnum {
    Disabled,
    Ready,
    NotReady,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashMatchmakingDodgeWarning {
    Penalty,
    Warning,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashPlayerState {
    Eliminated,
    BracketRoster,
    RegisteredRoster,
    PendingRoster,
    NoRoster,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClientConfigDepInjectorEntitlementsUpdateType {
    Delete,
    Update,
    Create,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolSuggestedPlayersSuggestedPlayersReason {
    LegacyPlayAgain,
    HonorInteractions,
    VictoriousComrade,
    FriendOfFriend,
    OnlineFriend,
    PreviousPremade,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPatchNotificationId {
    BrokenPermissions,
    NotEnoughDiskSpace,
    DidRestoreClientBackup,
    FailedToWriteError,
    MissingFilesError,
    ConnectionError,
    UnspecifiedError,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMatchmakingMatchmakingSearchState {
    ServiceShutdown,
    ServiceError,
    Error,
    Found,
    Searching,
    Canceled,
    AbandonedLowPriorityQueue,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLeaverBusterLeaverPenaltyType {
    QueueLockoutTimer,
    QueueDelayTimer,
    NoPenalty,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLoginLoginConnectionMode {
    RiotClient,
    Partner,
    Legacy,
    Preparing,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolGameSettingsLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLicenseAgreementLicenseServeLocation {
    External,
    Local,
    Preparing,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRewardsGrantStatus {
    Failed,
    Fulfilled,
    PendingSelection,
    PendingFulfillment,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolInventoryLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPremadeVoicePartyMemberRoleEnum {
    Declined,
    Kicked,
    Hold,
    Invited,
    Member,
    Leader,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolVanguardLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RemotingSerializedFormat {
    MsgPack,
    #[serde(rename = "YAML")]
    Yaml,
    #[serde(rename = "JSON")]
    Json,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRankedLeagueDivision {
    Na,
    V,
    Iv,
    Iii,
    Ii,
    I,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLoyaltyLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRewardsRewardStrategy {
    Selection,
    Random,
    All,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolGameQueuesLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderMatchmakingSearchState {
    ServiceShutdown,
    ServiceError,
    Error,
    Found,
    Searching,
    Canceled,
    AbandonedLowPriorityQueue,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LogSeverityLevels {
    Always,
    Error,
    Warning,
    Okay,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolEventHubOfferStates {
    KPurchasing,
    KUnrevealed,
    KUnavailable,
    KAvailable,
    KOwned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Position {
    Unselected,
    Fill,
    Utility,
    Jungle,
    Bottom,
    Middle,
    Top,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetricType {
    Rate,
    Duration,
    Count,
    Percentage,
    Unknown,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolKrShutdownLawPolicyType {
    DisableMatchMaking,
    WarningMessage,
    PlayTime,
    Shutdown,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClashRewardType {
    Toc,
    Vp,
    Loot,
    Logo,
    Frame,
    Flag,
    Trophy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolMissionsGrantStatus {
    Fulfilled,
    PendingSelection,
    PendingFulfillment,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolRegaliaRegaliaCrestType {
    Ranked,
    Prestige,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLoginAccountStateType {
    Generating,
    TransferredOut,
    TransferringIn,
    TransferringOut,
    Enabled,
    Creating,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolActiveBoostsLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPlayerReportSenderGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRankedEosNotificationType {
    SeasonEnded,
    SecondWarning,
    FirstWarning,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolTftTrovesLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyQueueAvailability {
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPftGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLoadoutsLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClashRewardKeyType {
    TocState,
    SeasonFlagCount,
    SeasonVp,
    ThemeVp,
    Points,
    Wins,
    TournamentWinPos,
    LowestPosition,
    TicketType,
    TicketCount,
    Cup,
    Tier,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TicketType {
    Premium,
    Basic,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLeagueSessionLeagueSessionStatus {
    AntiAddictionExpired,
    Duplicated,
    Expired,
    Initialized,
    Uninitialized,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEventHubItemOwnershipType {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolCareerStatsSummonersRiftPosition {
    Support,
    Bottom,
    Mid,
    Jungle,
    Top,
    Unknown,
    All,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChallengesClientState {
    Enabled,
    DarkDisabled,
    DarkHidden,
    Disabled,
    Hidden,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChampSelectLegacyGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolTftPassLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRankedLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PaymentsPaymentsTelemetryTransitions {
    #[serde(rename = "PMCCompleteToIdle")]
    PmcCompleteToIdle,
    #[serde(rename = "PMCClosedToIdle")]
    PmcClosedToIdle,
    #[serde(rename = "PMCOpenToPMCComplete")]
    PmcOpenToPmcComplete,
    #[serde(rename = "PMCOpenToPMCClose")]
    PmcOpenToPmcClose,
    #[serde(rename = "IdleToPMCOpen")]
    IdleToPmcOpen,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubTokenUpsellLockedType {
    Unlocked,
    Locked,
    Unassigned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolHoneyfruitHoneyfruitLinkingStatusError {
    #[serde(rename = "unknown_error")]
    UnknownError,
    #[serde(rename = "service_unavailable")]
    ServiceUnavailable,
    #[serde(rename = "not_signed_in")]
    NotSignedIn,
    #[serde(rename = "no_error")]
    NoError,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootCelebrationType {
    Fullscreen,
    Toast,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TournamentStatusEnum {
    Preresume,
    Paused,
    Cancelled,
    Default,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolChatMessageType {
    System,
    Dm,
    Groupchat,
    Chat,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPlayerPreferencesLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolReplaysGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChallengesNotificationDisplayType {
    Vignette,
    Toast,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashKdaClassification {
    High,
    Average,
    Low,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    None,
    Member,
    Captain,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPerksLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashFoundationError {
    InvalidSimpleStateFlag,
    LolInventoryNotReady,
    GameflowUnavailable,
    DeserializationFailed,
    ClashDisabled,
    ClashNotInitialized,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameQueuesQueueAvailability {
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyLobbyRemovedFromGameReason {
    ServiceShutdown,
    GameStartError,
    Timeout,
    Other,
    ServiceError,
    Left,
    Disbanded,
    Kicked,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEventHubExternalCatalogInventoryOwnership {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolReplaysMetadataState {
    Error,
    Unsupported,
    Lost,
    RetryDownload,
    MissingOrExpired,
    Incompatible,
    Downloading,
    Download,
    Watch,
    Found,
    Checking,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolFeaturedModesLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TracingPhaseImportanceV1 {
    Major,
    Minor,
    Trivial,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RiotMessagingServiceState {
    Connected,
    Connecting,
    Disconnected,
    Disconnecting,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LcdsLoyaltyStateChangeNotificationCategory {
    Disabled,
    Revoke,
    Change,
    Expiry,
    Grant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowQueueAvailability {
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolTftGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSettingsLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolRegaliaLeagueQueueType {
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    RankedTftDoubleUp,
    #[serde(rename = "RANKED_TFT_PAIRS")]
    RankedTftPairs,
    #[serde(rename = "RANKED_TFT_TURBO")]
    RankedTftTurbo,
    #[serde(rename = "RANKED_TFT")]
    RankedTft,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTt,
    #[serde(rename = "CHERRY")]
    Cherry,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RankedSolo5X5,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CapacityEnum {
    Full,
    High,
    Medium,
    Low,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEventHubRewardTrackItemTag {
    Multiple,
    Choice,
    Instant,
    Free,
    Rare,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ClientConfigConfigType {
    Player,
    Public,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolReplaysGameflowWatchPhase {
    WatchFailedToLaunch,
    WatchInProgress,
    WatchStarted,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampSelectChampSelectTradeState {
    Accepted,
    Cancelled,
    Declined,
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootLootMilestoneClaimStatus {
    Failed,
    Completed,
    InProgress,
    NotStarted,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolUserExperienceGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowQueueCustomGameSpectatorPolicy {
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolChatConfigType {
    Player,
    Public,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLootLootRarity {
    Ultimate,
    Mythic,
    Legendary,
    Epic,
    Default,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolHoneyfruitHoneyfruitPublisher {
    Vng,
    Twm,
    Tencent,
    Riot,
    Garena,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolHoneyfruitHoneyfruitLinkingState {
    #[serde(rename = "linked")]
    Linked,
    #[serde(rename = "linking_complete")]
    LinkingComplete,
    #[serde(rename = "in_progress")]
    InProgress,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "confirm_snooze")]
    ConfirmSnooze,
    #[serde(rename = "prompt")]
    Prompt,
    #[serde(rename = "snoozed")]
    Snoozed,
    #[serde(rename = "hidden")]
    Hidden,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolInventoryItemOwnershipType {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEventHubRewardTrackItemStates {
    Selected,
    Unselected,
    Unlocked,
    Locked,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BindingHelpFormat {
    Epytext,
    Full,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolCollectionsItemOwnershipType {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameQueuesQueueCustomGameSpectatorPolicy {
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPftLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolLobbyPartyEogStatusCategory {
    KOnEog,
    KPlayAgain,
    KLeft,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampSelectLegacyLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolHoneyfruitV1ResponseType {
    Error,
    Success,
    Multifactor,
    Signup,
    Healup,
    Auth,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubRewardStrategy {
    Selection,
    Random,
    All,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRewardsSelectGrantStatusResponse {
    Failed,
    Selected,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPerksChampSelectTradeState {
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMatchmakingMatchmakingReadyCheckState {
    Error,
    PartyNotReady,
    StrangerNotReady,
    EveryoneReady,
    InProgress,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolTftEventRewardStrategy {
    Selection,
    Random,
    All,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMatchmakingQueueCustomGameSpectatorPolicy {
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolItemSetsLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLootLootType {
    #[serde(rename = "TFT_Damage_Skin")]
    TftDamageSkin,
    #[serde(rename = "TFT_Map_Skin")]
    TftMapSkin,
    SkinBorder,
    Boost,
    #[serde(rename = "Statstone_Shard")]
    StatstoneShard,
    Statstone,
    #[serde(rename = "Egg_Color")]
    EggColor,
    Egg,
    Companion,
    SummonerIcon,
    #[serde(rename = "Skin_Rental")]
    SkinRental,
    Skin,
    WardSkin,
    Material,
    Currency,
    Chest,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLobbyPartyMemberRoleEnum {
    None,
    Declined,
    Kicked,
    Hold,
    Invited,
    Member,
    Leader,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PatcherNotificationId {
    BrokenPermissions,
    NotEnoughDiskSpace,
    DidRestoreClientBackup,
    FailedToWriteError,
    MissingFilesError,
    ConnectionError,
    UnspecifiedError,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootGrantStatus {
    Failed,
    Fulfilled,
    PendingSelection,
    PendingFulfillment,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetricPriority {
    High,
    Medium,
    Low,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSummonerPlayerNameMode {
    Alias,
    Darkmode,
    Summoner,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootItemOwnershipStatus {
    Owned,
    Rental,
    Free,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolContentTargetingRankedDivision {
    V,
    Iv,
    Iii,
    Ii,
    I,
    Na,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyLobbyInvitationState {
    Error,
    OnHold,
    Kicked,
    Declined,
    Joined,
    Accepted,
    Pending,
    Requested,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubSelectGrantStatusResponse {
    Failed,
    Selected,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMatchmakingMatchmakingReadyCheckResponse {
    Declined,
    Accepted,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubRewardStatus {
    Failed,
    Fulfilled,
    Pending,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RemotingPrivilege {
    Local,
    Admin,
    User,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolSimpleDialogMessagesGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PatcherComponentStateWorkType {
    Disk,
    Network,
    Scanning,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashMatchmakingReadyCheckResponse {
    Declined,
    Accepted,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolTftTeamPlannerGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClientBracketMatchStatus {
    Completed,
    Started,
    Upcoming,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolHoneyfruitHoneyfruitLinkingFailureReason {
    UnhandledServerSideError,
    RequestFailure,
    NotLinked,
    Disabled,
    Degraded,
    BadAuthorizationParam,
    AccessTokenExpired,
    AlreadyLinked,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubRewardTrackItemHeaderType {
    None,
    Free,
    Premium,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ExternalPluginsAvailability {
    Error,
    Recovering,
    Connected,
    Preparing,
    NotAvailable,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashMatchmakingDodgeState {
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolVanguardGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashNotifyReason {
    TeammateUnban,
    TeammateBan,
    MemberBan,
    Unban,
    Ban,
    RevertedRegistration,
    RewardGrantRetry,
    RewardGrantFailed,
    AcceptTicket,
    DeclineTicket,
    RevokedTicket,
    OfferTicket,
    SetTicket,
    Kick,
    CaptainLeave,
    Leave,
    RevokeInvite,
    AcceptInvite,
    DeclineInvite,
    ResentInvite,
    Invite,
    ChangeLft,
    ChangeNametaglogo,
    ChangePosition,
    ChangeShortname,
    ChangeName,
    ChangeLogo,
    OwnerTransfer,
    RosterDelete,
    Dismiss,
    OwnerClose,
    Unready,
    Ready,
    Selfjoin,
    RevokeSelfjoin,
    AcceptSelfjoin,
    DeclineSelfjoin,
    RevokeSuggestion,
    AcceptSuggestion,
    DeclineSuggestion,
    Suggestion,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolHoneyfruitGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyLobbyPartyRewardType {
    None,
    Icon,
    Ip,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRankedMiniseries {
    N,
    L,
    W,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChallengesSource {
    None,
    SummonerService,
    Eternals,
    Loot,
    Clash,
    RankedLeagues,
    ChampionMastery,
    Honor,
    CapInventory,
    Eogd,
    Challenges,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PickModes {
    Done,
    InProgress,
    NotPicking,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolRsoAuthConfigType {
    Player,
    Public,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEndOfGameGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPlayerBehaviorNotificationSource {
    Message,
    ForcedShutdown,
    Login,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolMatchmakingLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolFeaturedModesQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLobbyTeamBuilderChampSelectTradeState {
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashQueueAvailability {
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderMatchmakingReadyCheckState {
    Error,
    PartyNotReady,
    StrangerNotReady,
    EveryoneReady,
    InProgress,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolChatSessionState {
    Shuttingdown,
    Disconnected,
    Loaded,
    Connected,
    Initializing,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TracingModuleThreadingModelV1 {
    KParallel,
    KConcurrent,
    KSequential,
    KDedicated,
    KMainThread,
    KNone,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashTournamentNotifyReason {
    UpdateStatus,
    RevertPhase,
    UpdatePhase,
    AddPhase,
    CancelPeriod,
    CancelTournament,
    UpdateTournament,
    NewTournament,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLobbyLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolInventoryLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEndOfGameLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolTftEventGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashPresenceState {
    Scouting,
    LockedIn,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLootInventoryOwnership {
    F2P,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPerksGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLobbyTeamBuilderChampSelectSwapState {
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolDx9DeprecationDx9DeprecationNotificationType {
    #[serde(rename = "TURN_OFF_DX9_LEGACY_MODE")]
    TurnOffDx9LegacyMode,
    #[serde(rename = "HARDWARE_UPGRADE")]
    HardwareUpgrade,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootRedeemableStatus {
    SkinNotOwned,
    ChampionNotOwned,
    AlreadyRented,
    AlreadyOwned,
    NotRedeemableRental,
    NotRedeemable,
    RedeemableRental,
    Redeemable,
    Unknown,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashClashVisibility {
    Visible,
    Hidden,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootRewardStrategy {
    Selection,
    Random,
    All,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RiotMessagingServiceTokenType {
    Identity,
    Access,
    Unavailable,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMissionsGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClientConfigUpdateType {
    Delete,
    Update,
    Create,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolContentTargetingLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLoginLeagueSessionStatus {
    AntiAddictionExpired,
    Duplicated,
    Expired,
    Initialized,
    Uninitialized,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolNpeTutorialPathTutorialType {
    Reward,
    Card,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampionsLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashRosterMemberState {
    Ready,
    ForcedNotReady,
    NotReady,
    Pending,
    Declined,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolYourshopLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolAccountVerificationLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolContentTargetingQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolCollectionsLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolClashSimpleStateStatus {
    Acknowledged,
    Unacknowledged,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyEligibilityRestrictionCode {
    MmrStandardDeviationTooLarge,
    UserInfoNotAvailable,
    InventoryQueuesInfoNotAvailable,
    InventoryChampsInfoNotAvailable,
    LeaguesInfoNotAvailable,
    SummonerInfoNotAvailable,
    MinorInfoNotAvailable,
    BanInfoNotAvailable,
    TooManyIncompleteSubteamsRestriction,
    #[serde(rename = "QPScarcePositionsNotAvailableRestriction")]
    QpScarcePositionsNotAvailableRestriction,
    #[serde(rename = "QPNonUniquePrimarySlotRestriction")]
    QpNonUniquePrimarySlotRestriction,
    #[serde(rename = "QPInvalidChampionSelectionRestriction")]
    QpInvalidChampionSelectionRestriction,
    #[serde(rename = "QPInvalidPositionSelectionRestriction")]
    QpInvalidPositionSelectionRestriction,
    #[serde(rename = "QPInvalidNumberOfPlayerSlotsRestriction")]
    QpInvalidNumberOfPlayerSlotsRestriction,
    #[serde(rename = "QPPlayerChampionCoverageRestriction")]
    QpPlayerChampionCoverageRestriction,
    #[serde(rename = "QPPartyChampionCoverageRestriction")]
    QpPartyChampionCoverageRestriction,
    #[serde(rename = "QPPlayerPositionCoverageRestriction")]
    QpPlayerPositionCoverageRestriction,
    #[serde(rename = "QPPartyPositionCoverageRestriction")]
    QpPartyPositionCoverageRestriction,
    #[serde(rename = "QPPlayerScarcePositionCoverageRestriction")]
    QpPlayerScarcePositionCoverageRestriction,
    UnknownRestriction,
    SeasonVersionLockout,
    #[serde(rename = "TFTNewPlayerRestriction")]
    TftNewPlayerRestriction,
    QueueEntryNotEntitledRestriction,
    GameVersionNotSupported,
    GameVersionMissing,
    GameVersionMismatch,
    PrerequisiteQueuesNotPlayedRestriction,
    TeamSizeRestriction,
    #[serde(rename = "TeamHighMMRMaxSizeRestriction")]
    TeamHighMmrMaxSizeRestriction,
    PlayerRankedSuspensionRestriction,
    PlayerRankSoloOnlyRestriction,
    PlayerTimePlayedRestriction,
    PlayerMinorRestriction,
    PlayerMinLevelRestriction,
    PlayerMaxLevelRestriction,
    PlayerTimeBasedRankRestriction,
    PlayerGameBasedRankRestriction,
    PlayerLeaverTaintedWarningRestriction,
    PlayerLeaverQueueLockoutRestriction,
    PlayerLeaverBustedRestriction,
    PlayerInGameRestriction,
    PlayerDodgeRestriction,
    PlayerBingeRestriction,
    TeamMinSizeRestriction,
    TeamMaxSizeRestriction,
    TeamSkillRestriction,
    TeamDivisionRestriction,
    PlayerAvailableChampionRestriction,
    PlayerBannedRestriction,
    PlayerTimedRestriction,
    PlayerLevelRestriction,
    QueueUnsupported,
    QueueDisabled,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChatFriendSubscriptionType {
    #[serde(rename = "pending_in")]
    PendingIn,
    #[serde(rename = "pending_out")]
    PendingOut,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PluginThreadingModel {
    Parallel,
    Concurrent,
    Sequential,
    Dedicated,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolTftPassItemOwnershipType {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowGameflowGameDodgeState {
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPatchScdEnabled {
    On,
    Off,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolPremadeVoiceConfigType {
    Player,
    Public,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum AggregationType {
    Average,
    Sum,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRewardsCelebrationType {
    Fullscreen,
    Toast,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChampSelectLegacyChampSelectTradeState {
    Sent,
    Received,
    Invalid,
    Busy,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClientConfigConfigReadinessEnum {
    Disabled,
    Ready,
    NotReady,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolSocialLeaderboardLeagueQueueType {
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    RankedTftDoubleUp,
    #[serde(rename = "RANKED_TFT_PAIRS")]
    RankedTftPairs,
    #[serde(rename = "RANKED_TFT_TURBO")]
    RankedTftTurbo,
    #[serde(rename = "RANKED_TFT")]
    RankedTft,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTt,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RankedSolo5X5,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashMatchmakingReadyCheckState {
    Error,
    PartyNotReady,
    StrangerNotReady,
    EveryoneReady,
    InProgress,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolSummonerAliasAvailabilityCode {
    #[serde(rename = "server_error")]
    ServerError,
    #[serde(rename = "rate_limited")]
    RateLimited,
    #[serde(rename = "name_not_available")]
    NameNotAvailable,
    #[serde(rename = "name_change_forbidden")]
    NameChangeForbidden,
    #[serde(rename = "no_error")]
    NoError,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderMatchmakingDodgeState {
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum RemotingHelpFormat {
    Console,
    Brief,
    Epytext,
    Full,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolMatchHistoryLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootSelectGrantStatusResponse {
    Failed,
    Selected,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolKrShutdownLawShutdownLawStatus {
    CutOff,
    Warning,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashClashState {
    Enabled,
    Disabled,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolContentTargetingGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolChampSelectQuestSkinProductType {
    KTieredSkin,
    KQuestSkin,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolPremadeVoiceSessionStatus {
    OnHold,
    Active,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolTftTrovesLootRarity {
    Ultimate,
    Mythic,
    Legendary,
    Epic,
    Default,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolHeartbeatLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPlayerLevelUpLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubGrantStatus {
    Failed,
    Fulfilled,
    PendingSelection,
    PendingFulfillment,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSummonerProfilePrivacyEnabledState {
    Disabled,
    Enabled,
    Unknown,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolSettingsPublisher {
    Vng,
    Twm,
    Tencent,
    Garena,
    Riot,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolCatalogInventoryOwnership {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSocialLeaderboardLeagueDivision {
    Na,
    V,
    Iv,
    Iii,
    Ii,
    I,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPurchaseWidgetLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMatchmakingMatchmakingDodgeWarning {
    Penalty,
    Warning,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLootRewardStatus {
    Failed,
    Fulfilled,
    Pending,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolFeaturedModesEligibilityRestrictionCode {
    UnknownRestriction,
    QueueEntryNotEntitledRestriction,
    GameVersionNotSupported,
    GameVersionMissing,
    GameVersionMismatch,
    PrerequisiteQueuesNotPlayedRestriction,
    TeamSizeRestriction,
    #[serde(rename = "TeamHighMMRMaxSizeRestriction")]
    TeamHighMmrMaxSizeRestriction,
    PlayerRankedSuspensionRestriction,
    PlayerMinorRestriction,
    PlayerMinLevelRestriction,
    PlayerMaxLevelRestriction,
    PlayerLeaverTaintedWarningRestriction,
    PlayerLeaverQueueLockoutRestriction,
    PlayerLeaverBustedRestriction,
    PlayerInGameRestriction,
    PlayerDodgeRestriction,
    PlayerBingeRestriction,
    TeamMinSizeRestriction,
    TeamMaxSizeRestriction,
    TeamSkillRestriction,
    TeamDivisionRestriction,
    PlayerAvailableChampionRestriction,
    PlayerBannedRestriction,
    PlayerTimedRestriction,
    PlayerLevelRestriction,
    QueueUnsupported,
    QueueDisabled,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubCelebrationType {
    Fullscreen,
    Toast,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ClashRewardTime {
    Eot,
    Eob,
    Eog,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowGameflowWatchPhase {
    WatchFailedToLaunch,
    WatchInProgress,
    WatchStarted,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPreEndOfGameGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLobbyTeamBuilderLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameQueuesLcdsAllowSpectators {
    All,
    Dropinonly,
    Lobbyonly,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolRiotMessagingServiceGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolCareerStatsCareerStatsQueueType {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "quickplay5")]
    Quickplay5,
    #[serde(rename = "rank3flex")]
    Rank3Flex,
    #[serde(rename = "blind3")]
    Blind3,
    #[serde(rename = "aram")]
    Aram,
    #[serde(rename = "blind5")]
    Blind5,
    #[serde(rename = "rank5solo")]
    Rank5Solo,
    #[serde(rename = "rank5flex")]
    Rank5Flex,
    #[serde(rename = "draft5")]
    Draft5,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolTftEventRewardStatus {
    Fulfilled,
    Pending,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LcdsRemovalReason {
    Progressed,
    Destroyed,
    Kicked,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRegaliaLeagueDivision {
    Na,
    V,
    Iv,
    Iii,
    Ii,
    I,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolVanguardVanguardSessionState {
    Error,
    Connected,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolRankedGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolPurchaseWidgetPurchaseOfferOrderStates {
    Success,
    Fail,
    InProgress,
    NotStarted,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolChatFriendRequestDirection {
    Both,
    Out,
    In,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChatMuteType {
    SystemMute,
    SettingsMute,
    PlayerMute,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolFeaturedModesGameflowAvailabilityState {
    Configuration,
    InGameFlow,
    PlayerBanned,
    EligibilityInfoMissing,
    Patching,
    Initializing,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolLobbyLobbyBotDifficulty {
    Riotscript,
    Intro,
    Tutorial,
    Uber,
    Hard,
    Medium,
    Easy,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolTftEventGrantStatus {
    Fulfilled,
    PendingSelection,
    PendingFulfillment,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLoadoutsGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolHoneyfruitHoneyfruitActionType {
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "dismiss_permanently")]
    DismissPermanently,
    #[serde(rename = "dismiss_temporarily")]
    DismissTemporarily,
    #[serde(rename = "dismiss")]
    Dismiss,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyGameflowSampleTag {
    InventoryTokenMissing,
    RankedTokenMissing,
    SummonerTokenMissing,
    UserInfoTokenMissing,
    GameVersionMissing,
    Unregistered,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolAntiAddictionPolicyType {
    AntiAddictionHeartbeat,
    AntiAddictionShutdown,
    AntiAddictionWarning,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolMatchmakingMatchmakingDodgeState {
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolNpeTutorialPathTutorialStatus {
    Completed,
    Unlocked,
    Locked,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEsportStreamNotificationsGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolShutdownShutdownReason {
    PlayerBanned,
    LcuAlphaDisabled,
    PlatformMaintenance,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolFeaturedModesGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameQueuesQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPremadeVoiceGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ClientRequestError {
    #[serde(rename = "WITHDRAW_LOCKOUT")]
    WithdrawLockout,
    #[serde(rename = "WITHDRAW_CANCEL_NOT_ALLOWED")]
    WithdrawCancelNotAllowed,
    #[serde(rename = "WITHDRAW_NOT_ALLOWED")]
    WithdrawNotAllowed,
    #[serde(rename = "VOICE_NOT_AVAILABLE")]
    VoiceNotAvailable,
    #[serde(rename = "TICKET_NOT_SET")]
    TicketNotSet,
    #[serde(rename = "TICKET_OFFER_INVALID_COUNT")]
    TicketOfferInvalidCount,
    #[serde(rename = "TICKET_OFFER_NOT_EXIST")]
    TicketOfferNotExist,
    #[serde(rename = "TICKET_ALREADY_SET")]
    TicketAlreadySet,
    #[serde(rename = "SMS_NOT_VERIFIED")]
    SmsNotVerified,
    #[serde(rename = "SUMMONER_LEVEL_REQUIREMENT_NOT_MET")]
    SummonerLevelRequirementNotMet,
    #[serde(rename = "SUGGEST_INVITEE_NOT_EXIST")]
    SuggestInviteeNotExist,
    #[serde(rename = "ROSTER_DISBAND_NOT_ALLOWED")]
    RosterDisbandNotAllowed,
    #[serde(rename = "ROSTER_ELIMINATED")]
    RosterEliminated,
    #[serde(rename = "PHASE_FULL")]
    PhaseFull,
    #[serde(rename = "PHASE_CANCELLED")]
    PhaseCancelled,
    #[serde(rename = "PENDING_ROSTER_CLOSE")]
    PendingRosterClose,
    #[serde(rename = "PENDING_ROSTER_FULL")]
    PendingRosterFull,
    #[serde(rename = "PENDING_ROSTER_NOT_READY")]
    PendingRosterNotReady,
    #[serde(rename = "OVER_INVITE")]
    OverInvite,
    #[serde(rename = "OVER_SUGGESTION_INVITE")]
    OverSuggestionInvite,
    #[serde(rename = "NO_MORE_RECOMMEND")]
    NoMoreRecommend,
    #[serde(rename = "NO_PERMISSION")]
    NoPermission,
    #[serde(rename = "NOT_ALLOWED_DELETE_TOURNAMENT_REWARD_CONFIG")]
    NotAllowedDeleteTournamentRewardConfig,
    #[serde(rename = "NOT_ALLOWED_DELETE_TOURNAMENT")]
    NotAllowedDeleteTournament,
    #[serde(rename = "NOT_ENOUGH_TICKETS")]
    NotEnoughTickets,
    #[serde(rename = "NOT_SEED_INTO_LEAGUE")]
    NotSeedIntoLeague,
    #[serde(rename = "NOT_INVITEE")]
    NotInvitee,
    #[serde(rename = "NOT_MEMBER")]
    NotMember,
    #[serde(rename = "NOT_CAPTAIN")]
    NotCaptain,
    #[serde(rename = "NO_AVAILABLE_PHASE")]
    NoAvailablePhase,
    #[serde(rename = "NO_SAME_PLAYER")]
    NoSamePlayer,
    #[serde(rename = "MAX_ROSTER_FETCHSIZE")]
    MaxRosterFetchsize,
    #[serde(rename = "MAX_INVITED")]
    MaxInvited,
    #[serde(rename = "INVALID_SEASON")]
    InvalidSeason,
    #[serde(rename = "INVALID_REWARD_CONFIG_NAME")]
    InvalidRewardConfigName,
    #[serde(rename = "INVALID_MATCHSTATUS_FORGAMEEND")]
    InvalidMatchstatusForgameend,
    #[serde(rename = "INVALID_WITHDRAW")]
    InvalidWithdraw,
    #[serde(rename = "INVALID_Tier")]
    InvalidTier,
    #[serde(rename = "INVALID_PLAYER")]
    InvalidPlayer,
    #[serde(rename = "INVALID_MATCHID")]
    InvalidMatchid,
    #[serde(rename = "INVALID_BRACKET")]
    InvalidBracket,
    #[serde(rename = "INVALID_CHECKELIGIBILITY_SIZE")]
    InvalidCheckeligibilitySize,
    #[serde(rename = "INVALID_SHORTNAME")]
    InvalidShortname,
    #[serde(rename = "INVALID_NAME")]
    InvalidName,
    #[serde(rename = "INVALID_LOGOCOLOR")]
    InvalidLogocolor,
    #[serde(rename = "INVALID_LOGO")]
    InvalidLogo,
    #[serde(rename = "INVALID_POSITION")]
    InvalidPosition,
    #[serde(rename = "INVALID_ROSTER_MEMBER_SIZE")]
    InvalidRosterMemberSize,
    #[serde(rename = "INVALID_INVITEE")]
    InvalidInvitee,
    #[serde(rename = "INVALID_PHASE")]
    InvalidPhase,
    #[serde(rename = "INVALID_TOURNAMENT")]
    InvalidTournament,
    #[serde(rename = "INVALID_BUY_BACK")]
    InvalidBuyBack,
    #[serde(rename = "INVALID_ROSTER")]
    InvalidRoster,
    #[serde(rename = "INTERNAL_ERROR")]
    InternalError,
    #[serde(rename = "INACTIVE_PHASE")]
    InactivePhase,
    #[serde(rename = "INACTIVE_REGISTRATION")]
    InactiveRegistration,
    #[serde(rename = "IN_OTHER_PHASE_OF_PERIOD")]
    InOtherPhaseOfPeriod,
    #[serde(rename = "IN_OTHER_PENDINGROSTER")]
    InOtherPendingroster,
    #[serde(rename = "IN_OTHER_ROSTER")]
    InOtherRoster,
    #[serde(rename = "LOGOCOLOR_NOT_ALLOWED")]
    LogocolorNotAllowed,
    #[serde(rename = "LOGO_NOT_ALLOWED")]
    LogoNotAllowed,
    #[serde(rename = "HONOR_INELIGIBILITY")]
    HonorIneligibility,
    #[serde(rename = "FAIL_SUGGESTINVITE")]
    FailSuggestinvite,
    #[serde(rename = "FAIL_INVITE")]
    FailInvite,
    #[serde(rename = "ELIGIBILITY_SERVER_ERROR")]
    EligibilityServerError,
    #[serde(rename = "CLASH_BANNED_INVITEE")]
    ClashBannedInvitee,
    #[serde(rename = "CLASH_BANNED")]
    ClashBanned,
    #[serde(rename = "CAPTAIN_NOT_ALLOWED")]
    CaptainNotAllowed,
    #[serde(rename = "ALREADY_IN_PHASE")]
    AlreadyInPhase,
    #[serde(rename = "ALREADY_VOTE_WITHDRAW")]
    AlreadyVoteWithdraw,
    #[serde(rename = "ALREADY_DECLINED")]
    AlreadyDeclined,
    #[serde(rename = "ALREADY_DECLINE_WITHDRAW")]
    AlreadyDeclineWithdraw,
    #[serde(rename = "ALREADY_SUGGESTED")]
    AlreadySuggested,
    #[serde(rename = "ALREADY_INVITED")]
    AlreadyInvited,
    #[serde(rename = "ALREADY_MEMBER")]
    AlreadyMember,
    #[serde(rename = "ALREADY_IN_GAME")]
    AlreadyInGame,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PluginManagerState {
    PluginsInitialized,
    NotReady,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolLoginConfigType {
    Player,
    Public,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolChampionsQuestSkinProductType {
    KTieredSkin,
    KQuestSkin,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRankedSeverity {
    Alert,
    Warning,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReplayResponseStatus {
    InternalServerError,
    BadRequest,
    Expired,
    NotFound,
    Ok,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderMatchmakingReadyCheckResponse {
    Declined,
    Accepted,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolPremadeVoiceInputMode {
    PushToTalk,
    VoiceActivity,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPurchaseWidgetInventoryOwnership {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChallengesGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRewardsRewardStatus {
    Failed,
    Fulfilled,
    Pending,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolRankedLeagueQueueType {
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    RankedTftDoubleUp,
    #[serde(rename = "RANKED_TFT_PAIRS")]
    RankedTftPairs,
    #[serde(rename = "RANKED_TFT_TURBO")]
    RankedTftTurbo,
    #[serde(rename = "RANKED_TFT")]
    RankedTft,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTt,
    #[serde(rename = "CHERRY")]
    Cherry,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RankedSolo5X5,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyQueueCustomGameSpectatorPolicy {
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolSocialLeaderboardLeagueTierNumValue {
    Challenger,
    Grandmaster,
    Master,
    Diamond,
    Emerald,
    Platinum,
    Gold,
    Silver,
    Bronze,
    Iron,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolSuggestedPlayersGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChatQueueCustomGameSpectatorPolicy {
    AllAllowed,
    FriendsAllowed,
    LobbyAllowed,
    NotAllowed,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PendingRosterInviteeState {
    SelfjoinRevoked,
    SelfjoinDeclined,
    Selfjoin,
    Accepted,
    SuggestRevoked,
    SuggestDeclined,
    Revoked,
    Declined,
    Pending,
    Suggested,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum PluginResourceEventType {
    Delete,
    Update,
    Create,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPatchComponentStateWorkType {
    Disk,
    Network,
    Scanning,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPatchComponentStateAction {
    Migrating,
    Repairing,
    Patching,
    CheckingForUpdates,
    Idle,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolTftEventQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolEventHubLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolTftPassLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLicenseAgreementLicenseAgreementType {
    TermsOfUse,
    PrivacyNotice,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolContentTargetingRankedQueue {
    #[serde(rename = "RANKED_TFT_DOUBLE_UP")]
    RankedTftDoubleUp,
    #[serde(rename = "RANKED_TFT_PAIRS")]
    RankedTftPairs,
    #[serde(rename = "RANKED_TFT_TURBO")]
    RankedTftTurbo,
    #[serde(rename = "RANKED_TFT")]
    RankedTft,
    #[serde(rename = "RANKED_FLEX_TT")]
    RankedFlexTt,
    #[serde(rename = "RANKED_FLEX_SR")]
    RankedFlexSr,
    #[serde(rename = "RANKED_SOLO_5x5")]
    RankedSolo5X5,
    #[serde(rename = "NONE")]
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyPlayerUpdateType {
    #[serde(rename = "RMS")]
    Rms,
    ServiceProxy,
    Direct,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TracingModuleTypeV1 {
    KRemotingSource,
    KFrontEndPlugin,
    KBackendOther,
    KBackEndPlugin,
    KRemoteAppModule,
    KUnknown,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolChampSelectLegacyGameflowGameDodgeState {
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolEventHubInventoryOwnership {
    F2P,
    #[serde(rename = "LOYALTY")]
    Loyalty,
    #[serde(rename = "RENTED")]
    Rented,
    #[serde(rename = "OWNED")]
    Owned,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowPatcherProductStateAction {
    Migrating,
    Repairing,
    Patching,
    CheckingForUpdates,
    Idle,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolClashQueueGameCategory {
    Alpha,
    VersusAi,
    PvP,
    Custom,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolChatChatSessionState {
    Connected,
    Connecting,
    Disconnected,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MetricDataType {
    String,
    Bool,
    Float,
    Uint,
    Int,
    Unknown,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPftGameflowGameDodgeState {
    TournamentDodged,
    StrangerDodged,
    PartyDodged,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolLobbyInvitationType {
    Party,
    Lobby,
    Invalid,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolYourshopPurchaseOfferOrderStates {
    Success,
    Fail,
    InProgress,
    NotStarted,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolGameflowGameflowAvailabilityState {
    EligibilityInfoMissing,
    Configuration,
    InGameFlow,
    PlayerBanned,
    Patching,
    Initializing,
    Available,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolNpeTutorialPathGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolRsoAuthRsoAuthorizationTrustLevel {
    #[serde(rename = "always_verify")]
    AlwaysVerify,
    #[serde(rename = "trusted_device")]
    TrustedDevice,
    #[serde(rename = "always_trusted")]
    AlwaysTrusted,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolTftEventLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolChatAuthType {
    RsoRefresh,
    RsoCreate,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LolRegaliaRegaliaBannerType {
    LastSeasonHighestRank,
    Blank,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolMissionsLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolYourshopLoyaltyStatus {
    Disabled,
    Revoke,
    Change,
    Expiry,
    RewardsGrant,
    Legacy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolGameflowLoginSessionStates {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPlayerBehaviorGameflowPhase {
    TerminatedInError,
    EndOfGame,
    PreEndOfGame,
    WaitingForStats,
    Reconnect,
    InProgress,
    FailedToLaunch,
    GameStart,
    ChampSelect,
    ReadyCheck,
    CheckedIntoTournament,
    Matchmaking,
    Lobby,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum BindingAsyncState {
    Failed,
    Succeeded,
    Cancelled,
    Cancelling,
    Running,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolPremadeVoiceConfigReadinessEnum {
    Disabled,
    Ready,
    NotReady,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LolChatAccountState {
    Dnd,
    Chat,
    Away,
    Mobile,
    Offline,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolRankedRatedTier {
    Orange,
    Purple,
    Blue,
    Green,
    Gray,
    None,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LcdsInvitationState {
    Revoked,
    OnHold,
    Active,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LolChatChatPlatformLoginSessionState {
    Error,
    LoggingOut,
    Succeeded,
    InProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolRsoAuthConfigReadinessEnum {
    Disabled,
    Ready,
    NotReady,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PendingRosterMemberState {
    Kick,
    Left,
    Ready,
    ForcedNotReady,
    NotReady,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum LolLobbyTeamBuilderQueueAvailability {
    DoesntMeetRequirements,
    PlatformDisabled,
    Available,
}

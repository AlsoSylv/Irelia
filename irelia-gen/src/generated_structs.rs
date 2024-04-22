use crate::generated_enums::*;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubXboxSubscriptionStatus {
    active: String,
    subscription_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SLIDoubleDiagnostic")]
#[serde(rename_all = "kebab-case")]
pub struct SliDoubleDiagnostic {
    key: String,
    value: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectTimer {
    internal_now_in_epoch_ms: u64,
    phase: String,
    adjusted_time_left_in_phase: i64,
    total_time_in_phase: i64,
    is_infinite: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPlayerBehaviorReporterFeedbackMessage {
    category: String,
    title: String,
    message: String,
    locale: String,
    key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootLcdsRecipeSlotClientDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeSlotClientDto {
    slot_number: i32,
    query: String,
    quantity_expression: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsStoreEntitlementItem {
    item_id: String,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolItemSetsItemSetItem {
    id: String,
    count: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameDodge {
    state: LolGameflowGameflowGameDodgeState,
    dodge_ids: Vec<u64>,
    phase: LolGameflowGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPublishingSettings {
    publishing_locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfig {
    edge: LolPublishingContentPubHubConfigEdge,
    app_context: LolPublishingContentPubHubConfigAppContext,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesTotalRollsInfoDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTotalRollsInfoDto {
    max_total_rolls: u16,
    total_rolls_counter_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTroves {
    use_display_metadata: bool,
    enabled: bool,
    banner_list: Vec<LolTftTrovesTrovesActiveBanner>,
    v_2_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolDropsTotalRollsInfoDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolDropsTotalRollsInfoDto {
    total_rolls_counter_id: String,
    max_total_rolls: u8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthPublicClientConfig {
    client_id: String,
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectChatRoomDetails {
    multi_user_chat_password: String,
    muc_jwt_dto: LolLobbyTeamBuilderMucJwtDto,
    multi_user_chat_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassRmsXboxSubscriptionChange {
    identity_provider: Vec<String>,
    active: String,
    subscription_id: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeMetadata {
    bonus_descriptions: Vec<LootLcdsLootDescriptionDto>,
    tooltips_disabled: bool,
    guaranteed_descriptions: Vec<LootLcdsLootDescriptionDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct YourshopLcdsClientDynamicConfigurationNotification {
    delta: bool,
    configs: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItem {
    ownership_type: LolCatalogInventoryOwnership,
    tags: Vec<String>,
    description: String,
    image_path: String,
    item_instance_id: String,
    purchase_date: u64,
    owned: bool,
    offer_id: String,
    metadata: Vec<LolCatalogItemMetadataEntry>,
    sale: LolCatalogSale,
    release_date: u64,
    prices: Vec<LolCatalogCatalogPluginPrice>,
    inventory_type: String,
    name: String,
    active: bool,
    inactive_date: u64,
    sub_title: String,
    item_id: i32,
    sub_inventory_type: String,
    quest_skin_info: LolCatalogSkinLineInfo,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkBook {
    pages: Vec<LolPerksPerkPageResource>,
    current_page_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardSummonerIdAndName {
    display_name: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPlayerBehaviorCodeOfConductNotification {
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingPlayerMessagingNotificationResource {
    status: i32,
    account_id: u64,
    id: i32,
    msg_id: String,
    title: String,
    body: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricMetadataNotify {
    hipchat: Vec<MetricMetadataHipchatNotification>,
    email: Vec<String>,
    pagerduty: Vec<MetricMetadataPagerDutyNotification>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBalance {
    currency_type: String,
    amount: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTPlaybookAugmentEffectAmount")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTftPlaybookAugmentEffectAmount {
    format_string: String,
    value: f32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectAction {
    actor_cell_id: i64,
    _type: String,
    id: i64,
    champion_id: i32,
    completed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorPlayerNotificationResource {
    created: String,
    critical: bool,
    data: Value,
    detail_key: String,
    expires: String,
    icon_url: String,
    id: u64,
    background_url: String,
    state: String,
    title_key: String,
    _type: String,
    source: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetRiotMessagingServiceMessage {
    resource: String,
    payload: String,
    version: String,
    timestamp: i64,
    service: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSTeamBan")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsTeamBan {
    pick_turn: u16,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStatstonesPriceInfo {
    currency: String,
    price: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersSuggestedPlayer {
    game_id: u64,
    summoner_id: u64,
    summoner_name: String,
    common_friend_id: u64,
    common_friend_name: String,
    reason: LolSuggestedPlayersSuggestedPlayersReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftGameflowGameData {
    game_id: u64,
    queue: LolTftQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolYourshopLoyaltyRewardsSimplified {
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "xpBoost")]
    xp_boost: i32,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootTFTMapSkinGroupViewModel")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootTftMapSkinGroupViewModel {
    items: Vec<LolLootCosmeticsTftMapSkinViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsLoginSession {
    platform_id: String,
    puuid: String,
    account_id: u64,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGameflowSession {
    game_client: LolLobbyTeamBuilderGameflowGameClient,
    map: LolLobbyTeamBuilderGameflowGameMap,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolMissionsCollectionsOwnership {
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "rental")]
    rental: LolMissionsCollectionsRental,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryPlayerNotification {
    icon_url: String,
    source: String,
    state: String,
    detail_key: String,
    critical: bool,
    title_key: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawAntiAddictionState {
    policy_type: LolKrShutdownLawPolicyType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsFailedJoinPlayer {
    summoner: LcdsSummoner,
    reason_failed: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataChampionSummary {
    skins: Vec<LolCatalogGameDataChampionSkin>,
    id: i64,
    name: String,
    square_portrait_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LCDSGlobalRewards")]
#[serde(rename_all = "camelCase")]
pub struct LcdsGlobalRewards {
    all_champions: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameLoginDataPacket {
    simple_messages: Vec<LolEndOfGameSimpleMessage>,
    platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMucJwtDto {
    jwt: String,
    domain: String,
    target_region: String,
    channel_claim: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGatekeeperRestriction {
    payload: String,
    summoner_id: u64,
    remaining_millis: u32,
    reason: String,
    queue_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationPortalSegmentData {
    single_pull_legendary_webm_path: String,
    multi_pull_epic_webm_path: String,
    single_pull_rare_webm_path: String,
    single_pull_mythic_webm_path: String,
    single_pull_sound_path: String,
    multi_pull_rare_webm_path: String,
    multi_pull_legendary_webm_path: String,
    multi_pull_mythic_webm_path: String,
    single_pull_epic_webm_path: String,
    multi_pull_sound_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStorePageGroupingDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolStorePageGroupingDto {
    hidden: bool,
    grouped: bool,
    items: Vec<LolStoreItemKey>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsChallengesSettings {
    all_missions_completed: bool,
    total_count: i8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatBlockedList {
    blocked: Vec<LolChatBlocked>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubRequestDTO-vector-string")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubRequestDtoVectorString {
    data: Vec<String>,
    metadata: LolEventHubRequestMetadataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatisticsPercentilesResponse {
    champion_id: i32,
    position: LolCareerStatsSummonersRiftPosition,
    season: u32,
    queue_type: LolCareerStatsCareerStatsQueueType,
    rank_tier: String,
    stats: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyCollectionsChampion {
    free_to_play: bool,
    ownership: LolChampSelectLegacyCollectionsOwnership,
    id: i32,
    disabled_queues: Vec<String>,
    active: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsGameDataTFTCosmeticsDefaults")]
#[serde(rename_all = "kebab-case")]
pub struct LolCosmeticsGameDataTftCosmeticsDefaults {
    playbook: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMatchmakingSearchResource {
    ready_check: LolClashMatchmakingReadyCheckResource,
    dodge_data: LolClashMatchmakingDodgeData,
    queue_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyServiceProxyPayload {
    body: String,
    url: String,
    method: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassTFTPassDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassTftPassDto {
    #[serde(rename = "status")]
    status: String,
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "deactivationTimeMS")]
    deactivation_time_ms: i64,
    #[serde(rename = "assetID")]
    asset_id: String,
    #[serde(rename = "premiumTitle")]
    premium_title: String,
    #[serde(rename = "productID")]
    product_id: String,
    #[serde(rename = "currentLevel")]
    current_level: i32,
    #[serde(rename = "storePurchaseIDs")]
    store_purchase_i_ds: Value,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "totalPointsEarned")]
    total_points_earned: i32,
    #[serde(rename = "milestones")]
    milestones: Vec<LolTftPassTftPassMilestoneDto>,
    #[serde(rename = "activiationTimeMS")]
    activiation_time_ms: i64,
    #[serde(rename = "playerID")]
    player_id: String,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "counterID")]
    counter_id: String,
    #[serde(rename = "premiumEntitlementID")]
    premium_entitlement_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalPluginsResource {
    state: ExternalPluginsAvailability,
    error_string: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesGameflowGameData {
    queue: LolChallengesQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerRerollDataBagForClientV1 {
    points_until_next_reroll: i32,
    queue_id: i32,
    maximum_rerolls: i32,
    point_cost_of_reroll: i32,
    reroll_count: i32,
    total_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsChampionSkin {
    champion_id: i32,
    id: i32,
    ownership: LolMissionsCollectionsOwnership,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubReward {
    localizations: Value,
    quantity: i32,
    item_type: String,
    id: String,
    fulfillment_source: String,
    item_id: String,
    media: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatFriend {
    #[serde(rename = "game_name")]
    game_name: String,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "group")]
    group: String,
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "displayGroup")]
    display_group: String,
    #[serde(rename = "game_tag")]
    game_tag: String,
    #[serde(rename = "note")]
    note: String,
    #[serde(rename = "puuid")]
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardSpec {
    name: String,
    quantity: String,
    tier: String,
    bracket: String,
    season_id: String,
    level: String,
    cup: String,
    pedestal: String,
    theme: String,
    gem: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemDetails {
    title: String,
    icon_url: String,
    sub_title: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceDetail {
    item_key: LolPurchaseWidgetItemKey,
    price: LolPurchaseWidgetItemPrice,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventDataModelResponse {
    response_code: i64,
    model_data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MucJwtDto {
    domain: String,
    channel_claim: String,
    jwt: String,
    target_region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleReplayMetadataRequestV2 {
    platform_id: String,
    game_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassTFTPassMilestoneDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPassMilestoneDto {
    total_points_needed_for_milestone: i32,
    points_earned_for_milestone: i32,
    bonus: bool,
    keystone: bool,
    status: String,
    rewards: Vec<LolTftPassTftPassRewardDto>,
    asset_internal_name: String,
    id: String,
    level: i32,
    title: String,
    description: String,
    points_needed_for_milestone: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingRankedStats {
    queues: Vec<LolContentTargetingRankedQueueStats>,
    highest_ranked_entry: LolContentTargetingRankedQueueStats,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolContentTargetingToken {
    entitlements: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterAggregatedStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterAggregatedStatsDto {
    period_stats: Vec<RosterPeriodAggregatedStatsDto>,
    player_stats: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedQueuesAndPuuidsPayload {
    queue_types: Vec<LolRankedLeagueQueueType>,
    summoner_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaInventoryItem {
    is_owned: bool,
    items: Vec<LolRegaliaGameDataRegalia>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolClashSimpleStateFlag {
    status: LolClashSimpleStateStatus,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatRankedStats {
    highest_ranked_entry: LolChatRankedQueueStats,
    ranked_regalia_level: i32,
    highest_previous_season_end_tier: String,
    highest_previous_season_end_division: LolChatLeagueDivision,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGame {
    spectator_slot_limit: u32,
    subcategories: Vec<LolGameQueuesQueueCustomGameSubcategory>,
    queue_availability: LolGameQueuesQueueAvailability,
    spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
    game_server_regions: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmActivationPinRequest {
    one_time_pin: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolSuggestedPlayersSuggestedPlayersConfig {
    max_honor_interaction_players: u64,
    friends_of_friends_enabled: bool,
    max_num_replacements: u64,
    online_friends_limit: u64,
    vicorious_comrades_limit: u64,
    enabled: bool,
    max_num_suggested_players: u64,
    previous_premades_limit: u64,
    friends_of_friends_limit: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsChampionQueueStatsResponse {
    rank_tier: String,
    queue_type: LolCareerStatsCareerStatsQueueType,
    champion_id: i32,
    position: LolCareerStatsSummonersRiftPosition,
    stats: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsESportsAPI_streamgroups")]
#[serde(rename_all = "kebab-case")]
pub struct LolEsportStreamNotificationsESportsApiStreamgroups {
    id: i64,
    title: String,
    live: bool,
    content: String,
    slug: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacySummoner {
    summoner_id: u64,
    summoner_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchPatcherInstallSettings {
    #[serde(rename = "game_patcher")]
    game_patcher: String,
    #[serde(rename = "max_download_speed_mbps")]
    max_download_speed_mbps: u32,
    #[serde(rename = "locales")]
    locales: Vec<String>,
    #[serde(rename = "game_patcher_available")]
    game_patcher_available: bool,
    #[serde(rename = "game_patch_url")]
    game_patch_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyUserInfoToken {
    user_info: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaPreferences {
    selected_prestige_crest: u8,
    preferred_crest_type: String,
    preferred_banner_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLobbyCollectionsOwnership {
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "rental")]
    rental: LolLobbyCollectionsRental,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginServiceProxyResponse {
    service_name: String,
    error: String,
    method_name: String,
    payload: String,
    uuid: String,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopWalletDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopWalletDto {
    puuid: String,
    expires: String,
    balances: Value,
    account_id: i64,
    balances_jwt: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentStateInfo {
    next_state_change_time: i64,
    state: LolClashTournamentState,
    current_phase_id: i64,
    next_phase_id: i64,
    num_remaining_periods: i32,
    tournament_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerPlayerNameState {
    is_tagline_customizable: bool,
    is_alias_change_required: bool,
    is_alias_missing: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubHallOfLegends {
    event_hub_type: String,
    narrative_elements: Vec<LolEventHubNarrativeElement>,
    skin_ids: Vec<String>,
    upsell_background_image_url: String,
    event_pass_bundles_catalog_entry: Vec<LolEventHubCatalogEntry>,
    localized_upsell_tooltip_title: String,
    localized_upsell_tooltip_description: String,
    promotion_banner_image: String,
    progress_end_date: String,
    header_icon_image: String,
    end_date: String,
    start_date: String,
    navbar_icon_image: String,
    localized_upsell_title: String,
    upsell_icon_url: String,
    event_id: String,
    help_modal_image: String,
    progression_purchase_catalog_entry: LolEventHubCatalogEntry,
    localized_name: String,
    inductee_name: String,
    reward_track: LolEventHubRewardTrack,
    localized_upsell_button_text: String,
    upsell_tooltip_background_image_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesLoginDataPacket {
    game_type_configs: Vec<LolGameQueuesQueueGameTypeConfig>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerCreateRequest {
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreServiceWallet {
    balances: Vec<LolStoreServiceBalance>,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowSession {
    game_client: LolGameflowGameflowGameClient,
    map: LolGameflowGameflowGameMap,
    game_dodge: LolGameflowGameflowGameDodge,
    phase: LolGameflowGameflowPhase,
    game_data: LolGameflowGameflowGameData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHovercardcookie {
    value: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLoginSession {
    account_id: u64,
    puuid: String,
    platform_id: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RankedScoutingMemberDTO")]
#[serde(rename_all = "camelCase")]
pub struct RankedScoutingMemberDto {
    player_id: u64,
    champion_scouting_data: Vec<RankedScoutingTopChampionDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTeamOpenState {
    invitation_id: String,
    captain_id: i64,
    open_team: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSummoner {
    name: String,
    sum_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingLoginDataPacket {
    simple_messages: Vec<LolPlayerMessagingSimpleMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberObfuscated {
    country_code: String,
    length: i32,
    ends_with: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesLoginSession {
    summoner_id: u64,
    state: LolFeaturedModesLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolInventoryRiotMessagingServiceMessage {
    service: String,
    resource: String,
    version: String,
    timestamp: i64,
    payload: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2GameflowSession {
    game_data: LolHonorV2GameflowGameData,
    phase: LolHonorV2GameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGamePieceViewModel")]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameTftEndOfGamePieceViewModel {
    price: u32,
    level: u32,
    items: Vec<LolEndOfGameTftEndOfGameItemViewModel>,
    traits: Vec<LolEndOfGameTftEndOfGameTraitViewModel>,
    name: String,
    icon: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolClashEogPlayerUpdateDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolClashEogPlayerUpdateDto {
    winner: bool,
    reward_progress: Vec<ClashRewardDefinition>,
    lowest_position: i32,
    bid: i32,
    game_id: i64,
    tier: i32,
    theme_vp: i32,
    tournament_id: i64,
    earned_rewards: Vec<ClashRewardDefinition>,
    season_vp: i32,
    bracket_size: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsEsportsAPI_highlanderTournaments_brackets")]
#[serde(rename_all = "kebab-case")]
pub struct LolEsportStreamNotificationsEsportsApiHighlanderTournamentsBrackets {
    id: String,
    matches: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolSettingsLCUGameSettingsConfig")]
#[serde(rename_all = "PascalCase")]
pub struct LolSettingsLcuGameSettingsConfig {
    gameplay_enabled: bool,
    sound_enabled: bool,
    replays_enabled: bool,
    interface_enabled: bool,
    hotkeys_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueGameTypeConfig {
    team_champion_pool: bool,
    max_allowable_bans: i32,
    id: i64,
    exclusive_pick: bool,
    advanced_learning_quests: bool,
    allow_trades: bool,
    ban_timer_duration: i32,
    ban_mode: String,
    game_mode_override: String,
    onboard_coop_beginner: bool,
    death_match: bool,
    num_players_per_team_override: i32,
    duplicate_pick: bool,
    post_pick_timer_duration: i32,
    main_pick_timer_duration: i32,
    pick_mode: String,
    cross_team_champion_pool: bool,
    reroll: bool,
    learning_quests: bool,
    battle_boost: bool,
    name: String,
    do_not_remove: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Describes the exposed native API."]
pub struct BindingFullApiHelp {
    types: Vec<BindingFullTypeHelp>,
    events: Vec<BindingFullEventHelp>,
    functions: Vec<BindingFullFunctionHelp>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LCDSChampionReward")]
#[serde(rename_all = "camelCase")]
pub struct LcdsChampionReward {
    champion_id: i32,
    skins: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsCompanion {
    #[serde(rename = "upgrades")]
    upgrades: Vec<String>,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "color")]
    color: String,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "species")]
    species: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "level")]
    level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolKrShutdownLawShutdownLawNotification {
    _type: LolKrShutdownLawShutdownLawStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatsQueryRequest {
    rank_tier: String,
    queue_type: LolCareerStatsCareerStatsQueueType,
    season: u32,
    position: LolCareerStatsSummonersRiftPosition,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderPickOrderSwapV1 {
    id: i32,
    cell_id: i32,
    state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepLinksClientConfigDeepLinksValue {
    launch_lor_url: String,
    launch_lor_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHovercardPatchlineMetadata {
    #[serde(rename = "content_paths")]
    content_paths: Value,
    #[serde(rename = "content_cookies")]
    content_cookies: Vec<LolHovercardContentCookies>,
    #[serde(rename = "product_id")]
    product_id: String,
    #[serde(rename = "id")]
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampion {
    title: String,
    passive: LolChampionsGameDataChampionSpell,
    skins: Vec<LolChampionsGameDataChampionSkin>,
    spells: Vec<LolChampionsGameDataChampionSpell>,
    tactical_info: LolChampionsGameDataChampionTacticalInfo,
    id: i32,
    alias: String,
    roles: Vec<String>,
    ban_vo_path: String,
    choose_vo_path: String,
    name: String,
    square_portrait_path: String,
    stinger_sfx_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolGameflowGameflowGameMap {
    #[serde(rename = "gameModeShortName")]
    game_mode_short_name: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "gameModeName")]
    game_mode_name: String,
    #[serde(rename = "mapStringId")]
    map_string_id: String,
    #[serde(rename = "platformId")]
    platform_id: String,
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "gameMutator")]
    game_mutator: String,
    #[serde(rename = "isRGM")]
    is_rgm: bool,
    #[serde(rename = "platformName")]
    platform_name: String,
    #[serde(rename = "assets")]
    assets: Value,
    #[serde(rename = "categorizedContentBundles")]
    categorized_content_bundles: Value,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "properties")]
    properties: Value,
    #[serde(rename = "gameMode")]
    game_mode: String,
    #[serde(rename = "perPositionDisallowedSummonerSpells")]
    per_position_disallowed_summoner_spells: Value,
    #[serde(rename = "perPositionRequiredSummonerSpells")]
    per_position_required_summoner_spells: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAntiAddictionAntiAddictionState {
    localization_key: String,
    policy_type: LolAntiAddictionPolicyType,
    anti_addiction_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsSignGCORequestDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsSignGcoRequestDto {
    loadout: Value,
    service_to_jwts_map: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsGroupedViewModel {
    groups: Vec<LolCosmeticsCompanionsGroupViewModel>,
    selected_loadout_item: LolCosmeticsCosmeticsCompanionViewModel,
    default_item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowProcessInfo {
    standalone: bool,
    raw_args: Vec<String>,
    pid: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLobbyTFTNPESettingsResource")]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTftnpeSettingsResource {
    games_played: i32,
    games_won: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemDetails {
    icon_url: String,
    description: String,
    title: String,
    sub_title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSelectionStrategyConfig {
    min_selections_allowed: u32,
    max_selections_allowed: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksQueue {
    map_id: i32,
    is_team_builder_managed: bool,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TracingPhaseEndV1 {
    name: String,
    when: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoadoutsGameflowGameData {
    queue: LolLoadoutsQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubItemUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemUiData {
    price: u32,
    inventory_type: String,
    item_id: String,
    quantity: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootTableGdsResource {
    image: String,
    id: String,
    description: String,
    drop_chance: Vec<LolLootLootDropTableEntryGdsResource>,
    description_long: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftPaidBattlepassInfo {
    start_date: u64,
    end_date: u64,
    media: Value,
    premium_entitlement_id: String,
    title: String,
    pc_purchase_requirement: String,
    premium_title: String,
    pass_id: String,
    description: String,
    premium: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatPlayerPreferences {
    _type: String,
    modified: u64,
    data: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolYourshopInventoryCacheEntry {
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
    #[serde(rename = "signedInventoryJwt")]
    signed_inventory_jwt: String,
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOfferRequest {
    purchase_quantity: u32,
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMissionsCollectionsSummonerIcons {
    icons: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolYourshopInventoryItemWithPayload {
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "uuid")]
    uuid: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "ownershipType")]
    ownership_type: LolYourshopItemOwnershipType,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "payload")]
    payload: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerNotificationsPlayerNotificationResource {
    expires: String,
    created: String,
    id: u64,
    background_url: String,
    critical: bool,
    source: String,
    _type: String,
    detail_key: String,
    icon_url: String,
    state: String,
    dismissible: bool,
    data: Value,
    title_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstone {
    completion_value: f32,
    name: String,
    is_retired: bool,
    image_url: String,
    formatted_milestone_level: String,
    description: String,
    bound_champion_item_id: u32,
    category: String,
    next_milestone: String,
    is_epic: bool,
    formatted_value: String,
    is_complete: bool,
    is_featured: bool,
    player_record: LolStatstonesStatstonePlayerRecord,
    statstone_id: String,
    formatted_personal_best: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogEntry {
    content_id: String,
    offer_id: String,
    item_id: i32,
    type_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubRMSPayload")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRmsPayload {
    product_id: String,
    affinities: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHovercardProductMetadataMap {
    products: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesGameflowAvailability {
    is_available: bool,
    state: LolFeaturedModesGameflowAvailabilityState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Represents generic data for an event."]
pub struct BindingGenericEvent {
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTrovesCapCounterBalanceDto {
    active: bool,
    version: i32,
    amount: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueReward {
    party_size_ip_rewards: Vec<i32>,
    is_champion_points_enabled: bool,
    is_xp_enabled: bool,
    is_ip_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootPlayerLootNotification {
    count: i32,
    acknowledged: bool,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneFeaturedRequest {
    index: i32,
    existing_featured: Vec<LolStatstonesStatstone>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopOfferData {
    offers: Vec<LolYourshopOffer>,
    promotion: LolYourshopPromotion,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameGameDataTftTrait {
    name: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRegisteredRosterNotification {
    notify_reason: LolClashRosterNotifyReason,
    roster: RosterDto,
}
type LolLootResponseMetadataDto = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGameflowGameMap {
    per_position_required_summoner_spells: Value,
    per_position_disallowed_summoner_spells: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetValidationResponse {
    valid: bool,
    items: Vec<LolPurchaseWidgetValidationResponseItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedReward {
    dynamic_honor_message: LolHonorV2DynamicHonorMessage,
    reward_type: String,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampSelectChampionSkinAugmentOverlays {
    #[serde(rename = "uncenteredLCOverlayPath")]
    uncentered_lc_overlay_path: String,
    #[serde(rename = "centeredLCOverlayPath")]
    centered_lc_overlay_path: String,
    #[serde(rename = "socialCardLCOverlayPath")]
    social_card_lc_overlay_path: String,
    #[serde(rename = "tileLCOverlayPath")]
    tile_lc_overlay_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHoneyfruitHoneyfruitLinkingAction {
    action: LolHoneyfruitHoneyfruitActionType,
    target: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesMilestoneProgressNotification {
    statstone_id: String,
    threshold: i32,
    image_url: String,
    statstone_name: String,
    level: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerDto {
    lft: bool,
    secondary_pos: String,
    tmnt_losses: i32,
    notifications: Vec<ClashOfflineNotification>,
    tmnt_wins: i32,
    primary_pos: String,
    banned: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardDetails {
    tournament_id: i64,
    roster_id: i64,
    team_member_ids: Vec<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyTeamBuilderTeambuilderLeagueEdgeResponse {
    payload: LolLobbyTeamBuilderTbLobbyStateResource,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeSlot {
    loot_ids: Vec<String>,
    tags: String,
    slot_number: i32,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolMatchHistoryGAMHSMatchHistoryList")]
#[serde(rename_all = "PascalCase")]
pub struct LolMatchHistoryGamhsMatchHistoryList {
    #[serde(rename = "active_puuid")]
    active_puuid: String,
    #[serde(rename = "games")]
    games: Vec<LolMatchHistoryGamhsMatchHistoryData>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneNotificationDto {
    best: i32,
    delta: i32,
    level: i32,
    value: i32,
    statstone_id: String,
    puuid: String,
    is_new_best: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaGameDataRegalia {
    regalia_type: String,
    id_secondary: String,
    asset_path: String,
    localized_name: String,
    is_selectable: bool,
    localized_description: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolReplaysReplaysSettingsData {
    replays_folder_path: String,
    highlights_folder_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingChampions {
    player_id: u64,
    top_season_champions: Vec<LolClashScoutingSeasonChampion>,
    top_masteries: Vec<LolClashScoutingChampionMastery>,
    total_mastery_score: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc = "Represents generic data for an asynchronous event."]
pub struct BindingGenericAsyncEvent {
    data: Value,
    async_token: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolAccountVerificationSendDeactivationPinRequest {
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubMilestoneInstance {
    instance_id: String,
    trigger_value: i64,
    product_id: String,
    owner_id: String,
    group_id: String,
    repeat_sequence: u32,
    triggers: Vec<LolEventHubTrigger>,
    counter_id: String,
    triggered: bool,
    milestone_id: String,
    triggered_timestamp: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginLcdsResponse {
    type_name: String,
    body: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMissionsRewardPackMetaData {
    npe_reward_pack: LolNpeRewardsRewardPack,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsLiveMatch {
    stream_group: String,
    title: String,
    tournament_description: String,
    teams: Vec<LolEsportStreamNotificationsLiveMatchTeam>,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLobbyTeamBuilderTBDMatchmakingState")]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTbdMatchmakingState {
    time_in_matchmaking_millis: i64,
    backwards_transition_reason: String,
    estimated_matchmaking_time_millis: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventTftWeeklyMissions {
    missions: Vec<PlayerMissionDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolClashThirdPartyApiRoster {
    captain: LolClashThirdPartyApiPlayer,
    members: Vec<LolClashThirdPartyApiPlayer>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLeaverBusterNotificationResource {
    id: u32,
    punished_games_remaining: i32,
    queue_lockout_timer_expiry_utc_millis_diff: u64,
    from_rms: bool,
    _type: LolLeaverBusterLeaverBusterNotificationType,
    account_id: u64,
    msg_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "basicOperatingSystemInfo")]
#[serde(rename_all = "camelCase")]
#[doc = "User Experience Settings Operating System Information"]
pub struct BasicOperatingSystemInfo {
    build_number: String,
    platform: String,
    edition: String,
    version_major: String,
    version_minor: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftEventTFTEventMissionChain")]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftEventMissionChain {
    missions: Vec<PlayerMissionDto>,
    chain_index: i32,
    chain_size: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameLoginSession {
    account_id: u64,
    state: LolEndOfGameLoginSessionStates,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTPlaybook")]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsTftPlaybook {
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "lateAugments")]
    late_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "isDisabledInDoubleUp")]
    is_disabled_in_double_up: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "iconPathSmall")]
    icon_path_small: String,
    #[serde(rename = "splashPath")]
    splash_path: String,
    #[serde(rename = "midAugments")]
    mid_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "earlyAugments")]
    early_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "iconPath")]
    icon_path: String,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEntityInstance {
    group_id: String,
    counters: Vec<LolEventHubCounterInstance>,
    milestones: Vec<LolEventHubMilestoneInstance>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryAlias {
    game_name: String,
    tag_line: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPinDropNotification {
    map_side: String,
    pin_drop_summoners: Vec<LolChampSelectChampSelectPinDropSummoner>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPftGameClientEndOfGameStats {
    game_id: u64,
    queue_id: i32,
    is_ranked: bool,
    game_mode: String,
    stats_block: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentSummonerInfo {
    display_name: String,
    summoner_level: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationMessageResource {
    id: String,
    is_historical: bool,
    from_pid: String,
    timestamp: String,
    from_obfuscated_summoner_id: u64,
    _type: String,
    from_summoner_id: u64,
    from_id: String,
    body: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventorySimpleInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolInventorySimpleInventoryDto {
    expires: String,
    items_jwt: String,
    items: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsLcdsRentalUpdateNotification {
    inventory_type: String,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubChampionSkinEmblem {
    emblem_position: LolEventHubChampionSkinEmblemPosition,
    name: String,
    emblem_path: LolEventHubChampionSkinEmblemPath,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionSet {
    completed: bool,
    total_milestone: i32,
    champions: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceKeycodePushToTalkResource {
    key_combos: Vec<LolPremadeVoiceKeyCombo>,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsScopedLoadout {
    id: String,
    scope: String,
    loadout: Value,
    refresh_time: String,
    name: String,
    item_id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassTFTPassRewardNotification")]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassTftPassRewardNotification {
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "framedIcon")]
    framed_icon: bool,
    #[serde(rename = "iconURL")]
    icon_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "CraftLootDTO")]
#[serde(rename_all = "camelCase")]
pub struct CraftLootDto {
    repeat: i32,
    recipe_name: String,
    loot_names: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventSelectionStrategyConfig {
    min_selections_allowed: u32,
    max_selections_allowed: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInventory {
    champions: Vec<i32>,
    skins: Vec<i32>,
    ward_skins: Vec<i64>,
    icons: Vec<i32>,
    inventory_jwts: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubNarrativeElement {
    narrative_background_image: String,
    narrative_starting_track_level: u16,
    narrative_video: LolEventHubNarrativeVideo,
    localized_narrative_title: String,
    localized_narrative_description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubTransactionResponseDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTransactionResponseDto {
    inventory_type: String,
    id: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootRequestDTO-SelectionRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootRequestDtoSelectionRequestDto {
    metadata: LolLootRequestMetadataDto,
    data: LolLootSelectionRequestDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTEvent")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftEvent {
    player_survey_id: u64,
    data: Vec<Value>,
    action: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathLobbyDto {
    party_id: String,
    game_config: LolNpeTutorialPathLobbyGameConfigDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedSeasonDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonDto {
    current_season_id: i32,
    current_season_end: i64,
    next_season_start: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventRewardGroup {
    product_id: String,
    id: String,
    child_reward_group_ids: Vec<String>,
    active: bool,
    rewards: Vec<LolTftEventSvcReward>,
    reward_strategy: LolTftEventRewardStrategy,
    internal_name: String,
    selection_strategy_config: LolTftEventSelectionStrategyConfig,
    media: Value,
    localizations: Value,
    types: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatMultiGamePresence {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "game_tag")]
    game_tag: String,
    #[serde(rename = "details")]
    details: String,
    #[serde(rename = "time")]
    time: u64,
    #[serde(rename = "product")]
    product: String,
    #[serde(rename = "patchline")]
    patchline: String,
    #[serde(rename = "msg")]
    msg: String,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "private")]
    private: String,
    #[serde(rename = "location")]
    location: String,
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "privateJwt")]
    private_jwt: String,
    #[serde(rename = "actor")]
    actor: String,
    #[serde(rename = "summary")]
    summary: String,
    #[serde(rename = "puuid")]
    puuid: String,
    #[serde(rename = "resource")]
    resource: String,
    #[serde(rename = "platform")]
    platform: String,
    #[serde(rename = "state")]
    state: LolChatAccountState,
    #[serde(rename = "game_name")]
    game_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsMatchMakerParams {
    queue_ids: Vec<i32>,
    team_id: u64,
    invitation_id: String,
    languages: String,
    bot_difficulty: String,
    team: Vec<u64>,
    last_maestro_message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryLoginSession {
    state: LolInventoryLoginSessionStates,
    id_token: String,
    summoner_id: u64,
    account_id: u64,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCollectionsInventoryItemDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolCollectionsInventoryItemDto {
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "lsb")]
    lsb: bool,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "usedInGameDate")]
    used_in_game_date: String,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "ownedQuantity")]
    owned_quantity: u64,
    #[serde(rename = "entitlementId")]
    entitlement_id: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "entitlementTypeId")]
    entitlement_type_id: String,
    #[serde(rename = "instanceTypeId")]
    instance_type_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolBannersLoadout {
    name: String,
    scope: String,
    id: String,
    loadout: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryCatalogItem {
    item_id: i32,
    inventory_type: String,
    item_instance_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesCatalogItemDetails {
    inventory_type: String,
    prices: Vec<LolStatstonesCatalogBundlePrice>,
    release_date: String,
    item_instance_id: String,
    sub_inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameGameDataSkinChroma {
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersEndOfGameTeam {
    is_winning_team: bool,
    players: Vec<LolSuggestedPlayersEndOfGamePlayer>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreAccessTokenResource {
    scopes: Vec<String>,
    token: String,
    expiry: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftPaidBattlepassMilestone {
    is_paid: bool,
    status: String,
    points_earned_for_milestone: i32,
    internal_name: String,
    description: String,
    is_keystone: bool,
    state: String,
    is_claim_request_pending: bool,
    mission_id: String,
    is_locked: bool,
    is_bonus: bool,
    level: i32,
    total_points_for_milestone: i32,
    icon_needs_frame: bool,
    rewards: Vec<LolTftEventTftPaidBattlepassReward>,
    points_needed_for_milestone: i32,
    title: String,
    icon_image_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootNameRefId {
    loot_name: String,
    ref_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizeRequest {
    aggressive_scan: bool,
    text: String,
    level: u32,
    texts: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolMatchHistoryMHSummoner")]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMhSummoner {
    account_id: u64,
    puuid: String,
    display_name: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CriticalFlowCapture {
    expected: bool,
    id_chain: String,
    payload_string: String,
    qualifier_chain: String,
    timestamp_in_us: u64,
    succeeded: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClashEventData {
    #[serde(rename = "teamName")]
    team_name: String,
    #[serde(rename = "tier")]
    tier: String,
    #[serde(rename = "earnedDate")]
    earned_date: String,
    #[serde(rename = "theme")]
    theme: String,
    #[serde(rename = "teamShortName")]
    team_short_name: String,
    #[serde(rename = "tournamentId")]
    tournament_id: i64,
    #[serde(rename = "tournamentName")]
    tournament_name: String,
    #[serde(rename = "teamLogoName")]
    team_logo_name: String,
    #[serde(rename = "teamLogoChromaId")]
    team_logo_chroma_id: String,
    #[serde(rename = "rosterId")]
    roster_id: i64,
    #[serde(rename = "playerUUIDs")]
    player_uui_ds: Vec<String>,
    #[serde(rename = "rewardSpec")]
    reward_spec: ClashRewardSpec,
    #[serde(rename = "rewardType")]
    reward_type: String,
    #[serde(rename = "seasonId")]
    season_id: i32,
    #[serde(rename = "bracket")]
    bracket: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreLoginDataPacket {
    all_summoner_data: LolStoreAllSummonerData,
    simple_messages: Vec<LolStoreSimpleDialogMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionMinimal {
    roles: Vec<String>,
    name: String,
    base_splash_path: String,
    choose_vo_path: String,
    ownership: LolChampSelectCollectionsOwnership,
    stinger_sfx_path: String,
    free_to_play: bool,
    disabled_queues: Vec<String>,
    ban_vo_path: String,
    square_portrait_path: String,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatConversationList {
    conversations: Vec<LolChatConversation>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolFeaturedModesGameflowSession {
    phase: LolFeaturedModesGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubResponseDTO-vector-SvcRewardGrant")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubResponseDtoVectorSvcRewardGrant {
    metadata: LolEventHubResponseMetadataDto,
    data: Vec<LolEventHubSvcRewardGrant>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopSimpleInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopSimpleInventoryDto {
    items_jwt: String,
    items: Value,
    expires: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardConfigClient {
    name: String,
    entries: Vec<ClashRewardConfigEntry>,
    key_def: Vec<ClashRewardKeyType>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionQuestSkin {
    collection_splash_video_path: String,
    skin_augments: LolChampionsCollectionsChampionSkinAugments,
    uncentered_splash_path: String,
    tile_path: String,
    splash_video_path: String,
    load_screen_path: String,
    id: i32,
    short_name: String,
    name: String,
    stage: u64,
    description: String,
    splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameLobbyInvitation {
    state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatConversationResource {
    game_tag: String,
    id: String,
    game_name: String,
    _type: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionQuestSkin {
    name: String,
    splash_video_path: String,
    id: i32,
    splash_path: String,
    skin_augments: LolCollectionsGameDataChampionSkinAugments,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceSettingsResource {
    loopback_enabled: bool,
    mute_on_connect: bool,
    mic_level: u32,
    vad_active: bool,
    input_mode: LolPremadeVoiceInputMode,
    vad_sensitivity: u32,
    vad_hangover_time: u32,
    ptt_key: String,
    ptt_active: bool,
    current_capture_device_handle: String,
    auto_join: bool,
    local_mic_muted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterStatsDto {
    tournament_theme_id: i32,
    tournament_name_loc_key_secondary: String,
    schedule_end_time: i64,
    tournament_periods: i32,
    tier: i32,
    roster_short_name: String,
    roster_logo: i32,
    tournament_name_loc_key: String,
    roster_logo_color: i32,
    schedule_time: i64,
    roster_name: String,
    roster_id: i64,
    stats: RosterAggregatedStatsDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubProgressInfoUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubProgressInfoUiData {
    token_image: String,
    event_pass_bundles_catalog_entry: Vec<LolEventHubCatalogEntry>,
    pass_purchased: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftPaidBattlepassMilestone {
    is_claim_request_pending: bool,
    description: String,
    points_needed_for_milestone: i32,
    icon_image_url: String,
    internal_name: String,
    level: i32,
    rewards: Vec<LolMissionsTftPaidBattlepassReward>,
    status: String,
    total_points_for_milestone: i32,
    is_bonus: bool,
    is_paid: bool,
    title: String,
    points_earned_for_milestone: i32,
    is_keystone: bool,
    is_locked: bool,
    state: String,
    mission_id: String,
    icon_needs_frame: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLoginSession {
    account_id: u64,
    summoner_id: u64,
    state: LolLobbyLoginSessionStates,
    user_auth_token: String,
    id_token: String,
    connected: bool,
    puuid: String,
    username: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRepeatGroupTrigger {
    _type: String,
    counter_id: String,
    start_trigger_value: u16,
    increase_by: u16,
    multiplier: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemPrice {
    price: i64,
    currency_type: String,
    purchasable: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesEventMarkerV1 {
    event_name: String,
    when: u64,
    marker_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTbLobbyStateResource {
    game_id: u64,
    recovery_counter: i32,
    phase_name: String,
    queue_id: i32,
    afk_check_state: LolLobbyTeamBuilderAfkCheckStateV1,
    champion_select_state: LolLobbyTeamBuilderChampionSelectStateV1,
    matchmaking_state: LolLobbyTeamBuilderTbdMatchmakingState,
    counter: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginUsernameAndPassword {
    username: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubRequestDTO-vector-SelectionRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubRequestDtoVectorSelectionRequestDto {
    metadata: LolEventHubRequestMetadataDto,
    data: Vec<LolEventHubSelectionRequestDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsGameDataChampionSkinAugments {
    augments: Vec<LolCollectionsGameDataChampionSkinAugment>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsGameDataChampionMasteryTree {
    groups: Vec<LolCollectionsGameDataChampionMasteryGroup>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameflowLcdsReconnectInfoDto {
    game: GameflowLcdsGameDto,
    reconnect_delay: u32,
    player_credentials: GameflowLcdsPlayerCredentialsDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashGameflowGameDodge {
    state: LolClashMatchmakingDodgeState,
    dodge_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubPurchaseResponse {
    #[serde(rename = "useRMSConfirmation")]
    use_rms_confirmation: bool,
    #[serde(rename = "items")]
    items: Vec<LolEventHubPurchaseItem>,
    #[serde(rename = "transactions")]
    transactions: Vec<LolEventHubTransaction>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGroup {
    selection_strategy_config: LolRewardsSelectionStrategyConfig,
    celebration_type: LolRewardsCelebrationType,
    id: String,
    media: Value,
    product_id: String,
    types: Vec<String>,
    rewards: Vec<LolRewardsReward>,
    child_reward_group_ids: Vec<String>,
    localizations: Value,
    reward_strategy: LolRewardsRewardStrategy,
    active: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsLoadoutRequestDTOBase")]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsLoadoutRequestDtoBase {
    service_to_jwts_map: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionChroma {
    colors: Vec<String>,
    chroma_path: String,
    ownership: LolChampionsCollectionsOwnership,
    still_obtainable: bool,
    last_selected: bool,
    disabled: bool,
    id: i32,
    champion_id: i32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHonorV2HonorConfig {
    enabled: bool,
    seconds_to_vote: i32,
    honor_visibility_enabled: bool,
    honor_suggestions_enabled: bool,
    #[serde(rename = "honorEndpointsV2Enabled")]
    honor_endpoints_v_2_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassLoyaltyRewardsSimplified {
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "xpBoost")]
    xp_boost: i32,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShutdownLcdsForcedClientShutdown {
    reason: String,
    additional_info: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectChatRoomDetails {
    multi_user_chat_id: String,
    muc_jwt_dto: LolChampSelectMucJwtDto,
    multi_user_chat_password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubWallet {
    account_id: u64,
    version: i32,
    balances: Vec<LolEventHubBalance>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSuggestionInvite {
    inviter_id: u64,
    invitee_players: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerPreferencesLoginSession {
    summoner_id: u64,
    id_token: String,
    account_id: u64,
    state: LolPlayerPreferencesLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitAccountClaimStatus {
    #[serde(rename = "linking_status")]
    linking_status: LolHoneyfruitHoneyfruitLinkingServiceResponse,
    #[serde(rename = "migration_status")]
    migration_status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubCurrencyDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCurrencyDto {
    amount: i32,
    sub_currencies: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyRiotMessagingServiceMessage {
    version: String,
    payload: String,
    resource: String,
    timestamp: i64,
    service: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyInviteAnalytics {
    party_id: String,
    platform_id: String,
    game_mode: String,
    event_type: String,
    event_timestamp: u64,
    from_summoner_id: u64,
    to_summoner_id: u64,
    party_type: String,
    event_data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceGameflowSession {
    game_client: LolPremadeVoiceGameflowGameClient,
    phase: LolPremadeVoiceGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesDropsDropTableWithPityDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesDropsDropTableWithPityDto {
    id: String,
    total_rolls_info: LolTftTrovesTotalRollsInfoDto,
    source_id: String,
    start_date: String,
    currency_id: String,
    roll_offer: String,
    cost: i32,
    pity_info: LolTftTrovesDropsDropTablePityInfo,
    end_date: String,
    product_id: String,
    display_metadata: LolTftTrovesCapDropsDropTableDisplayMetadata,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSParticipant")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsParticipant {
    spell_2_id: u16,
    team_id: u16,
    champion_id: i32,
    participant_id: u16,
    spell_1_id: u16,
    highest_achieved_season_tier: String,
    stats: LolReplaysGamhsParticipantStatistics,
    timeline: LolReplaysGamhsTimeline,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsSummoner {
    summoner_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesFriendResource {
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerOrTeamAvailabilty {
    available_for_watching: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftEventMissionsSettingsDataResource {
    #[serde(rename = "selected_series")]
    selected_series: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersCapClashFlagEntitlementPayload {
    reward_type: String,
    reward_spec: LolBannersClashV2FlagRewardSpec,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsGrantorDescription {
    app_name: String,
    entity_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestoneInstance {
    product_id: String,
    owner_id: String,
    group_id: String,
    milestone_id: String,
    instance_id: String,
    repeat_sequence: u32,
    counter_id: String,
    triggered: bool,
    trigger_value: i64,
    triggered_timestamp: String,
    triggers: Vec<LolLootTrigger>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsSettingsStorageContainer {
    data: LolCosmeticsCosmeticsSettings,
    schema_version: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsExpertPlayer {
    num_of_games: i32,
    expert_rank: i32,
    position: LolCareerStatsSummonersRiftPosition,
    summoner_name: String,
    champion_id: i32,
    win_rate: f32,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatMultiGamePresenceSharedPayload {
    actor: String,
    platform: String,
    product: String,
    patchline: String,
    details: String,
    location: String,
    time: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCosmeticsLoadoutUpdateDto {
    loadout: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSets {
    timestamp: u64,
    account_id: u64,
    item_sets: Vec<LolItemSetsItemSet>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbySubteamDataDto {
    intra_subteam_position: i8,
    subteam_index: i8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolAccountVerificationPinResponseData {
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLoginSession {
    account_id: u64,
    state: LolRankedLoginSessionStates,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameGameDataTFTChampion")]
#[serde(rename_all = "PascalCase")]
pub struct LolEndOfGameGameDataTftChampion {
    #[serde(rename = "character_record")]
    character_record: LolEndOfGameGameDataTftCharacterRecord,
    #[serde(rename = "name")]
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LolRegaliaRegaliaLoadout {
    regalia_banner_slot: LolRegaliaItemKey,
    regalia_crest_slot: LolRegaliaItemKey,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassInventoryResponseDto {
    data: LolTftPassInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterDto {
    points: i32,
    dynamic_state: RosterDynamicStateDto,
    tournament_id: i64,
    banned: bool,
    wins: i32,
    members: Vec<RosterMemberDto>,
    losses: i32,
    name: String,
    eliminated: bool,
    tier: i32,
    version: i32,
    logo_color: i32,
    id: i64,
    short_name: String,
    invitation_id: String,
    phases: Vec<PhaseRosterDto>,
    muc_jwt_dto: MucJwtDto,
    logo: i32,
    captain_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "cookie")]
#[serde(rename_all = "kebab-case")]
pub struct Cookie {
    httponly: bool,
    value: String,
    secure: bool,
    expires: i64,
    path: String,
    domain: String,
    url: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLobbyPartyMemberMetadataDto {
    #[serde(rename = "positionPref")]
    position_pref: Vec<String>,
    #[serde(rename = "properties")]
    properties: Value,
    #[serde(rename = "tftNPEQueueBypass")]
    tft_npe_queue_bypass: bool,
    #[serde(rename = "subteamData")]
    subteam_data: LolLobbySubteamDataDto,
    #[serde(rename = "quickplayPlayerState")]
    quickplay_player_state: String,
    #[serde(rename = "playerSlots")]
    player_slots: Vec<LolLobbyQuickPlayPresetSlotDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "QueryEvaluationResultDTO")]
#[serde(rename_all = "camelCase")]
pub struct QueryEvaluationResultDto {
    loot_item_names: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendDeactivationPinResponse {
    data: LolAccountVerificationSendActivationPinResponseData,
    error: LolAccountVerificationResponseError,
    client_message_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectSession {
    has_simultaneous_bans: bool,
    allow_rerolling: bool,
    rerolls_remaining: u64,
    timer: LolLobbyTeamBuilderChampSelectTimer,
    chat_details: LolLobbyTeamBuilderChampSelectChatRoomDetails,
    my_team: Vec<LolLobbyTeamBuilderChampSelectPlayerSelection>,
    trades: Vec<LolLobbyTeamBuilderChampSelectTradeContract>,
    allow_locked_events: bool,
    their_team: Vec<LolLobbyTeamBuilderChampSelectPlayerSelection>,
    locked_event_index: i32,
    recovery_counter: i64,
    allow_battle_boost: bool,
    actions: Vec<Value>,
    boostable_skin_count: i32,
    has_simultaneous_picks: bool,
    bench_champions: Vec<LolLobbyTeamBuilderBenchChampion>,
    bench_enabled: bool,
    skip_champion_select: bool,
    game_id: u64,
    local_player_cell_id: i64,
    allow_skin_selection: bool,
    is_spectating: bool,
    pick_order_swaps: Vec<LolLobbyTeamBuilderChampSelectSwapContract>,
    allow_duplicate_picks: bool,
    counter: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSettingsRegionLocale {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesChallengePoints {
    position: i32,
    level: String,
    current: i32,
    percentile: f64,
    max: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyInventoryItemWithPayload {
    item_id: i64,
    payload: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterMemberDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterMemberDto {
    tier: i32,
    tournament_id: i64,
    bid_type: TicketType,
    pending_spend: i32,
    position: Position,
    join_time: i64,
    current_bid: i32,
    roster_id: i64,
    pending_premium_spend: i32,
    player_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMasteryMini {
    champion_level: i32,
    puuid: String,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectChampionSkinAugments {
    augments: Vec<LolChampSelectChampionSkinAugment>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsInventoryItemDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolLoadoutsInventoryItemDto {
    #[serde(rename = "instanceTypeId")]
    instance_type_id: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "lsb")]
    lsb: bool,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "usedInGameDate")]
    used_in_game_date: String,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "entitlementTypeId")]
    entitlement_type_id: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "entitlementId")]
    entitlement_id: String,
    #[serde(rename = "quantity")]
    quantity: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPurchaseOfferOrderStatus {
    message: String,
    order_state: LolYourshopPurchaseOfferOrderStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoadoutsFrontendInventoryResponse {
    entitlements: Vec<LolLoadoutsItemKey>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedRankedQueueWarningsDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarningsDto {
    display_decay_warning: bool,
    time_until_inactivity_status_changes: i64,
    apex_days_until_decay: i32,
    days_until_decay: i32,
    demotion_warning: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSelectionStrategyConfig {
    min_selections_allowed: u32,
    max_selections_allowed: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootResponseDTO-map-RewardGroupId-SelectGrantStatus")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootResponseDtoMapRewardGroupIdSelectGrantStatus {
    metadata: LolLootResponseMetadataDto,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashLftState {
    lft: bool,
    secondary_pos: String,
    primary_pos: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SLICount")]
#[serde(rename_all = "camelCase")]
pub struct SliCount {
    double_diagnostics: Value,
    idempotency_key: String,
    failures: f64,
    bool_diagnostics: Value,
    string_diagnostics: Value,
    start_time_epoch_ms: i64,
    sli_name: String,
    end_time_epoch_ms: i64,
    labels: Value,
    successes: f64,
    int_diagnostics: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTPlaybookGroupedViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftPlaybookGroupedViewModel {
    groups: Vec<LolCosmeticsTftPlaybookGroupViewModel>,
    default_item_id: i32,
    selected_loadout_item: LolCosmeticsCosmeticsTftPlaybookViewModel,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderActionV1 {
    champion_id: i32,
    action_id: i32,
    _type: String,
    completed: bool,
    actor_cell_id: i32,
    duration: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassAccessTokenResource {
    token: String,
    expiry: u64,
    scopes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolDropsCapDropTableCounterDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropTableCounterDto {
    drop_table_id: String,
    count: u8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubValidationRequestItem {
    quantity: i32,
    item_key: LolEventHubItemKey,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowPatcherProductState {
    is_stopped: bool,
    is_up_to_date: bool,
    is_update_available: bool,
    is_corrupted: bool,
    action: LolGameflowPatcherProductStateAction,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentSystemInfoOperatingSystem {
    platform: String,
    version_major: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersTournamentFlagInventoryItem {
    purchase_date: String,
    item_id: i32,
    payload: LolBannersCapClashFlagEntitlementPayload,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Describes a member of a struct."]
pub struct BindingFullFieldHelp {
    _type: BindingFullTypeIdentifier,
    description: String,
    optional: bool,
    name: String,
    offset: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterSimpleMessage {
    _type: String,
    account_id: u64,
    msg_id: String,
    params: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesCapClashTrophyEntitlementPayload {
    reward_spec: LolTrophiesClashV2TrophyRewardSpec,
    reward_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyStatus {
    queue_id: i32,
    is_leader: bool,
    lobby_id: String,
    is_spectator: bool,
    custom_spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    member_summoner_ids: Vec<u64>,
    invited_summoner_ids: Vec<u64>,
    is_practice_tool: bool,
    allowed_play_again: bool,
    is_custom: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeOutput {
    loot_name: String,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSetSummary {
    stones_available: u32,
    name: String,
    stones_owned: u32,
    milestones_passed: u32,
    stones_illuminated: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "MatchedPlayerDTO")]
#[serde(rename_all = "camelCase")]
pub struct MatchedPlayerDto {
    roster_id: String,
    captain: PlayerInfoDto,
    players: Vec<PlayerInfoDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesGameDataTrovesBanner {
    pity_counter_id: String,
    name: String,
    chase_table: LolTftTrovesGameDataTrovesBannerTable,
    banner_texture: String,
    is_collector_bounty: bool,
    deactivation_time: String,
    event_hub_banner_texture: String,
    mythic_token_offer_id: String,
    background_texture: String,
    pity_threshold: u32,
    id: String,
    celebration_theme: LolTftTrovesTrovesCelebrationThemeData,
    description: String,
    activation_time: String,
    thumbnail_texture: String,
    root_table: LolTftTrovesGameDataTrovesBannerTable,
    platform_texture: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesChampionStatstoneSummary {
    stones_illuminated: u32,
    champion_id: i32,
    stones_available: u32,
    milestones_passed: u32,
    sets: Vec<LolStatstonesChampionStatstoneSetSummary>,
    stones_owned: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsSimpleDialogMessage {
    account_id: u64,
    msg_id: String,
    params: Vec<String>,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameDataChallengeConfig {
    queue_ids: Vec<i32>,
    icon_path: String,
    seasons: Vec<i32>,
    reverse_direction: bool,
    thresholds: Value,
    leaderboard: bool,
    level_to_icon_path: Value,
    end_timestamp: i64,
    description_short: String,
    name: String,
    tags: Value,
    description: String,
    source: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacySettingCategoryResource {
    schema_version: i32,
    data: LolChampSelectLegacyChampionSelectPreferences,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectCollectionsChampionSkinEmblemPath {
    large: String,
    small: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchScdCookies {
    cookies: Vec<LolPatchScdCookie>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsChampionStatistics {
    champion_id: i32,
    queue_stats: Vec<LolCareerStatsStatisticsByQueue>,
    experts: Vec<LolCareerStatsExpertPlayer>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsRequestMetadataDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRequestMetadataDto {
    transaction_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameflowGameData {
    game_id: u64,
    queue: LolEndOfGameQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc = "Represents a cancelled asynchronous operation."]
pub struct BindingAsyncCancelEvent {
    async_token: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomChampSelectStartResponse {
    success: bool,
    failed_players: Vec<LolLobbyLobbyCustomFailedPlayer>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPlayerAggregatedStats {
    raw_stats_sum: Value,
    raw_stats_max: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolActiveBoostsInventoryItemDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolActiveBoostsInventoryItemDto {
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "ownedQuantity")]
    owned_quantity: u64,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "entitlementId")]
    entitlement_id: String,
    #[serde(rename = "entitlementTypeId")]
    entitlement_type_id: String,
    #[serde(rename = "lsb")]
    lsb: bool,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "instanceTypeId")]
    instance_type_id: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "usedInGameDate")]
    used_in_game_date: String,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryDto {
    items: Value,
    puuid: String,
    summoner_id: u64,
    items_jwt: String,
    account_id: u64,
    expires: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowAvailability {
    is_available: bool,
    state: LolGameflowGameflowAvailabilityState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerUpdateNotification {
    player: PlayerDto,
    notify_reason: LolClashNotifyReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMastery {
    champion_id: i32,
    last_play_time: u64,
    champion_points: i32,
    champion_points_until_next_level: i32,
    puuid: String,
    chest_granted: bool,
    tokens_earned: i32,
    champion_season_milestone: i32,
    milestone_grades: Vec<String>,
    mark_required_for_next_level: i32,
    next_season_milestone: LolChampionMasterySeasonMilestoneRequireAndRewards,
    champion_level: i32,
    champion_points_since_last_level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistorySummoner {
    puuid: String,
    display_name: String,
    tag_line: String,
    game_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogPluginItemAssets {
    colors: Vec<String>,
    emblems: Vec<LolEventHubChampionSkinEmblem>,
    splash_path: String,
    icon_path: String,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolAccountVerificationAVSConfig")]
#[serde(rename_all = "PascalCase")]
pub struct LolAccountVerificationAvsConfig {
    enabled: bool,
    disable_get_active_phone_number_call: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesUICategoryProgress")]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiCategoryProgress {
    max: i32,
    current: i32,
    level: String,
    category: String,
    position_percentile: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "OpenedTeamMemberDTO")]
#[serde(rename_all = "camelCase")]
pub struct OpenedTeamMemberDto {
    position: Position,
    tier: i32,
    player_id: i64,
    friendship: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingCollectionsChampionMastery {
    player_id: u64,
    champion_level: i32,
    last_play_time: u64,
    champion_id: i32,
    champion_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSParticipantStatistics")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsParticipantStatistics {
    item_5: i32,
    gold_spent: i64,
    win: bool,
    perk_2_var_1: i64,
    perk_2_var_2: i64,
    wards_killed: i64,
    champ_level: i64,
    gold_earned: i64,
    time_c_cing_others: i64,
    perk_0_var_2: i64,
    perk_2_var_3: i64,
    item_1: i32,
    physical_damage_dealt: i64,
    total_heal: i64,
    sight_wards_bought_in_game: i64,
    first_blood_assist: bool,
    player_score_0: i64,
    perk_primary_style: i64,
    perk_1_var_3: i64,
    perk_4: i64,
    magic_damage_dealt_to_champions: i64,
    longest_time_spent_living: i64,
    total_score_rank: i64,
    perk_4_var_2: i64,
    first_tower_assist: bool,
    true_damage_taken: i64,
    perk_0: i64,
    perk_5: i64,
    perk_5_var_3: i64,
    player_subteam_id: i32,
    item_2: i32,
    kills: i64,
    double_kills: i64,
    quadra_kills: i64,
    neutral_minions_killed: i64,
    true_damage_dealt_to_champions: i64,
    player_score_7: i64,
    perk_1_var_2: i64,
    perk_2: i64,
    perk_3_var_2: i64,
    perk_5_var_1: i64,
    perk_5_var_2: i64,
    subteam_placement: i32,
    vision_wards_bought_in_game: i64,
    largest_critical_strike: i64,
    player_score_9: i64,
    magic_damage_dealt: i64,
    inhibitor_kills: i64,
    player_augment_2: i32,
    damage_dealt_to_turrets: i64,
    perk_1: i64,
    physical_damage_taken: i64,
    total_units_healed: i64,
    damage_dealt_to_objectives: i64,
    unreal_kills: i64,
    killing_sprees: i64,
    game_ended_in_surrender: bool,
    vision_score: i64,
    item_3: i32,
    deaths: i64,
    triple_kills: i64,
    total_damage_dealt: i64,
    game_ended_in_early_surrender: bool,
    objective_player_score: i64,
    assists: i64,
    total_damage_dealt_to_champions: i64,
    neutral_minions_killed_enemy_jungle: i64,
    first_tower_kill: bool,
    player_score_4: i64,
    magical_damage_taken: i64,
    turret_kills: i64,
    player_score_5: i64,
    player_score_6: i64,
    perk_3_var_3: i64,
    item_6: i32,
    largest_multi_kill: i64,
    item_0: i32,
    total_minions_killed: i64,
    neutral_minions_killed_team_jungle: i64,
    largest_killing_spree: i64,
    total_player_score: i64,
    perk_0_var_1: i64,
    team_early_surrendered: bool,
    perk_0_var_3: i64,
    combat_player_score: i64,
    first_blood_kill: bool,
    total_time_crowd_control_dealt: i64,
    player_score_3: i64,
    perk_1_var_1: i64,
    total_damage_taken: i64,
    early_surrender_accomplice: bool,
    perk_3: i64,
    first_inhibitor_assist: bool,
    wards_placed: i64,
    first_inhibitor_kill: bool,
    penta_kills: i64,
    participant_id: u16,
    physical_damage_dealt_to_champions: i64,
    true_damage_dealt: i64,
    damage_self_mitigated: i64,
    player_score_1: i64,
    player_score_8: i64,
    perk_4_var_1: i64,
    perk_sub_style: i64,
    player_score_2: i64,
    caused_early_surrender: bool,
    perk_4_var_3: i64,
    player_augment_3: i32,
    player_augment_4: i32,
    item_4: i32,
    perk_3_var_1: i64,
    player_augment_1: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SeriesMediaDTO")]
#[serde(rename_all = "camelCase")]
pub struct SeriesMediaDto {
    tracker_icon: String,
    accent_color: String,
    tracker_icon_url: String,
    background_url: String,
    background_image_small_url: String,
    background_image_large_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentWinnerHistory {
    tournament_id: i64,
    winners: Vec<LolClashTournamentWinnerInfo>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubWalletResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubWalletResponseDto {
    data: LolEventHubWalletDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCurrencyConfiguration {
    currencies_using_cap_wallets: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTimer {
    total_time_in_phase: i64,
    phase: String,
    is_infinite: bool,
    adjusted_time_left_in_phase: i64,
    internal_now_in_epoch_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsUserResource {
    summoner_id: u64,
    lol: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationEmailVerificationSession {
    email: String,
    email_verified: bool,
    fatal_error: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterRankedRestrictionEntryDto {
    restricted_games_remaining: i32,
    puuid: String,
    restricted_games_original: i32,
    ranked_restriction_ack_needed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootMap {
    version: i64,
    player_loot: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLolTftEvents {
    sub_nav_tabs: Vec<LolTftEventLolTftEvent>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolVanguardVanguardSession {
    vanguard_status: i32,
    state: LolVanguardVanguardSessionState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPublishingSettings {
    locale: String,
    web_region: String,
    web_locale: String,
    publishing_locale: String,
    rso_platform_id: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootItemClientDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootItemClientDto {
    upgrade_loot_name: String,
    display_categories: String,
    rental_seconds: i64,
    rental_games: i32,
    tags: String,
    loot_name: String,
    value: i32,
    rarity: String,
    _type: String,
    asset: String,
    store_item_id: i32,
    expiry_time: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubActivationClientConfig {
    end_date: String,
    hub_enabled: bool,
    start_date: String,
    active_event_id: String,
    first_activation_threshold_seconds: f64,
    progress_end_date: String,
    activation_spread_seconds: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyChangeGameDto {
    custom_game_lobby: LolLobbyLobbyCustomGameLobby,
    queue_id: i32,
    game_customization: Value,
    is_custom: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatuses {
    statuses: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPremadeVoiceDeviceResourceRiotClient {
    #[serde(rename = "handle")]
    handle: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "is_effective_device")]
    is_effective_device: bool,
    #[serde(rename = "is_current_device")]
    is_current_device: bool,
    #[serde(rename = "is_default")]
    is_default: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesLoadoutItem {
    inventory_type: String,
    content_id: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStorePlayer {
    rp: i64,
    account_id: u64,
    summoner_level: u32,
    ip: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGame {
    passback_url: String,
    lobby_name: String,
    has_password: bool,
    owner_display_name: String,
    party_id: String,
    map_id: i32,
    id: u64,
    filled_player_slots: i32,
    filled_spectator_slots: i32,
    max_spectator_slots: u64,
    max_player_slots: i32,
    game_type: String,
    spectator_policy: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSwitchObserverToPlayerRequestDto {
    game_id: u64,
    team_id: i32,
    player_gco_tokens: LcdsPlayerGcoTokens,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationLoginSession {
    state: LolAccountVerificationLoginSessionState,
    username: String,
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsInventoryItem {
    expiration_date: String,
    uuid: String,
    quantity: u64,
    wins: u64,
    ownership_type: LolActiveBoostsItemOwnershipType,
    item_id: i32,
    inventory_type: String,
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatGameflowGameMap {
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectAction {
    _type: String,
    champion_id: i32,
    actor_cell_id: i64,
    is_ally_action: bool,
    completed: bool,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoginSession {
    state: LolLoyaltyLoginSessionStates,
    account_id: u64,
    puuid: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftGameflowSession {
    game_data: LolTftGameflowGameData,
    phase: LolTftGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionMastery {
    champion_id: i32,
    champion_level: i32,
    chest_granted: bool,
    champion_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatChatMessageList {
    messages: Vec<LolChatChatMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSetTicketRequest {
    ticket_amount: i32,
    ticket_type: TicketType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolFeaturedModesFeaturedModesConfig {
    notifications_enabled: bool,
    max_notification_save_delay_minutes: u32,
    queue_toggle_notification_minutes_threshold: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedSplitRewardDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardDto {
    default_reward_id: String,
    reward_type: String,
    metadata: LolRankedSplitRewardsMetaData,
    tiered_reward_ids: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCard {
    player_facing_message: String,
    id: u64,
    punishment_length_games: i64,
    restricted_chat_games_remaining: i64,
    punishment_type: String,
    time_when_punishment_expires: u64,
    reason: String,
    punishment_length_time: u64,
    chat_logs: Vec<String>,
    game_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftPaidBattlepass {
    progress_mission_id: String,
    total_points_earned: i32,
    last_viewed_progress: i32,
    info: LolTftEventTftPaidBattlepassInfo,
    milestones: Vec<LolTftEventTftPaidBattlepassMilestone>,
    current_level: i32,
    last_viewed_milestone: LolTftEventTftPaidBattlepassMilestone,
    bonuses: Vec<LolTftEventTftPaidBattlepassMilestone>,
    active_milestone: LolTftEventTftPaidBattlepassMilestone,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderPlayerReport {
    obfuscated_offender_puuid: String,
    comment: String,
    game_id: u64,
    offender_puuid: String,
    offender_summoner_id: u64,
    categories: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesVerboseLootOddsResponse {
    chance_to_contain: Vec<LolTftTrovesLootOddsResponse>,
    has_pity_rules: bool,
    guaranteed_to_contain: Vec<LolTftTrovesLootOddsResponse>,
    loot_item: LolTftTrovesPlayerLoot,
    recipe_name: String,
    checks_ownership: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedAchievedTier {
    tier: String,
    queue_type: LolRankedLeagueQueueType,
    division: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectAction {
    pick_turn: i32,
    id: i64,
    is_ally_action: bool,
    actor_cell_id: i64,
    is_in_progress: bool,
    champion_id: i32,
    completed: bool,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginCrashReportingEnvironment {
    environment: String,
    user_name: String,
    user_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsSummonerIcon {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEmailVerificationEmailUpdate {
    password: String,
    email: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionSelectStateV1 {
    current_action_set_index: i32,
    allow_skin_selection: bool,
    battle_boost_state: LolLobbyTeamBuilderTeamBuilderBoostInfo,
    cells: LolLobbyTeamBuilderCellsV1,
    skip_champion_select: bool,
    ceremonies_by_action_set_index: Value,
    trades: Vec<LolLobbyTeamBuilderTradeV1>,
    allow_duplicate_picks: bool,
    team_chat_room_id: String,
    current_time_remaining_millis: i64,
    pick_order_swaps: Vec<LolLobbyTeamBuilderPickOrderSwapV1>,
    lcu_skips_sending_loadouts_gco: bool,
    locked_events_state: LolLobbyTeamBuilderLockedEventsStateV1,
    champion_bench_state: LolLobbyTeamBuilderChampionBenchStateV1,
    current_total_time_millis: i64,
    reroll_state: LolLobbyTeamBuilderRerollStateV1,
    allow_opting_out_of_banning: bool,
    local_player_cell_id: i32,
    is_spectating: bool,
    inventory_draft: LolLobbyTeamBuilderTbdInventory,
    team_id: String,
    subphase: String,
    pick_intent_cleared_reason: String,
    action_set_list: Vec<Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCatalogItemCost {
    cost: i64,
    discount: f32,
    currency: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsNamecheckPayload {
    name: String,
    puuid: String,
    name_validation_context: String,
    shard: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchasableItem {
    bundled_items: Vec<LolPurchaseWidgetItemDefinition>,
    dependencies: Vec<LolPurchaseWidgetItemDefinition>,
    sale: LolPurchaseWidgetItemSale,
    purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
    validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
    item: LolPurchaseWidgetItemDefinition,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkPageResource {
    order: i32,
    is_deletable: bool,
    tooltip_bg_path: String,
    secondary_style_icon_path: String,
    is_recommendation_override: bool,
    is_valid: bool,
    name: String,
    primary_style_id: i32,
    recommendation_champion_id: i32,
    quick_play_champion_ids: Vec<i32>,
    primary_style_icon_path: String,
    ui_perks: Vec<LolPerksUiPerkMinimal>,
    selected_perk_ids: Vec<i32>,
    secondary_style_name: String,
    is_temporary: bool,
    auto_modified_selections: Vec<u32>,
    last_modified: u64,
    recommendation_index: i32,
    page_keystone: LolPerksUiPerkMinimal,
    current: bool,
    rune_recommendation_id: String,
    sub_style_id: i32,
    id: i32,
    primary_style_name: String,
    is_active: bool,
    is_editable: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsPlayerNotification {
    id: u64,
    detail_key: String,
    state: String,
    background_url: String,
    data: Value,
    title_key: String,
    icon_url: String,
    critical: bool,
    source: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLootMilestonesProgressionGroupRepeatGdsResource {
    multiplier: f32,
    count: i32,
    scope: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubValidationResponse {
    valid: bool,
    items: Vec<LolEventHubValidationResponseItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinMinimal {
    chroma_path: String,
    tile_path: String,
    last_selected: bool,
    name: String,
    disabled: bool,
    ownership: LolChampionsCollectionsOwnership,
    skin_augments: LolChampionsCollectionsChampionSkinAugments,
    still_obtainable: bool,
    id: i32,
    is_base: bool,
    champion_id: i32,
    splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootGrantNotification {
    message_key: String,
    champion_id: i32,
    loot_name: String,
    msg_id: String,
    player_id: u64,
    game_id: u64,
    player_grade: String,
    id: i64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmDeactivationPinRequest {
    one_time_pin: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCatalogItem {
    metadata: Vec<LolStoreItemMetadataEntry>,
    active: bool,
    inactive_date: String,
    item_id: i32,
    offer_id: String,
    item_instance_id: String,
    bundled: LolStoreBundled,
    localizations: Value,
    sub_inventory_type: String,
    release_date: String,
    max_quantity: i32,
    inventory_type: String,
    item_requirements: Vec<LolStoreItemKey>,
    tags: Vec<String>,
    sale: LolStoreSale,
    prices: Vec<LolStoreItemCost>,
    icon_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionSkinAugment {
    overlays: Vec<LolCollectionsGameDataChampionSkinAugmentOverlays>,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchPatchSieveLabelValue {
    values: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLcdsGameMetaData {
    game_id: u64,
    map_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitSettingsAccountClaim {
    is_account_claim_auto_dismiss: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksGetGameCustomizationDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGetGameCustomizationDto {
    queue_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSTimeline")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsTimeline {
    participant_id: u16,
    gold_per_min_deltas: Value,
    role: String,
    creeps_per_min_deltas: Value,
    xp_diff_per_min_deltas: Value,
    damage_taken_per_min_deltas: Value,
    cs_diff_per_min_deltas: Value,
    xp_per_min_deltas: Value,
    damage_taken_diff_per_min_deltas: Value,
    lane: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SanitizerContainsSanitizedResponse {
    contains: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRankedStats {
    ranked_regalia_level: i32,
    queues: Vec<LolRegaliaRankedQueueStats>,
    highest_previous_season_end_tier: String,
    highest_ranked_entry: LolRegaliaRankedQueueStats,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Ballot {
    eligible_players: Vec<LolHonorV2EligiblePlayer>,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrackItem {
    state: LolEventHubRewardTrackItemStates,
    threshold: String,
    progress_required: i64,
    reward_options: Vec<LolEventHubRewardTrackItemOption>,
    reward_tags: Vec<LolEventHubRewardTrackItemTag>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowSampleDto {
    state: String,
    probability: f64,
    tags: Vec<LolLobbyGameflowSampleTag>,
    interval_secs: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksPerkUISlot")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiSlot {
    _type: String,
    slot_label: String,
    perks: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthAuthorizationResponse {
    _type: String,
    authorization: LolRsoAuthImplicitAuthorization,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusBroadcastNotification {
    broadcast_messages: Vec<LolServiceStatusBroadcastMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashUserResource {
    summoner_id: u64,
    lol: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameModeDto {
    game_type: String,
    game_customization: Value,
    max_team_size: i32,
    customs_settings: LolLobbyCustomGameSettingsDto,
    max_party_size: i32,
    bot_difficulty: String,
    allow_spectators: String,
    map_id: i32,
    queue_id: i32,
    game_type_config_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorSettingsResource {
    schema_version: i64,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMucJwtDto {
    jwt: String,
    domain: String,
    target_region: String,
    channel_claim: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLoginSession {
    state: LolLobbyTeamBuilderLoginSessionState,
    summoner_id: u64,
    connected: bool,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderGameflowGameData {
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassCurrencyDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassCurrencyDto {
    sub_currencies: Value,
    amount: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolGeoinfoGeoInfoConfig {
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataSummonerEmote {
    description: String,
    inventory_icon: String,
    id: i64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectCollectionsChampionSkinEmblemPosition {
    horizontal: String,
    vertical: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LolYourshopEndOfGameXp {
    per_win: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameSettingsLoginSession {
    account_id: u64,
    state: LolGameSettingsLoginSessionStates,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHovercardProductMetadata {
    id: String,
    name: String,
    patchlines: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerGameflowSession {
    phase: LolTftTeamPlannerGameflowPhase,
    game_data: LolTftTeamPlannerGameflowGameData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitationDto {
    state: LolLobbyLobbyInvitationState,
    to_summoner_name: String,
    to_summoner_id: u64,
    invitation_id: String,
    timestamp: String,
    invitation_type: LolLobbyInvitationType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSParticipantIdentities")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsParticipantIdentities {
    participant_id: u16,
    player: LolReplaysGamhsParticipantIdentityPlayer,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMissionSeriesOptIn {
    series_id: String,
    option: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemAssets {
    colors: Vec<String>,
    icon_path: String,
    emblems: Vec<LolCatalogChampionSkinEmblem>,
    splash_path: String,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatGameDataChampionSummary {
    alias: String,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClubsSummoner {
    summoner_id: u64,
    display_name: String,
    profile_icon_id: i32,
    summoner_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchSupportedGameReleases {
    #[serde(rename = "supported_game_releases")]
    supported_game_releases: Vec<LolPatchSupportedGameRelease>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersSummoner {
    summoner_id: u64,
    summoner_level: u32,
    display_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolBannersSummonerProfileUpdate {
    value: Value,
    key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTMapSkinViewModel")]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsTftMapSkinViewModel {
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "isRecentItem")]
    is_recent_item: bool,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "favorited")]
    favorited: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventSvcReward {
    fulfillment_source: String,
    item_id: String,
    id: String,
    media: Value,
    localizations: Value,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueThrottled {
    summoner: MatchmakingLcdsSummoner,
    message: String,
    queue_id: i32,
    reason_failed: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueCustomGameSubcategory {
    game_mode: String,
    custom_spectator_policies: Vec<LolGameQueuesQueueCustomGameSpectatorPolicy>,
    mutators: Vec<LolGameQueuesQueueGameTypeConfig>,
    map_id: i32,
    maximum_participant_list_size: i32,
    max_player_count: i32,
    min_level: u32,
    minimum_participant_list_size: i32,
    num_players_per_team: i32,
    queue_availability: LolGameQueuesQueueAvailability,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerGameDelta {
    game_id: u64,
    game_platform_id: String,
    platform_delta: LolMatchHistoryMatchHistoryPlayerPlatformDelta,
    champ_mastery: LolMatchHistoryMatchHistoryPlayerChampMasteryDelta,
    league_delta: LolMatchHistoryMatchHistoryPlayerLeagueDelta,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerMissionDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionDto {
    id: String,
    completed_date: i64,
    background_image_url: String,
    icon_image_url: String,
    series_name: String,
    locale: String,
    metadata: MissionMetadata,
    expiring_warnings: Vec<MissionAlertDto>,
    description: String,
    client_notify_level: String,
    internal_name: String,
    is_new: bool,
    helper_text: String,
    status: String,
    reward_strategy: RewardStrategy,
    completion_expression: String,
    objectives: Vec<PlayerMissionObjectiveDto>,
    display_type: String,
    title: String,
    start_time: i64,
    viewed: bool,
    rewards: Vec<PlayerMissionRewardDto>,
    sequence: i32,
    earned_date: i64,
    display: MissionDisplay,
    mission_type: String,
    media: Value,
    requirements: Vec<String>,
    end_time: i64,
    celebration_type: String,
    cooldown_time_millis: i64,
    last_updated_timestamp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataChampionSummary {
    alias: String,
    id: i32,
    name: String,
    square_portrait_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubSimpleInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubSimpleInventoryResponseDto {
    data: LolEventHubSimpleInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesUIChallengeThreshold")]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesUiChallengeThreshold {
    rewards: Vec<LolChallengesUiChallengeReward>,
    value: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesPityCounterDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesPityCounterDto {
    drop_table_id: String,
    count: u8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAccountIdAndSummonerId {
    account_id: u64,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubCapOrdersOrderDto {
    meta: LolEventHubCapOrdersMetaDto,
    data: LolEventHubCapOrdersDataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferResponseV3 {
    order_dto: LolPurchaseWidgetCapOrdersOrderDto,
    legacy: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsProgress {
    current_progress: i32,
    total_count: i32,
    last_viewed_progress: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsRequestDTO-SelectionRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsRequestDtoSelectionRequestDto {
    data: LolRewardsSelectionRequestDto,
    metadata: LolRewardsRequestMetadataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPlayerMuteUpdate {
    puuids: Vec<String>,
    is_muted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksPerksConfigDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolPerksPerksConfigDto {
    styles: Vec<LolPerksPerkStyleResource>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhaseInMember {
    bet: i32,
    player_id: u64,
    self_bet: i32,
    pending_pay: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyGameflowGameClient {
    running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftPrimeGaming {
    asset_id: String,
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatTeamPlayerEntry {
    summoner_id: u64,
    summoner_internal_name: String,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPurchaseWidgetCapOfferPayloadEntry {
    #[serde(rename = "itemPriceMap")]
    item_price_map: Value,
    #[serde(rename = "itemInstanceId")]
    item_instance_id: String,
    #[serde(rename = "fulfillmentTypeId")]
    fulfillment_type_id: String,
    #[serde(rename = "inventoryTypeUUID")]
    inventory_type_uuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterWithdrawNotification {
    roster_id: i64,
    source_player_id: u64,
    version: i32,
    tournament_id: i64,
    withdraw: RosterWithdraw,
    notify_reason: LolClashRosterNotifyReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashNoShowPingResponseData {
    gameflow_state: LolClashGameflowPhase,
    login_time: i64,
    dodge_time: i64,
    ready_check_info: LolClashReadyCheckInfo,
    is_playmode_restricted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubGroup {
    id: String,
    counters: Vec<LolEventHubCounter>,
    product_id: String,
    name: String,
    milestones: Vec<LolEventHubMilestone>,
    repeat: LolEventHubRepeat,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyReward {
    _type: LolLobbyLobbyPartyRewardType,
    premade_size: i32,
    value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampionPreferredStyle {
    style: i32,
    champion_id: u32,
    champion_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersTypedIdentifierDto {
    id: String,
    type_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginConfigStatus {
    readiness: LolLoginConfigReadinessEnum,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsStatisticsByQueue {
    queue_type: LolCareerStatsCareerStatsQueueType,
    stats: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatIdBody {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGamePlaybookViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGamePlaybookViewModel {
    name: String,
    item_id: u32,
    icon_small: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectBannedChampions {
    num_bans: i32,
    their_team_bans: Vec<i32>,
    my_team_bans: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPerksPlatformConfig {
    perks_enabled: bool,
    auto_repair_pages_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPremadeVoiceAccountSettingsCategoryDataResource {
    #[serde(rename = "showFirstExperienceInLCU")]
    show_first_experience_in_lcu: bool,
    #[serde(rename = "inputMode")]
    input_mode: LolPremadeVoiceInputMode,
    #[serde(rename = "pushToTalkKey")]
    push_to_talk_key: String,
    #[serde(rename = "muteOnConnect")]
    mute_on_connect: bool,
    #[serde(rename = "showFirstExperienceInGame")]
    show_first_experience_in_game: bool,
    #[serde(rename = "autoJoin")]
    auto_join: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardGroup {
    rewards: Vec<LolRankedSplitReward>,
    split_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSPosition")]
#[serde(rename_all = "kebab-case")]
pub struct LolReplaysGamhsPosition {
    y: i16,
    x: i16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolModeProgressionLoadoutsSlot {
    content_id: String,
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyTeamBoost {
    skin_unlock_mode: String,
    puuid: String,
    summoner_id: i64,
    ip_reward: i64,
    available_skins: Vec<i64>,
    unlocked: bool,
    ip_reward_for_purchaser: i64,
    price: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesInventoryItem {
    inventory_type: String,
    owned: bool,
    uuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassWalletResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassWalletResponseDto {
    data: LolTftPassWalletDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatConversation {
    #[serde(rename = "muted")]
    muted: bool,
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "mid")]
    mid: String,
    #[serde(rename = "cid")]
    cid: String,
    #[serde(rename = "unread_count")]
    unread_count: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubValidationResponseItem {
    quantity: i32,
    name: String,
    description: String,
    sale: LolEventHubSale,
    item_key: LolEventHubItemKey,
    validation_currency_info: Vec<LolEventHubItemPrice>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolMatchHistoryGAMHSMatchHistoryMetadata")]
#[serde(rename_all = "PascalCase")]
pub struct LolMatchHistoryGamhsMatchHistoryMetadata {
    #[serde(rename = "info_type")]
    info_type: String,
    #[serde(rename = "match_id")]
    match_id: String,
    #[serde(rename = "participants")]
    participants: Vec<String>,
    #[serde(rename = "product")]
    product: String,
    #[serde(rename = "private")]
    private: bool,
    #[serde(rename = "data_version")]
    data_version: u8,
    #[serde(rename = "timestamp")]
    timestamp: u64,
    #[serde(rename = "tags")]
    tags: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionDto {
    title: String,
    description: String,
    icon_image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginRegionLocaleChangedEvent {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSale {
    start_date: String,
    end_date: String,
    prices: Vec<LolEventHubItemCost>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMutedPlayerInfo {
    summoner_id: u64,
    obfuscated_summoner_id: u64,
    puuid: String,
    obfuscated_puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStorePurchaseOrderRequestDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolStorePurchaseOrderRequestDto {
    items: Vec<LolStoreItemOrderDto>,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchasableItem {
    item: LolEventHubItemDefinition,
    purchase_options: Vec<LolEventHubPurchaseOption>,
    sale: LolEventHubItemSale,
    bundled_items: Vec<LolEventHubItemDefinition>,
    dependencies: Vec<LolEventHubItemDefinition>,
    validation_errors: Vec<LolEventHubValidationErrorEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2MutualHonor {
    summoners: Vec<LolHonorV2MutualHonorPlayer>,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSimpleDialogMessage {
    params: Vec<String>,
    account_id: u64,
    msg_id: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeRewardsChallengesProgress {
    progress: LolNpeRewardsProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsAccountSettingsCategoryDataResource {
    type_to_last_opened_date: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubMilestone {
    id: String,
    counter_id: String,
    group_id: String,
    properties: Value,
    name: String,
    trigger_value: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubOffer {
    items: Vec<LolEventHubItem>,
    localized_title: String,
    offer_category: LolEventHubOfferCategory,
    image: String,
    localized_description: String,
    promotion_type: LolEventHubOfferPromotionType,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatMessagePost {
    _type: LolChatMessageType,
    cid: String,
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricMetadataHipchatNotification {
    roomid: String,
    tags: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferOrderStatus {
    message: String,
    order_state: LolPurchaseWidgetPurchaseOfferOrderStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSummonerLevelAndPoints {
    summoner_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGatekeeperRestrictionDto {
    reason: String,
    account_id: u64,
    payload: String,
    queue_id: i32,
    puuid: String,
    details: Value,
    remaining_millis: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthAccessToken {
    token: String,
    expiry: u64,
    scopes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLobbyTFTNPESettings")]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTftnpeSettings {
    schema_version: u32,
    data: LolLobbyTftnpeSettingsResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitV1SuccessResponseDetails {
    #[serde(rename = "login_token")]
    login_token: String,
    #[serde(rename = "redirect_url")]
    redirect_url: String,
    #[serde(rename = "linked")]
    linked: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsOddsTableDisplayMetadata {
    priority: u8,
    name_tra_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsRental {
    end_date: u64,
    purchase_date: u64,
    win_count_remaining: i32,
    rented: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRoster {
    short_name: String,
    id: String,
    num_completed_periods: i32,
    is_registered: bool,
    tournament_id: i64,
    phase_infos: Vec<LolClashRosterPhaseInfo>,
    available_logos: Vec<RewardLogo>,
    is_active_in_current_phase: bool,
    lft: bool,
    invitation_id: String,
    tier: i32,
    current_bracket_wins: i32,
    name: String,
    icon_color_id: i32,
    captain_summoner_id: u64,
    high_tier_variance: bool,
    icon_id: i32,
    suggested_invites: Vec<LolClashSuggestedInvite>,
    is_clash_banned: bool,
    points: i32,
    wins: i32,
    losses: i32,
    is_eliminated: bool,
    is_current_bracket_complete: bool,
    withdraw: RosterWithdraw,
    members: Vec<LolClashRosterMember>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTMapSkinFavoritesViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftMapSkinFavoritesViewModel {
    favorite_items: Vec<LolCosmeticsCosmeticsTftMapSkinViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMatchmakingGameflowGameData {
    queue: LolMatchmakingGameflowQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventoryCurrencyDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryCurrencyDto {
    amount: i32,
    sub_currencies: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Reward {
    reward_type: String,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsPlayerNotificationResource {
    _type: String,
    title_key: String,
    critical: bool,
    background_url: String,
    icon_url: String,
    detail_key: String,
    data: Value,
    source: String,
    expires: String,
    state: String,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftEvents {
    sub_nav_tabs: Vec<LolTftLolTftEvent>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataQuestSkinDescriptionInfo {
    description: String,
    icon_path: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPlayerBehaviorPlayerBehavior_GameflowSessionResource")]
#[serde(rename_all = "kebab-case")]
pub struct LolPlayerBehaviorPlayerBehaviorGameflowSessionResource {
    phase: LolPlayerBehaviorGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitHoneyfruitAuthRedirectMock {
    #[serde(rename = "redirectURL")]
    redirect_url: String,
    #[serde(rename = "redirectLatency")]
    redirect_latency: u32,
    #[serde(rename = "redirectStatusCode")]
    redirect_status_code: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPaidBattlepassReward {
    icon_url: String,
    reward_group: String,
    description: String,
    icon_needs_frame: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClientFailedInvite {
    exception: String,
    player_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCardV2 {
    punished_for_reform_card_chat_logs: Vec<LolPlayerBehaviorReformCardChatLogs>,
    punished_until_date_millis: u64,
    punished_for_game_ids: Vec<u64>,
    punishment_type: String,
    punishment_reason: String,
    player_facing_message: String,
    id: u64,
    punishment_length_millis: u64,
    punishment_length_games: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsLoadoutItem {
    inventory_type: String,
    item_id: i32,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyMemberDto {
    puuid: String,
    game_mode: LolLobbyGameModeDto,
    metadata: LolLobbyPartyMemberMetadataDto,
    platform_id: String,
    invited_by_summoner_id: u64,
    party_id: String,
    can_invite: bool,
    account_id: u64,
    team: String,
    role: LolLobbyPartyMemberRoleEnum,
    invite_timestamp: u64,
    ready: bool,
    party_version: i64,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueDisabled {
    reason_failed: String,
    summoner: MatchmakingLcdsSummoner,
    message: String,
    queue_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSpectateGameInfoResource {
    game_queue_type: String,
    puuid: String,
    allow_observe_mode: String,
    drop_in_spectate_game_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPlayerSelection {
    cell_id: i64,
    selected_skin_id: i32,
    ward_skin_id: i64,
    assigned_position: String,
    champion_pick_intent: i32,
    puuid: String,
    obfuscated_summoner_id: u64,
    summoner_id: u64,
    name_visibility_type: String,
    obfuscated_puuid: String,
    spell_2_id: u64,
    team: i32,
    spell_1_id: u64,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthIdToken {
    expiry: u64,
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTSettingsResource")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftSettingsResource {
    schema_version: u32,
    data: LolCosmeticsTftSettingsDataResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksValidateItemSetNameResponse {
    success: bool,
    name_check_response: LolPerksNamecheckResponse,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingRankedQueueStats {
    queue_type: LolContentTargetingRankedQueue,
    is_provisional: bool,
    division: LolContentTargetingRankedDivision,
    tier: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLootRewardsConfig {
    grant_filtering: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubValidationError {
    response_items: Vec<String>,
    error_details: Value,
    error_code: String,
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionTacticalInfo {
    difficulty: u32,
    damage_type: String,
    style: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGameflowGameModeSpellList {
    spells: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatUserResource {
    summary: String,
    product: String,
    icon: i32,
    game_name: String,
    last_seen_online_timestamp: String,
    obfuscated_summoner_id: u64,
    id: String,
    time: u64,
    status_message: String,
    game_tag: String,
    product_name: String,
    puuid: String,
    name: String,
    pid: String,
    patchline: String,
    platform_id: String,
    availability: String,
    lol: Value,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryRewardConfigurationEntry {
    maximum_reward: i32,
    reward_value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatEndOfGameStats {
    local_player: LolChatEndOfGamePlayer,
    teams: Vec<LolChatEndOfGameTeam>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPriceOptionDto {
    currency_type: String,
    currency_payment_option: String,
    currency_image_path: String,
    price: i64,
    currency_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterRestrictionInfoDto {
    puuid: String,
    leaver_buster_entry_dto: LolLeaverBusterLeaverBusterEntryDto,
    ranked_restriction_entry_dto: LolLeaverBusterRankedRestrictionEntryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootCollectionsRental {
    rented: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRewardGrant {
    info: LolLootSvcRewardGrant,
    reward_group: LolLootSvcRewardGroup,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthError {
    error_description: String,
    error: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventQueue {
    map_id: i32,
    game_mode: String,
    id: i32,
    category: LolTftEventQueueGameCategory,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolDiscordRpGameDataChampionSummary {
    id: i32,
    name: String,
    alias: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchPatchSieveDownload {
    #[serde(rename = "scd_required")]
    scd_required: bool,
    #[serde(rename = "url")]
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSeries {
    status: String,
    internal_name: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "QueryEvaluationRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct QueryEvaluationRequestDto {
    query: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerNotificationData {
    target_summoner_id: u64,
    tournament_notify_reason: LolClashTournamentNotifyReason,
    roster_notify_reason: LolClashRosterNotifyReason,
    notify_reason: LolClashNotifyReason,
    source_summoner_id: u64,
    notification: LolClashPlayerNotification,
    key_suffix: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowSession {
    game_dodge: LolMatchmakingGameflowGameDodge,
    game_data: LolMatchmakingGameflowGameData,
    phase: LolMatchmakingGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentItemIdentifier {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerNotification {
    expires: String,
    data: Value,
    dismissible: bool,
    icon_url: String,
    created: String,
    background_url: String,
    state: String,
    critical: bool,
    detail_key: String,
    _type: String,
    id: u64,
    title_key: String,
    source: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLeaverBusterPenaltyResponse {
    queue_lockout_timer_expiry_utc_millis: i64,
    has_active_penalty: bool,
    punishment_timer_type: LolLeaverBusterLeaverPenaltyType,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatEndOfGameTeam {
    players: Vec<LolChatEndOfGamePlayer>,
    is_player_team: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolMatchHistoryGAMHSMatchHistoryData")]
#[serde(rename_all = "kebab-case")]
pub struct LolMatchHistoryGamhsMatchHistoryData {
    metadata: LolMatchHistoryGamhsMatchHistoryMetadata,
    json: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneCompletion {
    category: String,
    is_epic: bool,
    statstone_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubBundledItemUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubBundledItemUiData {
    description: String,
    inventory_type: String,
    name: String,
    sub_inventory_type: String,
    owned: bool,
    quantity: u32,
    splash_image: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameMap {
    display_name: String,
    name: String,
    description: String,
    total_players: i32,
    map_id: i32,
    min_custom_players: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionChroma {
    colors: Vec<String>,
    chroma_path: String,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerLevelField {
    final_level: u32,
    initial_level: u32,
    progress: LolSummonerLevelProgression,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSfxNotification {
    path: String,
    delay_millis: i64,
    event_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosNotificationsConfigEntry {
    name: String,
    offset_time_3: i64,
    offset_time_2: i64,
    offset_time_1: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryItemKey {
    item_id: i32,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftPaidBattlepassReward {
    icon_needs_frame: bool,
    description: String,
    reward_group: String,
    icon_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginAuthorization {
    current_platform_id: String,
    subject: String,
    current_account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingGeoInfoResponse {
    geo_info: LolContentTargetingGeoInfo,
    is_initialized: bool,
    error_message: String,
    success: bool,
    is_latest: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubValidateOfferRequestV3 {
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeMetadata {
    tooltips_disabled: bool,
    bonus_descriptions: Vec<LolLootLootDescription>,
    guaranteed_descriptions: Vec<LolLootLootDescription>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksQuickPlayPresetSlotDto {
    position_preference: String,
    perks: String,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedLeagueLadderEntryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderEntryDto {
    provisional_games_remaining: i32,
    previous_day_league_position: i32,
    losses: i32,
    puuid: String,
    previous_season_end_tier: String,
    summoner_name: String,
    previous_season_end_rank: String,
    mini_series_progress: String,
    earned_regalia_reward_ids: Vec<String>,
    league_points: i32,
    wins: i32,
    summoner_id: u64,
    rank: String,
    tier: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsPositionStatistics {
    experts: Vec<LolCareerStatsExpertPlayer>,
    queue_stats: Vec<LolCareerStatsStatisticsByQueue>,
    position: LolCareerStatsSummonersRiftPosition,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitHoneyfruitSettings {
    #[serde(rename = "snoozeUntilMS")]
    snooze_until_ms: u64,
    #[serde(rename = "isSnoozedPermanently")]
    is_snoozed_permanently: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchPatchSieveRelease {
    #[serde(rename = "compat_version")]
    compat_version: LolPatchPatchSieveCompatVersion,
    #[serde(rename = "download")]
    download: LolPatchPatchSieveDownload,
    #[serde(rename = "release")]
    release: LolPatchPatchSieveReleaseInfo,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfo {
    user_info: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceGameEventHotkeys {
    evt_push_to_talk: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesSummoner {
    summoner_id: u64,
    puuid: String,
    display_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardsMetaData {
    quantity: i32,
    champion_id: i32,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreOrderNotificationResource {
    event_type_id: String,
    event_type: String,
    id: u64,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolInventoryInventoryItemWithPayload {
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "uuid")]
    uuid: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "ownershipType")]
    ownership_type: LolInventoryItemOwnershipType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIcon {
    inventory_token: String,
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataChampionSkin {
    id: i32,
    tile_path: String,
    chromas: Vec<LolEndOfGameGameDataSkinChroma>,
    splash_path: String,
    quest_skin_info: LolEndOfGameGameDataQuestSkinInfo,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftDataModelResponse {
    model_data: Value,
    response_code: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubInventoryItem {
    uuid: String,
    expiration_date: String,
    inventory_type: String,
    quantity: u64,
    ownership_type: LolEventHubItemOwnershipType,
    item_id: i32,
    wins: u64,
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataStatstone {
    is_duration: bool,
    category: String,
    icon_full: String,
    is_epic: bool,
    item_id: i32,
    is_retired: bool,
    name: String,
    description: String,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersOfferDto {
    id: String,
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Honor {
    sender_puuid: String,
    voter_relationship: String,
    honor_category: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesInventoryItem {
    uuid: String,
    purchase_date: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitLinkingNotification {
    linked_account: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsSummonerProfileUpdate {
    key: String,
    value: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesGameDataTrovesBannerTable {
    name: String,
    key: String,
    id: String,
    priority: u32,
    loadouts_item: LolTftTrovesLoadoutsItem,
    children: Vec<LolTftTrovesGameDataTrovesBannerTableEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsTeamId {
    full_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsMember {
    summoner_name: String,
    summoner_id: u64,
    has_delegated_invite_power: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrackConfiguration {
    id: String,
    premium_entitlement_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueue {
    maximum_participant_list_size: i32,
    min_level: u32,
    removal_from_game_delay_minutes: i32,
    is_team_builder_managed: bool,
    spectator_enabled: bool,
    show_position_selector: bool,
    _type: String,
    category: LolLobbyQueueGameCategory,
    show_quick_play_slot_selection: bool,
    id: i32,
    champions_required_to_play: u32,
    queue_rewards: LolLobbyQueueReward,
    max_division_for_premade_size_2: String,
    last_toggled_off_time: u64,
    removal_from_game_allowed: bool,
    num_players_per_team: i32,
    minimum_participant_list_size: i32,
    map_id: i32,
    asset_mutator: String,
    game_type_config: LolLobbyQueueGameTypeConfig,
    name: String,
    last_toggled_on_time: u64,
    allowable_premade_sizes: Vec<i32>,
    game_mode: String,
    detailed_description: String,
    is_ranked: bool,
    description: String,
    short_name: String,
    max_tier_for_premade_size_2: String,
    are_free_champions_allowed: bool,
    queue_availability: LolLobbyQueueAvailability,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentActionProgress {
    network: LolPatchComponentStateProgress,
    primary_work: LolPatchComponentStateWorkType,
    current_item: String,
    total: LolPatchComponentStateProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct SanitizerSanitizeResponse {
    modified: bool,
    text: String,
    texts: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubSelectionRequestDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSelectionRequestDto {
    reward_group_id: String,
    selections: Vec<String>,
    grant_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSMatchHistoryData")]
#[serde(rename_all = "kebab-case")]
pub struct LolReplaysGamhsMatchHistoryData {
    json: Value,
    metadata: LolReplaysGamhsMatchHistoryMetadata,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatChatDomainConfig {
    post_game_domain_name: String,
    club_domain_name: String,
    p_2_p_domain_name: String,
    champ_select_domain_name: String,
    custom_game_domain_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearchResource {
    errors: Vec<LolLobbyTeamBuilderMatchmakingSearchErrorResource>,
    ready_check: LolLobbyTeamBuilderMatchmakingReadyCheckResource,
    is_currently_in_queue: bool,
    lobby_id: String,
    queue_id: i32,
    time_in_queue: f32,
    low_priority_data: LolLobbyTeamBuilderMatchmakingLowPriorityData,
    estimated_queue_time: f32,
    search_state: LolLobbyTeamBuilderMatchmakingSearchState,
    dodge_data: LolLobbyTeamBuilderMatchmakingDodgeData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubNavigationButtonUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubNavigationButtonUiData {
    show_glow: bool,
    icon_path: String,
    active_event_id: String,
    show_pip: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPlayerParticipant {
    bot_skill_level: i32,
    summoner_internal_name: String,
    summoner_id: u64,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassCounterInstance {
    group_id: String,
    counter_value: i64,
    owner_id: String,
    product_id: String,
    counter_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesPlayerLoot {
    item_desc: String,
    localized_name: String,
    loot_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedRankedQueueStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStatsDto {
    tier: String,
    previous_season_highest_rank: String,
    rated_rating: i32,
    queue_type: String,
    provisional_game_threshold: i32,
    provisional_games_remaining: i32,
    rank: String,
    league_points: i32,
    losses: i32,
    previous_season_end_tier: String,
    mini_series_progress: String,
    rated_tier: String,
    highest_rank: String,
    previous_season_highest_tier: String,
    wins: i32,
    warnings: LolRankedRankedQueueWarningsDto,
    previous_season_end_rank: String,
    highest_tier: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowGameDodge {
    phase: LolLobbyGameflowPhase,
    state: LolLobbyMatchmakingDodgeState,
    dodge_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterMatchmakingSearchResource {
    errors: Vec<LolLeaverBusterMatchmakingSearchErrorResource>,
    queue_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesEventEndV1 {
    suffix: String,
    when: u64,
    event_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventorySummonerIcon {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueNotifications {
    reward_notifications: Vec<LolRankedRewardNotification>,
    league_notifications: Vec<LolRankedLeagueNotification>,
    global_notifications: Vec<LolRankedGlobalNotification>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMatchmakingReadyCheckResource {
    decliner_ids: Vec<u64>,
    dodge_warning: LolClashMatchmakingDodgeWarning,
    timer: f32,
    player_response: LolClashMatchmakingReadyCheckResponse,
    state: LolClashMatchmakingReadyCheckState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOffer {
    active: bool,
    label: String,
    id: String,
    type_id: String,
    product_id: String,
    merchant_id: String,
    payload: Vec<LolEventHubCapOfferPayloadEntry>,
    start_date: String,
    created_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceSummoner {
    puuid: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStorePurchaseOrderResponseDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolStorePurchaseOrderResponseDto {
    rp_balance: i64,
    ip_balance: i64,
    transactions: Vec<LolStoreTransactionResponseDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatPidBody {
    pid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsNamecheckLoginDataPacket {
    platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftLolTftTencentEventHubConfig {
    #[serde(rename = "troveURL")]
    trove_url: String,
    #[serde(rename = "troveAssetId")]
    trove_asset_id: String,
    #[serde(rename = "logoAssetId")]
    logo_asset_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendResource {
    status_message: String,
    game_name: String,
    is_p_2_p_conversation_muted: bool,
    summoner_id: u64,
    patchline: String,
    puuid: String,
    group_id: u32,
    name: String,
    display_group_id: u32,
    summary: String,
    platform_id: String,
    game_tag: String,
    display_group_name: String,
    pid: String,
    last_seen_online_timestamp: String,
    time: u64,
    icon: i32,
    id: String,
    availability: String,
    product: String,
    product_name: String,
    note: String,
    group_name: String,
    lol: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionBenchChampionV1 {
    champion_id: i32,
    is_priority: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryUpdate {
    bonus_points_gained: i64,
    member_grades: Vec<LolEndOfGameChampionMasteryGrade>,
    id: String,
    puuid: String,
    points_before_game: i64,
    points_gained: i64,
    points_gained_individual_contribution: i64,
    token_earned_after_game: bool,
    game_id: u64,
    points_until_next_level_before_game: i64,
    points_until_next_level_after_game: i64,
    points_since_last_level_before_game: i64,
    level_up_list: Vec<LolEndOfGameChampionMasteryMini>,
    grade: String,
    tokens_earned: i64,
    champion_id: i32,
    level: i64,
    score: i64,
    has_leveled_up: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGroupsSelection {
    reward_groups: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInviteFailed {
    summoner_id: u64,
    summoner_name: String,
    exception: LcdsGameInviteBaseRuntimeException,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGameTraitViewModel")]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameTftEndOfGameTraitViewModel {
    id: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsUpdateLoadoutRequestDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsUpdateLoadoutRequestDto {
    loadout: LolLoadoutsUpdateLoadoutDto,
    service_to_jwts_map: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsLobbyStatus {
    members: Vec<LcdsMember>,
    game_meta_data: String,
    chat_key: String,
    invitation_id: String,
    invitees: Vec<LcdsInvitee>,
    owner: LcdsPlayer,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2DynamicHonorMessage {
    value: i32,
    message_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonSplit {
    victorious_skin_reward_group: LolRankedVictoriousSkin,
    split_id: i32,
    season_id: i32,
    end_time_millis: u64,
    reward_track: Vec<LolRankedSplitRewardGroup>,
    start_time_millis: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSTeam")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsTeam {
    vilemaw_kills: u32,
    inhibitor_kills: u32,
    dominion_victory_score: u32,
    first_inhibitor: bool,
    bans: Vec<LolReplaysGamhsTeamBan>,
    first_dargon: bool,
    first_baron: bool,
    dragon_kills: u32,
    tower_kills: u32,
    team_id: u16,
    horde_kills: u32,
    first_blood: bool,
    rift_herald_kills: u32,
    baron_kills: u32,
    win: String,
    first_tower: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bracket {
    id: i64,
    size: i32,
    matches: Vec<BracketMatch>,
    rosters: Vec<BracketRoster>,
    tournament_id: i64,
    version: i32,
    phase_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DownloadUrlResponseV2 {
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTQuestionResponse")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftQuestionResponse {
    question_id: u64,
    response_data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootRequestDTO-vector-SelectionRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootRequestDtoVectorSelectionRequestDto {
    data: Vec<LolLootSelectionRequestDto>,
    metadata: LolLootRequestMetadataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosNotificationResource {
    notification_name: String,
    queue: String,
    notification_type: String,
    season_end_time: i64,
    division: String,
    tier: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesLootOddsResponse {
    query: String,
    drop_rate: f64,
    display_priority: i32,
    quantity: i32,
    loot_id: String,
    parent_id: String,
    children: Vec<LolTftTrovesLootOddsResponse>,
    label: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDialogMessage {
    msg_id: String,
    params: Vec<String>,
    account_id: u64,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipant {
    team_id: u16,
    spell_2_id: u16,
    timeline: LolMatchHistoryMatchHistoryTimeline,
    participant_id: u16,
    stats: LolMatchHistoryMatchHistoryParticipantStatistics,
    highest_achieved_season_tier: String,
    champion_id: i32,
    spell_1_id: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathIds {
    mission_ids: Vec<String>,
    series_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootBundleContentGdsResource {
    localized_description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LolEventHubEndOfGameXp {
    per_win: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolShutdownShutdownNotification {
    countdown: f32,
    additional_info: String,
    reason: LolShutdownShutdownReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRmsEntitlementPayload {
    entitlement_type_id: String,
    item_id: String,
    resource_operation: String,
    item_type_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryLoyaltyStatusNotification {
    status: LolInventoryLoyaltyStatus,
    rewards: LolInventoryLoyaltyRewardsSimplified,
    reload_inventory: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionEligibilityData {
    player_inventory: PlayerInventory,
    user_info_token: String,
    loyalty_enabled: bool,
    level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsGameDataChampionMasteryGroup {
    id: u32,
    rows: Vec<LolCollectionsGameDataChampionMasteryRow>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyPresenceData {
    party_id: String,
    queue_id: i32,
    max_players: u64,
    summoners: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayCreateMetadata {
    game_version: String,
    queue_id: i32,
    game_type: String,
    game_end: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatMessage {
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "time")]
    time: String,
    #[serde(rename = "body")]
    body: String,
    #[serde(rename = "read")]
    read: bool,
    #[serde(rename = "cid")]
    cid: String,
    #[serde(rename = "type")]
    _type: String,
    #[serde(rename = "game_tag")]
    game_tag: String,
    #[serde(rename = "game_name")]
    game_name: String,
    #[serde(rename = "mid")]
    mid: String,
    #[serde(rename = "name")]
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Describes a function parameter."]
pub struct BindingFullArgumentHelp {
    _type: BindingFullTypeIdentifier,
    name: String,
    description: String,
    optional: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInvitationRequest {
    invitation_id: String,
    game_meta_data: String,
    inviter: LcdsInviter,
    invite_type: String,
    invitation_state: LcdsInvitationState,
    invite_payload: String,
    owner: LcdsPlayer,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatMultiGamePresenceList {
    presences: Vec<LolChatMultiGamePresence>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubResponseDTO-vector-SvcRewardGroup")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubResponseDtoVectorSvcRewardGroup {
    metadata: LolEventHubResponseMetadataDto,
    data: Vec<LolEventHubSvcRewardGroup>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataTftItem {
    id: i32,
    name_id: String,
    name: String,
    square_icon_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSettingsResource {
    data: LolEventHubPlayerSettingsData,
    schema_version: i16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueue {
    is_ranked: bool,
    queue_availability: LolGameQueuesQueueAvailability,
    spectator_enabled: bool,
    show_position_selector: bool,
    last_toggled_on_time: u64,
    max_division_for_premade_size_2: String,
    maximum_participant_list_size: i32,
    category: LolGameQueuesQueueGameCategory,
    allowable_premade_sizes: Vec<i32>,
    removal_from_game_allowed: bool,
    min_level: u32,
    map_id: i32,
    short_name: String,
    _type: String,
    id: i32,
    game_mode: String,
    name: String,
    max_tier_for_premade_size_2: String,
    asset_mutator: String,
    minimum_participant_list_size: i32,
    game_type_config: LolGameQueuesQueueGameTypeConfig,
    show_quick_play_slot_selection: bool,
    description: String,
    is_team_builder_managed: bool,
    is_visible: bool,
    detailed_description: String,
    num_players_per_team: i32,
    queue_rewards: LolGameQueuesQueueReward,
    are_free_champions_allowed: bool,
    champions_required_to_play: u32,
    removal_from_game_delay_minutes: i32,
    last_toggled_off_time: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyDto {
    chat: LolLobbyPartyChatDto,
    eligibility_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
    eligibility_warnings: Vec<LolLobbyGatekeeperRestrictionDto>,
    version: u64,
    activity_locked: bool,
    max_party_size: i32,
    party_type: String,
    game_mode: LolLobbyGameModeDto,
    bot_participants: Vec<LolLobbyBotParticipantDto>,
    party_id: String,
    players: Vec<LolLobbyPartyMemberDto>,
    platform_id: String,
    eligibility_hash: i64,
    activity_resume_utc_millis: u64,
    activity_started_utc_millis: u64,
    active_restrictions: LolLobbyQueueRestrictionDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobby {
    multi_user_chat_id: String,
    removal_reason: LolLobbyTeamBuilderLobbyRemovedFromGameReason,
    local_member: LolLobbyTeamBuilderLobbyMember,
    is_team_builder_managed: bool,
    specifiable_position_preferences: Vec<String>,
    show_position_excluder: bool,
    multi_user_chat_password: String,
    auto_fill_protected_for_promos: bool,
    invitations: Vec<LolLobbyTeamBuilderLobbyInvitation>,
    invitation_id: String,
    muc_jwt_dto: LolLobbyTeamBuilderMucJwtDto,
    was_kicked: bool,
    auto_fill_protected_for_streaking: bool,
    show_position_selector: bool,
    queue_id: i32,
    required_position_coverage_met: bool,
    auto_fill_eligible: bool,
    members: Vec<LolLobbyTeamBuilderLobbyMember>,
    premade_size_allowed: bool,
    allowable_premade_sizes: Vec<i32>,
    can_start_matchmaking: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRiotMessagingServiceChampionMasteryLevelUp {
    champion_level: i64,
    id: u64,
    champion_id: i32,
    has_leveled_up: bool,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventSettingsResource {
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsRequestDTO-vector-SelectionRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsRequestDtoVectorSelectionRequestDto {
    data: Vec<LolRewardsSelectionRequestDto>,
    metadata: LolRewardsRequestMetadataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginDescriptionResource {
    name: String,
    riot_meta: PluginMetadataResource,
    plugin_dependencies: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterMatchAggregatedStats {
    player_champion_ids: Value,
    opponent_short_name: String,
    opponent_icon_color_id: i32,
    game_id: i64,
    kills: i32,
    win: bool,
    opponent_kills: i32,
    round: i32,
    duration_ms: i64,
    loser_bracket: bool,
    opponent_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolHoneyfruitVNGStatusResponse")]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitVngStatusResponse {
    #[serde(rename = "action_url")]
    action_url: String,
    #[serde(rename = "action_url_raw")]
    action_url_raw: String,
    #[serde(rename = "action_required")]
    action_required: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoyaltyRiotMessagingServiceMessage {
    timestamp: i64,
    version: String,
    resource: String,
    service: String,
    payload: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCollectionsChampion {
    bot_enabled: bool,
    active: bool,
    id: i32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubExternalCatalogPluginItem {
    active: bool,
    description: String,
    image_path: String,
    prices: Vec<LolEventHubExternalCatalogPluginPrice>,
    offer_id: String,
    owned: bool,
    item_id: i32,
    inactive_date: u64,
    name: String,
    release_date: u64,
    sale: LolEventHubExternalCatalogSale,
    item_instance_id: String,
    tags: Vec<String>,
    sub_inventory_type: String,
    sub_title: String,
    purchase_date: u64,
    ownership_type: LolEventHubExternalCatalogInventoryOwnership,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectMySelection {
    spell_1_id: u64,
    ward_skin_id: i64,
    selected_skin_id: i32,
    spell_2_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryLoginSession {
    puuid: String,
    state: LolChampionMasteryLoginSessionStates,
    summoner_id: u64,
    connected: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsBustedLeaver {
    leaver_penalty_millis_remaining: u64,
    summoner: MatchmakingLcdsSummoner,
    reason_failed: String,
    access_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "QueryResultDTO")]
#[serde(rename_all = "camelCase")]
pub struct QueryResultDto {
    last_update: i64,
    query_to_loot_names: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksSettingsStorageContainer {
    schema_version: u32,
    data: LolPerksPerkSettings,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsGameDataChampionMasteryRow {
    masteries: Vec<u32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitMatchHistoryList {
    platform_id: String,
    games: LolHoneyfruitMatchHistoryGameList,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootResponseDTO-SvcRewardGrant")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootResponseDtoSvcRewardGrant {
    metadata: LolLootResponseMetadataDto,
    data: LolLootSvcRewardGrant,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAntiAddictionAntiAddictionToken {
    anti_addiction_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleReplayMetadataResponseV2 {
    metadata_responses: Vec<MultipleReplayMetadataResponseItemV2>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceLocalSettingsCategoryResource {
    data: LolPremadeVoiceLocalSettingsCategoryDataResource,
    schema_version: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeagueSessionLeagueSessionTokenEnvelope {
    logout_on_failure: bool,
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchPatchSieveCompatVersion {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTeamPlannerGameflowGameData {
    queue: LolTftTeamPlannerQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolStatstonesStatstoneSet {
    #[serde(rename = "subInventoryType")]
    sub_inventory_type: String,
    #[serde(rename = "stonesOwned")]
    stones_owned: u32,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "ownedFromPacks")]
    owned_from_packs: Vec<LolStatstonesGameDataStatstonePack>,
    #[serde(rename = "milestonesPassed")]
    milestones_passed: u32,
    #[serde(rename = "itemId")]
    item_id: u32,
    #[serde(rename = "statstones")]
    statstones: Vec<LolStatstonesStatstone>,
    #[serde(rename = "itemInstanceID")]
    item_instance_id: String,
    #[serde(rename = "prices")]
    prices: Vec<LolStatstonesPriceInfo>,
    #[serde(rename = "name")]
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardGrant {
    reward_group: LolEventHubSvcRewardGroup,
    info: LolEventHubSvcRewardGrant,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesBanner {
    source_id: String,
    thumbnail_texture: String,
    background_texture: String,
    max_total_rolls: u32,
    start_date: String,
    is_collector_bounty: bool,
    pity_limit: u32,
    name: String,
    platform_texture: String,
    end_date: String,
    event_hub_banner_texture: String,
    banner_texture: String,
    description: String,
    chase_content_id: String,
    pity_counter_id: String,
    version: u8,
    pull_cost: u32,
    celebration_theme: LolTftTrovesTrovesCelebrationThemeData,
    roll_offer: String,
    status: LolTftTrovesTrovesStatus,
    id: String,
    mythic_offer: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatFriendGroupCreate {
    collapsed: bool,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "TournamentInfoDTO")]
#[serde(rename_all = "camelCase")]
pub struct TournamentInfoDto {
    roster: RosterDto,
    pending_roster: PendingRosterDto,
    theme_vp: i32,
    tournament: TournamentDto,
    invitee_pending_rosters: Vec<PendingRosterDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatPluginRegionLocaleChangedEvent {
    region: String,
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyGameflowSession {
    game_dodge: LolLobbyGameflowGameDodge,
    phase: LolLobbyGameflowPhase,
    game_data: LolLobbyGameflowGameData,
    game_client: LolLobbyGameflowGameClient,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LCDSBroadcastNotification")]
#[serde(rename_all = "camelCase")]
pub struct LcdsBroadcastNotification {
    broadcast_messages: Vec<LolServiceStatusBroadcastMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaFrontendConfig {
    selections_enabled: bool,
    hovercard_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesProfileStatstoneSummary {
    name: String,
    champion_id: i32,
    category: String,
    image_url: String,
    value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginLoginSessionWallet {
    ip: i64,
    rp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRankedEosRewardGroupsRewardsList {
    rewards: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsCreateLoadoutRequestDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsCreateLoadoutRequestDto {
    service_to_jwts_map: Value,
    loadout: LolLoadoutsCreateLoadoutDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksPerkUIStyle")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiStyle {
    allowed_sub_styles: Vec<i32>,
    default_perks: Vec<i32>,
    sub_style_bonus: Vec<LolPerksPerkSubStyleBonusResource>,
    id_name: String,
    id: i32,
    tooltip: String,
    default_page_name: String,
    asset_map: Value,
    default_sub_style: i32,
    name: String,
    icon_path: String,
    slots: Vec<LolPerksPerkUiSlot>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchUxResource {
    visible: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRankedEosRewardsConfig {
    seasons: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc = "Describes the type of a value."]
pub struct BindingFullTypeIdentifier {
    _type: String,
    element_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPerksDefaultStatModsPerSubStyle {
    perks: Vec<i32>,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayMetadata {
    download_progress: u32,
    state: LolReplaysMetadataState,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleDialogMessageResponse {
    msg_id: String,
    command: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampionsLcdsDynamicClientConfig {
    disabled_champions: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsAccountSettingsPayload {
    data: LolNpeRewardsAccountSettingsData,
    schema_version: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolVanguardGameflowSession {
    phase: LolVanguardGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerTierDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerTierDto {
    player_id: u64,
    primary_pos: Position,
    tier: i32,
    second_pos: Position,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineTier {
    id: i64,
    tile_path: String,
    splash_path: String,
    uncentered_splash_path: String,
    splash_video_path: String,
    stage: i64,
    description: String,
    load_screen_path: String,
    name: String,
    collection_splash_video_path: String,
    short_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolServiceStatusRiotStatusTitle {
    locale: String,
    content: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubChampionSkinEmblemPosition {
    horizontal: String,
    vertical: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesCapOrdersResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTrovesCapOrdersResponseDto {
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOption {
    price_details: Vec<LolEventHubPriceDetail>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlatformStats {
    player_stat_summaries: LolLobbyPlayerStatSummaryContainer,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemSale {
    discount: f32,
    end_date: String,
    start_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemSale {
    discount: f32,
    start_date: String,
    end_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolItemSetsGameDataChampion {
    alias: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotChampion {
    active: bool,
    id: i32,
    name: String,
    bot_difficulties: Vec<LolLobbyLobbyBotDifficulty>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubRequestDTO-SelectionRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubRequestDtoSelectionRequestDto {
    metadata: LolEventHubRequestMetadataDto,
    data: LolEventHubSelectionRequestDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchResource {
    low_priority_data: LolMatchmakingMatchmakingLowPriorityData,
    dodge_data: LolMatchmakingMatchmakingDodgeData,
    lobby_id: String,
    errors: Vec<LolMatchmakingMatchmakingSearchErrorResource>,
    estimated_queue_time: f32,
    is_currently_in_queue: bool,
    ready_check: LolMatchmakingMatchmakingReadyCheckResource,
    search_state: LolMatchmakingMatchmakingSearchState,
    queue_id: i32,
    time_in_queue: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMissionsLoyaltyStatusNotification {
    status: LolMissionsLoyaltyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeam {
    dragon_kills: u32,
    tower_kills: u32,
    horde_kills: u32,
    first_baron: bool,
    vilemaw_kills: u32,
    first_tower: bool,
    baron_kills: u32,
    dominion_victory_score: u32,
    bans: Vec<LolMatchHistoryMatchHistoryTeamBan>,
    first_blood: bool,
    team_id: u16,
    win: String,
    first_inhibitor: bool,
    first_dargon: bool,
    inhibitor_kills: u32,
    rift_herald_kills: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferResponseV3 {
    validation_errors: Vec<LolPurchaseWidgetValidateOfferError>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolContentTargetingGeoInfo {
    region: String,
    city: String,
    country: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStoreItemOrderDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemOrderDto {
    item_id: i32,
    inventory_type: String,
    quantity: u32,
    rp_cost: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "OpenedTeamDTO")]
#[serde(rename_all = "camelCase")]
pub struct OpenedTeamDto {
    name: String,
    logo: i32,
    invitation_id: String,
    tier: i32,
    members: Vec<OpenedTeamMemberDto>,
    open_positions: Vec<Position>,
    short_name: String,
    captain_id: u64,
    invitees: Vec<PendingRosterInviteeDto>,
    logo_color: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoadoutsAccessTokenResource {
    token: String,
    expiry: u64,
    scopes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPerksNamecheckAuthorization {
    subject: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingSummoner {
    summoner_level: u32,
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubValidateOfferError {
    meta: String,
    error_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundledItem {
    item_id: i32,
    quantity: u32,
    inventory_type: String,
    discount_prices: Vec<LolStoreBundledItemCost>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorInfo {
    show_skin_selector: bool,
    selected_champion_id: i32,
    skin_selection_disabled: bool,
    selected_skin_id: i32,
    is_skin_granted_from_boost: bool,
    champion_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopInventoryItemDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolYourshopInventoryItemDto {
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "entitlementId")]
    entitlement_id: String,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "usedInGameDate")]
    used_in_game_date: String,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "instanceTypeId")]
    instance_type_id: String,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "lsb")]
    lsb: bool,
    #[serde(rename = "entitlementTypeId")]
    entitlement_type_id: String,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "ownedQuantity")]
    owned_quantity: u64,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "itemId")]
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootProgressionConfigMilestoneGdsResource {
    trigger_value: u64,
    properties: Vec<LolLootProgressionConfigMilestonePropertiesGdsResource>,
    id: String,
    counter: LolLootMilestonesProgressionCounterGdsResource,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryLevelMark {
    champion_points: i32,
    level: i32,
    marks: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGrantorDescription {
    app_name: String,
    entity_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesEntitlementNotificationResource {
    item_id: String,
    item_type_id: String,
    entitlement_type_id: String,
    resource_operation: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginSale {
    discount: f32,
    end_date: String,
    start_date: String,
    cost: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowQueue {
    game_type_config: LolMatchmakingGameflowGameTypeConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerUserInfo {
    user_info: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryNotification {
    inventory_type: String,
    _type: String,
    id: i64,
    item_id: i32,
    acknowledged: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolClashChangeNameRequest {
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClashRewardConfigEntry {
    key: String,
    vals: Vec<ClashRewardOutput>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesGameDataTrovesBannerTableEntry {
    banner_node: LolTftTrovesGameDataTrovesBannerTable,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatPatchlineMetadata {
    #[serde(rename = "product_id")]
    product_id: String,
    #[serde(rename = "content_paths")]
    content_paths: Value,
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "content_cookies")]
    content_cookies: Vec<LolChatContentCookies>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyReadyDto {
    ready: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatSummoner {
    unnamed: bool,
    summoner_id: u64,
    display_name: String,
    summoner_level: u32,
    profile_icon_id: i32,
    game_name: String,
    puuid: String,
    tag_line: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampionsCollectionsChampionSkinAugments {
    augments: Vec<LolChampionsCollectionsChampionSkinAugment>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashNoShowPingResponse {
    tournament_id: i64,
    match_id: i64,
    data: String,
    dodge_time: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashRewardDefinition {
    reward_type: ClashRewardType,
    reward_spec: ClashRewardSpec,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGroupResource {
    is_localized: bool,
    collapsed: bool,
    priority: i32,
    id: u32,
    name: String,
    is_meta_group: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTMapSkinGroupedViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftMapSkinGroupedViewModel {
    default_item_id: i32,
    groups: Vec<LolCosmeticsTftMapSkinGroupViewModel>,
    selected_loadout_item: LolCosmeticsCosmeticsTftMapSkinViewModel,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataChampionQuestSkin {
    id: i32,
    splash_path: String,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClashDisabledConfig {
    disabled_reason: String,
    estimated_enable_time_millis: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSet {
    map: String,
    associated_champions: Vec<i32>,
    uid: String,
    title: String,
    mode: String,
    blocks: Vec<LolItemSetsItemSetBlock>,
    started_from: String,
    sortrank: i32,
    associated_maps: Vec<i32>,
    preferred_item_slots: Vec<LolItemSetsPreferredItemSlot>,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGameflowRegionLocale {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EndOfGameStats {
    game_id: u64,
    report_game_id: u64,
    local_player: LolHonorV2EndOfGamePlayer,
    ranked: bool,
    game_type: String,
    invalid: bool,
    teams: Vec<LolHonorV2EndOfGameTeam>,
    queue_type: String,
    game_ended_in_early_surrender: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderStanding {
    league_points: i32,
    puuid: String,
    summoner_id: u64,
    summoner_name: String,
    position: i32,
    wins: i32,
    rated_tier: LolRankedRatedTier,
    position_delta: i32,
    previous_position: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerCreatedId {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftSettingsResource {
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeSignedUpdatePayload {
    tokens_by_type: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyMucJwtDto {
    channel_claim: String,
    jwt: String,
    domain: String,
    target_region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootEntityInstance {
    group_id: String,
    milestones: Vec<LolLootMilestoneInstance>,
    counters: Vec<LolLootCounterInstance>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinMinimal {
    id: i32,
    name: String,
    ownership: LolChampSelectCollectionsOwnership,
    disabled: bool,
    splash_path: String,
    tile_path: String,
    skin_augments: LolChampSelectChampionSkinAugments,
    chroma_path: String,
    is_base: bool,
    still_obtainable: bool,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueReward {
    is_ip_enabled: bool,
    party_size_ip_rewards: Vec<i32>,
    is_champion_points_enabled: bool,
    is_xp_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyPositionPreferencesV2 {
    excluded_preference: String,
    first_preference: String,
    second_preference: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpQueue {
    id: i32,
    min_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetLoginSession {
    state: LolPurchaseWidgetLoginSessionStates,
    id_token: String,
    puuid: String,
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRmsStoreEntitlementItem {
    inventory_type: String,
    item_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectTimer {
    adjusted_time_left_in_phase: i64,
    phase: String,
    is_infinite: bool,
    total_time_in_phase: i64,
    internal_now_in_epoch_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectCollectionsRental {
    rented: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathRewardStrategy {
    group_strategy: String,
    select_max_group_count: i16,
    select_min_group_count: i16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkIdListResource {
    perk_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardData {
    next_update_time: i64,
    row_data: Vec<LolSocialLeaderboardSocialLeaderboardRowData>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesCapOrdersRequestMetaDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesCapOrdersRequestMetaDto {
    correlation_id: String,
    jwt: String,
    xid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesOddsTableDisplayMetadata {
    name_tra_key: String,
    priority: u8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorChildSkin {
    is_base: bool,
    chroma_preview_path: String,
    parent_skin_id: i32,
    ownership: LolChampSelectCollectionsOwnership,
    short_name: String,
    splash_path: String,
    name: String,
    is_champion_unlocked: bool,
    id: i32,
    tile_path: String,
    skin_augments: Value,
    stage: u64,
    disabled: bool,
    still_obtainable: bool,
    colors: Vec<String>,
    unlocked: bool,
    champion_id: i32,
    splash_video_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsGameDataQuestSkinInfo {
    tiers: Vec<LolCollectionsGameDataChampionQuestSkin>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTeamPlannerGameDataTFTSets")]
#[serde(rename_all = "PascalCase")]
pub struct LolTftTeamPlannerGameDataTftSets {
    #[serde(rename = "LCTFTModeData")]
    lctft_mode_data: LolTftTeamPlannerTftModeData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesRewardsResponse {
    rewards: LolTftTrovesTrovesRewards,
    pull_type: u8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopWallet {
    rp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatLcuSocialConfig {
    #[serde(rename = "allowGroupByGame")]
    allow_group_by_game: bool,
    force_chat_filter: bool,
    replace_rich_messages: bool,
    aggressive_scanning: bool,
    queue_job_grace_seconds: u64,
    silence_chat_while_in_game: bool,
    #[serde(rename = "gameNameTaglineEnabled")]
    game_name_tagline_enabled: bool,
    #[serde(rename = "platformToRegionMap")]
    platform_to_region_map: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesOperationalSpectatorConfig {
    is_enabled: bool,
    is_using_operational_config: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootLcdsRecipeListClientDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeListClientDto {
    last_update: i64,
    recipes: Vec<LootLcdsRecipeClientDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsInventoryItem {
    item_id: i32,
    quantity: u64,
    uuid: String,
    ownership_type: LolCollectionsItemOwnershipType,
    inventory_type: String,
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTransaction {
    item_name: String,
    icon_url: String,
    transaction_id: String,
    item_key: LolEventHubItemKey,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMember {
    summoner_internal_name: String,
    show_position_excluder: bool,
    bot_difficulty: LolLobbyLobbyBotDifficulty,
    bot_champion_id: i32,
    excluded_position_preference: String,
    auto_fill_eligible: bool,
    is_bot: bool,
    can_invite_others: bool,
    is_spectator: bool,
    auto_fill_protected_for_streaking: bool,
    auto_fill_protected_for_soloing: bool,
    is_owner: bool,
    auto_fill_protected_for_promos: bool,
    position_preferences: LolLobbyLobbyPositionPreferences,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorBanNotification {
    is_perma_ban: bool,
    reason: String,
    id: u64,
    time_until_ban_expires: u64,
    source: LolPlayerBehaviorNotificationSource,
    display_reform_card: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomJoinOptionsDto {
    team: String,
    lobby_password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolInventoryInventoryCacheEntry {
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
    #[serde(rename = "signedInventoryJwt")]
    signed_inventory_jwt: String,
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesLoadoutsItem {
    name: String,
    item_type_id: String,
    star_level: u32,
    item_instance_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopOfferRequests {
    offers: Vec<LolYourshopOfferRequest>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueDodger {
    reason_failed: String,
    dodge_penalty_remaining_time: u64,
    summoner: MatchmakingLcdsSummoner,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolSettingsSettingsConfig {
    #[serde(rename = "isSoundEnabled")]
    is_sound_enabled: bool,
    #[serde(rename = "isHotkeysEnabled")]
    is_hotkeys_enabled: bool,
    #[serde(rename = "isTermsEnabled")]
    is_terms_enabled: bool,
    #[serde(rename = "isPrivacyNoticeEnabled")]
    is_privacy_notice_enabled: bool,
    #[serde(rename = "isReplaysEnabled")]
    is_replays_enabled: bool,
    #[serde(rename = "isGameplayEnabled")]
    is_gameplay_enabled: bool,
    #[serde(rename = "isInterfaceEnabled")]
    is_interface_enabled: bool,
    #[serde(rename = "localizedLicensesURL")]
    localized_licenses_url: String,
    #[serde(rename = "isLegalStatementsEnabled")]
    is_legal_statements_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolInventoryLoyaltyRewards {
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "xpBoost")]
    xp_boost: Value,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolClashTournamentUpdatedNotificationDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentUpdatedNotificationDto {
    tournament: TournamentDto,
    reason: LolClashTournamentNotifyReason,
    tournament_id: i64,
    relevant_phase_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct EndOfGameLcdsPointsPenalty {
    _type: String,
    penalty: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyUserResource {
    summoner_id: u64,
    lol: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesNumberFormattingBehavior {
    trim_trailing_zeros_after_decimal: bool,
    western_number_grouping: bool,
    digits_for_thousands_seperator: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassEntityInstance {
    milestones: Vec<LolTftPassMilestoneInstance>,
    group_id: String,
    counters: Vec<LolTftPassCounterInstance>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsStoreEntitlementItem {
    item_id: String,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderSettingCategoryResource {
    schema_version: i32,
    data: LolLobbyTeamBuilderChampionSelectPreferences,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLobbyLobbyParticipantDto {
    #[serde(rename = "botDifficulty")]
    bot_difficulty: LolLobbyLobbyBotDifficulty,
    #[serde(rename = "autoFillProtectedForStreaking")]
    auto_fill_protected_for_streaking: bool,
    #[serde(rename = "summonerLevel")]
    summoner_level: u32,
    #[serde(rename = "summonerName")]
    summoner_name: String,
    #[serde(rename = "tftNPEQueueBypass")]
    tft_npe_queue_bypass: bool,
    #[serde(rename = "autoFillProtectedForRemedy")]
    auto_fill_protected_for_remedy: bool,
    #[serde(rename = "firstPositionPreference")]
    first_position_preference: String,
    #[serde(rename = "puuid")]
    puuid: String,
    #[serde(rename = "allowedKickOthers")]
    allowed_kick_others: bool,
    #[serde(rename = "allowedChangeActivity")]
    allowed_change_activity: bool,
    #[serde(rename = "subteamIndex")]
    subteam_index: i8,
    #[serde(rename = "summonerInternalName")]
    summoner_internal_name: String,
    #[serde(rename = "ready")]
    ready: bool,
    #[serde(rename = "showGhostedBanner")]
    show_ghosted_banner: bool,
    #[serde(rename = "allowedToggleInvite")]
    allowed_toggle_invite: bool,
    #[serde(rename = "autoFillProtectedForPromos")]
    auto_fill_protected_for_promos: bool,
    #[serde(rename = "isSpectator")]
    is_spectator: bool,
    #[serde(rename = "teamId")]
    team_id: i32,
    #[serde(rename = "secondPositionPreference")]
    second_position_preference: String,
    #[serde(rename = "summonerId")]
    summoner_id: u64,
    #[serde(rename = "summonerIconId")]
    summoner_icon_id: i32,
    #[serde(rename = "playerSlots")]
    player_slots: Vec<LolLobbyQuickPlayPresetSlotDto>,
    #[serde(rename = "allowedStartActivity")]
    allowed_start_activity: bool,
    #[serde(rename = "intraSubteamPosition")]
    intra_subteam_position: i8,
    #[serde(rename = "botId")]
    bot_id: String,
    #[serde(rename = "isLeader")]
    is_leader: bool,
    #[serde(rename = "autoFillEligible")]
    auto_fill_eligible: bool,
    #[serde(rename = "autoFillProtectedForSoloing")]
    auto_fill_protected_for_soloing: bool,
    #[serde(rename = "isBot")]
    is_bot: bool,
    #[serde(rename = "allowedInviteOthers")]
    allowed_invite_others: bool,
    #[serde(rename = "botChampionId")]
    bot_champion_id: i32,
    #[serde(rename = "quickplayPlayerState")]
    quickplay_player_state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineItemDto {
    large_video_path: String,
    thumbnail_image_path: String,
    localized_short_name: String,
    localized_long_name: String,
    large_image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentStateProgress {
    bytes_complete: u64,
    bytes_per_second: f64,
    bytes_required: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootProgressionConfigMilestoneRewardGdsResource {
    id: String,
    legacy_loot_item: String,
    reward_type: String,
    quantity: i32,
    loot_item_to_grant: LolLootMilestoneLootItemRewardGdsResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LolTftPassEndOfGameXp {
    per_win: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingLowPriorityData {
    penalty_time: f64,
    penalty_time_remaining: f64,
    penalized_summoner_ids: Vec<u64>,
    busted_leaver_access_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeagueSessionAntiAddictionTokenEnvelope {
    anti_addiction_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatFriendGroupOrder {
    groups: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesTrophyProfileData {
    season_id: i64,
    pedestal: String,
    gem: String,
    cup: String,
    bracket: i64,
    theme: String,
    tier: i64,
}
type LolEventHubRmsWalletPayload = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLolTftTencentEventHubConfigs {
    tencent_eventhub_configs: Vec<LolTftEventLolTftTencentEventHubConfig>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchChunkingPatcherEnvironment {
    #[serde(rename = "game_patcher_enabled")]
    game_patcher_enabled: bool,
    #[serde(rename = "game_patcher_available")]
    game_patcher_available: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemDefinition {
    loyalty_unlocked: bool,
    name: String,
    description: String,
    image_path: String,
    owned: bool,
    assets: LolEventHubCatalogPluginItemAssets,
    tags: Vec<String>,
    item_id: i32,
    has_visible_loot_odds: bool,
    sub_title: String,
    bundled_item_price: LolEventHubBundledItemPricingInfo,
    inventory_type: String,
    sub_inventory_type: String,
    metadata: Vec<LolEventHubItemMetadataEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassWalletCacheEntry {
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
    #[serde(rename = "signedBalancesJwt")]
    signed_balances_jwt: String,
    #[serde(rename = "valid")]
    valid: bool,
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchResource {
    errors: Vec<LolLobbyLobbyMatchmakingSearchErrorResource>,
    low_priority_data: LolLobbyLobbyMatchmakingLowPriorityDataResource,
    search_state: LolLobbyLobbyMatchmakingSearchState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsGameNotification {
    message_argument: String,
    _type: String,
    message_code: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChatFriend {
    lol: Value,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinEmblem {
    emblem_path: LolChampionsCollectionsChampionSkinEmblemPath,
    name: String,
    positions: LolChampionsCollectionsChampionSkinEmblemPosition,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterDynamicStateNotification {
    roster_dynamic_state: RosterDynamicStateDto,
    notify_reason: LolClashRosterNotifyReason,
    source_player_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopPurchaseResponse {
    items: Vec<LolYourshopPurchaseItem>,
    wallet: LolYourshopWallet,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSummonerIcon {
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkSettingResource {
    perk_ids: Vec<i32>,
    perk_style: i32,
    perk_sub_style: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigEntitlements {
    subject: String,
    entitlements: Vec<String>,
    issuer: String,
    access_token: String,
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatMessageSend {
    message: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGameflowSession {
    game_client: LolReplaysGameflowGameClient,
    phase: LolReplaysGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampionsCollectionsChampionPlayableCounts {
    #[serde(rename = "championsFreeToPlay")]
    champions_free_to_play: u32,
    #[serde(rename = "championsRented")]
    champions_rented: u32,
    #[serde(rename = "championsOwned")]
    champions_owned: u32,
    #[serde(rename = "championsXboxGPReward")]
    champions_xbox_gp_reward: u32,
    #[serde(rename = "championsLoyaltyReward")]
    champions_loyalty_reward: u32,
}
type LolHoneyfruitGamhsMatchHistoryData = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusLoginDataPacket {
    broadcast_notification: LolServiceStatusBroadcastNotification,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterSummoner {
    acct_id: u64,
    sum_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTMapSkin")]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsTftMapSkin {
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "owned")]
    owned: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassTFTPassAsset")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPassAsset {
    internal_name: String,
    icon_needs_frame: bool,
    icon_texture_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsChestEligibility {
    earnable_chests: u32,
    maximum_chests: u32,
    next_chest_recharge_time: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopLoyaltyStatusNotification {
    status: LolYourshopLoyaltyStatus,
    reload_inventory: bool,
    rewards: LolYourshopLoyaltyRewardsSimplified,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTeamPlannerTFTTeamPlannerConfig")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTftTeamPlannerConfig {
    reminders_enabled: bool,
    enabled: bool,
    trait_tooltip_champs_enabled: bool,
    multiple_sets_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRmsXboxSubscriptionChange {
    subscription_id: String,
    identity_provider: Vec<String>,
    puuid: String,
    active: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestone {
    properties: Value,
    group_id: String,
    trigger_value: i64,
    id: String,
    name: String,
    counter_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigAuthenticatedConnection {
    subscribed: bool,
    connection_id: u32,
    auth_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampionsGameDataChampionSummary {
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDetails {
    title: String,
    description: String,
    sub_title: String,
    icon_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTokenUpsell {
    id: String,
    tooltip_description: String,
    premium_currency_name: String,
    currently_locked: LolEventHubTokenUpsellLockedType,
    button_text: String,
    title: String,
    currency_url: String,
    dependent_inventory_id: i32,
    locked_count: i32,
    start_date: String,
    background_url: String,
    dependent_inventory_type: String,
    end_date: String,
    tooltip_background_url: String,
    purchase_url: String,
    tooltip_title: String,
    internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreGiftableResult {
    friends: Vec<LolStoreGiftingFriend>,
    config: LolStoreGiftingConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGiftingConfig {
    gifting_rp_max_daily_gifts_send: u32,
    gifting_item_min_level_send: u32,
    gifting_rp_min_level_send: u32,
    gifting_rp_max_daily_gifts_receive: u32,
    gifting_item_max_daily_gifts_send: u32,
    recipient_level_limit_item: u32,
    gifting_restriction_flag_rioter: u32,
    gifting_hextec_max_daily_gifts_receive: u32,
    gifting_item_max_daily_gifts_receive: u32,
    requires_identity_verification: bool,
    gifting_hextech_max_daily_gifts_send: u32,
    recipient_level_limit_rp: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeamBoost {
    unlocked: bool,
    available_skins: Vec<i64>,
    skin_unlock_mode: String,
    ip_reward_for_purchaser: i64,
    summoner_name: String,
    ip_reward: i64,
    price: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMapsTutorialCard {
    image_path: String,
    footer: String,
    description: String,
    header: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubInventoryCacheEntry {
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
    #[serde(rename = "signedInventoryJwt")]
    signed_inventory_jwt: String,
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BracketMatch {
    roster_id_1: i64,
    game_start_time: i64,
    game_id: i64,
    result_history: String,
    lowest_possible_position: i32,
    loser_bracket: bool,
    forfeit_roster_id: i64,
    status: ClientBracketMatchStatus,
    roster_id_2: i64,
    round: i32,
    highest_possible_position: i32,
    winner_id: i64,
    order: i32,
    round_start_time: i64,
    id: i64,
    fail_roster_status: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventPluginRegionLocaleChangedEvent {
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatConfigStatus {
    readiness: LolChatConfigReadinessEnum,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerSpells {
    spells: Vec<u64>,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsWardSkinList {
    ward_skin_list: Vec<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassInventoryItemWithPayload {
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "uuid")]
    uuid: String,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "ownershipType")]
    ownership_type: LolTftPassItemOwnershipType,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatRankedQueueStats {
    queue_type: LolChatLeagueQueueType,
    division: LolChatLeagueDivision,
    wins: i32,
    tier: String,
    games: i32,
    is_provisional: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassPlayerNotification {
    detail_key: String,
    source: String,
    state: String,
    _type: String,
    title_key: String,
    icon_url: String,
    critical: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPracticeGameConfig {
    region: String,
    spectator_delay_enabled: bool,
    passback_url: String,
    allow_spectators: String,
    game_password: String,
    game_name: String,
    game_map: LcdsGameMap,
    game_mode: String,
    max_num_players: i32,
    passback_data_packet: String,
    game_mutators: Vec<String>,
    game_version: String,
    game_type_config: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootMilestonesProgressionConfigRepeatGdsResource {
    name: LolLootMilestonesProgressionGroupRepeatGdsResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsPlayerUpdateResponse {
    player_missions: Vec<PlayerMissionDto>,
    player_series: Vec<SeriesDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesGameDataTrovesBannerReward {
    item_type_id: String,
    star_level: u32,
    currency_id: String,
    reward_texture_path: String,
    item_instance_id: String,
    name: String,
    highlight_reward_asset_path: String,
    rarity: LolTftTrovesLootRarity,
    quantity: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc = "Represents a failed asynchronous operation."]
pub struct BindingAsyncFailureEvent {
    async_token: u32,
    error: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubDiscountPricingInfo {
    cost_type: String,
    original_cost: i32,
    discount: f32,
    currency: String,
    cost: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChampionSkinSelection {
    champion_id: i32,
    skin_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChatBlockedPlayerResource {
    name: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksSummoner {
    display_name: String,
    internal_name: String,
    xp_since_last_level: u64,
    percent_complete_for_next_level: u32,
    profile_icon_id: i32,
    puuid: String,
    reroll_points: LolPerksSummonerRerollPoints,
    summoner_level: u32,
    xp_until_next_level: u64,
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersLoadoutsSlot {
    item_id: i32,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectSession {
    chat_details: LolChampSelectLegacyChampSelectChatRoomDetails,
    timer: LolChampSelectLegacyChampSelectTimer,
    is_spectating: bool,
    has_simultaneous_bans: bool,
    trades: Vec<LolChampSelectLegacyChampSelectTradeContract>,
    allow_rerolling: bool,
    allow_battle_boost: bool,
    bans: LolChampSelectLegacyChampSelectBannedChampions,
    local_player_cell_id: i64,
    my_team: Vec<LolChampSelectLegacyChampSelectPlayerSelection>,
    their_team: Vec<LolChampSelectLegacyChampSelectPlayerSelection>,
    is_custom_game: bool,
    allow_skin_selection: bool,
    rerolls_remaining: u64,
    has_simultaneous_picks: bool,
    actions: Vec<Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLootCollectionsOwnership {
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
    #[serde(rename = "rental")]
    rental: LolLootCollectionsRental,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsResponseDTO-SvcRewardGrant")]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsResponseDtoSvcRewardGrant {
    metadata: LolRewardsResponseMetadataDto,
    data: LolRewardsSvcRewardGrant,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubAccessTokenResource {
    scopes: Vec<String>,
    expiry: u64,
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationThemeData {
    currency_segment_data: LolTftTrovesTrovesCelebrationCurrencySegmentData,
    portal_segment_data: LolTftTrovesTrovesCelebrationPortalSegmentData,
    standard_segment_data: LolTftTrovesTrovesCelebrationStandardSegmentData,
    highlight_segment_data: LolTftTrovesTrovesCelebrationHighlightSegmentData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassSimpleInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassSimpleInventoryResponseDto {
    data: LolTftPassSimpleInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiotMessagingServicePluginRegionLocaleChangedEvent {
    region: String,
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersPlayerStatus {
    current_lobby_status: LolSuggestedPlayersSuggestedPlayersLobbyStatus,
    last_queued_lobby_status: LolSuggestedPlayersSuggestedPlayersLobbyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "BoostTeamSkinRentalDTO")]
#[serde(rename_all = "camelCase")]
pub struct BoostTeamSkinRentalDto {
    available_skins: Vec<i64>,
    unlocked: bool,
    puuid: String,
    price: i64,
    ip_reward_for_purchaser: i64,
    summoner_name: String,
    skin_unlock_mode: String,
    summoner_id: u64,
    ip_reward: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsStreams {
    team_b_id: i64,
    team_a_id: i64,
    team_b_acronym: String,
    team_a_logo_url: String,
    team_a_name: String,
    team_a_guid: String,
    tournament_description: String,
    team_a_acronym: String,
    team_b_guid: String,
    team_b_name: String,
    team_b_logo_url: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginQueue {
    max_displayed_wait_time_seconds: u64,
    estimated_position_in_queue: u64,
    approximate_wait_time_seconds: u64,
    max_displayed_position: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCatalogItemMetadataEntry {
    value: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginAccountStateResource {
    state: LolLoginAccountStateType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SLIBoolDiagnostic")]
#[serde(rename_all = "kebab-case")]
pub struct SliBoolDiagnostic {
    value: bool,
    key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPurchaseItem {
    order_id: String,
    offer_id: String,
    inventory_type: String,
    item_id: i32,
    price_paid: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingReadyCheckResource {
    timer: f32,
    player_response: LolLobbyTeamBuilderMatchmakingReadyCheckResponse,
    dodge_warning: LolLobbyTeamBuilderMatchmakingDodgeWarning,
    decliner_ids: Vec<u64>,
    state: LolLobbyTeamBuilderMatchmakingReadyCheckState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerLootDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerLootDto {
    count: i32,
    expiry_time: i64,
    loot_name: String,
    ref_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerData {
    lft: bool,
    is_clash_banned: bool,
    tickets: Value,
    tier: i32,
    secondary_pos: String,
    primary_pos: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLeaverBusterAbandoned {
    abandoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsItemSetBlock {
    show_if_summoner_spell: String,
    hide_if_summoner_spell: String,
    items: Vec<LolItemSetsItemSetItem>,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginIdToken {
    token: String,
    expiry: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPendingRosterNotification {
    notify_reason: LolClashNotifyReason,
    target_player_id: u64,
    source_player_id: u64,
    pending_roster: PendingRosterDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersSubOrderDto {
    offer: LolEventHubCapOrdersOfferDto,
    recipient_id: String,
    offer_context: LolEventHubCapOrdersOfferContextDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameDataChallengeTitle {
    background_image_path: String,
    name: String,
    icon_path: String,
    title_acquisition_type: String,
    title_acquisition_name: String,
    item_id: i32,
    is_permanent_title: bool,
    title_requirement_description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingLowPriorityData {
    penalty_time: f64,
    reason: String,
    penalized_summoner_ids: Vec<u64>,
    busted_leaver_access_token: String,
    penalty_time_remaining: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTDamageSkin")]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsTftDamageSkin {
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "level")]
    level: u32,
    #[serde(rename = "upgrades")]
    upgrades: Vec<String>,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCounterInstance {
    owner_id: String,
    product_id: String,
    counter_value: i64,
    counter_id: String,
    group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMassDisenchantClientConfig {
    max_loot_items_size_mass_craft: i16,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootTrigger {
    _type: String,
    trigger_value: u64,
    counter_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampionsCollectionsChampionSkinEmblemPosition {
    vertical: String,
    horizontal: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameQueue {
    _type: String,
    is_ranked: bool,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsLoginSeriesSettings {
    last_completed_mission_date: i64,
    all_rewards_claimed: bool,
    last_completed_mission_internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksNamecheckPayload {
    name_validation_context: String,
    shard: String,
    puuid: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PartiesVoiceDTO")]
#[serde(rename_all = "kebab-case")]
pub struct PartiesVoiceDto {
    jwt: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerLootDefinitionsDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerLootDefinitionsDto {
    loot_item_list: LootItemListClientDto,
    player_loot: Vec<PlayerLootDto>,
    recipe_list: LootLcdsRecipeListClientDto,
    query_result: QueryResultDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsGameDataChampionSkin {
    skin_augments: LolCollectionsGameDataChampionSkinAugments,
    id: i32,
    is_base: bool,
    quest_skin_info: LolCollectionsGameDataQuestSkinInfo,
    name: String,
    splash_path: String,
    splash_video_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTDamageSkinGroupViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftDamageSkinGroupViewModel {
    group_name: String,
    num_owned: u32,
    items: Vec<LolCosmeticsCosmeticsTftDamageSkinViewModel>,
    group_id: u32,
    num_available: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct MatchmakingLcdsMatchMakerPayload {
    leaver_buster_access_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsXboxSubscriptionChange {
    subscription_id: String,
    identity_provider: Vec<String>,
    puuid: String,
    active: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootRequestDTO-vector-string")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootRequestDtoVectorString {
    metadata: LolLootRequestMetadataDto,
    data: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLoginDataPacket {
    all_summoner_data: LolLeaverBusterAllSummonerData,
    simple_messages: Vec<LolLeaverBusterSimpleMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestone {
    counter_id: String,
    trigger_value: i64,
    name: String,
    properties: Value,
    group_id: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginResource {
    mounted_asset_bundles: Value,
    order_wad_file_mounted: i32,
    implemented_contracts: Vec<PluginResourceContract>,
    dependencies: Vec<PluginResourceContract>,
    subtype: String,
    supertype: String,
    full_name: String,
    feature: String,
    short_name: String,
    app: String,
    threading_model: String,
    asset_bundle_names: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatSession {
    #[serde(rename = "loaded")]
    loaded: bool,
    #[serde(rename = "game_tag")]
    game_tag: String,
    #[serde(rename = "resource")]
    resource: String,
    #[serde(rename = "state")]
    state: LolChatChatSessionState,
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "game_name")]
    game_name: String,
    #[serde(rename = "name")]
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerStatSummaryContainer {
    player_stat_summary_set: Vec<LolLobbyPlayerStatSummary>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksLoginSession {
    state: LolPerksLoginSessionState,
    account_id: u64,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberBanUnbanTournament {
    tournament_id: i64,
    tournamentname_loc_key_secondary: String,
    tournament_phase_lock_in_time: i64,
    tournamentname_loc_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampionsCollectionsChampionSpell {
    name: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemChoiceDetails {
    purchase_options: Vec<LolEventHubPurchaseOption>,
    item: LolEventHubCatalogPluginItem,
    contents: Vec<LolEventHubItemDetails>,
    background_image: String,
    full_price: u32,
    discount: String,
    display_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NpeRewardPackMetadata {
    major_reward: NpeReward,
    minor_rewards: Vec<NpeReward>,
    reward_key: String,
    index: i32,
    premium_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogItem {
    item_id: i32,
    item_instance_id: String,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathMission {
    status: String,
    start_time: i64,
    icon_image_url: String,
    helper_text: String,
    series_name: String,
    title: String,
    mission_type: String,
    completion_expression: String,
    id: String,
    completed_date: i64,
    requirements: Vec<String>,
    cooldown_time_millis: i64,
    rewards: Vec<LolNpeTutorialPathReward>,
    description: String,
    objectives: Vec<LolNpeTutorialPathObjective>,
    expiring_warnings: Vec<LolNpeTutorialPathExpiringWarning>,
    internal_name: String,
    locale: String,
    background_image_url: String,
    reward_strategy: LolNpeTutorialPathRewardStrategy,
    display: LolNpeTutorialPathMissionDisplay,
    metadata: LolNpeTutorialPathMissionMetadata,
    last_updated_timestamp: i64,
    end_time: i64,
    is_new: bool,
    celebration_type: String,
    display_type: String,
    client_notify_level: String,
    viewed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubProgressionPurchaseUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubProgressionPurchaseUiData {
    offer_id: String,
    price_per_level: i64,
    rp_balance: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsQueue {
    is_team_builder_managed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyPlayerCollectionDto {
    players: Vec<LolLobbyPlayerDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "BasePlayerDTO")]
#[serde(rename_all = "kebab-case")]
pub struct BasePlayerDto {
    data: MatchedPlayerDto,
    code: i32,
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentWinnerInfo {
    short_name: String,
    player_ids: Vec<u64>,
    logo_color: i32,
    logo: i32,
    roster_id: i64,
    tier: i32,
    name: String,
    create_time: i64,
    average_win_duration: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesOtherPlayerTrophyInventoryItem {
    item_id: i32,
    payload: LolTrophiesCapClashTrophyEntitlementPayload,
    inventory_type: String,
    purchase_date: String,
    uuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashGameflowPartiesRegistrationStatus {
    complete: bool,
    error_codes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventRewardGrantElement {
    localizations: Value,
    media: Value,
    fulfillment_source: String,
    quantity: i32,
    reward_status: LolTftEventRewardStatus,
    item_type: String,
    id: String,
    item_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopSimpleInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopSimpleInventoryResponseDto {
    data: LolYourshopSimpleInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyGlobalRewards {
    all_champions: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeTutorialPathCollectionsChampionSpell {
    description: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatcherChunkingPatcherEnvironment {
    #[serde(rename = "game_patcher_available")]
    game_patcher_available: bool,
    #[serde(rename = "game_patcher_enabled")]
    game_patcher_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsFrontEndRequest {
    is_prepaid: bool,
    summoner_level: i16,
    giftee_message: String,
    rso_token: String,
    use_pmc_sessions: bool,
    opened_from: String,
    giftee_account_id: String,
    game: String,
    locale_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateAvailabilityResponseDtoV2 {
    available_for_watching: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "ChampSelectLcdsPotentialTradersDTO")]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPotentialTradersDto {
    potential_traders: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsChampionMastery {
    formatted_champion_points: String,
    champion_id: i32,
    champion_points_since_last_level: i32,
    tokens_earned: i32,
    formatted_mastery_goal: String,
    highest_grade: String,
    puuid: String,
    champion_level: i32,
    champion_points: i32,
    last_play_time: u64,
    champion_points_until_next_level: i32,
    chest_granted: bool,
    player_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolItemSetsNamecheckAuthorization {
    subject: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RankedScoutingTopChampionDTO")]
#[serde(rename_all = "camelCase")]
pub struct RankedScoutingTopChampionDto {
    rank: i32,
    champion_id: i32,
    win_count: i32,
    game_count: i32,
    kda: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolClashTournamentPhaseProgressNotificationDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentPhaseProgressNotificationDto {
    phase_id: i64,
    capacity_status: CapacityEnum,
    tournament_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathAccountSettingsTutorial {
    has_seen_tutorial_path: bool,
    has_skipped_tutorial_path: bool,
    should_see_new_player_experience: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetOrderNotificationResource {
    event_type_id: String,
    event_type: String,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoWhereAmIRequest {
    ip_address: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsStoreFulfillmentNotification {
    ip: i64,
    inventory_type: String,
    rp: i64,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCatalogChampionSkinEmblemPosition {
    horizontal: String,
    vertical: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsDropTablePityInfo {
    pity_limit: u8,
    chase_content_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesGameDataTFTContent")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesGameDataTftContent {
    name: String,
    name_tra_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PaymentsFrontEndResult {
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseWidgetConfig {
    always_show_purchase_disclaimer: bool,
    enabled: bool,
    non_refundable_disclaimer_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPlayerInventory {
    can_add_custom_page: bool,
    is_custom_page_creation_unlocked: bool,
    owned_page_count: u32,
    custom_page_count: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatAuthResourceRsoAccessToken {
    token: String,
    expiry: u64,
    scopes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersClashV2FrameRewardSpec {
    season_id: String,
    level: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRankedScoutingTopChampion {
    win_rate: i32,
    rank: i32,
    champion_id: i32,
    win_count: i32,
    kda: String,
    kda_classification: LolClashKdaClassification,
    game_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSkinLineDescriptionInfo {
    title: String,
    description: String,
    icon_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubUnclaimedRewardsUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubUnclaimedRewardsUiData {
    rewards_count: i32,
    locked_tokens_count: i32,
    time_of_last_unclaimed_reward: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeRewardsAllRewards {
    level: LolNpeRewardsRewardSeries,
    login: LolNpeRewardsRewardSeries,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FailedInvite {
    exception: ClientRequestError,
    player_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LCDSLoyaltyRewards")]
#[serde(rename_all = "camelCase")]
pub struct LcdsLoyaltyRewards {
    xp_boost: i32,
    global: LcdsGlobalRewards,
    ip_boost: i32,
    champions: Vec<LcdsChampionReward>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatBlocked {
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "game_name")]
    game_name: String,
    #[serde(rename = "game_tag")]
    game_tag: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoGeoInfoResponse {
    is_initialized: bool,
    error_message: String,
    success: bool,
    geo_info: LolGeoinfoGeoInfo,
    is_latest: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationResponseItem {
    quantity: i32,
    description: String,
    name: String,
    sale: LolPurchaseWidgetSale,
    validation_currency_info: Vec<LolPurchaseWidgetItemPrice>,
    item_key: LolPurchaseWidgetItemKey,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventRewardGrantInfo {
    viewed: bool,
    status: LolTftEventGrantStatus,
    grant_elements: Vec<LolTftEventRewardGrantElement>,
    id: String,
    grantee_id: String,
    selected_ids: Vec<String>,
    reward_group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesPlatformConfigEnabledMap {
    min_players: i32,
    game_map_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampionsCollectionsChampionSkinEmblemPath {
    small: String,
    large: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatConversationUpdate {
    hidden: bool,
    muted: bool,
    cid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatPlayerMuteStatus {
    is_player_muted: bool,
    is_system_muted: bool,
    puuid: String,
    is_settings_muted: bool,
    obfuscated_puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedLeagueLadderDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderDto {
    top_number_of_players: i32,
    entries: Vec<LolRankedLeagueLadderEntryDto>,
    queue_type: String,
    next_apex_update: i64,
    tier: String,
    provisional_game_threshold: i32,
    max_league_size: i32,
    min_lp_for_tier: i32,
    apex_unlock_time_millis: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsSelectionRequestDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSelectionRequestDto {
    grant_id: String,
    selections: Vec<String>,
    reward_group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryRecentlyPlayedSummoner {
    champion_id: u64,
    summoner_id: u64,
    game_name: String,
    tag_line: String,
    puuid: String,
    summoner_name: String,
    game_creation_date: String,
    game_id: u64,
    team_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsEndOfGameStats {
    time_until_next_first_win_bonus: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubItemOrderDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemOrderDto {
    inventory_type: String,
    quantity: u32,
    item_id: i32,
    rp_cost: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMatchHistoryMatchHistoryPosition {
    x: i16,
    y: i16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEventAck {
    seen_this_event: bool,
    new_summoner_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopCatalogItem {
    inventory_type: String,
    item_instance_id: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootLcdsLootDescriptionDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsLootDescriptionDto {
    loot_name: String,
    localization_map: Value,
    localization_long_description_map: Value,
    child_loot_table_names: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameInviteBaseRuntimeException {
    root_cause_classname: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestones {
    active: bool,
    loot_items: Vec<String>,
    error_caching_milestone_set: bool,
    id: String,
    milestones: Vec<LolLootLootMilestone>,
    start_date: String,
    end_date: String,
    store_group_title: String,
    recipes: Vec<String>,
    repeat: LolLootLootMilestoneRepeat,
    progression_config_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLoot {
    _type: String,
    is_rental: bool,
    localized_description: String,
    value: i32,
    display_categories: String,
    tags: String,
    rental_games: i32,
    localized_recipe_title: String,
    splash_path: String,
    rental_seconds: i64,
    store_item_id: i32,
    item_desc: String,
    tile_path: String,
    upgrade_loot_name: String,
    ref_id: String,
    upgrade_essence_name: String,
    parent_item_status: LolLootItemOwnershipStatus,
    shadow_path: String,
    redeemable_status: LolLootRedeemableStatus,
    is_new: bool,
    parent_store_item_id: i32,
    localized_recipe_subtitle: String,
    upgrade_essence_value: i32,
    item_status: LolLootItemOwnershipStatus,
    disenchant_value: i32,
    loot_name: String,
    asset: String,
    count: i32,
    loot_id: String,
    disenchant_loot_name: String,
    disenchant_recipe_name: String,
    expiry_time: i64,
    localized_name: String,
    rarity: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSvcRewardGroup {
    child_reward_group_ids: Vec<String>,
    id: String,
    selection_strategy_config: LolEventHubSelectionStrategyConfig,
    celebration_type: LolEventHubCelebrationType,
    types: Vec<String>,
    active: bool,
    rewards: Vec<LolEventHubReward>,
    reward_strategy: LolEventHubRewardStrategy,
    media: Value,
    localizations: Value,
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolSummonerAliasDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAliasDto {
    playstation_nameset: LolSummonerConsoleNameset,
    switch_nameset: LolSummonerConsoleNameset,
    provider_id: String,
    xbox_nameset: LolSummonerConsoleNameset,
    error: String,
    puuid: String,
    gnt: LolSummonerAlias,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginPlatformGeneratedCredentials {
    password: String,
    username: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatcherNotification {
    id: String,
    data: Value,
    notification_id: PatcherNotificationId,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootTFTDamageSkinGroupViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolLootTftDamageSkinGroupViewModel {
    group_name: String,
    group_id: u32,
    items: Vec<LolLootCosmeticsTftDamageSkinViewModel>,
    num_owned: u32,
    num_available: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRegistrationStatus {
    error_codes: Vec<String>,
    complete: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataQuestSkinInfo {
    product_type: LolChampionsQuestSkinProductType,
    collection_card_path: String,
    tile_path: String,
    collection_description: String,
    splash_path: String,
    uncentered_splash_path: String,
    description_info: Vec<LolChampionsGameDataQuestSkinDescriptionInfo>,
    name: String,
    tiers: Vec<LolChampionsGameDataChampionQuestSkin>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootResponseDTO-vector-SvcRewardGroup")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootResponseDtoVectorSvcRewardGroup {
    metadata: LolLootResponseMetadataDto,
    data: Vec<LolLootSvcRewardGroup>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectTradeContract {
    state: LolChampSelectLegacyChampSelectTradeState,
    cell_id: i64,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesOrderNotificationResource {
    event_type_id: String,
    event_type: String,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChampSelection {
    selected_skin_index: i32,
    summoner_internal_name: String,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2RetrieveProfileResponse {
    rewards_locked: bool,
    honor_level: i32,
    checkpoint: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PhaseRosterDTO")]
#[serde(rename_all = "PascalCase")]
pub struct PhaseRosterDto {
    #[serde(rename = "phaseId")]
    phase_id: i64,
    #[serde(rename = "bracketDTO")]
    bracket_dto: Bracket,
    #[serde(rename = "checkinTime")]
    checkin_time: i64,
    #[serde(rename = "bracketId")]
    bracket_id: i64,
    #[serde(rename = "period")]
    period: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyCustomEligibilityDto {
    eligible: bool,
    restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationRequest {
    owned_items: Vec<LolPurchaseWidgetItemOwnership>,
    items: Vec<LolPurchaseWidgetValidationRequestItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifySuccessRequest {
    availability_item_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubResponseDTO-map-RewardGroupId-SelectGrantStatus")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubResponseDtoMapRewardGroupIdSelectGrantStatus {
    metadata: LolEventHubResponseMetadataDto,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLobbyRankedStatsUnsignedDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRankedStatsUnsignedDto {
    jwt: String,
    queues: Vec<LolLobbyRankedPositionInfoDto>,
    highest_previous_season_end_tier: String,
    highest_previous_season_end_rank: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyStatusDto {
    ready_players: Vec<String>,
    left_players: Vec<String>,
    eog_players: Vec<String>,
    party_size: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaWithPreferences {
    crest_type: String,
    highest_ranked_entry: LolRegaliaRegaliaRankedEntry,
    last_season_highest_rank: String,
    profile_icon_id: i32,
    banner_type: String,
    preferred_crest_type: String,
    summoner_level: u32,
    selected_prestige_crest: u8,
    preferred_banner_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorSummoner {
    summoner_id: u64,
    display_name: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRemedyMail {
    mail_id: String,
    created_at: u64,
    message: String,
    state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsFavoriteCosmetics {
    #[serde(rename = "companions")]
    companions: Vec<String>,
    #[serde(rename = "tft_map_skins")]
    tft_map_skins: Vec<String>,
    #[serde(rename = "tft_damage_skins")]
    tft_damage_skins: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginConnectionState {
    is_partner_riot_client: bool,
    mode: LolLoginLoginConnectionMode,
    is_using_developer_auth_token: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderInfo {
    standings: Vec<LolRankedRatedLadderStanding>,
    queue_type: LolRankedLeagueQueueType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesEogNotificationEnvelope {
    self_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
    self_milestone_progress: Vec<LolStatstonesMilestoneProgressNotification>,
    self_statstone_progress: Vec<LolStatstonesStatstoneProgress>,
    others_personal_bests: Vec<LolStatstonesPersonalBestNotification>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLockedEventsStateV1 {
    locked_event_index: i32,
    allow_locked_events: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetDiscountPricingInfo {
    original_cost: i32,
    discount: f32,
    currency: String,
    cost_type: String,
    cost: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectChatRoomDetails {
    multi_user_chat_password: String,
    muc_jwt_dto: LolPerksMucJwtDto,
    multi_user_chat_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedHonorState {
    level: i32,
    checkpoint: i32,
    rewards_locked: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCapOffer {
    label: String,
    id: String,
    product_id: String,
    type_id: String,
    merchant_id: String,
    start_date: String,
    created_date: String,
    active: bool,
    payload: Vec<Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "GameflowLcdsGameDTO")]
#[serde(rename_all = "camelCase")]
pub struct GameflowLcdsGameDto {
    queue_type_name: String,
    game_type: String,
    team_two: Vec<Value>,
    game_queue_config_id: i32,
    player_champion_selections: Vec<Value>,
    game_mode: String,
    game_type_config_id: i32,
    game_state: String,
    map_id: i32,
    spectator_delay: i32,
    max_num_players: i32,
    id: u64,
    team_one: Vec<Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsPlayerParticipantStatsSummary {
    game_id: u64,
    selected_position: String,
    leaver: bool,
    team_id: i32,
    summoner_name: String,
    profile_icon_id: i32,
    wins: i32,
    spell_1_id: i32,
    user_id: u64,
    detected_team_position: String,
    statistics: Vec<EndOfGameLcdsRawStatDto>,
    puuid: String,
    skin_name: String,
    champion_id: i32,
    losses: i32,
    leaves: i32,
    level: i32,
    spell_2_id: i32,
    bot_player: bool,
    skin_index: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubInventoryNotification {
    id: i64,
    inventory_type: String,
    item_id: i32,
    acknowledged: bool,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendActivationPinRequest {
    phone_number: String,
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootCosmeticsTFTDamageSkin")]
#[serde(rename_all = "PascalCase")]
pub struct LolLootCosmeticsTftDamageSkin {
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "level")]
    level: u32,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "upgrades")]
    upgrades: Vec<String>,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestoneReward {
    item_id: i32,
    item_instance_id: String,
    reward_group_id: String,
    reward_type: String,
    inventory_type: String,
    loot_item: LolLootPlayerLoot,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathProgress {
    last_viewed_progress: i32,
    total_count: i32,
    current_progress: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItem {
    price: u32,
    item_id: String,
    quantity: u32,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesAccountIdAndSummonerId {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootTFTDamageSkinGroupedViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolLootTftDamageSkinGroupedViewModel {
    default_item_id: i32,
    groups: Vec<LolLootTftDamageSkinGroupViewModel>,
    selected_loadout_item: LolLootCosmeticsTftDamageSkinViewModel,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCollectionsCollectionsOwnership {
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "rental")]
    rental: LolCollectionsCollectionsRental,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCollectionsGameDataChampionSkinAugmentOverlays {
    #[serde(rename = "tileLCOverlayPath")]
    tile_lc_overlay_path: String,
    #[serde(rename = "centeredLCOverlayPath")]
    centered_lc_overlay_path: String,
    #[serde(rename = "uncenteredLCOverlayPath")]
    uncentered_lc_overlay_path: String,
    #[serde(rename = "socialCardLCOverlayPath")]
    social_card_lc_overlay_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGameViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameViewModel {
    queue_id: i32,
    game_length: u32,
    is_ranked: bool,
    game_id: u64,
    queue_type: String,
    players: Vec<LolEndOfGameTftEndOfGamePlayerViewModel>,
    local_player: LolEndOfGameTftEndOfGamePlayerViewModel,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootSelectionRequestDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLootSelectionRequestDto {
    selections: Vec<String>,
    grant_id: String,
    reward_group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsResponseDTO-vector-SvcRewardGrant")]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsResponseDtoVectorSvcRewardGrant {
    metadata: LolRewardsResponseMetadataDto,
    data: Vec<LolRewardsSvcRewardGrant>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatBuddy {
    game_name: String,
    puuid: String,
    summoner_id: u64,
    tag_line: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequestResource {
    name: String,
    direction: LolChatFriendRequestDirection,
    id: String,
    icon: i32,
    tag_line: String,
    game_name: String,
    pid: String,
    summoner_id: u64,
    note: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMissionsCollectionsWardSkin {
    id: i64,
    ownership: LolMissionsCollectionsOwnership,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGrantElement {
    element_id: String,
    localizations: Value,
    status: LolRewardsRewardStatus,
    item_id: String,
    media: Value,
    quantity: i32,
    item_type: String,
    fulfillment_source: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardFriendResource {
    availability: String,
    name: String,
    game_name: String,
    game_tag: String,
    icon: i32,
    summoner_id: u64,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "ChampSelectLcdsPlayerChampionSelectionDTO")]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPlayerChampionSelectionDto {
    spell_2_id: i32,
    summoner_id: u64,
    puuid: String,
    champion_id: i32,
    selected_skin_index: i32,
    spell_1_id: i32,
    summoner_internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRepeat {
    repeat_triggers: Vec<LolEventHubRepeatGroupTrigger>,
    multiplier: f32,
    count: i32,
    milestones: Vec<LolEventHubMilestone>,
    scope: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSkinLineItemDto {
    large_image_path: String,
    localized_long_name: String,
    large_video_path: String,
    thumbnail_image_path: String,
    localized_short_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsResponseDTO-vector-SvcRewardGroup")]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsResponseDtoVectorSvcRewardGroup {
    metadata: LolRewardsResponseMetadataDto,
    data: Vec<LolRewardsSvcRewardGroup>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyBotParticipantDto {
    champion_id: i32,
    bot_skill_level: i32,
    team: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreNotification {
    _type: String,
    created: String,
    dismissible: bool,
    title_key: String,
    detail_key: String,
    data: Value,
    background_url: String,
    icon_url: String,
    state: String,
    expires: String,
    source: String,
    critical: bool,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreWallet {
    ip: i64,
    rp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchInstallPaths {
    game_install_root: String,
    game_executable_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsCompanionViewModel {
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "color")]
    color: String,
    #[serde(rename = "isRecentItem")]
    is_recent_item: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "upgrades")]
    upgrades: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
    #[serde(rename = "favorited")]
    favorited: bool,
    #[serde(rename = "offerData")]
    offer_data: LolCosmeticsCapOffer,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "species")]
    species: String,
    #[serde(rename = "starShardsPrice")]
    star_shards_price: LolCosmeticsCosmeticsOfferPrice,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "level")]
    level: u32,
    #[serde(rename = "f2p")]
    f_2_p: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesCapOrdersRequestDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTrovesCapOrdersRequestDto {
    data: Value,
    meta: LolTftTrovesCapOrdersRequestMetaDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginResourceEvent {
    uri: String,
    event_type: PluginResourceEventType,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyJoinPartyAnalytics {
    num_friends_online: i32,
    platform_id: String,
    party_id: String,
    event_data: Value,
    event_timestamp: u64,
    num_open_friends: i32,
    num_other_invites: i32,
    num_total_invites: i32,
    party_size: i32,
    num_party_invites: i32,
    event_type: String,
    game_mode: String,
    num_open_parties: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameClientEndOfGameStats {
    queue_type: String,
    is_ranked: bool,
    queue_id: i32,
    stats_block: Value,
    game_id: u64,
    game_mode: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionSkin {
    emblems: Vec<LolChampionsCollectionsChampionSkinEmblem>,
    splash_path: String,
    uncentered_splash_path: String,
    rarity_gem_path: String,
    splash_video_path: String,
    collection_splash_video_path: String,
    id: i32,
    name: String,
    quest_skin_info: LolChampionsGameDataQuestSkinInfo,
    load_screen_path: String,
    chromas: Vec<LolChampionsGameDataChampionChroma>,
    skin_type: String,
    features_text: String,
    skin_augments: LolChampionsCollectionsChampionSkinAugments,
    chroma_path: String,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubValidationRequest {
    items: Vec<LolEventHubValidationRequestItem>,
    owned_items: Vec<LolEventHubItemOwnership>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigBuildInfo {
    game_branch: String,
    code_build_id: i32,
    branch: String,
    branch_full: String,
    game_branch_full: String,
    patchline: String,
    version: String,
    game_data_build_id: i32,
    content_build_id: i32,
    patchline_visible_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubValidateOfferResponseV3 {
    validation_errors: Vec<LolEventHubValidateOfferError>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSeriesOpt {
    series_id: String,
    option: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionEntityInstance {
    counters: Vec<LolProgressionCounterInstance>,
    group_id: String,
    milestones: Vec<LolProgressionMilestoneInstance>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PatcherComponentState {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "isCorrupted")]
    is_corrupted: bool,
    #[serde(rename = "isUpdateAvailable")]
    is_update_available: bool,
    #[serde(rename = "isUpToDate")]
    is_up_to_date: bool,
    #[serde(rename = "timeOfLastUpToDateCheckISO8601")]
    time_of_last_up_to_date_check_iso_8601: String,
    #[serde(rename = "progress")]
    progress: PatcherComponentActionProgress,
    #[serde(rename = "action")]
    action: PatcherComponentStateAction,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatSanitizeResponse {
    texts: Vec<String>,
    modified: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectTradeContract {
    state: LolLobbyTeamBuilderChampSelectTradeState,
    cell_id: i64,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesLoginSession {
    state: LolGameQueuesLoginSessionStates,
    summoner_id: u64,
    account_id: u64,
    connected: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbySummoner {
    display_name: String,
    summoner_id: u64,
    internal_name: String,
    account_id: u64,
    puuid: String,
    game_name: String,
    summoner_level: u32,
    tag_line: String,
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolMapsMaps {
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "gameModeName")]
    game_mode_name: String,
    #[serde(rename = "mapStringId")]
    map_string_id: String,
    #[serde(rename = "locStrings")]
    loc_strings: Value,
    #[serde(rename = "gameMode")]
    game_mode: String,
    #[serde(rename = "gameModeDescription")]
    game_mode_description: String,
    #[serde(rename = "tutorialCards")]
    tutorial_cards: Vec<LolMapsTutorialCard>,
    #[serde(rename = "assets")]
    assets: Value,
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "isDefault")]
    is_default: bool,
    #[serde(rename = "gameModeShortName")]
    game_mode_short_name: String,
    #[serde(rename = "platformId")]
    platform_id: String,
    #[serde(rename = "properties")]
    properties: Value,
    #[serde(rename = "categorizedContentBundles")]
    categorized_content_bundles: Value,
    #[serde(rename = "platformName")]
    platform_name: String,
    #[serde(rename = "isRGM")]
    is_rgm: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "perPositionRequiredSummonerSpells")]
    per_position_required_summoner_spells: Value,
    #[serde(rename = "perPositionDisallowedSummonerSpells")]
    per_position_disallowed_summoner_spells: Value,
    #[serde(rename = "gameMutator")]
    game_mutator: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YourshopStoreFulfillmentNotification {
    inventory_type: String,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubNarrativeVideo {
    localized_narrative_video_url: String,
    localized_play_narrative_button_label: String,
    narrative_video_is_locked_on_level: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubExternalCatalogPluginItemWithDetails {
    minimum_bundle_prices: Vec<LolEventHubExternalCatalogPluginPrice>,
    bundled_discount_prices: Vec<LolEventHubExternalCatalogPluginPrice>,
    bundled_items: Vec<LolEventHubExternalCatalogPluginItemWithDetails>,
    metadata: Vec<LolEventHubExternalItemMetadataEntry>,
    quantity: u32,
    item: LolEventHubExternalCatalogPluginItem,
    required_items: Vec<LolEventHubExternalCatalogPluginItemWithDetails>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolServiceStatusRiotStatusIncident {
    #[serde(rename = "platforms")]
    platforms: Vec<String>,
    #[serde(rename = "updates")]
    updates: Vec<LolServiceStatusRiotStatusUpdate>,
    #[serde(rename = "archive_at")]
    archive_at: String,
    #[serde(rename = "updated_at")]
    updated_at: String,
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "titles")]
    titles: Vec<LolServiceStatusRiotStatusTitle>,
    #[serde(rename = "incident_severity")]
    incident_severity: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollectionsLcdsStoreFulfillmentNotification {
    inventory_type: String,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePremadeVoiceParticipantDto {
    participant_id: String,
    summoner_id: u64,
    display_name: String,
    energy: u32,
    is_speaking: bool,
    puuid: String,
    is_muted: bool,
    volume: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreAllSummonerData {
    summoner: LolStoreSummoner,
    summoner_level_and_points: LolStoreSummonerLevelAndPoints,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeMenuConfig {
    always_show_loot_ids: Vec<String>,
    enabled: bool,
    loot_items_using_breakout_recipe_menu: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectLegacyGameflowGameClient {
    visible: bool,
    running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTeamSettings {
    reminders_enabled: bool,
    has_viewed_team_planner: bool,
    teams: Vec<LolTftTeamPlannerTeamPlan>,
    registered_team_index: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMapsGameModeSpellList {
    spells: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthImplicitAuthorization {
    access_token: LolRsoAuthAccessToken,
    id_token: LolRsoAuthIdToken,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsGameflowSession {
    game_data: LolLoadoutsGameflowGameData,
    phase: LolLoadoutsGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesTournamentTrophyInventoryItem {
    payload: LolTrophiesCapClashTrophyEntitlementPayload,
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectPinDropSummoner {
    lane_position: u64,
    is_placeholder: bool,
    position: String,
    lane: String,
    slot_id: u64,
    is_local_summoner: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsToken {
    access_token: String,
    issuer: String,
    token: String,
    subject: String,
    entitlements: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyLoginSession {
    connected: bool,
    state: LolChampSelectLegacyLoginSessionStates,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosSettingsData {
    notification_shown: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedParticipantTiers {
    achieved_tiers: Vec<LolRankedAchievedTier>,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoyaltyInventoryDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolLoyaltyInventoryDto {
    items: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerSettingsStorageContainer {
    data: LolTftTeamPlannerTeamSettings,
    schema_version: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLobbyMember {
    id: u64,
    is_owner: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendActivationPinResponse {
    data: LolAccountVerificationSendActivationPinResponseData,
    client_message_id: String,
    error: LolAccountVerificationResponseError,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOffer {
    original_price: i64,
    spot_index: u32,
    inventory_type: String,
    name: String,
    discount_price: i64,
    offer_id: String,
    owned: bool,
    item_id: i32,
    revealed: bool,
    expiration_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGameflowSession {
    game_data: LolChatGameflowGameData,
    map: LolChatGameflowGameMap,
    phase: LolChatGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftOrb {
    mission_id: String,
    unlock_time: i64,
    reward_level: u8,
    rewards: Vec<PlayerMissionRewardDto>,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingLoginSession {
    summoner_id: u64,
    connected: bool,
    state: LolMatchmakingLoginSessionState,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthSessionResponse {
    error: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectBenchChampion {
    champion_id: i32,
    is_priority: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesClaimResponse {
    claimed_milestones: Vec<String>,
    status: LolLootLootMilestoneClaimStatus,
    loot_milestone_set_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesUITitle")]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiTitle {
    item_id: i32,
    challenge_title_data: LolChallengesChallengeTitleData,
    icon_path: String,
    title_acquisition_type: String,
    purchase_date: String,
    content_id: String,
    name: String,
    background_image_path: String,
    is_permanent_title: bool,
    title_acquisition_name: String,
    title_requirement_description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterRankedRestrictionInfo {
    punished_games_remaining: i32,
    needs_ack: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHighlightsHighlightsSettingsResource {
    data: LolHighlightsHighlightsSettingsData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTEndOfGameStats")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftEndOfGameStats {
    boost_ip_earned: i32,
    battle_boost_ip_earned: i32,
    experience_total: i32,
    loyalty_boost_xp_earned: i32,
    first_win_bonus: i32,
    rp_earned: i32,
    leveled_up: bool,
    my_team_status: String,
    game_mode: String,
    previous_xp_total: u64,
    game_id: u64,
    is_aram_game: bool,
    invalid: bool,
    game_type: String,
    new_spells: Vec<i32>,
    previous_level: u64,
    ip_earned: i32,
    time_until_next_first_win_bonus: i32,
    team_early_surrendered: bool,
    ip_total: i32,
    teams: Vec<LolPftPftEndOfGameTeam>,
    caused_early_surrender: bool,
    game_length: i32,
    queue_type: String,
    game_ended_in_early_surrender: bool,
    local_player: LolPftPftEndOfGamePlayer,
    reroll_data: LolPftPftEndOfGamePoints,
    base_points: i32,
    experience_earned: i32,
    game_mutators: Vec<String>,
    ranked: bool,
    report_game_id: u64,
    early_surrender_accomplice: bool,
    account_id: u64,
    difficulty: String,
    boost_xp_earned: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StoreLcdsStoreAccountBalanceNotification {
    ip: i64,
    rp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectatorDynamicConfiguration {
    is_spectator_delay_configurable: bool,
    is_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovePurchaseResponse {
    error_code: u32,
    status: String,
    data: Value,
    order_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPlayerSettingsData {
    last_time_seen: String,
    last_seen_token_balance: i64,
    last_seen_token_shop_offers_version: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventCollectionsRental {
    rented: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsValidateItemSetNameResponse {
    success: bool,
    name_check_response: LolItemSetsNamecheckResponse,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameConfiguration {
    tournament_passback_url: String,
    spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    spectator_delay_enabled: bool,
    team_size: i32,
    game_mode: String,
    tournament_passback_data_packet: String,
    game_server_region: String,
    max_player_count: u32,
    game_type_config: LolLobbyQueueGameTypeConfig,
    map_id: i32,
    mutators: LolLobbyQueueGameTypeConfig,
    tournament_game_mode: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootLootMilestone {
    rewards: Vec<LolLootLootMilestoneReward>,
    id: String,
    threshold: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftOrb {
    reward_level: u8,
    rewards: Vec<PlayerMissionRewardDto>,
    status: String,
    unlock_time: i64,
    mission_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathExpiringWarning {
    message: String,
    alert_time: i64,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerSummonerProfileUpdate {
    value: Value,
    inventory: String,
    key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PatcherProductResource {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerLootResultDTO")]
#[serde(rename_all = "kebab-case")]
pub struct PlayerLootResultDto {
    added: Vec<PlayerLootDto>,
    status: String,
    redeemed: Vec<String>,
    failed: Vec<String>,
    removed: Vec<PlayerLootDto>,
    details: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQuickPlayPresetSlotDto {
    spell_1: u64,
    spell_2: u64,
    skin_id: i32,
    perks: String,
    champion_id: i32,
    position_preference: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSvcRewardGrant {
    grantee_id: String,
    reward_group_id: String,
    viewed: bool,
    message_parameters: Value,
    date_created: String,
    selected_ids: Vec<String>,
    grant_elements: Vec<LolLootSvcRewardGrantElement>,
    status: LolLootGrantStatus,
    id: String,
    grantor_description: LolLootGrantorDescription,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusBroadcastMessage {
    severity: String,
    content: String,
    message_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassRMSRewardsNotificationPayload")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassRmsRewardsNotificationPayload {
    recipient_id: String,
    reward_id: String,
    transaction_id: String,
    reward_instance_id: String,
    status: String,
    purchaser_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameNotification {
    message_code: String,
    message_argument: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PatcherUxResource {
    visible: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRiotclientUpgraderGameflowAvailability {
    is_available: bool,
    state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Punishment {
    punished_for_chat_logs: Vec<String>,
    punished_until_date_millis: u64,
    punishment_length_games: i64,
    punishment_reason: String,
    perma_ban: bool,
    punishment_length_millis: u64,
    punished_for_game_ids: Vec<u64>,
    punishment_type: String,
    player_facing_message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGamePlayerViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGamePlayerViewModel {
    icon_id: i32,
    board_pieces: Vec<LolEndOfGameTftEndOfGamePieceViewModel>,
    partner_group_id: u8,
    rank: u8,
    playbook: LolEndOfGameTftEndOfGamePlaybookViewModel,
    is_local_player: bool,
    is_interactable: bool,
    puuid: String,
    health: u8,
    companion: LolEndOfGameTftEndOfGameCompanionViewModel,
    riot_id_game_name: String,
    riot_id_tag_line: String,
    summoner_id: u64,
    ffa_standing: u8,
    summoner_name: String,
    custom_augment_container: LolEndOfGameTftEndOfGameCustomAugmentContainerViewModel,
    set_core_name: String,
    augments: Vec<LolEndOfGameTftEndOfGameItemViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendCountsResource {
    num_friends_away: u32,
    num_friends_online: u32,
    num_friends_available: u32,
    num_friends_in_queue: u32,
    num_friends_in_champ_select: u32,
    num_friends: u32,
    num_friends_in_game: u32,
    num_friends_mobile: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampionBenchStateV1 {
    bench_enabled: bool,
    champion_ids: Vec<i32>,
    bench_champions: Vec<LolLobbyTeamBuilderChampionBenchChampionV1>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsChampion {
    ownership: LolMissionsCollectionsOwnership,
    free_to_play: bool,
    skins: Vec<LolMissionsCollectionsChampionSkin>,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatFriendList {
    friends: Vec<LolChatFriend>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOption {
    price_details: Vec<LolPurchaseWidgetPriceDetail>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStoreTransactionResponseDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolStoreTransactionResponseDto {
    item_id: i32,
    inventory_type: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatcherProductState {
    is_update_available: bool,
    percent_patched: f64,
    is_stopped: bool,
    action: PatcherComponentStateAction,
    is_corrupted: bool,
    components: Vec<PatcherComponentState>,
    is_up_to_date: bool,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftEvent {
    url: String,
    series_id: String,
    enabled: bool,
    url_faq: String,
    title_translation_key: String,
    end_date: String,
    default_landing_page: bool,
    event_hub_template_type: String,
    queue_ids: Vec<i32>,
    start_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerDto {
    registration: LolLobbyRegistrationCredentials,
    tft_games_won: i64,
    created_at: u64,
    eligibility_hash: i64,
    platform_id: String,
    tft_games_played: i64,
    puuid: String,
    summoner_id: u64,
    server_utc_millis: i64,
    version: u64,
    account_id: u64,
    parties: Vec<LolLobbyPartyMemberDto>,
    current_party: LolLobbyPartyDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSettingsResource {
    schema_version: i64,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedRatedLadderEntryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRatedLadderEntryDto {
    rated_tier: String,
    wins: i32,
    rated_rating: i32,
    summoner_id: u64,
    puuid: String,
    previous_update_ladder_position: i32,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSkinLineTier {
    stage: i64,
    name: String,
    load_screen_path: String,
    collection_splash_video_path: String,
    description: String,
    splash_video_path: String,
    splash_path: String,
    short_name: String,
    id: i64,
    uncentered_splash_path: String,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PaymentsCurrencyDTO")]
#[serde(rename_all = "camelCase")]
pub struct PaymentsCurrencyDto {
    sub_currencies: Value,
    amount: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolModeProgressionLoadout {
    id: String,
    scope: String,
    loadout: Value,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassLoyaltyRewards {
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "xpBoost")]
    xp_boost: Value,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsResponseDTO-map-RewardGroupId-SelectGrantStatus")]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsResponseDtoMapRewardGroupIdSelectGrantStatus {
    metadata: LolRewardsResponseMetadataDto,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyChatFriendCounts {
    num_friends: u32,
    num_friends_in_game: u32,
    num_friends_in_champ_select: u32,
    num_friends_online: u32,
    num_friends_away: u32,
    num_friends_in_queue: u32,
    num_friends_mobile: u32,
    num_friends_available: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LobbyClientDynamicConfigurationNotification {
    delta: bool,
    configs: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryMini {
    puuid: String,
    champion_id: i32,
    champion_level: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksLobbyParticipantDto {
    player_slots: Vec<LolPerksQuickPlayPresetSlotDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassRmsEntitlementPayload {
    item_id: String,
    entitlement_type_id: String,
    item_type_id: String,
    resource_operation: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemPrice {
    currency_type: String,
    purchasable: bool,
    price: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaRankedEntry {
    split_reward_level: i32,
    division: LolRegaliaLeagueDivision,
    queue_type: LolRegaliaLeagueQueueType,
    tier: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsSettingCategory {
    data: Value,
    schema_version: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentityPlayer {
    current_platform_id: String,
    summoner_id: u64,
    tag_line: String,
    summoner_name: String,
    platform_id: String,
    current_account_id: u64,
    game_name: String,
    match_history_uri: String,
    account_id: u64,
    profile_icon: i32,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCatalogInstanceToItemKeyMap {
    platform_ids: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubRequestMetadataDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRequestMetadataDto {
    transaction_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestoneLootItemRewardGdsResource {
    internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorMail {
    state: String,
    created_at: u64,
    message: String,
    mail_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedGameflowSession {
    phase: LolRankedGameflowPhase,
    game_data: LolRankedGameflowGameData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPlayerPreferencesPlayerPreferences {
    version: String,
    data: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePushToTalkResource {
    ptt_key_binding: String,
    ptt_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTrigger {
    trigger_value: u64,
    _type: String,
    counter_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassEndOfGameXpNotification {
    xp: LolTftPassEndOfGameXp,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubTokenShopUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTokenShopUiData {
    token_name: String,
    token_image: String,
    offers_version: u32,
    token_uuid: String,
    token_bundles_catalog_entry: Vec<LolEventHubCatalogEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchComponentState {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "isUpdateAvailable")]
    is_update_available: bool,
    #[serde(rename = "isUpToDate")]
    is_up_to_date: bool,
    #[serde(rename = "isCorrupted")]
    is_corrupted: bool,
    #[serde(rename = "timeOfLastUpToDateCheckISO8601")]
    time_of_last_up_to_date_check_iso_8601: String,
    #[serde(rename = "action")]
    action: LolPatchComponentStateAction,
    #[serde(rename = "progress")]
    progress: LolPatchComponentActionProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorRecipient {
    puuid: String,
    game_id: u64,
    honors: Vec<LolHonorV2Honor>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolL10nRegionLocale")]
#[serde(rename_all = "camelCase")]
pub struct LolL10NRegionLocale {
    web_language: String,
    locale: String,
    region: String,
    web_region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengePlayerData {
    id: String,
    category_points: Value,
    preferences: LolChallengesChallengesPlayerPreferences,
    source: LolChallengesSource,
    puuid: String,
    level_points: Value,
    player_challenges: Vec<LolChallengesChallengeData>,
    apex_lader_update_time: u64,
    tags: Value,
    total_points: LolChallengesChallengePoints,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersLobbyStatus {
    member_summoner_ids: Vec<u64>,
    queue_id: i32,
    invited_summoner_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPaidBattlepassMilestone {
    is_keystone: bool,
    rewards: Vec<LolTftPassTftPaidBattlepassReward>,
    title: String,
    total_points_for_milestone: i32,
    is_locked: bool,
    internal_name: String,
    points_needed_for_milestone: i32,
    mission_id: String,
    status: String,
    description: String,
    is_claim_request_pending: bool,
    is_paid: bool,
    icon_image_url: String,
    icon_needs_frame: bool,
    is_bonus: bool,
    state: String,
    points_earned_for_milestone: i32,
    level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchStatus {
    connected_to_patch_server: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogInventoryContent {
    item_id: i64,
    ownership_type: LolCatalogInventoryOwnership,
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentGameEnd {
    bracket_id: i64,
    old_bracket: LolClashBracket,
    tournament_name_loc_key: String,
    tournament_name_loc_key_secondary: String,
    tournament_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingDodgeData {
    state: LolLobbyTeamBuilderMatchmakingDodgeState,
    dodger_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubOrderNotificationResource {
    event_type: String,
    event_type_id: String,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHoneyfruitHoneyfruitRegionLocale {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersCapClashFrameEntitlementPayload {
    reward_spec: LolBannersClashV2FrameRewardSpec,
    reward_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLobbyRankedPositionInfoDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyRankedPositionInfoDto {
    queue: String,
    tier: String,
    rank: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesChallengesRMSPayload")]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesChallengesRmsPayload {
    puuid: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameStats {
    queue_type: String,
    caused_early_surrender: bool,
    multi_user_chat_id: String,
    team_early_surrendered: bool,
    reroll_data: LolEndOfGameEndOfGamePoints,
    muc_jwt_dto: LolEndOfGameMucJwtDto,
    battle_boost_ip_earned: i32,
    ranked: bool,
    report_game_id: u64,
    ip_total: i32,
    game_mutators: Vec<String>,
    missions_xp_earned: i32,
    pre_level_up_next_level_xp: u64,
    first_win_bonus: i32,
    local_player: LolEndOfGameEndOfGamePlayer,
    game_length: i32,
    multi_user_chat_password: String,
    leveled_up: bool,
    rp_earned: i32,
    experience_earned: i32,
    game_type: String,
    boost_ip_earned: i32,
    ip_earned: i32,
    experience_total: i32,
    loyalty_boost_xp_earned: i32,
    previous_level: u64,
    my_team_status: String,
    team_boost: LolEndOfGameEndOfGameTeamBoost,
    invalid: bool,
    global_boost_xp_earned: i32,
    game_ended_in_early_surrender: bool,
    pre_level_up_experience_total: u64,
    xbgp_boost_xp_earned: i32,
    new_spells: Vec<i32>,
    previous_xp_total: u64,
    game_id: u64,
    early_surrender_accomplice: bool,
    next_level_xp: u64,
    boost_xp_earned: i32,
    difficulty: String,
    current_level: u64,
    time_until_next_first_win_bonus: i32,
    base_points: i32,
    game_mode: String,
    teams: Vec<LolEndOfGameEndOfGameTeam>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootPlayerLootUpdate {
    removed: Vec<LolLootPlayerLootDelta>,
    added: Vec<LolLootPlayerLootDelta>,
    redeemed: Vec<LolLootPlayerLootDelta>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLoginSession {
    account_id: u64,
    state: LolLootLoginSessionStates,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTMetadata")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftMetadata {
    app_version: String,
    homepage_timer: f64,
    stats: LolPftPftEndOfGameStats,
    account_id: u64,
    web_region: String,
    env: String,
    locale: String,
    app_name: String,
    system_os: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassMilestone {
    group_id: String,
    id: String,
    counter_id: String,
    name: String,
    trigger_value: i64,
    properties: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashChangeIconRequest {
    icon_color_id: i32,
    icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassTFTPassRewardDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPassRewardDto {
    description: String,
    premium: bool,
    title: String,
    asset_internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopClientCacheClearMessageDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopClientCacheClearMessageDto {
    clear_all: bool,
    inventory_types: Vec<String>,
    regions: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassInventoryItem {
    uuid: String,
    purchase_date: String,
    expiration_date: String,
    item_id: i32,
    ownership_type: LolTftPassItemOwnershipType,
    quantity: u64,
    inventory_type: String,
    wins: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsEsportsAPI_highlanderTournaments_roster")]
#[serde(rename_all = "kebab-case")]
pub struct LolEsportStreamNotificationsEsportsApiHighlanderTournamentsRoster {
    roster: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGameItemViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameItemViewModel {
    name_id: String,
    icon: String,
    name: String,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootOutputGdsResource {
    localized_description: String,
    loot_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginError {
    message_id: String,
    description: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BracketRoster {
    name: String,
    short_name: String,
    roster_id: i64,
    logo_color: i32,
    logo: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolModeProgressionInventoryRewardItem {
    uuid: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTrigger {
    _type: String,
    counter_id: String,
    trigger_value: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyLobbyTimer {
    countdown: i64,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSelectionStrategyConfig {
    min_selections_allowed: u32,
    max_selections_allowed: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPerksInventoryRunePageCount {
    quantity: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameData {
    queue: LolGameflowQueue,
    spectators_allowed: bool,
    game_name: String,
    player_champion_selections: Vec<Value>,
    team_one: Vec<Value>,
    is_custom_game: bool,
    game_id: u64,
    password: String,
    team_two: Vec<Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPremadeVoiceStateResource {
    connected: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionTrigger {
    _type: String,
    trigger_value: u64,
    counter_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyFailureRequest {
    failure_info: String,
    availability_item_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashOfferTicketRequest {
    ticket_amount: i32,
    ticket_type: TicketType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimelineFrame {
    events: Vec<LolMatchHistoryMatchHistoryEvent>,
    participant_frames: Value,
    timestamp: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetCapOrdersMetaDto {
    xid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolUserExperienceGameflowSession {
    phase: LolUserExperienceGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PendingOpenedTeamDTO")]
#[serde(rename_all = "camelCase")]
pub struct PendingOpenedTeamDto {
    name: String,
    invitation_id: String,
    logo: i32,
    logo_color: i32,
    short_name: String,
    tier: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolDropsCapDropsOddsTreeNodeDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsOddsTreeNodeDto {
    name_tra_key: String,
    node_id: String,
    odds: f32,
    quantity: u16,
    priority: u8,
    children: Vec<LolDropsCapDropsOddsTreeNodeDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubOfferUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubOfferUiData {
    max_quantity: u32,
    localized_title: String,
    highlighted: bool,
    offer_state: LolEventHubOfferStates,
    id: String,
    localized_description: String,
    image: String,
    price: u32,
    items: Vec<LolEventHubItemUiData>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTDamageSkinViewModel")]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsTftDamageSkinViewModel {
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "favorited")]
    favorited: bool,
    #[serde(rename = "isRecentItem")]
    is_recent_item: bool,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "level")]
    level: u32,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "upgrades")]
    upgrades: Vec<LolCosmeticsCosmeticsTftDamageSkinViewModel>,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "description")]
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashSeasonRewardResult {
    player_id: u64,
    honor_level: i32,
    eligible: bool,
    rewards: Vec<ClashRewardDefinition>,
    banned: bool,
    season_id: i32,
    season_vp: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchSupportedGameRelease {
    #[serde(rename = "artifact_id")]
    artifact_id: String,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "download")]
    download: LolPatchPatchSieveDownload,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorial {
    status: LolNpeTutorialPathTutorialStatus,
    step_number: i32,
    queue_id: String,
    title: String,
    description: String,
    background_url: String,
    use_chosen_champion: bool,
    is_viewed: bool,
    id: String,
    rewards: Vec<LolNpeTutorialPathTutorialReward>,
    use_quick_search_matchmaking: bool,
    _type: LolNpeTutorialPathTutorialType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesQueue {
    last_toggled_off_time: u64,
    last_toggled_on_time: u64,
    name: String,
    map_id: i32,
    queue_availability: LolFeaturedModesQueueAvailability,
    game_mode: String,
    category: LolFeaturedModesQueueGameCategory,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRiotMessagingServiceGameflowSession {
    phase: LolRiotMessagingServiceGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyDto {
    multi_user_chat_id: String,
    can_start_activity: bool,
    party_id: String,
    party_type: String,
    members: Vec<LolLobbyLobbyParticipantDto>,
    game_config: LolLobbyLobbyGameConfigDto,
    warnings: Vec<LolLobbyEligibilityRestriction>,
    invitations: Vec<LolLobbyLobbyInvitationDto>,
    muc_jwt_dto: LolLobbyMucJwtDto,
    scarce_positions: Vec<String>,
    restrictions: Vec<LolLobbyEligibilityRestriction>,
    local_member: LolLobbyLobbyParticipantDto,
    multi_user_chat_password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolServiceStatusRiotStatusUpdate {
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "publish")]
    publish: bool,
    #[serde(rename = "translations")]
    translations: Vec<LolServiceStatusRiotStatusTranslation>,
    #[serde(rename = "publish_locations")]
    publish_locations: Vec<String>,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "updated_at")]
    updated_at: String,
    #[serde(rename = "author")]
    author: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2AccountIdAndSummonerId {
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSessionResource {
    session_expire: u32,
    session_state: LolChatSessionState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLootLootDataGdsResource {
    loot_tables: Vec<LolLootLootTableGdsResource>,
    loot_items: Vec<LolLootLootItemGdsResource>,
    loot_recipes: Vec<LolLootLootRecipeGdsResource>,
    loot_bundles: Vec<LolLootLootBundleGdsResource>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreItemLocalization {
    language: String,
    name: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameRerollDataBagForClientV1 {
    points_gained_last_game: i32,
    total_points: i32,
    point_cost_of_reroll: i32,
    points_until_next_reroll: i32,
    reroll_count: i32,
    maximum_rerolls: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibility {
    queue_id: i32,
    restrictions: Vec<LolLobbyEligibilityRestriction>,
    eligible: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolLoadoutsInventoryResponseDto {
    data: LolLoadoutsInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkStyleResource {
    name: String,
    id: i32,
    slots: Vec<LolPerksPerkStyleSlotResource>,
    is_advanced: bool,
    tooltip: String,
    default_page_name: String,
    asset_map: Value,
    default_perks_when_splashed: Vec<i32>,
    default_stat_mods_per_sub_style: Vec<LolPerksDefaultStatModsPerSubStyle>,
    sub_style_bonus: Vec<LolPerksPerkSubStyleBonusResource>,
    default_sub_style: i32,
    default_perks: Vec<i32>,
    icon_path: String,
    allowed_sub_styles: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassRiotMessagingServiceMessage {
    resource: String,
    version: String,
    payload: String,
    service: String,
    timestamp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBaseSkinLineDto {
    collection_card_path: String,
    pricing_options: Vec<LolPurchaseWidgetPriceOptionDto>,
    localized_name: String,
    splash_path: String,
    tile_path: String,
    collection_description: String,
    items: Vec<LolPurchaseWidgetSkinLineItemDto>,
    skin_line_descriptions: Vec<LolPurchaseWidgetSkinLineDescriptionDto>,
    uncentered_splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PendingRosterMemberDTO")]
#[serde(rename_all = "camelCase")]
pub struct PendingRosterMemberDto {
    bet_type: TicketType,
    pending_pay: i32,
    player_id: u64,
    self_bet: i32,
    member_state: PendingRosterMemberState,
    bet: i32,
    position: Position,
    join_time: i64,
    pending_premium_pay: i32,
    tier: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyMucJwtDto {
    jwt: String,
    domain: String,
    target_region: String,
    channel_claim: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMatchmakingGameflowGameTypeConfig {
    reroll: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedSignedRankedStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSignedRankedStatsDto {
    jwt: String,
    earned_regalia_reward_ids: Vec<String>,
    highest_previous_season_end_tier: String,
    current_season_split_points: i32,
    seasons: Value,
    queues: Vec<LolRankedRankedQueueStatsDto>,
    splits_progress: Value,
    previous_season_split_points: i32,
    highest_previous_season_end_rank: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysClashPlaymodeRestrictedInfo {
    is_restricted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateAvailabilityResponseDtoV3 {
    available_for_watching: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersVictoriousComrade {
    summoner_id: u64,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPlayerPermissions {
    use_data: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolClashRosterPlayerNotification {
    #[serde(rename = "notifyReason")]
    notify_reason: LolClashRosterNotifyReason,
    #[serde(rename = "sourcePlayerId")]
    source_player_id: u64,
    #[serde(rename = "playerNotificationDTO")]
    player_notification_dto: PlayerDto,
    #[serde(rename = "roster")]
    roster: RosterDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerStatus {
    current_lobby_status: LolLobbyLobbyStatus,
    last_queued_lobby_status: LolLobbyLobbyStatus,
    can_invite_others_at_eog: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassGroup {
    counters: Vec<LolTftPassCounter>,
    id: String,
    product_id: String,
    repeat: LolTftPassRepeat,
    name: String,
    milestones: Vec<LolTftPassMilestone>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeriesState {
    all_rewards_claimed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsEsportsAPI_highlanderTournaments")]
#[serde(rename_all = "kebab-case")]
pub struct LolEsportStreamNotificationsEsportsApiHighlanderTournaments {
    brackets: Value,
    rosters: Value,
    description: String,
    id: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameflowGameClient {
    observer_server_ip: String,
    observer_server_port: u16,
    running: bool,
    visible: bool,
    server_port: u16,
    server_ip: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginPrice {
    cost: i64,
    currency: String,
    cost_type: String,
    sale: LolPurchaseWidgetCatalogPluginSale,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassSummonerIcon {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftPaidBattlepassInfo {
    media: Value,
    premium_entitlement_id: String,
    premium_title: String,
    start_date: u64,
    end_date: u64,
    pc_purchase_requirement: String,
    title: String,
    pass_id: String,
    premium: bool,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerNotificationsPlayerNotificationConfigResource {
    expiration_check_frequency: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TutorialMetadata {
    use_chosen_champion: bool,
    queue_id: String,
    use_quick_search_matchmaking: bool,
    step_number: i32,
    display_rewards: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHighlightsHighlightsDynamicConfig {
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoyaltyInventoryItemDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyInventoryItemDto {
    loyalty: bool,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysRoflFileMetadata {
    game_length: u32,
    game_version: String,
    last_game_chunk_id: u32,
    last_key_frame_id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUpdatePageOrderRequest {
    destination_page_id: i32,
    target_page_id: i32,
    offset: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthorization {
    current_account_id: u64,
    subject: String,
    current_platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersQueue {
    id: i32,
    min_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLobbyStatus {
    is_custom: bool,
    is_leader: bool,
    queue_id: i32,
    member_summoner_ids: Vec<u64>,
    custom_spectator_policy: LolChatQueueCustomGameSpectatorPolicy,
    is_practice_tool: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolRsoAuthRegionStatus {
    #[serde(rename = "enabled")]
    enabled: bool,
    #[serde(rename = "platformId")]
    platform_id: String,
    #[serde(rename = "isUserInfoEnabled")]
    is_user_info_enabled: bool,
    #[serde(rename = "isLQFallbackAllowed")]
    is_lq_fallback_allowed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolFeaturedModesMaps {
    #[serde(rename = "gameModeName")]
    game_mode_name: String,
    #[serde(rename = "isRGM")]
    is_rgm: bool,
    #[serde(rename = "assets")]
    assets: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLobbyInventoryCacheEntry {
    #[serde(rename = "signedInventoryJwt")]
    signed_inventory_jwt: String,
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashQueueReward {
    is_champion_points_enabled: bool,
    party_size_ip_rewards: Vec<i32>,
    is_ip_enabled: bool,
    is_xp_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolClashGameflowSession {
    phase: LolClashGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "ChampSelectLcdsGameDTO")]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsGameDto {
    name: String,
    game_state: String,
    room_password: String,
    status_of_participants: String,
    muc_jwt_dto: MucJwtDto,
    queue_type_name: String,
    room_name: String,
    team_two: Vec<Value>,
    banned_champions: Vec<BannedChampion>,
    optimistic_lock: i64,
    pick_turn: i32,
    player_champion_selections: Vec<ChampSelectLcdsPlayerChampionSelectionDto>,
    team_one: Vec<Value>,
    game_mutators: Vec<String>,
    id: u64,
    spectator_delay: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubClientCacheClearMessageDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubClientCacheClearMessageDto {
    regions: Vec<String>,
    clear_all: bool,
    inventory_types: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatContentCookies {
    #[serde(rename = "cookies")]
    cookies: Vec<LolChatcookie>,
    #[serde(rename = "content_path")]
    content_path: String,
    #[serde(rename = "content_id")]
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsReward {
    fulfillment_source: String,
    media: Value,
    localizations: Value,
    quantity: i32,
    item_id: String,
    id: String,
    item_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceExternalSession {
    patchline_full_name: String,
    patchline_id: String,
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardGiftingFriend {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPerksNamecheckResponse {
    errors: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateAvailabilityResponseDto {
    available_for_watching: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesChallengeLevelData {
    level: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyAmbassadorMessage {
    implementation_details: String,
    message: String,
    http_status: i32,
    payload: Value,
    error_code: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTeamPlan {
    champions: Vec<LolTftTeamPlannerChampion>,
    set_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolInventoryLoyaltyRewardsSimplified {
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "xpBoost")]
    xp_boost: i32,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedVictoriousSkin {
    item_instance_id: String,
    split_points_by_highest_season_end_tier: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersKudoedPlayer {
    kudoed_summoner_id: u64,
    kudoed_summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "MatchmakingLcdsGameDTO")]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsGameDto {
    team_two: Vec<MatchmakingLcdsPlayerParticipant>,
    game_state: String,
    team_one: Vec<MatchmakingLcdsPlayerParticipant>,
    terminated_condition: String,
    status_of_participants: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInvitee {
    invitee_state: LcdsInviteeState,
    summoner_id: u64,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolHoneyfruitHoneyfruitVNGPublisherSettings")]
#[serde(rename_all = "kebab-case")]
pub struct LolHoneyfruitHoneyfruitVngPublisherSettings {
    visible: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsGameDataTFTPlaybook")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataTftPlaybook {
    icon_path_small: String,
    mid_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    enabled: bool,
    icon_path: String,
    content_id: String,
    translated_name: String,
    translated_description: String,
    late_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    early_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    is_disabled_in_double_up: bool,
    loadouts_icon: String,
    item_id: i32,
    description: String,
    name: String,
    splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCatalogGameDataStatstoneSet {
    name: String,
    statstones: Vec<LolCatalogGameDataStatstone>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingQueue {
    id: i32,
    is_team_builder_managed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaSummoner {
    summoner_id: u64,
    summoner_level: u32,
    puuid: String,
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolRewardsRewardsConfig {
    grant_filtering: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassRepeatGroupTrigger {
    start_trigger_value: u16,
    increase_by: u16,
    counter_id: String,
    multiplier: f32,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsFailedJoinPlayer {
    summoner: MatchmakingLcdsSummoner,
    reason_failed: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRankedEosRewardsConfigEntry {
    rewards: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubInventoryDto {
    items: Value,
    items_jwt: String,
    expires: String,
    puuid: String,
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkSubStyleBonusResource {
    perk_id: i32,
    style_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerNamesetsResponse {
    namesets: Vec<LolSummonerAliasDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubWalletCacheEntry {
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
    #[serde(rename = "signedBalancesJwt")]
    signed_balances_jwt: String,
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopEndOfGameXpNotification {
    xp: LolYourshopEndOfGameXp,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCosmeticsCosmeticsSettings {
    favorites: LolCosmeticsFavoriteCosmetics,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogChampionSkinEmblem {
    emblem_position: LolCatalogChampionSkinEmblemPosition,
    name: String,
    emblem_path: LolCatalogChampionSkinEmblemPath,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadeRelationshipAnalytics {
    puuid: String,
    event_timestamp: u64,
    summoner_id: u64,
    premade_players: Vec<String>,
    friend_players: Vec<String>,
    event_type: String,
    event_data: Value,
    platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootGameflowSession {
    phase: LolLootGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegalia {
    banner_type: String,
    highest_ranked_entry: LolRegaliaRegaliaRankedEntry,
    selected_prestige_crest: u8,
    profile_icon_id: i32,
    last_season_highest_rank: String,
    crest_type: String,
    summoner_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassRMSRequest")]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassRmsRequest {
    version: String,
    service: String,
    resource: String,
    recipients: Vec<String>,
    payload: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesLocalMessageRequest {
    msg_type: String,
    msg_body: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventTftPaidBattlepassReward {
    description: String,
    reward_group: String,
    icon_needs_frame: bool,
    icon_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassSimpleInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassSimpleInventoryDto {
    expires: String,
    items: Value,
    items_jwt: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesDropCounterDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesDropCounterDto {
    count: u16,
    drop_table_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventLolTftNewsHub {
    enabled: bool,
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatFriendGroupList {
    groups: Vec<LolChatFriendGroup>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueDivisionInfo {
    min_lp_for_apex_tier: i32,
    top_number_of_players: i64,
    apex_unlock_time_millis: i64,
    standings: Vec<LolRankedLeagueStanding>,
    max_league_size: i32,
    division: LolRankedLeagueDivision,
    tier: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubHallOfLegendsClientConfig {
    narrative_elements: Vec<LolEventHubNarrativeElement>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawQueueShutdownStatus {
    is_disabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSimpleMessageResponse {
    msg_id: String,
    command: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTMapSkinGroupViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftMapSkinGroupViewModel {
    group_id: u32,
    num_available: u32,
    group_name: String,
    num_owned: u32,
    items: Vec<LolCosmeticsCosmeticsTftMapSkinViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetItemMetadataEntry {
    _type: String,
    value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesClashV2TrophyRewardSpec {
    season_id: String,
    pedestal: String,
    gem: String,
    theme: String,
    cup: String,
    tier: String,
    bracket: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopUIStatus")]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopUiStatus {
    start_time: String,
    name: String,
    hub_enabled: bool,
    end_time: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginResourceContract {
    full_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftPromoButton {
    enabled: bool,
    show_timer_while_event_active: bool,
    url: String,
    event_asset_id: String,
    event_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesUIChallengePenalty")]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesUiChallengePenalty {
    reason: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatMessageToPlayer {
    game_name: String,
    tag_line: String,
    body: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubBaseSkinLineDto {
    tile_path: String,
    collection_description: String,
    pricing_options: Vec<LolEventHubPriceOptionDto>,
    skin_line_descriptions: Vec<LolEventHubSkinLineDescriptionDto>,
    items: Vec<LolEventHubSkinLineItemDto>,
    localized_name: String,
    uncentered_splash_path: String,
    splash_path: String,
    collection_card_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigConfigParams {
    version: String,
    namespace: String,
    region: String,
    _type: ClientConfigConfigType,
    app_name: String,
    patchline: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersOfferContextDto {
    quantity: u32,
    payment_option: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubResponseDTO-SvcRewardGrant")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubResponseDtoSvcRewardGrant {
    metadata: LolEventHubResponseMetadataDto,
    data: LolEventHubSvcRewardGrant,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderQueueGameTypeConfig {
    allow_trades: bool,
    ban_mode: String,
    battle_boost: bool,
    do_not_remove: bool,
    advanced_learning_quests: bool,
    onboard_coop_beginner: bool,
    id: i64,
    allow_pick_order_swaps: bool,
    post_pick_timer_duration: i32,
    death_match: bool,
    ban_timer_duration: i32,
    exclusive_pick: bool,
    duplicate_pick: bool,
    team_champion_pool: bool,
    reroll: bool,
    cross_team_champion_pool: bool,
    learning_quests: bool,
    pick_mode: String,
    max_allowable_bans: i32,
    name: String,
    main_pick_timer_duration: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerLeagueDelta {
    reason: String,
    mini_series_progress: Vec<String>,
    league_point_delta: u64,
    timestamp: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingLoginSession {
    state: LolContentTargetingLoginSessionState,
    summoner_id: u64,
    puuid: String,
    id_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadePartyDto {
    party_id: String,
    comms_enabled: bool,
    players: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsLoginSession {
    state: LolCollectionsLoginSessionStates,
    puuid: String,
    summoner_id: u64,
    connected: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginSummonerSessionResource {
    summoner_id: u64,
    display_name: String,
    is_new_player: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftPromoButtons {
    promo_buttons: Vec<LolTftLolTftPromoButton>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersDataDto {
    id: String,
    purchaser: LolEventHubCapOrdersTypedIdentifierDto,
    location: String,
    sub_orders: Vec<LolEventHubCapOrdersSubOrderDto>,
    source: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryEvent {
    position: LolMatchHistoryMatchHistoryPosition,
    _type: String,
    skill_slot: u16,
    monster_type: String,
    victim_id: u16,
    assisting_participant_ids: Vec<u16>,
    lane_type: String,
    participant_id: u16,
    killer_id: u16,
    building_type: String,
    timestamp: u64,
    tower_type: String,
    team_id: u16,
    item_id: u16,
    monster_sub_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorQueue {
    spectator_enabled: bool,
    id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationHighlightSegmentData {
    lottie_json_path: String,
    reveal_sound_path: String,
    transition_wipe_sound_path: String,
    promise_token_description: String,
    promise_token_title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventoryWalletResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolInventoryWalletResponseDto {
    data: LolInventoryWalletDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaSettings {
    crest_type: LolRegaliaRegaliaCrestType,
    banner_type: LolRegaliaRegaliaBannerType,
    selected_prestige_crest: u8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsEndOfGameStats {
    other_team_info: EndOfGameLcdsTeamInfo,
    loyalty_boost_xp_earned: i32,
    game_id: u64,
    room_name: String,
    team_player_participant_stats: Vec<EndOfGameLcdsPlayerParticipantStatsSummary>,
    game_ended_in_early_surrender: bool,
    ranked: bool,
    room_password: String,
    other_team_player_participant_stats: Vec<EndOfGameLcdsPlayerParticipantStatsSummary>,
    experience_earned: i32,
    muc_jwt_dto: MucJwtDto,
    previous_level: u64,
    early_surrender_accomplice: bool,
    game_length: u32,
    ip_total: i32,
    previous_xp_total: u64,
    team_early_surrendered: bool,
    battle_boost_ip_earned: i32,
    user_id: u64,
    game_type: String,
    invalid: bool,
    report_game_id: u64,
    rp_earned: i32,
    my_team_info: EndOfGameLcdsTeamInfo,
    skin_id: i32,
    caused_early_surrender: bool,
    difficulty: String,
    boost_ip_earned: i32,
    new_spells: Vec<EndOfGameLcdsSpell>,
    time_until_next_first_win_bonus: i32,
    experience_total: i32,
    first_win_bonus: i32,
    game_mode: String,
    leveled_up: bool,
    skin_index: i32,
    ip_earned: i32,
    queue_type: String,
    my_team_status: String,
    base_points: i32,
    boost_xp_earned: i32,
    game_mutators: Vec<String>,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootDescription {
    loot_name: String,
    localized_description_long: String,
    child_loot_table_names: Vec<String>,
    children_descriptions: Vec<LolLootLootDescription>,
    localized_description: String,
    image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsActiveBoosts {
    first_win_of_the_day_start_time: String,
    xp_loyalty_boost: i32,
    xp_boost_end_date: String,
    xp_boost_per_win_count: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashLoginSession {
    state: LolClashLoginSessionState,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeRewardsMissionDisplay {
    locations: Vec<String>,
}
type LolHoneyfruitMatchHistoryGame = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitReward {
    id: String,
    split_id: i32,
    regalia_level: i32,
    reward_type: String,
    description: String,
    points_required: i32,
    champion_id: i32,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricMetadata {
    #[serde(rename = "destination")]
    destination: String,
    #[serde(rename = "priority")]
    priority: MetricPriority,
    #[serde(rename = "notify")]
    notify: MetricMetadataNotify,
    #[serde(rename = "pretty_name")]
    pretty_name: String,
    #[serde(rename = "category")]
    category: String,
    #[serde(rename = "info")]
    info: String,
    #[serde(rename = "transientAggregation")]
    transient_aggregation: AggregationType,
    #[serde(rename = "sample_window_ms")]
    sample_window_ms: u32,
    #[serde(rename = "data_type")]
    data_type: MetricDataType,
    #[serde(rename = "units")]
    units: String,
    #[serde(rename = "period")]
    period: u32,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "type")]
    _type: MetricType,
    #[serde(rename = "alerts")]
    alerts: Vec<MetricMetadataAlert>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendGroup {
    name: String,
    is_meta_group: bool,
    collapsed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolReplaysGameflowGameClient {
    running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClientConfigDepInjectorEntitlementsUpdate {
    entitlements_update_type: ClientConfigDepInjectorEntitlementsUpdateType,
    entitlements_token_resource: ClientConfigDepInjectorEntitlements,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubRiotMessagingServiceMessage {
    timestamp: i64,
    payload: String,
    service: String,
    resource: String,
    version: String,
}
type LolTftPassRmsWalletPayload = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrack {
    track_config: LolEventHubRewardTrackConfiguration,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSkinLineInfo {
    tile_path: String,
    tiers: Vec<LolEventHubSkinLineTier>,
    description_info: Vec<LolEventHubSkinLineDescriptionInfo>,
    name: String,
    uncentered_splash_path: String,
    splash_path: String,
    collection_description: String,
    collection_card_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2Queue {
    id: i32,
    map_id: i32,
    _type: String,
    game_mode: String,
    removal_from_game_allowed: bool,
    removal_from_game_delay_minutes: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGameflowCrashReportingSettings {
    enabled: bool,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatSummonerStatus {
    ready: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantIdentities {
    participant_id: u16,
    player: LolMatchHistoryMatchHistoryParticipantIdentityPlayer,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SLIIntDiagnostic")]
#[serde(rename_all = "kebab-case")]
pub struct SliIntDiagnostic {
    key: String,
    value: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProcessControlProcess {
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersInventoryItemWithPayload {
    inventory_type: String,
    uuid: String,
    item_id: i32,
    purchase_date: String,
    payload: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatQueue {
    game_type_config: LolChatQueueGameTypeConfig,
    id: i32,
    _type: String,
    game_mode: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBlockedPlayerResource {
    name: String,
    game_name: String,
    summoner_id: u64,
    game_tag: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyLobbyLastQueuedMember {
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberResponse {
    client_message_id: String,
    data: LolAccountVerificationPhoneNumberResponseData,
    error: LolAccountVerificationResponseError,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolReplaysReplaysSettingsResource {
    data: LolReplaysReplaysSettingsData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TracingEventV1 {
    tags: String,
    details: String,
    src: String,
    when: u64,
    name: String,
    dest: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootLcdsClientConfigurationDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsClientConfigurationDto {
    loot_materials_to_always_render: Vec<String>,
    loot_items_using_breakout_recipe_menu: Vec<String>,
    currencies_using_cap_wallets: Vec<String>,
    disabled_redemptions: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRsoAuthRSOPlayerCredentials")]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthRsoPlayerCredentials {
    username: String,
    password: String,
    platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPremadeVoiceLoginSession {
    connected: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitV1AuthenticationRedirectInput {
    #[serde(rename = "redirect_uri")]
    redirect_uri: String,
    #[serde(rename = "language")]
    language: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatError {
    ts: String,
    text: String,
    cid: String,
    code: String,
    pid: String,
    class: String,
    _type: String,
    subtype: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchPatcherRegionSettings {
    #[serde(rename = "patchline")]
    patchline: String,
    #[serde(rename = "game_patcher")]
    game_patcher: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGameflowReplaysSettingsResource {
    data: LolGameflowReplaysSettingsData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatSanitizerStatus {
    #[serde(rename = "ready")]
    ready: bool,
    #[serde(rename = "locale")]
    locale: String,
    #[serde(rename = "platformID")]
    platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootBundleGdsResource {
    description: String,
    id: String,
    description_long: String,
    image: String,
    contents: Vec<LolLootLootBundleContentGdsResource>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogItem {
    sub_inventory_type: String,
    bundled: LolCatalogBundled,
    metadata: Vec<LolCatalogItemMetadataEntry>,
    offer_id: String,
    item_id: i32,
    localizations: Value,
    release_date: String,
    prices: Vec<LolCatalogItemCost>,
    inactive_date: String,
    item_requirements: Vec<LolCatalogItemKey>,
    inventory_type: String,
    item_instance_id: String,
    active: bool,
    tags: Vec<String>,
    icon_url: String,
    sale: LolCatalogSale,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTrovesCounterNotificationResource {
    balances: Value,
    deltas: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersHonorInteraction {
    display_name: String,
    summoner_id: u64,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerCacheData {
    summoner_icon: i32,
    summoner: LolSummonerSummoner,
    privacy: LolSummonerProfilePrivacySetting,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMasteryChangeNotification {
    score: i32,
    tokens_earned: i32,
    token_earned_after_game: bool,
    member_grades: Vec<LolChampionMasteryChampionMasteryGrade>,
    champion_season_milestone_up: bool,
    champion_points_until_next_level_after_game: i32,
    champion_id: i32,
    win: bool,
    champion_level: i32,
    bonus_champion_points_gained: i32,
    player_grade: String,
    champion_points_gained: i32,
    mark_required_for_next_level: i32,
    champion_season_milestone: i32,
    season_milestone: LolChampionMasterySeasonMilestoneRequireAndRewards,
    champion_points_since_last_level_before_game: i32,
    puuid: String,
    milestone_grades: Vec<String>,
    level_up_list: Vec<LolChampionMasteryChampionMasteryMini>,
    champion_points_until_next_level_before_game: i32,
    map_id: i32,
    game_id: i64,
    champion_level_up: bool,
    champion_points_before_game: i32,
    champion_points_gained_individual_contribution: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLoginDataPacket {
    simple_messages: Vec<LolLootLoginSimpleMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolInventoryEndOfGameXpNotification {
    xp: LolInventoryEndOfGameXp,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentSummary {
    tournament_id: i64,
    bracket_id: i64,
    roster_id: String,
    state: LolClashTournamentState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGroup {
    types: Vec<String>,
    child_reward_group_ids: Vec<String>,
    id: String,
    active: bool,
    rewards: Vec<LolMissionsSvcReward>,
    selection_strategy_config: LolMissionsSelectionStrategyConfig,
    internal_name: String,
    media: Value,
    localizations: Value,
    reward_strategy: LolMissionsRewardStrategy,
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectAction {
    actor_cell_id: i64,
    id: i64,
    completed: bool,
    is_ally_action: bool,
    _type: String,
    champion_id: i32,
    is_in_progress: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampionsCollectionsOwnership {
    #[serde(rename = "rental")]
    rental: LolChampionsCollectionsRental,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueInfo {
    queue_id: i32,
    wait_time: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBracket {
    size: i32,
    is_complete: bool,
    id: i64,
    period: i32,
    tournament_id: i64,
    matches: Vec<BracketMatch>,
    version: i32,
    rosters: Vec<BracketRoster>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsStatstone {
    item_id: i64,
    ownership_type: LolLootInventoryOwnership,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventLoyaltyStatusNotification {
    status: LolTftEventLoyaltyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsRemovedFromLobbyNotification {
    removal_reason: LcdsRemovalReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetItemCost {
    discount: f32,
    currency: String,
    cost: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSMatchHistoryMetadata")]
#[serde(rename_all = "PascalCase")]
pub struct LolReplaysGamhsMatchHistoryMetadata {
    #[serde(rename = "participants")]
    participants: Vec<String>,
    #[serde(rename = "info_type")]
    info_type: String,
    #[serde(rename = "timestamp")]
    timestamp: u64,
    #[serde(rename = "match_id")]
    match_id: String,
    #[serde(rename = "private")]
    private: bool,
    #[serde(rename = "product")]
    product: String,
    #[serde(rename = "data_version")]
    data_version: u8,
    #[serde(rename = "tags")]
    tags: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesStatus {
    available_contents: u32,
    drop_table_id: String,
    pity_count: u8,
    is_collector_bounty_max_rolls_met: bool,
    has_pull_error: bool,
    owned: bool,
    total_rolls_count: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLeagueSessionLeagueSessionRMSNotification")]
#[serde(rename_all = "camelCase")]
pub struct LolLeagueSessionLeagueSessionRmsNotification {
    product: String,
    state: String,
    puuid: String,
    session_initiated_at: u64,
    session_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaChatPresence {
    summoner_id: u64,
    icon: i32,
    lol: LolRegaliaChatPresenceLolData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLcdsServiceProxyResponse {
    status: String,
    payload: String,
    compressed_payload: bool,
    message_id: String,
    method_name: String,
    service_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatConversationMessageResource {
    _type: String,
    from_id: String,
    body: String,
    from_summoner_id: u64,
    timestamp: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesDropsOddsListEntryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesDropsOddsListEntryDto {
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTrophiesLoadout {
    name: String,
    scope: String,
    loadout: Value,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RedeemLootTransactionDTO")]
#[serde(rename_all = "camelCase")]
pub struct RedeemLootTransactionDto {
    loot_name: String,
    client_id: String,
    transaction_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreCapOffer {
    label: String,
    created_date: String,
    id: String,
    product_id: String,
    type_id: String,
    active: bool,
    start_date: String,
    payload: Value,
    merchant_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceAudioPropertiesResource {
    mic_energy: u32,
    is_loopback_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsQueueRestricted {
    summoner: MatchmakingLcdsSummoner,
    message: String,
    queue_id: i32,
    reason_failed: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventUserInfo {
    user_info: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampSelectCollectionsOwnership {
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
    #[serde(rename = "rental")]
    rental: LolChampSelectCollectionsRental,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaAccountIdAndSummonerId {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootSimpleDialogMessageResponse {
    command: String,
    msg_id: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedSplitRewardGroupDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitRewardGroupDto {
    split_points: i32,
    rewards: Vec<LolRankedSplitRewardDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BannedChampion {
    champion_id: i32,
    team_id: i32,
    pick_turn: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPftGameflowGameDodge {
    state: LolPftGameflowGameDodgeState,
    dodge_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEndOfGameStatsBlock {
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CriticalFlowSummary {
    active_flow_events: Vec<CriticalFlowExpectation>,
    errors: Vec<String>,
    active_flows: Vec<String>,
    events: Vec<CriticalFlowCapture>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyTeamBuilderChampionSelectPreferences {
    skins: Value,
    spells: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingDodgeData {
    dodger_id: u64,
    state: LolMatchmakingMatchmakingDodgeState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventRewardGrant {
    info: LolTftEventRewardGrantInfo,
    reward_group: LolTftEventRewardGroup,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRegaliaSummonerProfile {
    regalia: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsObserveGameRequestDto {
    player_gco_tokens: LcdsPlayerGcoTokens,
    game_id: u64,
    password: String,
    game_version: String,
    champ_select_inventory_jwt: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsRequestDTO-vector-string")]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsRequestDtoVectorString {
    data: Vec<String>,
    metadata: LolRewardsRequestMetadataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsDropTableDisplayMetadata {
    mythic_offer_id: String,
    is_collectors_bounty: bool,
    tables: Value,
    odds_tree: LolDropsCapDropsOddsTreeNodeDto,
    version: u8,
    chase_content_id: String,
    priority: u8,
    name_tra_key: String,
    progression_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameGameflowAvailability {
    state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobby {
    was_kicked: bool,
    invitation_id: String,
    map_id: i32,
    removal_reason: LolLobbyLobbyRemovedFromGameReason,
    can_start_matchmaking: bool,
    multi_user_chat_password: String,
    auto_fill_eligible: bool,
    is_team_builder_managed: bool,
    premade_size_allowed: bool,
    game_mode: String,
    auto_fill_protected_for_promos: bool,
    custom_game_lobby: LolLobbyLobbyCustomGameLobby,
    maximum_participant_list_size: u32,
    multi_user_chat_id: String,
    show_position_excluder: bool,
    required_position_coverage_met: bool,
    muc_jwt_dto: LolLobbyMucJwtDto,
    specifiable_position_preferences: Vec<String>,
    show_position_selector: bool,
    auto_fill_protected_for_streaking: bool,
    queue_availability: LolLobbyQueueAvailability,
    allowable_premade_sizes: Vec<i32>,
    queue_id: i32,
    invitations: Vec<LolLobbyLobbyInvitation>,
    is_custom: bool,
    members: Vec<LolLobbyLobbyMember>,
    local_member: LolLobbyLobbyMember,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksPerkGDSResource")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkGdsResource {
    icon_path: String,
    name: String,
    short_desc: String,
    major_change_patch_version: String,
    tooltip: String,
    long_desc: String,
    recommendation_descriptor_attributes: Value,
    id: i32,
    recommendation_descriptor: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCatalogItemLocalization {
    name: String,
    language: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectSkinSelectorSkin {
    chroma_preview_path: String,
    tile_path: String,
    splash_video_path: String,
    rarity_gem_path: String,
    emblems: Vec<LolChampSelectCollectionsChampionSkinEmblem>,
    is_base: bool,
    skin_augments: Value,
    is_champion_unlocked: bool,
    unlocked: bool,
    champion_id: i32,
    child_skins: Vec<LolChampSelectSkinSelectorChildSkin>,
    ownership: LolChampSelectCollectionsOwnership,
    still_obtainable: bool,
    group_splash: String,
    disabled: bool,
    product_type: LolChampSelectQuestSkinProductType,
    name: String,
    id: i32,
    splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopRiotMessagingServiceMessage {
    payload: String,
    version: String,
    timestamp: i64,
    service: String,
    resource: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOfferRequestV3 {
    price: u32,
    currency_type: String,
    offer_id: String,
    quantity: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearchErrorResource {
    penalty_time_remaining: f64,
    message: String,
    id: i32,
    error_type: String,
    penalized_summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubInventoryResponseDto {
    data: LolEventHubInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderCellsV1 {
    enemy_team: Vec<LolLobbyTeamBuilderCellV1>,
    allied_team: Vec<LolLobbyTeamBuilderCellV1>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsAccountIdAndSummonerId {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootCollectionsChampionMinimal {
    ownership: LolLootCollectionsOwnership,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatcherComponentActionProgress {
    total: PatcherComponentStateProgress,
    primary_work: PatcherComponentStateWorkType,
    network: PatcherComponentStateProgress,
    current_item: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRegaliaSummonerProfileUpdate {
    key: String,
    value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerLevelProgression {
    initial_xp: u64,
    final_xp: u64,
    final_level_boundary: u64,
    initial_level_boundary: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsLoginSession {
    state: LolChampionsLoginSessionStates,
    summoner_id: u64,
    connected: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubExternalCatalogSale {
    prices: Vec<LolEventHubExternalCatalogItemCost>,
    start_date: String,
    end_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Describes an enumerator."]
pub struct BindingFullEnumValueHelp {
    name: String,
    description: String,
    value: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPlayerBehaviorUserInfoRestriction {
    scope: String,
    _type: String,
    reason: String,
    dat: LolPlayerBehaviorUserInfoRestrictionData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatChatMessage {
    #[serde(rename = "id")]
    id: String,
    #[serde(rename = "type")]
    _type: LolChatMessageType,
    #[serde(rename = "time")]
    time: String,
    #[serde(rename = "cid")]
    cid: String,
    #[serde(rename = "body")]
    body: String,
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "mid")]
    mid: String,
    #[serde(rename = "game_name")]
    game_name: String,
    #[serde(rename = "read")]
    read: bool,
    #[serde(rename = "game_tag")]
    game_tag: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolClashClashConfig {
    max_time_before_lockin_notify_seconds: i32,
    honor_refresh_retry_seconds: i32,
    voice_random_start_min_seconds: i32,
    voice_no_delay_auto_start_seconds: i32,
    disabled_reason: String,
    event_sending_enabled: bool,
    voice_retry_delay_seconds: i32,
    check_parties_registration: bool,
    is_playmode_restriction_enabled: bool,
    min_clash_notifications_summoner_level: u32,
    voice_random_start_max_seconds: i32,
    disabled_events: Vec<String>,
    enabled_state: LolClashClashState,
    min_clash_summoner_level: u32,
    honor_level_required: i32,
    estimated_enable_time_millis: u64,
    reward_grant_retry_interval_seconds: i32,
    icon_config: String,
    voice_retry_count_limit: i32,
    visibility: LolClashClashVisibility,
    voice_eob_quit_delay_seconds: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPlayerNotification {
    detail_key: String,
    _type: String,
    title_key: String,
    critical: bool,
    source: String,
    icon_url: String,
    state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardRankedQueueStats {
    provisional_games_remaining: i32,
    is_provisional: bool,
    tier: String,
    division: LolSocialLeaderboardLeagueDivision,
    wins: i32,
    queue_type: LolSocialLeaderboardLeagueQueueType,
    league_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsSearchingForMatchNotification {
    player_join_failures: Vec<Value>,
    joined_queues: Vec<MatchmakingLcdsQueueInfo>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOfferId {
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGeoinfoLoginSession {
    account_id: u64,
    state: LolGeoinfoLoginSessionState,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsFavoritesViewModel {
    favorite_items: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatcherP2PStatusUpdate {
    is_allowed_by_user: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPointSummary {
    points_cost_to_roll: i32,
    points_to_next_roll: i32,
    current_points: i32,
    number_of_rolls: i32,
    max_rolls: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsStoreEntitlementPayload {
    transaction_id: String,
    items: Vec<LolInventoryRmsStoreEntitlementItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardSummonerIdAndIcon {
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaChatPresenceLolData {
    ranked_league_tier: String,
    ranked_prev_season_division: LolRegaliaLeagueDivision,
    ranked_league_division: LolRegaliaLeagueDivision,
    ranked_split_reward_level: i32,
    regalia: LolRegaliaRegaliaSettings,
    level: u32,
    ranked_prev_season_tier: String,
    ranked_league_queue: LolRegaliaLeagueQueueType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassTFTPassClientConfig")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPassClientConfig {
    daily_login_pass_id: String,
    enabled: bool,
    battle_pass_id: String,
    event_pass_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolDiscordRpPartyPresenceData {
    max_players: u64,
    queue_id: i32,
    summoners: Vec<u64>,
    party_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PaymentsRiotMessagingServiceMessage {
    resource: String,
    payload: String,
    version: String,
    service: String,
    timestamp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTeamBan {
    champion_id: i32,
    pick_turn: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionRepeatGroupTrigger {
    counter_id: String,
    increase_by: u16,
    multiplier: f32,
    _type: String,
    start_trigger_value: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesSequenceEvent {
    name: String,
    priority: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2MutualHonorPlayer {
    summoner_id: u64,
    champion_id: i32,
    skin_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeRewardsReward {
    data: Value,
    renderer: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "TournamentDTO")]
#[serde(rename_all = "camelCase")]
pub struct TournamentDto {
    buy_in_options_premium: Vec<i32>,
    bracket_formation_interval_ms: i64,
    scouting_time_ms: i64,
    entry_fee: i32,
    roster_size: i32,
    name_loc_key: String,
    max_invites: i32,
    roster_create_deadline: i64,
    honor_restriction: bool,
    reward_config: Vec<ClashRewardConfigClient>,
    sms_restriction: bool,
    min_games: i32,
    status: TournamentStatusEnum,
    tier_configs: Vec<TierConfig>,
    schedule_time: i64,
    voice_enabled: bool,
    schedule_end_time: i64,
    phases: Vec<TournamentPhaseDto>,
    theme_id: i32,
    name_loc_key_secondary: String,
    queue_id: i32,
    bracket_formation_init_delay_ms: i64,
    bracket_size: String,
    lft: bool,
    id: i64,
    buy_in_options: Vec<i32>,
    last_theme_of_season: bool,
    resume_time: i64,
    rank_restriction: bool,
    max_suggestions_per_player: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIdAndIcon {
    puuid: String,
    summoner_id: u64,
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedSocialLeaderboardRankedQueueStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSocialLeaderboardRankedQueueStatsDto {
    mini_series_progress: String,
    queue_type: String,
    wins: i32,
    rated_tier: String,
    league_points: i32,
    tier: String,
    provisional_game_threshold: i32,
    rated_rating: i32,
    rank: String,
    losses: i32,
    provisional_games_remaining: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataItemReference {
    inventory_type: String,
    content_id: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashGameflowAvailability {
    is_available: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTEndOfGamePlayer")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftEndOfGamePlayer {
    leaves: i32,
    spell_2_id: i32,
    summoner_id: u64,
    leaver: bool,
    level: i32,
    profile_icon_id: i32,
    team_id: i32,
    wins: i32,
    game_id: u64,
    spell_1_id: i32,
    stats: Value,
    summoner_name: String,
    items: Vec<i32>,
    bot_player: bool,
    champion_id: i32,
    losses: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGameDataSummonerIcon {
    title: String,
    id: i32,
    image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionRepeat {
    count: i32,
    multiplier: f32,
    milestones: Vec<LolProgressionMilestone>,
    scope: u32,
    repeat_triggers: Vec<LolProgressionRepeatGroupTrigger>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseItem {
    purchase_currency_info: LolPurchaseWidgetItemPrice,
    source: String,
    item_key: LolPurchaseWidgetItemKey,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingChampionMastery {
    champion_level: i32,
    champion_id: i32,
    champion_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchRegionLocale {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItem {
    quest_skin_info: LolPurchaseWidgetSkinLineInfo,
    image_path: String,
    ownership_type: LolPurchaseWidgetInventoryOwnership,
    item_id: i32,
    prices: Vec<LolPurchaseWidgetCatalogPluginPrice>,
    sub_inventory_type: String,
    active: bool,
    tags: Vec<String>,
    sub_title: String,
    inventory_type: String,
    item_instance_id: String,
    owned: bool,
    purchase_date: u64,
    description: String,
    release_date: u64,
    metadata: Vec<LolPurchaseWidgetItemMetadataEntry>,
    inactive_date: u64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOfferResponseV3 {
    order_dto: LolEventHubCapOrdersOrderDto,
    legacy: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftTencentEventHubConfigs {
    tencent_eventhub_configs: Vec<LolTftLolTftTencentEventHubConfig>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPlayerNotification {
    detail_key: String,
    icon_url: String,
    title_key: String,
    source: String,
    critical: bool,
    state: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerRerollPoints {
    number_of_rolls: u32,
    points_to_reroll: u32,
    points_cost_to_roll: u32,
    max_rolls: u32,
    current_points: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyTeamBuilderGameflowGameClient {
    running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsGameDataTFTDamageSkin")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataTftDamageSkin {
    rarity_value: u32,
    description: String,
    group_id: u32,
    item_id: i32,
    group_name: String,
    content_id: String,
    name: String,
    loadouts_icon: String,
    level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfoRestrictionData {
    game_data: LolPlayerBehaviorUserInfoRestrictionGameData,
    expiration_millis: u64,
    game_location: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsCreateLoadoutDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsCreateLoadoutDto {
    refresh_time: String,
    name: String,
    loadout: Value,
    item_id: u32,
    scope: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksPerkUIPerk")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiPerk {
    name: String,
    recommendation_descriptor: String,
    slot_type: String,
    tooltip: String,
    icon_path: String,
    short_desc: String,
    id: i32,
    style_id_name: String,
    style_id: i32,
    long_desc: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootQueryEvaluatedLootItem {
    loot_name: String,
    localized_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSwitchTeamsRequestDto {
    player_gco_tokens: LcdsPlayerGcoTokens,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSuggestionInvitee {
    invitee_id: u64,
    captain_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkSettings {
    per_shard_perk_books: Value,
    pages: Vec<LolPerksPerkPageResource>,
    settings: LolPerksUiSettings,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigDepInjectorEntitlements {
    token: String,
    subject: String,
    access_token: String,
    issuer: String,
    entitlements: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpPlayerLevelUpEvent {
    leveled_up: bool,
    now_has_access_to_public_chat_rooms: bool,
    now_has_access_to_loot: bool,
    rp_earned: i32,
    new_rune_slot_unlocked: bool,
    switched_to_standard_free_to_play_champ_rotation: bool,
    new_summoner_level: u32,
    new_spells: Vec<u64>,
    new_queues: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRewardGrant {
    info: LolRewardsSvcRewardGrant,
    reward_group: LolRewardsSvcRewardGroup,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubWalletDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubWalletDto {
    puuid: String,
    balances: Value,
    balances_jwt: String,
    account_id: i64,
    expires: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCollectionsLcdsDynamicClientConfig {
    disabled_champions: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsGameDataChampionMasteries {
    tree: LolCollectionsGameDataChampionMasteryTree,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsSvcRewardGrant {
    selected_ids: Vec<String>,
    status: LolRewardsGrantStatus,
    date_created: String,
    grantee_id: String,
    reward_group_id: String,
    message_parameters: Value,
    viewed: bool,
    grant_elements: Vec<LolRewardsSvcRewardGrantElement>,
    grantor_description: LolRewardsGrantorDescription,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthConfigStatus {
    readiness: LolRsoAuthConfigReadinessEnum,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsPlayerNotification {
    critical: bool,
    detail_key: String,
    id: u64,
    source: String,
    data: Value,
    title_key: String,
    icon_url: String,
    state: String,
    _type: String,
    background_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSimpleDialogMessagesMessage {
    body: Value,
    id: i64,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LcdsGameDTO")]
#[serde(rename_all = "camelCase")]
pub struct LcdsGameDto {
    practice_game_rewards_disabled_reasons: Vec<String>,
    passback_data_packet: String,
    optimistic_lock: i64,
    max_num_players: i32,
    spectators_allowed: String,
    game_mutators: Vec<String>,
    observers: Vec<LcdsPlayerParticipant>,
    game_mode: String,
    map_id: i32,
    team_two: Vec<LcdsPlayerParticipant>,
    owner_summary: LcdsPlayerParticipant,
    id: i64,
    room_name: String,
    muc_jwt_dto: MucJwtDto,
    passback_url: String,
    game_type: String,
    team_one: Vec<LcdsPlayerParticipant>,
    room_password: String,
    game_state: String,
    name: String,
    game_type_config_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingDataModelResponse {
    model_data: Value,
    response_code: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopSummonerIcon {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginMetadataResource {
    threading: PluginThreadingModel,
    app: String,
    subtype: String,
    _type: String,
    per_locale_asset_bundles: Value,
    mock: String,
    has_bundled_assets: bool,
    global_asset_bundles: Vec<String>,
    implements: Vec<String>,
    feature: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EligiblePlayer {
    skin_splash_path: String,
    puuid: String,
    champion_name: String,
    summoner_id: u64,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolRegaliaRegaliaPlatformConfig {
    hovercard_enabled: bool,
    selections_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PluginLcdsEvent {
    subtopic: String,
    body: Value,
    client_id: String,
    type_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesUIChallengeReward")]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesUiChallengeReward {
    asset: String,
    quantity: u64,
    name: String,
    category: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolContentTargetingContentTargetingFilterResponse {
    filters: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CriticalFlowExpectation {
    id_chain: String,
    qualifier_chain: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatMessageList {
    messages: Vec<LolChatMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneRekindledVignette {
    statstone: LolStatstonesStatstoneCompletion,
    portrait_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerIdAndName {
    display_name: String,
    puuid: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmDeactivationPinResponse {
    data: LolAccountVerificationPinResponseData,
    client_message_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLoadoutsGetItemsRequest {
    #[serde(rename = "id")]
    id: u32,
    #[serde(rename = "inventoryTypes")]
    inventory_types: Vec<String>,
    #[serde(rename = "inventoryJWTs")]
    inventory_jw_ts: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPublishingContentPublishingLocaleSetting {
    data: LolPublishingContentPublishingLocaleSettingData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectBannedChampions {
    their_team_bans: Vec<i32>,
    num_bans: i32,
    my_team_bans: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderAfkCheckStateV1 {
    muc_jwt_dto: LolLobbyTeamBuilderMucJwtDto,
    additional_inventory_types: Vec<String>,
    afk_ready: bool,
    remaining_afk_millis: i32,
    compress_afk_check_payload: bool,
    max_afk_millis: u32,
    inventory_draft: LolLobbyTeamBuilderTbdInventory,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SLILabel")]
#[serde(rename_all = "kebab-case")]
pub struct SliLabel {
    value: String,
    key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineInfo {
    collection_card_path: String,
    description_info: Vec<LolCatalogSkinLineDescriptionInfo>,
    tiers: Vec<LolCatalogSkinLineTier>,
    tile_path: String,
    splash_path: String,
    uncentered_splash_path: String,
    collection_description: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsStoreEntitlementPayload {
    transaction_id: String,
    items: Vec<LolYourshopRmsStoreEntitlementItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatSettingsResource {
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClashRewardOutput {
    grant: ClashRewardTime,
    primary: ClashRewardDefinition,
    alternative: ClashRewardDefinition,
    show: ClashRewardTime,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubEventInfoUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventInfoUiData {
    event_pass_bundles: Vec<LolEventHubCatalogEntry>,
    event_icon: String,
    event_token_image: String,
    unclaimed_reward_count: i32,
    is_pass_purchased: bool,
    current_token_balance: i32,
    event_type: String,
    event_name: String,
    event_id: String,
    locked_token_count: i32,
    token_bundles: Vec<LolEventHubCatalogEntry>,
    time_of_last_unclaimed_reward: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitAccountDetails {
    puuid: String,
    summoner_name: String,
    platform_id: String,
    summoner_level: u32,
    summoner_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPeriodAggregatedStats {
    match_stats: Vec<LolClashRosterMatchAggregatedStats>,
    period: i32,
    time: i64,
    player_bids: Value,
    bracket_size: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootCosmeticsTFTDamageSkinViewModel")]
#[serde(rename_all = "PascalCase")]
pub struct LolLootCosmeticsTftDamageSkinViewModel {
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "isRecentItem")]
    is_recent_item: bool,
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "groupName")]
    group_name: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "groupId")]
    group_id: u32,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "level")]
    level: u32,
    #[serde(rename = "upgrades")]
    upgrades: Vec<LolLootCosmeticsTftDamageSkinViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyGameflowGameData {
    queue: LolLobbyQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRankedEosNotificationsConfig {
    config: Vec<LolRankedEosNotificationsConfigEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueLadderInfo {
    next_apex_update_millis: i64,
    provisional_game_threshold: i32,
    tier: String,
    requested_ranked_entry: LolRankedLeagueStanding,
    queue_type: LolRankedLeagueQueueType,
    divisions: Vec<LolRankedLeagueDivisionInfo>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubRewardsConfig {
    grant_filtering: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCareerStatsEntitlementsToken {
    entitlements: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolRankedRankedStats {
    #[serde(rename = "splitsProgress")]
    splits_progress: Value,
    #[serde(rename = "currentSeasonSplitPoints")]
    current_season_split_points: i32,
    #[serde(rename = "highestRankedEntry")]
    highest_ranked_entry: LolRankedRankedQueueStats,
    #[serde(rename = "highestCurrentSeasonReachedTierSR")]
    highest_current_season_reached_tier_sr: String,
    #[serde(rename = "seasons")]
    seasons: Value,
    #[serde(rename = "previousSeasonSplitPoints")]
    previous_season_split_points: i32,
    #[serde(rename = "queueMap")]
    queue_map: Value,
    #[serde(rename = "earnedRegaliaRewardIds")]
    earned_regalia_reward_ids: Vec<String>,
    #[serde(rename = "highestRankedEntrySR")]
    highest_ranked_entry_sr: LolRankedRankedQueueStats,
    #[serde(rename = "queues")]
    queues: Vec<LolRankedRankedQueueStats>,
    #[serde(rename = "highestPreviousSeasonEndTier")]
    highest_previous_season_end_tier: String,
    #[serde(rename = "rankedRegaliaLevel")]
    ranked_regalia_level: i32,
    #[serde(rename = "highestPreviousSeasonEndDivision")]
    highest_previous_season_end_division: LolRankedLeagueDivision,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolFeaturedModesCollectionsOwnership {
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatTranslateRequest {
    #[serde(rename = "patchline")]
    patchline: String,
    #[serde(rename = "locale")]
    locale: String,
    #[serde(rename = "keys")]
    keys: Vec<String>,
    #[serde(rename = "blocking")]
    blocking: bool,
    #[serde(rename = "product_id")]
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkin {
    features_text: String,
    id: i32,
    splash_video_path: String,
    splash_path: String,
    disabled: bool,
    chroma_path: String,
    quest_skin_info: LolChampionsChampionQuestSkinInfo,
    ownership: LolChampionsCollectionsOwnership,
    champion_id: i32,
    last_selected: bool,
    emblems: Vec<LolChampionsCollectionsChampionSkinEmblem>,
    tile_path: String,
    chromas: Vec<LolChampionsCollectionsChampionChroma>,
    collection_splash_video_path: String,
    skin_augments: LolChampionsCollectionsChampionSkinAugments,
    load_screen_path: String,
    rarity_gem_path: String,
    is_base: bool,
    skin_type: String,
    name: String,
    still_obtainable: bool,
    uncentered_splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTDamageSkinFavoritesViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftDamageSkinFavoritesViewModel {
    favorite_items: Vec<LolCosmeticsCosmeticsTftDamageSkinViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRankedSequenceEvent {
    priority: i32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashQueue {
    game_mode: String,
    category: LolClashQueueGameCategory,
    game_type_config: LolClashQueueGameTypeConfig,
    description: String,
    min_level: u32,
    short_name: String,
    minimum_participant_list_size: i32,
    are_free_champions_allowed: bool,
    is_ranked: bool,
    maximum_participant_list_size: i32,
    num_players_per_team: i32,
    queue_rewards: LolClashQueueReward,
    _type: String,
    id: i32,
    spectator_enabled: bool,
    name: String,
    detailed_description: String,
    is_team_builder_managed: bool,
    map_id: i32,
    queue_availability: LolClashQueueAvailability,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSimpleDialogMessagesGameflowSession {
    phase: LolSimpleDialogMessagesGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesEligibilityRestriction {
    restriction_code: LolFeaturedModesEligibilityRestrictionCode,
    expired_timestamp: u64,
    restriction_args: Value,
    summoner_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassLoyaltyStatusNotification {
    rewards: LolTftPassLoyaltyRewardsSimplified,
    reload_inventory: bool,
    status: LolTftPassLoyaltyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassRepeat {
    scope: u32,
    milestones: Vec<LolTftPassMilestone>,
    repeat_triggers: Vec<LolTftPassRepeatGroupTrigger>,
    count: i32,
    multiplier: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardRankedStats {
    queue_map: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRsoAuthRSOJWTConfig")]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthRsojwtConfig {
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchNotification {
    id: String,
    data: Value,
    notification_id: LolPatchNotificationId,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesReward {
    name: String,
    item_type_id: String,
    quantity: u32,
    rarity: LolTftTrovesLootRarity,
    reward_texture_path: String,
    item_id: String,
    star_level: u32,
    highlight_reward_asset_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopInventoryResponseDto {
    data: LolYourshopInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigConfigStatus {
    update_id: u64,
    readiness: ClientConfigConfigReadinessEnum,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueGameTypeConfig {
    id: i64,
    max_allowable_bans: i32,
    allow_trades: bool,
    exclusive_pick: bool,
    team_champion_pool: bool,
    reroll: bool,
    battle_boost: bool,
    learning_quests: bool,
    death_match: bool,
    main_pick_timer_duration: i32,
    cross_team_champion_pool: bool,
    game_mode_override: String,
    do_not_remove: bool,
    ban_timer_duration: i32,
    ban_mode: String,
    duplicate_pick: bool,
    num_players_per_team_override: i32,
    pick_mode: String,
    onboard_coop_beginner: bool,
    name: String,
    post_pick_timer_duration: i32,
    advanced_learning_quests: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsESportsAPI_streamgroups_root")]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsApiStreamgroupsRoot {
    highlander_tournaments: Vec<LolEsportStreamNotificationsEsportsApiHighlanderTournaments>,
    teams: Vec<LolEsportStreamNotificationsEsportsApiTeams>,
    streamgroups: Vec<LolEsportStreamNotificationsESportsApiStreamgroups>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyLoyaltyStatusNotification {
    status: LolLoyaltyLoyaltyStatus,
    rewards: LolLoyaltyLoyaltyRewardsSimplified,
    reload_inventory: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatcherP2PStatus {
    is_enabled_for_patchline: bool,
    is_allowed_by_user: bool,
    requires_restart: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterDetails {
    icon_id: i32,
    short_name: String,
    icon_color_id: i32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubInventoryItemDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubInventoryItemDto {
    #[serde(rename = "entitlementId")]
    entitlement_id: String,
    #[serde(rename = "ownedQuantity")]
    owned_quantity: u64,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "instanceTypeId")]
    instance_type_id: String,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "usedInGameDate")]
    used_in_game_date: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "entitlementTypeId")]
    entitlement_type_id: String,
    #[serde(rename = "lsb")]
    lsb: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSummoner {
    are_summoner_actions_complete: bool,
    spell_2_icon_path: String,
    pick_sniped_class: String,
    name_visibility_type: String,
    summoner_id: u64,
    is_self: bool,
    obfuscated_puuid: String,
    champion_icon_style: String,
    cell_id: i64,
    puuid: String,
    is_on_players_team: bool,
    should_show_expanded: bool,
    swap_id: i64,
    should_show_spells: bool,
    acting_background_animation_state: String,
    is_acting_now: bool,
    trade_id: i64,
    current_champion_vote_percent_integer: i32,
    slot_id: u64,
    obfuscated_summoner_id: u64,
    show_swaps: bool,
    should_show_ban_intent_icon: bool,
    active_action_type: String,
    is_done_picking: bool,
    ban_intent_square_portrat_path: String,
    is_pick_intenting: bool,
    show_muted: bool,
    status_message_key: String,
    spell_1_icon_path: String,
    should_show_ring_animations: bool,
    should_show_selected_skin: bool,
    show_trades: bool,
    skin_splash_path: String,
    skin_id: i32,
    champion_id: i32,
    assigned_position: String,
    champion_name: String,
    should_show_acting_bar: bool,
    is_placeholder: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItemAssets {
    emblems: Vec<LolPurchaseWidgetChampionSkinEmblem>,
    icon_path: String,
    splash_path: String,
    colors: Vec<String>,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsNumberFormattingData {
    meters_abbreviation: String,
    ten_thousand_abbreviation: String,
    trillion_abbreviation: String,
    kilometers_abbreviation: String,
    thousand_abbreviation: String,
    million_abbreviation: String,
    number_formatting_behavior: LolCollectionsNumberFormattingBehavior,
    second_abbreviation: String,
    thousand_seperator: String,
    one_hundred_million_abbreviation: String,
    hour_abbreviation: String,
    decimal_seperator: String,
    billion_abbreviation: String,
    minute_abbreviation: String,
    percentage_format: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomGameSettingsDto {
    game_id: u64,
    lobby_password: String,
    lobby_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsUpdateLoadoutDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolLoadoutsUpdateLoadoutDto {
    id: String,
    loadout: Value,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTEndOfGameTeam")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftEndOfGameTeam {
    team_id: i32,
    is_winning_team: bool,
    name: String,
    stats: Value,
    players: Vec<LolPftPftEndOfGamePlayer>,
    tag: String,
    is_bottom_team: bool,
    is_player_team: bool,
    member_status_string: String,
    full_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsNumberFormattingBehavior {
    digits_for_thousands_seperator: u32,
    trim_trailing_zeros_after_decimal: bool,
    western_number_grouping: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "IdsDTO")]
#[serde(rename_all = "camelCase")]
pub struct IdsDto {
    mission_ids: Vec<String>,
    series_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderQueueReward {
    is_champion_points_enabled: bool,
    party_size_ip_rewards: Vec<i32>,
    is_ip_enabled: bool,
    is_xp_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounterInstance {
    counter_value: i64,
    owner_id: String,
    product_id: String,
    counter_id: String,
    group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsLoginSession {
    puuid: String,
    summoner_id: u64,
    account_id: u64,
    id_token: String,
    state: LolLoadoutsLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesQueue {
    game_mode: String,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTicketOffer {
    is_accepted: bool,
    summoner_id: u64,
    amount: i32,
    ticket_type: TicketType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubEndOfGameXpNotification {
    xp: LolEventHubEndOfGameXp,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSParticipantIdentityPlayer")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsParticipantIdentityPlayer {
    current_platform_id: String,
    current_account_id: u64,
    match_history_uri: String,
    summoner_id: u64,
    puuid: String,
    platform_id: String,
    account_id: u64,
    game_name: String,
    summoner_name: String,
    tag_line: String,
    profile_icon: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePlayerDto {
    display_name: String,
    summoner_id: u64,
    role: LolPremadeVoicePartyMemberRoleEnum,
    puuid: String,
    party_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStoreFeaturedPageDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolStoreFeaturedPageDto {
    player: LolStorePlayer,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftPassWallet {
    rp: i64,
    ip: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayMetadataV2 {
    file_size: u32,
    game_version: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTeamPlannerTFTModeData")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerTftModeData {
    m_default_set: LolTftTeamPlannerTftMapSetData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingSearchErrorResource {
    id: i32,
    penalized_summoner_id: u64,
    penalty_time_remaining: f64,
    message: String,
    error_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSkinLineDescriptionDto {
    icon_image_path: String,
    description: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerChatRoster {
    multi_user_chat_id: String,
    start_time_ms: i64,
    is_registered: bool,
    player_state: LolClashPlayerState,
    name: String,
    short_name: String,
    tournament_state: LolClashTournamentState,
    multi_user_chat_password: String,
    icon_id: i32,
    muc_jwt_dto: LolClashMucJwtDto,
    invitation_id: String,
    icon_color_id: i32,
    end_time_ms: i64,
    tournament_id: i64,
    logo_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesChallengeThresholdReward {
    quantity: u32,
    category: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassRmsStoreEntitlementPayload {
    items: Vec<LolTftPassRmsStoreEntitlementItem>,
    transaction_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartiesInvitationPlayerAnalytics {
    summoner_id: u64,
    role: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectPlayerSelection {
    assigned_position: String,
    champion_pick_intent: i32,
    champion_id: i32,
    summoner_id: u64,
    cell_id: i64,
    team: i32,
    puuid: String,
    player_type: String,
    selected_skin_id: i32,
    ward_skin_id: i64,
    spell_1_id: u64,
    spell_2_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderQueue {
    num_players_per_team: i32,
    game_mode: String,
    maximum_participant_list_size: i32,
    queue_rewards: LolLobbyTeamBuilderQueueReward,
    is_ranked: bool,
    show_position_selector: bool,
    map_id: i32,
    removal_from_game_allowed: bool,
    last_toggled_on_time: u64,
    detailed_description: String,
    show_quick_play_slot_selection: bool,
    champions_required_to_play: u32,
    short_name: String,
    queue_availability: LolLobbyTeamBuilderQueueAvailability,
    id: i32,
    last_toggled_off_time: u64,
    min_level: u32,
    game_type_config: LolLobbyTeamBuilderQueueGameTypeConfig,
    name: String,
    description: String,
    category: LolLobbyTeamBuilderQueueGameCategory,
    asset_mutator: String,
    _type: String,
    removal_from_game_delay_minutes: i32,
    are_free_champions_allowed: bool,
    is_team_builder_managed: bool,
    minimum_participant_list_size: i32,
    spectator_enabled: bool,
    allowable_premade_sizes: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "ChampionMasteryPublicDTO")]
#[serde(rename_all = "camelCase")]
pub struct ChampionMasteryPublicDto {
    champion_id: i32,
    champion_level: i32,
    champion_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolInventoryAccessTokenResource {
    token: String,
    scopes: Vec<String>,
    expiry: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOfferOrderStatus {
    order_state: LolEventHubPurchaseOfferOrderStates,
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPositionPreferences {
    second_preference: String,
    first_preference: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsEsportsAPI_highlanderTournaments_matches")]
#[serde(rename_all = "kebab-case")]
pub struct LolEsportStreamNotificationsEsportsApiHighlanderTournamentsMatches {
    id: String,
    input: Vec<LolEsportStreamNotificationsEsportsApiHighlanderTournamentsRoster>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathLobbyGameConfigDto {
    queue_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesOperationalQueueConfig {
    is_visible_in_client: bool,
    is_spectatable: bool,
    queue_id: i32,
    is_enabled: bool,
    mutators: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeRewardsAccountSettingsData {
    login: LolNpeRewardsLoginSeriesSettings,
    challenges: LolNpeRewardsChallengesSettings,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneVignetteNotificationEnvelopeDto {
    champ_id: i32,
    mastery_vignette_notifications: Vec<LolStatstonesStatstoneMasteryVignette>,
    rekindled_vignette_notifications: Vec<LolStatstonesStatstoneRekindledVignette>,
    champ_name: String,
    set_complete_vignette_notifications: Vec<LolStatstonesStatstoneSetCompleteVignette>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassCatalogItem {
    inventory_type: String,
    item_instance_id: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksPerkUIRecommendedPage")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkUiRecommendedPage {
    is_recommendation_override: bool,
    recommendation_champion_id: i32,
    secondary_recommendation_attribute: String,
    primary_perk_style_id: i32,
    secondary_perk_style_id: i32,
    recommendation_id: String,
    is_default_position: bool,
    position: String,
    perks: Vec<LolPerksPerkUiPerk>,
    primary_recommendation_attribute: String,
    keystone: LolPerksPerkUiPerk,
    summoner_spell_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGameCustomAugmentContainerViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameCustomAugmentContainerViewModel {
    name_id: String,
    icon_path: String,
    display_name: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSession {
    has_simultaneous_picks: bool,
    rerolls_remaining: u64,
    is_custom_game: bool,
    actions: Vec<Value>,
    pick_order_swaps: Vec<LolChampSelectChampSelectSwapContract>,
    counter: i64,
    their_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    allow_battle_boost: bool,
    local_player_cell_id: i64,
    timer: LolChampSelectChampSelectTimer,
    trades: Vec<LolChampSelectChampSelectTradeContract>,
    allow_skin_selection: bool,
    chat_details: LolChampSelectChampSelectChatRoomDetails,
    allow_duplicate_picks: bool,
    is_spectating: bool,
    boostable_skin_count: i32,
    recovery_counter: i64,
    allow_rerolling: bool,
    skip_champion_select: bool,
    game_id: u64,
    bench_enabled: bool,
    my_team: Vec<LolChampSelectChampSelectPlayerSelection>,
    bans: LolChampSelectChampSelectBannedChampions,
    allow_locked_events: bool,
    locked_event_index: i32,
    bench_champions: Vec<LolChampSelectBenchChampion>,
    has_simultaneous_bans: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGeoinfoGeoInfo {
    city: String,
    country: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCounterInstance {
    owner_id: String,
    product_id: String,
    counter_id: String,
    counter_value: i64,
    group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersTournamentFrameInventoryItem {
    payload: LolBannersCapClashFrameEntitlementPayload,
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashFindPlayers {
    member_id: i64,
    page: i32,
    invitation_id: String,
    count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MultipleReplayMetadataResponseItemV2 {
    status: ReplayResponseStatus,
    metadata: ReplayMetadataV2,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolServiceStatusRiotStatusTranslation {
    locale: String,
    content: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournament {
    status: TournamentStatusEnum,
    scouting_duration_ms: i64,
    roster_size: i32,
    last_theme_of_season: bool,
    buy_in_options: Vec<i32>,
    buy_in_options_premium: Vec<i32>,
    name_loc_key_secondary: String,
    reward_config: Vec<ClashRewardConfigClient>,
    theme_id: i32,
    allow_roster_creation: bool,
    start_time_ms: i64,
    max_invites: i32,
    roster_create_deadline: i64,
    id: i64,
    is_ranked_restriction_enabled: bool,
    max_suggestions_per_player: i32,
    name_loc_key: String,
    queue_id: i32,
    end_time_ms: i64,
    is_honor_restriction_enabled: bool,
    lft: bool,
    entry_fee: i32,
    bracket_size: String,
    phases: Vec<LolClashTournamentPhase>,
    tier_configs: Vec<TierConfig>,
    bracket_formation_interval_ms: i64,
    bracket_formation_init_delay_ms: i64,
    resume_time: i64,
    is_sms_restriction_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsEsportsAPI_highlanderTournaments_rosters")]
#[serde(rename_all = "kebab-case")]
pub struct LolEsportStreamNotificationsEsportsApiHighlanderTournamentsRosters {
    team: i64,
    id: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatFriendRequestList {
    requests: Vec<LolChatFriendRequest>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterMember {
    incoming_offers: Vec<LolClashTicketOffer>,
    position: Position,
    current_buyin: i32,
    tier: i32,
    replaced_summoner_id: u64,
    state: LolClashRosterMemberState,
    buyin_type: TicketType,
    summoner_id: u64,
    invite_type: InviteType,
    inviter_id: u64,
    previous_buyin: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowSpectateGameInfoResource {
    allow_observe_mode: String,
    puuid: String,
    drop_in_spectate_game_id: String,
    game_queue_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolSummonerAliasLookupResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerAliasLookupResponseDto {
    puuid: String,
    alias: LolSummonerAliasLookupDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassInventoryCacheEntry {
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
    #[serde(rename = "signedInventoryJwt")]
    signed_inventory_jwt: String,
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventPassInfo {
    is_pass_purchased: bool,
    event_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventCollectionsWardSkin {
    id: i64,
    ownership: LolTftEventCollectionsOwnership,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerChampion {
    champion_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "YourshopLcdsChampionSkinDTO")]
#[serde(rename_all = "camelCase")]
pub struct YourshopLcdsChampionSkinDto {
    champion_id: i32,
    owned: bool,
    skin_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTrophiesInventoryResponse {
    items: LolTrophiesInventoryItemsByType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsJoinGameRequestDto {
    game_id: u64,
    password: String,
    game_version: String,
    simple_inventory_jwt: String,
    player_gco_tokens: LcdsPlayerGcoTokens,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineDescriptionInfo {
    icon_path: String,
    title: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPracticeGameSearchResult {
    game_mode: String,
    id: u64,
    spectator_count: i32,
    owner: LcdsPlayerParticipant,
    game_map: LcdsGameMap,
    game_map_id: i32,
    private_game: bool,
    allow_spectators: String,
    team_2_count: i32,
    name: String,
    team_1_count: i32,
    max_num_players: i32,
    pick_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRewardNotification {
    reward_group_id: String,
    season_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatQueueGameTypeConfig {
    id: i64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaChatPresenceExternal {
    id: String,
    summoner_id: u64,
    icon: i32,
    lol: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventMissionsNotificationResource {
    created: String,
    expires: String,
    state: String,
    source: String,
    title_key: String,
    critical: bool,
    icon_url: String,
    background_url: String,
    id: u64,
    detail_key: String,
    _type: String,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftLolTftHomeHub {
    tactician_promo_offer_ids: Vec<String>,
    fallback_store_promo_offer_ids: Vec<String>,
    prime_gaming_promo_offer: LolTftLolTftPrimeGaming,
    battle_pass_offer_ids: Vec<String>,
    header_buttons_override_url: String,
    store_promo_offer_ids: Vec<String>,
    enabled: bool,
    override_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TierConfig {
    estimate_time: i64,
    tier: i32,
    delay_time: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHeartbeatLoginSession {
    state: LolHeartbeatLoginSessionStates,
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataChampionChroma {
    name: String,
    colors: Vec<String>,
    chroma_path: String,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventHubError {
    error_id: String,
    error_message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsSummoner {
    summoner_level: u32,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathReward {
    reward_group_selected: bool,
    reward_type: String,
    quantity: i32,
    item_id: String,
    reward_fulfilled: bool,
    unique_name: String,
    reward_group: String,
    icon_url: String,
    sequence: i32,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectPlayerSelection {
    cell_id: i64,
    champion_pick_intent: i32,
    selected_skin_id: i32,
    assigned_position: String,
    champion_id: i32,
    summoner_id: u64,
    spell_2_id: u64,
    ward_skin_id: i64,
    spell_1_id: u64,
    team: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogPluginItemWithDetails {
    bundled_discount_prices: Vec<LolEventHubCatalogPluginPrice>,
    minimum_bundle_prices: Vec<LolEventHubCatalogPluginPrice>,
    required_items: Vec<LolEventHubCatalogPluginItemWithDetails>,
    bundled_items: Vec<LolEventHubCatalogPluginItemWithDetails>,
    quantity: u32,
    assets: LolEventHubCatalogPluginItemAssets,
    item: LolEventHubCatalogPluginItem,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRegistrationCredentials {
    ranked_overview_token: String,
    player_tokens: Value,
    game_client_version: String,
    summoner_token: String,
    inventory_tokens: Vec<String>,
    inventory_token: String,
    simple_inventory_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameGameDataChampion {
    id: i32,
    skins: Vec<LolEndOfGameGameDataChampionSkin>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMissionsCollectionsRental {
    rented: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkinEmblem {
    name: String,
    emblem_path: LolChampSelectCollectionsChampionSkinEmblemPath,
    positions: LolChampSelectCollectionsChampionSkinEmblemPosition,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowGameStateUpdate {
    id: u64,
    game_type: String,
    error_message: String,
    game_state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatChatServiceDynamicClientConfig {
    chat_domain: LolChatChatDomainConfig,
    lcu_social: LolChatLcuSocialConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneProgress {
    statstone_id: String,
    statstone_name: String,
    value: String,
    next_milestone: String,
    image_url: String,
    new_progress_percent: String,
    new_milestone_difference: String,
    total_progress_percent: String,
    category: String,
    delta: String,
    existing_progress_percent: String,
    level: i32,
    best: i32,
    description: String,
    is_new_best: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGetPlatformIdsFromInstanceIdsRequest {
    instance_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreItemMetadataEntry {
    _type: String,
    value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLolTftPromoButtons {
    promo_buttons: Vec<LolTftEventLolTftPromoButton>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatProductMetadata {
    name: String,
    id: String,
    patchlines: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPremadeMemberDto {
    tag_line: String,
    display_name: String,
    summoner_id: u64,
    game_name: String,
    party_id: String,
    puuid: String,
    role: LolLobbyPartyMemberRoleEnum,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyChangeQueue {
    custom_game_lobby: LolLobbyLobbyCustomGameLobby,
    queue_id: i32,
    is_custom: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesGameflowSession {
    game_data: LolChallengesGameflowGameData,
    phase: LolChallengesGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubEventBackgroundUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventBackgroundUiData {
    background_image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRepeat {
    repeat_triggers: Vec<LolLootRepeatGroupTrigger>,
    multiplier: f32,
    milestones: Vec<LolLootMilestone>,
    count: i32,
    scope: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryXboxSubscriptionStatus {
    subscription_id: String,
    active: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeData {
    friends_at_levels: Vec<LolChallengesFriendLevelsData>,
    position: i32,
    next_level: String,
    init_value: f64,
    current_threshold: f64,
    current_value: f64,
    current_level_achieved_time: i64,
    previous_level: String,
    current_level: String,
    id: i64,
    new_levels: Vec<String>,
    percentile: f64,
    legacy: bool,
    players_in_level: i32,
    category: String,
    previous_value: f64,
    previous_threshold: f64,
    next_threshold: f64,
    id_list_type: LolChallengesChallengeRequirementMappingName,
    available_ids: Vec<i32>,
    completed_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "CollectionsLcdsChampionSkinDTO")]
#[serde(rename_all = "PascalCase")]
pub struct CollectionsLcdsChampionSkinDto {
    #[serde(rename = "lastSelected")]
    last_selected: bool,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "purchaseDate")]
    purchase_date: u64,
    #[serde(rename = "championId")]
    champion_id: i32,
    #[serde(rename = "freeToPlayReward")]
    free_to_play_reward: bool,
    #[serde(rename = "f2pRewardSources")]
    f_2_p_reward_sources: Vec<String>,
    #[serde(rename = "winCountRemaining")]
    win_count_remaining: i32,
    #[serde(rename = "endDate")]
    end_date: u64,
    #[serde(rename = "skinId")]
    skin_id: i32,
    #[serde(rename = "stillObtainable")]
    still_obtainable: bool,
    #[serde(rename = "sources")]
    sources: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationGameConfigDto {
    queue_id: i32,
    game_mode: String,
    invite_game_type: String,
    map_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPremadeVoiceConfigStatus {
    readiness: LolPremadeVoiceConfigReadinessEnum,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolContentTargetingContentTargetingLocaleResponse {
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSale {
    prices: Vec<LolCatalogItemCost>,
    start_date: String,
    end_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectatorConfig {
    is_bracket_spectating_enabled: bool,
    spectatable_queues: Vec<u32>,
    is_enabled: bool,
    is_spectator_delay_configurable: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundled {
    flexible: bool,
    items: Vec<LolStoreBundledItem>,
    minimum_prices: Vec<LolStoreBundledItemCost>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataItemReference {
    item_id: i32,
    inventory_type: String,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventMissionAsset {
    path: String,
    internal_name: String,
    icon_needs_frame: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatcookie {
    value: String,
    domain: String,
    secure: bool,
    httponly: bool,
    expires: i64,
    path: String,
    url: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc = "Describes a struct or enum type."]
pub struct BindingFullTypeHelp {
    tags: Vec<String>,
    name: String,
    values: Vec<BindingFullEnumValueHelp>,
    fields: Vec<BindingFullFieldHelp>,
    name_space: String,
    size: u32,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentAccountData {
    account_id: String,
    summoner_level: String,
    display_name: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueStanding {
    position: i32,
    provisional_games_remaining: i32,
    division: LolRankedLeagueDivision,
    losses: u64,
    puuid: String,
    league_points: i64,
    previous_season_end_division: LolRankedLeagueDivision,
    pending_promotion: bool,
    previous_season_end_tier: String,
    summoner_name: String,
    previous_position: i32,
    pending_demotion: bool,
    earned_regalia_reward_ids: Vec<String>,
    position_delta: i32,
    tier: String,
    miniseries_results: Vec<LolRankedMiniseries>,
    is_provisional: bool,
    summoner_id: u64,
    wins: u64,
    ranked_regalia_level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsWardSkin {
    ward_shadow_image_path: String,
    ownership: LolLootCollectionsOwnership,
    name: String,
    id: i64,
    ward_image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftEventLolTftTencentEventHubConfig {
    #[serde(rename = "troveAssetId")]
    trove_asset_id: String,
    #[serde(rename = "troveURL")]
    trove_url: String,
    #[serde(rename = "logoAssetId")]
    logo_asset_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSwitchPlayerToObserverRequestDto {
    player_gco_tokens: LcdsPlayerGcoTokens,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionQuestSkinInfo {
    splash_path: String,
    product_type: LolChampSelectQuestSkinProductType,
    tile_path: String,
    tiers: Vec<LolChampSelectCollectionsChampionQuestSkin>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsRollResult {
    champion_id: i32,
    point_summary: ChampSelectLcdsPointSummary,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLoadoutsCreateOrUpdateItemsRequest {
    #[serde(rename = "inventoryJWTs")]
    inventory_jw_ts: Vec<String>,
    #[serde(rename = "items")]
    items: Value,
    #[serde(rename = "id")]
    id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyGameflowSession {
    game_data: LolChampSelectLegacyGameflowGameData,
    game_client: LolChampSelectLegacyGameflowGameClient,
    phase: LolChampSelectLegacyGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyAutoFillQueueDto {
    auto_fill_protected_for_remedy: bool,
    auto_fill_protected_for_promos: bool,
    queue_id: i32,
    auto_fill_eligible: bool,
    auto_fill_protected_for_streaking: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathLobbyChangeQueue {
    is_custom: bool,
    queue_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerInfoDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfoDto {
    player_id: u64,
    team_id: String,
    role: Role,
    position: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPriceDetail {
    item_key: LolEventHubItemKey,
    price: LolEventHubItemPrice,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingMission {
    completed_date: i64,
    id: String,
    status: String,
    internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGrantElement {
    item_id: String,
    reward_status: LolMissionsRewardStatus,
    quantity: i32,
    media: Value,
    localizations: Value,
    item_type: String,
    fulfillment_source: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksNamecheckLoginDataPacket {
    platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopAccessTokenResource {
    token: String,
    scopes: Vec<String>,
    expiry: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLeagueSessionTokenEnvelope {
    logout_on_failure: bool,
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MetricMetadataAlert {
    #[serde(rename = "info")]
    info: String,
    #[serde(rename = "pretty_name")]
    pretty_name: String,
    #[serde(rename = "min")]
    min: f64,
    #[serde(rename = "level")]
    level: String,
    #[serde(rename = "max")]
    max: f64,
    #[serde(rename = "notify")]
    notify: MetricMetadataNotify,
    #[serde(rename = "description")]
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorUserInfoRestrictionGameData {
    game_location: String,
    additional_game_ids: Vec<String>,
    product_name: String,
    trigger_game_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolItemSetsNamecheckResponse {
    errors: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderRerollStateV1 {
    rerolls_remaining: u64,
    allow_rerolling: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyLobbyStatus {
    allowed_play_again: bool,
    is_leader: bool,
    member_summoner_ids: Vec<u64>,
    is_spectator: bool,
    queue_id: i32,
    is_custom: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChampionMasteryUIAllChampionMasteryWithSets")]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryUiAllChampionMasteryWithSets {
    total_score: i32,
    custom_rewards: Vec<LolChampionMasteryUiChampionMasteryCustomReward>,
    champion_masteries: Vec<LolChampionMasteryChampionMastery>,
    champion_set_rewards: Value,
    season_milestone_require_and_rewards: Value,
    champion_set: LolChampionMasteryChampionSet,
    default_champion_mastery: LolChampionMasteryChampionMastery,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTSurvey")]
#[serde(rename_all = "kebab-case")]
pub struct LolPftPftSurvey {
    _type: String,
    data: Value,
    id: u64,
    display: String,
    title: String,
    caption: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferContextDto {
    payment_option: String,
    quantity: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsWardSkin {
    ownership: LolCollectionsCollectionsOwnership,
    description: String,
    name: String,
    id: i64,
    ward_image_path: String,
    ward_shadow_image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataTftPlaybook {
    item_id: i32,
    name: String,
    translated_name: String,
    icon_path_small: String,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyLastQueuedLobby {
    queue_id: i32,
    was_owner: bool,
    members: Vec<LolLobbyLobbyLastQueuedMember>,
    can_play_again: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatChatFriendUpdate {
    pid: String,
    group: String,
    note: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameGameDataTFTCharacterRecord")]
#[serde(rename_all = "PascalCase")]
pub struct LolEndOfGameGameDataTftCharacterRecord {
    #[serde(rename = "squareIconPath")]
    square_icon_path: String,
    #[serde(rename = "display_name")]
    display_name: String,
    #[serde(rename = "character_id")]
    character_id: String,
    #[serde(rename = "rarity")]
    rarity: i32,
    #[serde(rename = "path")]
    path: String,
    #[serde(rename = "traits")]
    traits: Vec<LolEndOfGameGameDataTftTrait>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootRegionLocale {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEsportStreamNotificationsGameflowSession {
    phase: LolEsportStreamNotificationsGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGame {
    game_id: u64,
    game_creation_date: String,
    game_version: String,
    end_of_game_result: String,
    map_id: u16,
    platform_id: String,
    queue_id: i32,
    game_type: String,
    game_mode: String,
    game_creation: u64,
    game_duration: u32,
    participant_identities: Vec<LolMatchHistoryMatchHistoryParticipantIdentities>,
    season_id: u16,
    participants: Vec<LolMatchHistoryMatchHistoryParticipant>,
    teams: Vec<LolMatchHistoryMatchHistoryTeam>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubGrantorDescription {
    app_name: String,
    entity_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesTable {
    name_tra_key: String,
    translated_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolYourshopLoyaltyRewards {
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "xpBoost")]
    xp_boost: Value,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersBannerFlag {
    earned_date_iso_8601: String,
    level: i64,
    item_id: i32,
    theme: String,
    season_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerXpAndLevelMessage {
    xp: Value,
    level: LolSummonerLevelField,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardData {
    id: String,
    _type: String,
    override_image_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EndOfGameTeam {
    players: Vec<LolHonorV2EndOfGamePlayer>,
    is_player_team: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftTrovesTrovesCelebrationStandardSegmentData {
    #[serde(rename = "PCBackgroundPath")]
    pc_background_path: String,
    #[serde(rename = "PCRewardFadeInDuration")]
    pc_reward_fade_in_duration: f32,
    #[serde(rename = "PCThumbnailFadeInDuration")]
    pc_thumbnail_fade_in_duration: f32,
    #[serde(rename = "revealGlobalSoundPath")]
    reveal_global_sound_path: String,
    #[serde(rename = "revealRareSoundPath")]
    reveal_rare_sound_path: String,
    #[serde(rename = "PCRewardFramePath")]
    pc_reward_frame_path: String,
    #[serde(rename = "PCRewardTwoStarPath")]
    pc_reward_two_star_path: String,
    #[serde(rename = "PCRewardSheenDuration")]
    pc_reward_sheen_duration: f32,
    #[serde(rename = "PCHeaderText")]
    pc_header_text: String,
    #[serde(rename = "PCRewardSheenDelay")]
    pc_reward_sheen_delay: f32,
    #[serde(rename = "PCLegendaryHitSprite")]
    pc_legendary_hit_sprite: LolTftTrovesTrovesPcSpriteAnimation,
    #[serde(rename = "PCRewardEpicGemPath")]
    pc_reward_epic_gem_path: String,
    #[serde(rename = "PCRewardLegendaryGemPath")]
    pc_reward_legendary_gem_path: String,
    #[serde(rename = "PCRewardMythicGemPath")]
    pc_reward_mythic_gem_path: String,
    #[serde(rename = "PCButtonText")]
    pc_button_text: String,
    #[serde(rename = "PCRewardOneStarPath")]
    pc_reward_one_star_path: String,
    #[serde(rename = "PCRewardThreeStarPath")]
    pc_reward_three_star_path: String,
    #[serde(rename = "PCThumbnailFadeInDelay")]
    pc_thumbnail_fade_in_delay: f32,
    #[serde(rename = "PCRewardSheenPath")]
    pc_reward_sheen_path: String,
    #[serde(rename = "PCGlintSprite")]
    pc_glint_sprite: LolTftTrovesTrovesPcSpriteAnimation,
    #[serde(rename = "pullSingleIndividualGlintLegendarySoundPath")]
    pull_single_individual_glint_legendary_sound_path: String,
    #[serde(rename = "PCLegendarySparkSprite")]
    pc_legendary_spark_sprite: LolTftTrovesTrovesPcSpriteAnimation,
    #[serde(rename = "PCRewardRareGemPath")]
    pc_reward_rare_gem_path: String,
    #[serde(rename = "PCRewardFadeInDelay")]
    pc_reward_fade_in_delay: f32,
    #[serde(rename = "revealMythicSoundPath")]
    reveal_mythic_sound_path: String,
    first_item_timing_offset: f32,
    #[serde(rename = "pullSingleIndividualGlintSoundPath")]
    pull_single_individual_glint_sound_path: String,
    inter_item_timing_offset: f32,
    #[serde(rename = "revealEpicSoundPath")]
    reveal_epic_sound_path: String,
}
type LolLobbyCollectionsRental = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsESportsLiveStreams {
    live_streams: Vec<LolEsportStreamNotificationsESportsStreams>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeTutorialPathMissionDisplay {
    attributes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootTFTMapSkinGroupedViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolLootTftMapSkinGroupedViewModel {
    selected_loadout_item: LolLootCosmeticsTftMapSkinViewModel,
    groups: Vec<LolLootTftMapSkinGroupViewModel>,
    default_item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubLoyaltyStatusNotification {
    status: LolEventHubLoyaltyStatus,
    rewards: LolEventHubLoyaltyRewardsSimplified,
    reload_inventory: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTbdInventory {
    initial_spell_ids: Vec<i32>,
    all_champion_ids: Vec<i32>,
    disabled_champion_ids: Vec<i32>,
    last_selected_skin_id_by_champion_id: Value,
    skin_ids: Vec<i32>,
    spell_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCapOrdersTypedIdentifierDto {
    id: String,
    type_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchScdCookie {
    domain: String,
    path: String,
    cookie: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferError {
    error_key: String,
    meta: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRegaliaLoadout {
    loadout: LolRegaliaRegaliaLoadout,
    scope: String,
    id: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLolTftPromoButton {
    show_timer_while_event_active: bool,
    event_asset_id: String,
    event_key: String,
    url: String,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassClientCacheClearMessageDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassClientCacheClearMessageDto {
    regions: Vec<String>,
    clear_all: bool,
    inventory_types: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryChampionMasteryGrade {
    champion_id: i32,
    grade: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectMySelection {
    ward_skin_id: i64,
    spell_2_id: u64,
    spell_1_id: u64,
    selected_skin_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTPlaybookViewModel")]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsCosmeticsTftPlaybookViewModel {
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "isDisabledInDoubleUp")]
    is_disabled_in_double_up: bool,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "iconPath")]
    icon_path: String,
    #[serde(rename = "isRecentItem")]
    is_recent_item: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "iconPathSmall")]
    icon_path_small: String,
    #[serde(rename = "earlyAugments")]
    early_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "midAugments")]
    mid_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "lateAugments")]
    late_augments: Vec<LolCosmeticsCosmeticsTftPlaybookAugment>,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "splashPath")]
    splash_path: String,
    #[serde(rename = "contentId")]
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionMinimal {
    title: String,
    id: i32,
    stinger_sfx_path: String,
    square_portrait_path: String,
    ownership: LolChampionsCollectionsOwnership,
    roles: Vec<String>,
    ban_vo_path: String,
    disabled_queues: Vec<String>,
    name: String,
    alias: String,
    base_load_screen_path: String,
    base_splash_path: String,
    active: bool,
    bot_enabled: bool,
    free_to_play: bool,
    choose_vo_path: String,
    purchased: u64,
    ranked_play_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubContentDropClientConfig {
    activation_date: String,
    patch: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesEligibility {
    restrictions: Vec<LolFeaturedModesEligibilityRestriction>,
    queue_id: i32,
    eligible: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoadoutsInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsInventoryDto {
    expires: String,
    items_jwt: String,
    puuid: String,
    summoner_id: u64,
    account_id: u64,
    items: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMatchmakingSearch {
    search_state: LolLobbyTeamBuilderMatchmakingSearchState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPremadeVoicePushToTalkKey {
    key: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyChatDto {
    muc_jwt_dto: LolLobbyMucJwtDto,
    jid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardHovercardUserInfo {
    patchline: String,
    lol: Value,
    remote_product_icon_url: String,
    remote_product: bool,
    summoner_id: u64,
    product_name: String,
    account_id: u64,
    mastery_score: u64,
    name: String,
    game_tag: String,
    remote_platform: bool,
    remote_product_backdrop_url: String,
    id: String,
    icon: i32,
    note: String,
    puuid: String,
    summoner_icon: i32,
    summoner_level: u32,
    availability: String,
    product: String,
    game_name: String,
    platform_id: String,
    party_summoners: Vec<String>,
    status_message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsInventoryItemWithPayload {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsSpell {
    spell_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "TournamentPhaseDTO")]
#[serde(rename_all = "camelCase")]
pub struct TournamentPhaseDto {
    cancelled: bool,
    id: i64,
    start_time: i64,
    limit_tiers: Vec<i32>,
    tournament_id: i64,
    period: i32,
    registration_time: i64,
    capacity_status: CapacityEnum,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersDataDto {
    purchaser: LolPurchaseWidgetCapOrdersTypedIdentifierDto,
    source: String,
    sub_orders: Vec<LolPurchaseWidgetCapOrdersSubOrderDto>,
    id: String,
    location: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyNotification {
    summoner_ids: Vec<u64>,
    notification_reason: String,
    timestamp: u64,
    notification_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerSession {
    summoner_id: u64,
    display_name: String,
    is_new_player: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubRegionLocale {
    region: String,
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashScoutingSeasonChampion {
    kda: String,
    kda_classification: LolClashKdaClassification,
    champion_id: i32,
    win_count: i32,
    win_rate: i32,
    game_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VendedHonorChange {
    reward: LolHonorV2Reward,
    dynamic_honor_message: LolHonorV2DynamicHonorMessage,
    current_state: LolHonorV2VendedHonorState,
    action_type: String,
    previous_state: LolHonorV2VendedHonorState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectPlayerSelection {
    spell_2_id: u64,
    assigned_position: String,
    player_type: String,
    obfuscated_puuid: String,
    champion_id: i32,
    champion_pick_intent: i32,
    puuid: String,
    ward_skin_id: i64,
    name_visibility_type: String,
    selected_skin_id: i32,
    cell_id: i64,
    obfuscated_summoner_id: u64,
    spell_1_id: u64,
    team: i32,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPremadeVoiceEntitlementsToken {
    entitlements: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesDataGdsResource {
    id: String,
    store_group_title: String,
    recipes: Vec<LolLootMilestonesRecipeGdsResource>,
    start_date: String,
    end_date: String,
    progress_track: LolLootProgressionConfigGdsResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "basicSystemInfo")]
#[serde(rename_all = "camelCase")]
#[doc = "User Experience Settings System Information"]
pub struct BasicSystemInfo {
    operating_system: BasicOperatingSystemInfo,
    physical_memory: u64,
    physical_processor_cores: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesLoginDataPacket {
    simple_messages: Vec<LolSimpleDialogMessagesSimpleMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubBundledItemPricingInfo {
    item_id: i32,
    inventory_type: String,
    quantity: i32,
    discount_prices: Vec<LolEventHubDiscountPricingInfo>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthEntitlementsToken {
    expiry: u64,
    entitlements: Vec<String>,
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitLinkingStatus {
    state: LolHoneyfruitHoneyfruitLinkingState,
    linked_account: String,
    error: LolHoneyfruitHoneyfruitLinkingStatusError,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRankedScoutingMember {
    player_id: u64,
    champion_scouting_data: Vec<LolClashRankedScoutingTopChampion>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolServiceStatusLegacyServiceStatusResponse {
    status: String,
    messages: Vec<LolServiceStatusLegacyServiceStatusMessage>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PendingRosterInviteeDTO")]
#[serde(rename_all = "camelCase")]
pub struct PendingRosterInviteeDto {
    inviter: u64,
    invite_type: InviteType,
    invite_time: i64,
    invitee_id: u64,
    invitee_state: PendingRosterInviteeState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInviter {
    previous_season_highest_tier: String,
    summoner_id: u64,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLobbyLcdsPartyRewardsConfig {
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolActiveBoostsLoyaltyStatusNotification {
    status: LolActiveBoostsLoyaltyStatus,
    rewards: LolActiveBoostsLoyaltyRewardsSimplified,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitGarenaRegionLeagueAccount {
    #[serde(rename = "summoner_icon_id")]
    summoner_icon_id: i32,
    #[serde(rename = "has_played_a_game")]
    has_played_a_game: bool,
    #[serde(rename = "summoner_name")]
    summoner_name: String,
    #[serde(rename = "platform_id")]
    platform_id: String,
    #[serde(rename = "summoner_level")]
    summoner_level: u32,
    #[serde(rename = "is_reserved_summoner_name")]
    is_reserved_summoner_name: bool,
    #[serde(rename = "garena_id")]
    garena_id: u64,
    #[serde(rename = "garena_puuid")]
    garena_puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardStrategy {
    select_min_group_count: u16,
    group_strategy: String,
    select_max_group_count: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardSummoner {
    puuid: String,
    summoner_level: u32,
    game_name: String,
    tag_line: String,
    display_name: String,
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LootProgressionRecipeConfiguration {
    deactivation_time: String,
    counter_uuid: String,
    active: bool,
    progression_uuid: String,
    recipe_name: String,
    activation_time: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerMissionRewardDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionRewardDto {
    reward_group_selected: bool,
    sequence: i32,
    icon_url: String,
    quantity: i32,
    reward_fulfilled: bool,
    small_icon_url: String,
    is_objective_based_reward: bool,
    media: Value,
    icon_needs_frame: bool,
    unique_name: String,
    description: String,
    item_id: String,
    reward_type: String,
    reward_group: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThemeVp {
    theme_id: i32,
    theme_vp: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ClientConfigEntitlementsUpdate {
    update_type: ClientConfigUpdateType,
    entitlements_token_resource: ClientConfigEntitlements,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampionsCollectionsChampionSkinAugmentOverlays {
    #[serde(rename = "socialCardLCOverlayPath")]
    social_card_lc_overlay_path: String,
    #[serde(rename = "centeredLCOverlayPath")]
    centered_lc_overlay_path: String,
    #[serde(rename = "uncenteredLCOverlayPath")]
    uncentered_lc_overlay_path: String,
    #[serde(rename = "tileLCOverlayPath")]
    tile_lc_overlay_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHovercardContentCookies {
    #[serde(rename = "content_path")]
    content_path: String,
    #[serde(rename = "cookies")]
    cookies: Vec<LolHovercardcookie>,
    #[serde(rename = "content_id")]
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSettingCategory {
    schema_version: i32,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGameTeam {
    is_winning_team: bool,
    full_id: String,
    team_id: i32,
    member_status_string: String,
    name: String,
    tag: String,
    is_player_team: bool,
    players: Vec<LolEndOfGameEndOfGamePlayer>,
    stats: Value,
    is_bottom_team: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchPatcherSettings {
    #[serde(rename = "channel")]
    channel: String,
    #[serde(rename = "self_update")]
    self_update: LolPatchPatcherSelfUpdateSettings,
    #[serde(rename = "product_refresh_period")]
    product_refresh_period: f64,
    #[serde(rename = "headers")]
    headers: Value,
    #[serde(rename = "patchsieve_url")]
    patchsieve_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstoneSet {
    name: String,
    statstones: Vec<LolStatstonesGameDataStatstone>,
    price: i32,
    inventory_type: String,
    content_id: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventoryInventoryItemDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolInventoryInventoryItemDto {
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "usedInGameDate")]
    used_in_game_date: String,
    #[serde(rename = "instanceTypeId")]
    instance_type_id: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "entitlementId")]
    entitlement_id: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "entitlementTypeId")]
    entitlement_type_id: String,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "lsb")]
    lsb: bool,
    #[serde(rename = "ownedQuantity")]
    owned_quantity: u64,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "quantity")]
    quantity: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerSummonerRequestedName {
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TracingPhaseBeginV1 {
    name: String,
    importance: TracingPhaseImportanceV1,
    when: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatPresenceProduct {
    product: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "MissionAlertDTO")]
#[serde(rename_all = "camelCase")]
pub struct MissionAlertDto {
    message: String,
    _type: String,
    alert_time: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSimpleDialogMessagesSimpleMessage {
    msg_id: String,
    params: Vec<String>,
    _type: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReporterFeedback {
    account_id: u64,
    message_id: String,
    id: u64,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "ChampSelectLcdsGameTimerDTO")]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsGameTimerDto {
    current_game_state: String,
    remaining_time_in_millis: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolKickoutKickoutMessage {
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigEdge {
    client_region: String,
    client_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTSettingsDataResource")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftSettingsDataResource {
    icon_override: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesFriendLevelsData {
    level: String,
    friends: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatProductMetadataMap {
    products: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginItemWithDetails {
    quantity: u32,
    bundled_items: Vec<LolCatalogCatalogPluginItemWithDetails>,
    required_items: Vec<LolCatalogCatalogPluginItemWithDetails>,
    metadata: Vec<LolCatalogItemMetadataEntry>,
    bundled_discount_prices: Vec<LolCatalogCatalogPluginPrice>,
    item: LolCatalogCatalogPluginItem,
    assets: LolCatalogCatalogPluginItemAssets,
    minimum_bundle_prices: Vec<LolCatalogCatalogPluginPrice>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantFrame {
    level: u16,
    dominion_score: u16,
    jungle_minions_killed: u16,
    xp: u32,
    current_gold: i32,
    position: LolMatchHistoryMatchHistoryPosition,
    team_score: u16,
    minions_killed: u16,
    participant_id: u16,
    total_gold: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStatstonesStatstoneSetCompleteVignette {
    statstones: Vec<LolStatstonesStatstoneCompletion>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyRankedRestrictionInfo {
    punished_games_remaining: i32,
    needs_ack: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolVanguardVanguardSystemCheckTelemetryEvent {
    passed_os_check: bool,
    passed_secure_features_check: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreServiceBalance {
    currency: String,
    amount: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardPack {
    index: i32,
    _type: String,
    unlock_time: i64,
    state: String,
    minor_rewards: Vec<LolNpeRewardsReward>,
    reward_key: String,
    major_reward: LolNpeRewardsReward,
    delay: i64,
    requirements: LolNpeRewardsRequirements,
    premium_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopPurchaseOfferOrderStatuses {
    statuses: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCosmeticsGameDataCompanion {
    #[serde(rename = "contentId")]
    content_id: String,
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "level")]
    level: u32,
    #[serde(rename = "rarityValue")]
    rarity_value: u32,
    #[serde(rename = "upgrades")]
    upgrades: Vec<String>,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "TFTOnly")]
    tft_only: bool,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "colorName")]
    color_name: String,
    #[serde(rename = "speciesName")]
    species_name: String,
    #[serde(rename = "speciesId")]
    species_id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopPromotion {
    start_time: String,
    name: String,
    end_time: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2VoteCompletion {
    game_id: u64,
    full_team_vote: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationResource {
    password: String,
    _type: String,
    game_tag: String,
    inviter_id: String,
    is_muted: bool,
    last_message: LolChatConversationMessageResource,
    muc_jwt_dto: LolChatMucJwtDto,
    game_name: String,
    unread_message_count: u64,
    name: String,
    target_region: String,
    pid: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventSeriesOpt {
    option: String,
    series_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRewardSeries {
    reward_packs: Vec<LolNpeRewardsRewardPack>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectBannedChampions {
    num_bans: i32,
    my_team_bans: Vec<i32>,
    their_team_bans: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRankedQueue {
    _type: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksLobbyDto {
    local_member: LolPerksLobbyParticipantDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginPrice {
    cost: i64,
    sale: LolCatalogCatalogPluginRetailDiscount,
    currency: String,
    cost_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersReportedPlayer {
    reported_summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummoner {
    display_name: String,
    percent_complete_for_next_level: u32,
    summoner_id: u64,
    profile_icon_id: i32,
    reroll_points: LolSummonerSummonerRerollPoints,
    summoner_level: u32,
    privacy: LolSummonerProfilePrivacySetting,
    puuid: String,
    xp_since_last_level: u64,
    game_name: String,
    xp_until_next_level: u64,
    unnamed: bool,
    internal_name: String,
    account_id: u64,
    tag_line: String,
    name_change_flag: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTbRemovedFromServiceNotification {
    reason: String,
    backwards_transition_info: LolLobbyTeamBuilderBackwardsTransitionInfoV1,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsJoinOrCreatePracticeGameRequestDto {
    player_gco_tokens: LcdsPlayerGcoTokens,
    practice_game_config: LcdsPracticeGameConfig,
    simple_inventory_jwt: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatNameBody {
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSvcRewardGrant {
    reward_group_id: String,
    message_parameters: Value,
    viewed: bool,
    grant_elements: Vec<LolEventHubSvcRewardGrantElement>,
    grantee_id: String,
    grantor_description: LolEventHubGrantorDescription,
    date_created: String,
    status: LolEventHubGrantStatus,
    selected_ids: Vec<String>,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPftGameflowSession {
    phase: LolPftGameflowPhase,
    game_dodge: LolPftGameflowGameDodge,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetBundledItemPricingInfo {
    item_id: i32,
    quantity: i32,
    discount_prices: Vec<LolPurchaseWidgetDiscountPricingInfo>,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPlayer {
    summoner_id: u64,
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPremadeVoiceGameInputSettings {
    game_events: LolPremadeVoiceGameEventHotkeys,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubExternalCatalogPluginRetailDiscount {
    discount: f32,
    cost: i64,
    start_date: String,
    end_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStorePageDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolStorePageDto {
    #[serde(rename = "itemGroups")]
    item_groups: Value,
    #[serde(rename = "catalog")]
    catalog: Vec<LolStoreCatalogItem>,
    #[serde(rename = "groupOrder")]
    group_order: Vec<String>,
    player: LolStorePlayer,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventRewardGroupsSelection {
    reward_groups: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesCapDropsDropTableDisplayMetadata {
    name_tra_key: String,
    tables: Value,
    progression_id: String,
    mythic_offer_id: String,
    priority: u8,
    chase_content_id: String,
    odds_tree: LolTftTrovesDropsOddsTreeNodeDto,
    version: u8,
    is_collectors_bounty: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftEventLolTftBattlePassHub {
    #[serde(rename = "battlePassXPBoosted")]
    battle_pass_xp_boosted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassRmsStoreEntitlementItem {
    inventory_type: String,
    item_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPremadeVoiceDeviceResource {
    #[serde(rename = "handle")]
    handle: String,
    #[serde(rename = "is_current_device")]
    is_current_device: bool,
    #[serde(rename = "is_default")]
    is_default: bool,
    #[serde(rename = "usable")]
    usable: bool,
    #[serde(rename = "name")]
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStatstonesCatalogBundle {
    item: LolStatstonesCatalogItemDetails,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubExternalCatalogPluginPrice {
    cost_type: String,
    cost: i64,
    currency: String,
    sale: LolEventHubExternalCatalogPluginRetailDiscount,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolStoreAliasDetail {
    #[serde(rename = "field")]
    field: String,
    #[serde(rename = "new_value")]
    new_value: String,
    #[serde(rename = "old_value")]
    old_value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatConversationJoinFederated {
    domain: String,
    hidden: bool,
    _type: String,
    id: String,
    password: String,
    target_region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAlias {
    tag_line: String,
    game_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "EndOfGameLcdsRawStatDTO")]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsRawStatDto {
    value: i64,
    stat_type_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreItemSale {
    item: LolStoreItemKey,
    active: bool,
    id: u64,
    sale: LolStoreSale,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsSummoner {
    summoner_level: u32,
    puuid: String,
    profile_icon_id: i32,
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyEligibilityRestriction {
    summoner_ids_string: String,
    restriction_code: LolLobbyEligibilityRestrictionCode,
    expired_timestamp: u64,
    restriction_args: Value,
    summoner_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGameflowGameData {
    queue: LolPerksQueue,
    is_custom_game: bool,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc = "Describes an event."]
pub struct BindingFullEventHelp {
    name_space: String,
    description: String,
    tags: Vec<String>,
    _type: BindingFullTypeIdentifier,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPubHubConfigAppContext {
    app_language: String,
    app_id: String,
    device_operating_system: String,
    app_locale: String,
    user_region: String,
    user_id: String,
    device_operating_system_version: String,
    publishing_locale: String,
    device_category: String,
    app_session_id: String,
    app_version: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatGameflowGameData {
    team_two: Vec<LolChatTeamPlayerEntry>,
    player_champion_selections: Vec<LolChatChampSelection>,
    game_id: u64,
    team_one: Vec<LolChatTeamPlayerEntry>,
    queue: LolChatQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetCapOrdersOrderDto {
    meta: LolPurchaseWidgetCapOrdersMetaDto,
    data: LolPurchaseWidgetCapOrdersDataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationConfirmActivationPinResponse {
    data: LolAccountVerificationPinResponseData,
    client_message_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorialMetadata {
    use_chosen_champion: bool,
    queue_id: String,
    display_rewards: Value,
    step_number: i32,
    use_quick_search_matchmaking: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubContentDrop {
    offers: Vec<LolEventHubOffer>,
    activation_date: String,
    patch: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRsoAuthRSOConfigReadyState")]
#[serde(rename_all = "kebab-case")]
pub struct LolRsoAuthRsoConfigReadyState {
    ready: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLoginSession {
    summoner_id: u64,
    username: String,
    account_id: u64,
    id_token: String,
    connected: bool,
    is_in_login_queue: bool,
    state: LolLoginLoginSessionStates,
    user_auth_token: String,
    error: LolLoginLoginError,
    puuid: String,
    is_new_player: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectLegacyChampionSelectPreferences {
    spells: Value,
    skins: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyPlayerNotification {
    detail_key: String,
    source: String,
    title_key: String,
    _type: String,
    icon_url: String,
    state: String,
    critical: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsAllSeasonsProduct {
    metadata: LolSeasonsSeasonMetaData,
    season_end: i64,
    act: bool,
    season_id: i32,
    season_start: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopXboxSubscriptionStatus {
    active: String,
    subscription_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathSummoner {
    profile_icon_id: i32,
    unnamed: bool,
    xp_since_last_level: u64,
    summoner_id: u64,
    summoner_level: u32,
    name_change_flag: bool,
    percent_complete_for_next_level: u32,
    display_name: String,
    account_id: u64,
    xp_until_next_level: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHonorV2LoginSession {
    state: LolHonorV2LoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataStatstonePack {
    content_id: String,
    description: String,
    item_id: i32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyTeamBuilderCeremonyV1 {
    duration: i64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampionsGameDataChampionSpell {
    description: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsUserInfo {
    user_info: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpLoginSession {
    summoner_id: u64,
    state: LolPlayerLevelUpLoginSessionStates,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsChampionQuestSkinInfo {
    tiers: Vec<LolChampionsCollectionsChampionQuestSkin>,
    tile_path: String,
    product_type: LolChampionsQuestSkinProductType,
    uncentered_splash_path: String,
    description_info: Vec<LolChampionsQuestSkinDescriptionInfo>,
    collection_description: String,
    name: String,
    collection_card_path: String,
    splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectLegacyCollectionsRental {
    rented: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatTranslateResult {
    #[serde(rename = "key")]
    key: String,
    #[serde(rename = "product_id")]
    product_id: String,
    #[serde(rename = "found")]
    found: bool,
    #[serde(rename = "value")]
    value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolReplaysGameflowAvailability {
    state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoicePartyDto {
    party_id: String,
    players: Value,
    comms_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaInventoryItem {
    item_id: i32,
    uuid: String,
    purchase_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataStatstonesInfo {
    series_id_to_stat_stone_ids: Value,
    pack_id_to_champ_ids: Value,
    champ_id_to_pack_ids: Value,
    pack_data: Vec<LolCatalogGameDataStatstonePack>,
    statstone_data: Vec<LolCatalogGameDataStatstoneSet>,
    pack_id_to_sub_pack_ids: Value,
    pack_id_to_stat_stones_ids: Value,
    collection_id_to_stat_stone_ids: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolSummonerAliasLookupDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolSummonerAliasLookupDto {
    #[serde(rename = "tag_line")]
    tag_line: String,
    #[serde(rename = "game_name")]
    game_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadUrlRequestV2 {
    platform_id: String,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Represents the parameters of a call to a provided callback."]
pub struct BindingCallbackEvent {
    parameters: Vec<Value>,
    id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterRankedRestrictionGamesUpdate {
    punished_games_remaining: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryNotification {
    _type: String,
    id: i64,
    item_id: i32,
    inventory_type: String,
    acknowledged: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowInstallPaths {
    game_install_root: String,
    game_executable_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetTransaction {
    item_key: LolPurchaseWidgetItemKey,
    icon_url: String,
    transaction_id: String,
    item_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosRewardGroupsConfig {
    reward_groups: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSvcReward {
    item_id: String,
    quantity: i32,
    fulfillment_source: String,
    id: String,
    media: Value,
    localizations: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MissionDisplay {
    attributes: Vec<String>,
    locations: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeThreshold {
    rewards: Vec<LolChallengesChallengeThresholdReward>,
    value: f64,
    reward_group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameChampionMasteryGrade {
    champion_id: i32,
    grade: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequestAdd {
    game_name: String,
    note: String,
    puuid: String,
    tag_line: String,
    region: String,
    pid: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventShopClientConfig {
    content_drops: Vec<LolEventHubContentDropClientConfig>,
    disabled_offer_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPurchaseWidgetPurchaseResponse {
    #[serde(rename = "useRMSConfirmation")]
    use_rms_confirmation: bool,
    #[serde(rename = "items")]
    items: Vec<LolPurchaseWidgetPurchaseItem>,
    #[serde(rename = "transactions")]
    transactions: Vec<LolPurchaseWidgetTransaction>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolBannersBannerFrame {
    level: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerGameloopPlayerInfoV2 {
    auto_fill_data_bags: Vec<LolSummonerAutoFillQueueDto>,
    reroll_data_bags: Vec<LolSummonerRerollDataBagForClientV1>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationSendActivationPinResponseData {
    pin_expires_at_epoch_millis: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClashOfflineNotification {
    tournament_id: i64,
    meta_data: Value,
    reason: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEmailVerificationAccessToken {
    token: String,
    expiry: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryGameList {
    game_begin_date: String,
    game_index_begin: u64,
    game_count: u64,
    game_index_end: u64,
    game_end_date: String,
    games: Vec<LolMatchHistoryMatchHistoryGame>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassCounter {
    name: String,
    direction: String,
    group_id: String,
    start_value: i64,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTastesDataModelResponse {
    response_code: i64,
    model_data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "MissionProgressDTO")]
#[serde(rename_all = "camelCase")]
pub struct MissionProgressDto {
    total_count: i32,
    last_viewed_progress: i32,
    current_progress: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectTradeContract {
    cell_id: i64,
    state: LolPerksChampSelectTradeState,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubCapOfferPayloadEntry {
    #[serde(rename = "itemPriceMap")]
    item_price_map: Value,
    #[serde(rename = "inventoryTypeUUID")]
    inventory_type_uuid: String,
    #[serde(rename = "fulfillmentTypeId")]
    fulfillment_type_id: String,
    #[serde(rename = "itemInstanceId")]
    item_instance_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LolInventoryEndOfGameXp {
    per_win: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LCDSLoyaltyStateChangeNotification")]
#[serde(rename_all = "camelCase")]
pub struct LcdsLoyaltyStateChangeNotification {
    notification_category: LcdsLoyaltyStateChangeNotificationCategory,
    rewards: LcdsLoyaltyRewards,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGameDataSummonerEmote {
    description: String,
    inventory_icon: String,
    id: i64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyGameConfigDto {
    custom_spectator_policy: LolLobbyQueueCustomGameSpectatorPolicy,
    custom_spectators: Vec<LolLobbyLobbyParticipantDto>,
    pick_type: String,
    is_custom: bool,
    should_force_scarce_position_selection: bool,
    queue_id: i32,
    custom_lobby_name: String,
    game_mode: String,
    show_position_selector: bool,
    custom_team_100: Vec<LolLobbyLobbyParticipantDto>,
    allowable_premade_sizes: Vec<i32>,
    is_lobby_full: bool,
    premade_size_allowed: bool,
    max_human_players: i32,
    max_lobby_size: i32,
    is_team_builder_managed: bool,
    show_quick_play_slot_selection: bool,
    custom_team_200: Vec<LolLobbyLobbyParticipantDto>,
    custom_rewards_disabled_reasons: Vec<String>,
    map_id: i32,
    max_team_size: i32,
    custom_mutator_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsMissionAsset {
    icon_needs_frame: bool,
    path: String,
    internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseOfferRequestV3 {
    offer_id: String,
    currency_type: String,
    price: u32,
    quantity: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolStoreBundleItemDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundleItemDto {
    description: String,
    inventory_type: String,
    owned: bool,
    item_id: i32,
    name: String,
    rp: i64,
    icon_url: String,
    quantity: u32,
    ip: i64,
    discounted_rp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionSelection {
    pick_intented_by_me: bool,
    ban_intented_by_me: bool,
    ban_intented: bool,
    pick_intented_position: String,
    picked_by_other_or_banned: bool,
    selected_by_me: bool,
    pick_intented: bool,
    is_banned: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryAcsPlayer {
    platform_id: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPftLoginSession {
    id_token: String,
    state: LolPftLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyMember {
    is_owner: bool,
    auto_fill_eligible: bool,
    position_preferences: LolLobbyTeamBuilderLobbyPositionPreferences,
    excluded_position_preference: String,
    auto_fill_protected_for_streaking: bool,
    auto_fill_protected_for_soloing: bool,
    can_invite_others: bool,
    auto_fill_protected_for_promos: bool,
    show_position_excluder: bool,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHovercardTopChampionMastery {
    score: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCounter {
    group_id: String,
    id: String,
    start_value: i64,
    direction: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogCatalogPluginRetailDiscount {
    cost: i64,
    start_date: String,
    end_date: String,
    discount: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesCelebrationCurrencySegmentData {
    multi_pull_sound_path: String,
    lottie_json_path: String,
    mythic_pull_sound_path: String,
    single_pull_sound_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsRequirements {
    level: u32,
    day: u32,
    mission_internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampionSkinAugment {
    overlays: Vec<LolChampSelectChampionSkinAugmentOverlays>,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesNumberFormattingData {
    percentage_format: String,
    hour_abbreviation: String,
    decimal_seperator: String,
    second_abbreviation: String,
    one_hundred_million_abbreviation: String,
    trillion_abbreviation: String,
    meters_abbreviation: String,
    thousand_seperator: String,
    million_abbreviation: String,
    kilometers_abbreviation: String,
    number_formatting_behavior: LolStatstonesNumberFormattingBehavior,
    ten_thousand_abbreviation: String,
    thousand_abbreviation: String,
    minute_abbreviation: String,
    billion_abbreviation: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubCategoryOffersUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCategoryOffersUiData {
    category_icon_path: String,
    category: LolEventHubOfferCategory,
    offers: Vec<LolEventHubOfferUiData>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTeamPlannerTFTMapSetData")]
#[serde(rename_all = "PascalCase")]
pub struct LolTftTeamPlannerTftMapSetData {
    set_core_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolActiveBoostsInventoryItemWithPayload {
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "ownershipType")]
    ownership_type: LolActiveBoostsItemOwnershipType,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "uuid")]
    uuid: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolSuggestedPlayersSuggestedPlayersDynamicClientConfig {
    suggested_players: LolSuggestedPlayersSuggestedPlayersConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentUpdatedNotification {
    retry_seconds: i64,
    current_retry: i32,
    missing_player_ids: Vec<i64>,
    max_retry: i32,
    notify_reason: LolClashRosterNotifyReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTradeContract {
    state: LolChampSelectChampSelectTradeState,
    id: i64,
    cell_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerPlatformDelta {
    ip_delta: u64,
    timestamp: u64,
    xp_delta: u64,
    compensation_mode_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCollectionsGameDataChampionSummary {
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolHoneyfruitGAMHSMatchHistoryList")]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitGamhsMatchHistoryList {
    #[serde(rename = "games")]
    games: Vec<LolHoneyfruitGamhsMatchHistoryData>,
    #[serde(rename = "active_puuid")]
    active_puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsGameDataTFTMapSkin")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsGameDataTftMapSkin {
    group_name: String,
    item_id: i32,
    loadouts_icon: String,
    name: String,
    description: String,
    group_id: u32,
    rarity_value: u32,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootReward {
    id: String,
    item_id: String,
    item_type: String,
    fulfillment_source: String,
    media: Value,
    localizations: Value,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootOddsDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootOddsDto {
    loot_id: String,
    drop_rate: f64,
    quantity: i32,
    label: String,
    parent_id: String,
    loot_order: i32,
    children: Vec<LootOddsDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentPublishingLocaleSettingData {
    publishing_locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolItemSetsValidateItemSetNameInput {
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootLcdsRecipeOutputDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeOutputDto {
    loot_name: String,
    quantity_expression: String,
    allow_duplicates: bool,
    probability: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstone {
    content_id: String,
    icon_lit: String,
    milestones: Vec<u32>,
    description: String,
    bound_champion: LolStatstonesGameDataItemReference,
    icon_full: String,
    name: String,
    icon_unlit: String,
    is_epic: bool,
    icon_unowned: String,
    is_retired: bool,
    category: String,
    tracking_type: u32,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterPeriodAggregatedStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterPeriodAggregatedStatsDto {
    time: i64,
    player_bids: Value,
    match_stats: Vec<RosterMatchAggregatedStatsDto>,
    period: i32,
    bracket_size: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemOwnership {
    quantity: i32,
    item_key: LolPurchaseWidgetItemKey,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubInventoryItemWithPayload {
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "uuid")]
    uuid: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "ownershipType")]
    ownership_type: LolEventHubItemOwnershipType,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyPartyRewards {
    queue_id: i32,
    is_custom: bool,
    party_rewards: Vec<LolLobbyPartyReward>,
    is_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyCountdownTimer {
    counter: i32,
    timer: i64,
    phase_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaSettingsExternal {
    banner_type: i32,
    selected_prestige_crest: u8,
    crest_type: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstonePlayerRecord {
    value: u32,
    personal_best: u32,
    milestone_level: u32,
    date_modified: String,
    date_archived: String,
    date_completed: String,
    statstone_id: String,
    puuid: String,
    entitled: bool,
    date_acquired: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootOddsResponse {
    loot_id: String,
    children: Vec<LolLootLootOddsResponse>,
    drop_rate: f64,
    parent_id: String,
    label: String,
    query: String,
    loot_order: i32,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingGameflowGameDodge {
    dodge_ids: Vec<u64>,
    state: LolMatchmakingMatchmakingDodgeState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeTutorialPathGameflowSession {
    phase: LolNpeTutorialPathGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PaymentsPMCStartUrlRequest")]
#[serde(rename_all = "camelCase")]
pub struct PaymentsPmcStartUrlRequest {
    summoner_level: i16,
    giftee_message: String,
    game: String,
    is_prepaid: bool,
    giftee_account_id: String,
    locale_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootItemGdsResource {
    mapped_store_id: i32,
    lifetime_max: i32,
    _type: LolLootLootType,
    id: String,
    image: String,
    name: String,
    recipe_menu_active: bool,
    start_date: String,
    recipe_menu_title: String,
    auto_redeem: bool,
    rarity: LolLootLootRarity,
    end_date: String,
    description: String,
    recipe_menu_subtitle: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeTitleData {
    challenge_id: i64,
    level_to_icon_path: Value,
    challenge_description: String,
    level: String,
    challenge_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionMilestoneInstance {
    trigger_value: i64,
    product_id: String,
    owner_id: String,
    triggered: bool,
    group_id: String,
    triggered_timestamp: String,
    counter_id: String,
    instance_id: String,
    repeat_sequence: u32,
    triggers: Vec<LolProgressionTrigger>,
    milestone_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathAccountSettingsCategoryResource {
    schema_version: u32,
    data: LolNpeTutorialPathAccountSettingsTutorial,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentRegionLocale {
    region: String,
    web_region: String,
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolContentTargetingSettingsResource {
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingQueue {
    category: LolContentTargetingQueueGameCategory,
    game_mode: String,
    id: i32,
    map_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLolTftPrimeGaming {
    url: String,
    asset_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsReplaysConfig {
    replay_service_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SLIStringDiagnostic")]
#[serde(rename_all = "kebab-case")]
pub struct SliStringDiagnostic {
    value: String,
    key: String,
}
type LolRewardsResponseMetadataDto = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMissionsGameflowSession {
    phase: LolMissionsGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksMucJwtDto {
    domain: String,
    jwt: String,
    target_region: String,
    channel_claim: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomGameLobby {
    team_one: Vec<LolLobbyLobbyMember>,
    configuration: LolLobbyLobbyCustomGameConfiguration,
    lobby_name: String,
    practice_game_rewards_disabled_reasons: Vec<String>,
    lobby_password: String,
    spectators: Vec<LolLobbyLobbyMember>,
    team_two: Vec<LolLobbyLobbyMember>,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChampSelectLegacyGameflowGameData {
    queue: LolChampSelectLegacyQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowLoginSession {
    connected: bool,
    summoner_id: u64,
    state: LolGameflowLoginSessionStates,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectSwapContract {
    cell_id: i64,
    state: LolLobbyTeamBuilderChampSelectSwapState,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksSummonerRerollPoints {
    points_cost_to_roll: u32,
    number_of_rolls: u32,
    current_points: u32,
    points_to_reroll: u32,
    max_rolls: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftQueue {
    map_id: i32,
    category: LolTftQueueGameCategory,
    id: i32,
    game_mode: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolDropsCapDropsOddsListEntryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsOddsListEntryDto {
    content_id: String,
    node_id: String,
    odds: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesDropsOddsTreeNodeDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesDropsOddsTreeNodeDto {
    node_id: String,
    priority: u8,
    children: Vec<LolTftTrovesDropsOddsTreeNodeDto>,
    odds: f32,
    name_tra_key: String,
    quantity: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueRestrictionDto {
    gatekeeper_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoyaltyInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolLoyaltyInventoryResponseDto {
    data: LolLoyaltyInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowRegistrationStatus {
    complete: bool,
    error_codes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubSimpleInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSimpleInventoryDto {
    items_jwt: String,
    expires: String,
    items: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderSummoner {
    summoner_id: u64,
    xp_until_next_level: u64,
    puuid: String,
    summoner_level: u32,
    xp_since_last_level: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBehaviorLcdsSimpleMessage {
    account_id: u64,
    params: Vec<String>,
    msg_id: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMissionsTftWeeklyMissions {
    missions: Vec<PlayerMissionDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameMucJwtDto {
    domain: String,
    jwt: String,
    channel_claim: String,
    target_region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderMucJwtDto {
    domain: String,
    jwt: String,
    channel_claim: String,
    target_region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationValidationStatus {
    email_status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGrant {
    reward_group: LolMissionsRewardGroup,
    info: LolMissionsRewardGrantInfo,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPlayerBehaviorUserInfoToken {
    ban: LolPlayerBehaviorUserInfoBanData,
}
type LolYourshopRmsWalletPayload = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHovercardPartyInfo {
    summoners: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootCosmeticsTFTMapSkinViewModel")]
#[serde(rename_all = "PascalCase")]
pub struct LolLootCosmeticsTftMapSkinViewModel {
    #[serde(rename = "loadoutsIcon")]
    loadouts_icon: String,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "selected")]
    selected: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "description")]
    description: String,
    #[serde(rename = "name")]
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsLoginSession {
    summoner_id: u64,
    account_id: u64,
    state: LolItemSetsLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsGameDataChampionTacticalInfo {
    difficulty: u32,
    damage_type: String,
    style: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCounter {
    group_id: String,
    id: String,
    direction: String,
    start_value: i64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingAccountIdAndSummonerId {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPlayerStatSummary {
    player_stat_summary_type: String,
    wins: i32,
    games_played: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesUserResource {
    lol: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyAccountIdAndSummonerId {
    summoner_id: u64,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreLoginSession {
    account_id: u64,
    id_token: String,
    summoner_id: u64,
    state: LolStoreLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSummoner {
    acct_id: u64,
    sum_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassXboxSubscriptionStatus {
    subscription_id: String,
    active: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPaidBattlepass {
    last_viewed_milestone: LolTftPassTftPaidBattlepassMilestone,
    active_milestone: LolTftPassTftPaidBattlepassMilestone,
    info: LolTftPassTftPaidBattlepassInfo,
    milestones: Vec<LolTftPassTftPaidBattlepassMilestone>,
    bonuses: Vec<LolTftPassTftPaidBattlepassMilestone>,
    total_points_earned: i32,
    last_viewed_progress: i32,
    current_level: i32,
    progress_mission_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesRollRequest {
    number_of_rolls: u32,
    is_mythic: bool,
    drop_table_id: String,
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubPurchaseOrderResponseDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseOrderResponseDto {
    ip_balance: i64,
    rp_balance: i64,
    transactions: Vec<LolEventHubTransactionResponseDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectMucJwtDto {
    target_region: String,
    jwt: String,
    domain: String,
    channel_claim: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRewardsRMSPayload")]
#[serde(rename_all = "camelCase")]
pub struct LolRewardsRmsPayload {
    affinities: Vec<String>,
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchEntitlementsTokenResource {
    entitlements: Vec<String>,
    token: String,
    access_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGameflowSession {
    phase: LolPerksGameflowPhase,
    game_data: LolPerksGameflowGameData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerStatus {
    ready: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolInventoryWallet {
    ip: i64,
    rp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatMultiGamePresenceUpdate {
    private: String,
    shared_jwt: String,
    shared: LolChatMultiGamePresenceSharedPayload,
    msg: String,
    state: LolChatAccountState,
    private_jwt: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterLeaverBusterEntryDto {
    tainted: bool,
    pre_lockout_ack_needed: bool,
    warn_acked_millis: i64,
    leaver_score: i32,
    on_lockout_ack_needed: bool,
    puuid: String,
    leaver_level: i32,
    warn_sent_millis: i64,
    leaver_penalty: LolLeaverBusterLeaverBusterPenaltyResponse,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryPlayerDelta {
    deltas: Vec<LolMatchHistoryMatchHistoryPlayerGameDelta>,
    original_account_id: u64,
    original_platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerSanitizerStatus {
    locale: String,
    projected_chars_count: u32,
    breaking_chars_count: u32,
    ready: bool,
    region: String,
    filtered_word_counts_by_level: Value,
    whitelisted_word_counts_by_level: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingDynamicCelebrationMessagingNotificationResource {
    account_id: u64,
    celebration_message: String,
    celebration_body: String,
    item_id: String,
    celebration_type: String,
    id: i32,
    msg_id: String,
    inventory_type: String,
    status: i32,
    item_quantity: String,
    celebration_title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionCounter {
    group_id: String,
    id: String,
    name: String,
    direction: String,
    start_value: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMemberBanUnbanNotification {
    notify_puuid: String,
    player_id: i64,
    notify_player_id: i64,
    tournaments: Vec<MemberBanUnbanTournament>,
    notify_reason: LolClashNotifyReason,
    reason: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSvcRewardGrantElement {
    quantity: i32,
    fulfillment_source: String,
    item_id: String,
    item_type: String,
    element_id: String,
    status: LolEventHubRewardStatus,
    localizations: Value,
    media: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyNotificationEnvelopeDto {
    player: LolLobbyPlayerDto,
    queue_restriction: LolLobbyQueueRestrictionDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathCollectionsChampion {
    stinger_sfx_path: String,
    spells: Vec<LolNpeTutorialPathCollectionsChampionSpell>,
    choose_vo_path: String,
    roles: Vec<String>,
    id: i32,
    passive: LolNpeTutorialPathCollectionsChampionSpell,
    alias: String,
    square_portrait_path: String,
    ban_vo_path: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPublishingContentClientData {
    #[serde(rename = "env")]
    env: String,
    #[serde(rename = "system_os")]
    system_os: String,
    #[serde(rename = "port")]
    port: u16,
    #[serde(rename = "summoner_level")]
    summoner_level: u16,
    #[serde(rename = "protocol")]
    protocol: String,
    #[serde(rename = "account_id")]
    account_id: u64,
    #[serde(rename = "assetUrls")]
    asset_urls: Value,
    #[serde(rename = "app_version")]
    app_version: String,
    #[serde(rename = "locale")]
    locale: String,
    #[serde(rename = "puuid")]
    puuid: String,
    #[serde(rename = "web_region")]
    web_region: String,
    #[serde(rename = "app_name")]
    app_name: String,
    #[serde(rename = "summoner_name")]
    summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneNotificationEnvelopeDto {
    game_id: u64,
    updates: Vec<LolStatstonesStatstoneNotificationDto>,
    milestones: Vec<LolStatstonesMilestoneNotificationDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLolTftEvent {
    title_translation_key: String,
    enabled: bool,
    queue_ids: Vec<i32>,
    default_landing_page: bool,
    url: String,
    url_faq: String,
    start_date: String,
    end_date: String,
    series_id: String,
    event_hub_template_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBehaviorLcdsForcedClientShutdown {
    additional_info: String,
    reason: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsEmote {
    ownership_type: LolLootInventoryOwnership,
    item_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashClashSettingCategory {
    simple_state_flag_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubPurchaseItem {
    quantity: i32,
    source: String,
    item_key: LolEventHubItemKey,
    purchase_currency_info: LolEventHubItemPrice,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TracingModuleV1 {
    threading_model: TracingModuleThreadingModelV1,
    _type: TracingModuleTypeV1,
    module_id: u32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSelectionStrategyConfig {
    min_selections_allowed: u32,
    max_selections_allowed: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTSurveyResults")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftSurveyResults {
    actions: Vec<LolPftPftEvent>,
    question_responses: Vec<LolPftPftQuestionResponse>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventoryClientCacheClearMessageDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryClientCacheClearMessageDto {
    regions: Vec<String>,
    clear_all: bool,
    inventory_types: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedSeasonSplitDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSeasonSplitDto {
    reward_track: Vec<LolRankedSplitRewardGroupDto>,
    end_time: u64,
    split_id: i32,
    season_id: i32,
    victorious_skin_reward_group: LolRankedVictoriousSkinDto,
    start_time: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogPluginItem {
    sub_inventory_type: String,
    active: bool,
    image_path: String,
    item_id: i32,
    purchase_date: u64,
    description: String,
    inventory_type: String,
    quest_skin_info: LolEventHubSkinLineInfo,
    tags: Vec<String>,
    metadata: Vec<LolEventHubItemMetadataEntry>,
    item_instance_id: String,
    ownership_type: LolEventHubInventoryOwnership,
    inactive_date: u64,
    sub_title: String,
    release_date: u64,
    owned: bool,
    prices: Vec<LolEventHubCatalogPluginPrice>,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyQueueCustomGame {
    queue_availability: LolLobbyQueueAvailability,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoyaltyRmsEntitlementPayload {
    item_id: String,
    item_type_id: String,
    entitlement_type_id: String,
    resource_operation: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerConsoleNameset {
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionQuestSkin {
    tile_path: String,
    short_name: String,
    splash_video_path: String,
    skin_augments: LolChampSelectChampionSkinAugments,
    chroma_path: String,
    ownership: LolChampSelectCollectionsOwnership,
    id: i32,
    stage: u64,
    splash_path: String,
    name: String,
    disabled: bool,
    still_obtainable: bool,
    is_base: bool,
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlightsConfig {
    invalid_highlight_name_characters: String,
    is_highlights_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolHoneyfruitHoneyfruitLinkingServiceResponse {
    #[serde(rename = "eligible")]
    eligible: bool,
    #[serde(rename = "email")]
    email: String,
    #[serde(rename = "account_details")]
    account_details: LolHoneyfruitGarenaRegionLeagueAccount,
    #[serde(rename = "reason_code")]
    reason_code: LolHoneyfruitHoneyfruitLinkingFailureReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatErrorResource {
    text: String,
    code: u64,
    from: String,
    id: u64,
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersOfferDto {
    id: String,
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksChampionRuneRecommendationsGDSResource")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampionRuneRecommendationsGdsResource {
    champion_id: i32,
    is_override: bool,
    rune_recommendations: Vec<LolPerksRuneRecommendationGdsResource>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBracketUpdateNotification {
    bracket_id: i64,
    tournament_id: i64,
    current_match_id: i64,
    winner_roster_id: i64,
    notify_reason: LolClashRosterNotifyReason,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersSuggestedPlayersFriend {
    availability: String,
    summoner_id: u64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHighlightsHighlight {
    mtime_iso_8601: String,
    name: String,
    id: u64,
    file_size_bytes: u64,
    filepath: String,
    url: String,
    mtime_ms_utc: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPurchaseWidgetConfig {
    non_refundable_disclaimer_enabled: bool,
    always_show_purchase_disclaimer: bool,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameflowClient {
    observer_server_ip: String,
    observer_server_port: u16,
    running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubCapOrdersMetaDto {
    xid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSEvent")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsEvent {
    building_type: String,
    lane_type: String,
    position: LolReplaysGamhsPosition,
    timestamp: u64,
    participant_id: u16,
    assisting_participant_ids: Vec<u16>,
    monster_sub_type: String,
    monster_type: String,
    skill_slot: u16,
    tower_type: String,
    victim_id: u16,
    killer_id: u16,
    item_id: u16,
    _type: String,
    team_id: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RmsMessage {
    service: String,
    version: String,
    ack_required: bool,
    timestamp: i64,
    id: String,
    resource: String,
    payload: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLobbyLcdsDynamicClientConfig {
    party_rewards: LolLobbyLcdsPartyRewardsConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSummoner {
    puuid: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "EogLcdsGameDTO")]
#[serde(rename_all = "camelCase")]
pub struct EogLcdsGameDto {
    banned_champions: Vec<BannedChampion>,
    team_one: Vec<PlayerParticipant>,
    team_two: Vec<PlayerParticipant>,
    game_state: String,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMissionsPluginRegionLocaleChangedEvent {
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftLolTftNewsHub {
    enabled: bool,
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolKrShutdownLawAllQueueShutdownStatus {
    is_all_queues_disabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameQueuesLcdsGameQueueConfig {
    game_mutators: Vec<String>,
    disallow_free_champions: bool,
    num_players_per_team: u32,
    supported_map_ids: Vec<i32>,
    queue_state: String,
    game_mode: String,
    map_id: i32,
    last_toggled_on_time: u64,
    _type: String,
    minimum_participant_list_size: u32,
    id: i32,
    removal_from_game_delay_minutes: i32,
    maximum_participant_list_size: u32,
    min_level: u32,
    last_toggled_off_time: u64,
    removal_from_game_allowed: bool,
    game_type_config_id: i32,
    ranked: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAliasAvailability {
    is_success: bool,
    error_message: String,
    error_code: LolSummonerAliasAvailabilityCode,
    alias: LolSummonerAlias,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthAuthorizationRequest {
    scope: Vec<String>,
    client_id: String,
    claims: Vec<String>,
    trust_levels: Vec<LolRsoAuthRsoAuthorizationTrustLevel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePlayer {
    skin_tile_path: String,
    summoner_id: u64,
    level: i32,
    champion_id: i32,
    losses: i32,
    profile_icon_id: i32,
    skin_splash_path: String,
    champion_name: String,
    spell_2_id: i32,
    bot_player: bool,
    leaver: bool,
    game_id: u64,
    items: Vec<i32>,
    leaves: i32,
    spell_1_id: i32,
    wins: i32,
    detected_team_position: String,
    is_local_player: bool,
    skin_emblem_paths: Vec<String>,
    stats: Value,
    team_id: i32,
    champion_square_portrait_path: String,
    summoner_name: String,
    selected_position: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubTokenShop {
    token_uuid: String,
    token_image: String,
    content_drops: Vec<LolEventHubContentDrop>,
    token_name: String,
    offers: Vec<LolEventHubOffer>,
    token_bundles_catalog_entry: Vec<LolEventHubCatalogEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LCDSPlayerMessagingSimpleMessageResponse")]
#[serde(rename_all = "camelCase")]
pub struct LcdsPlayerMessagingSimpleMessageResponse {
    msg_id: String,
    command: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsStartChampSelectDto {
    invalid_players: Vec<LcdsFailedJoinPlayer>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ProfileInfo {
    honor_level: i32,
    checkpoint: i32,
    rewards_locked: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardChatSession {
    session_state: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMatchHistoryMatchHistoryPlayerChampMasteryDelta {
    grade: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootLcdsRecipeClientDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootLcdsRecipeClientDto {
    metadata: LootLcdsRecipeMetadata,
    single_open: bool,
    outputs: Vec<LootLcdsRecipeOutputDto>,
    recipe_name: String,
    display_categories: String,
    _type: String,
    slots: Vec<LootLcdsRecipeSlotClientDto>,
    crafter_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataChampionSkin {
    emblems: Vec<LolCatalogChampionSkinEmblem>,
    id: i64,
    uncentered_splash_path: String,
    chroma_path: String,
    chromas: Vec<LolCatalogGameDataChampionChroma>,
    name: String,
    splash_path: String,
    colors: Vec<String>,
    quest_skin_info: LolCatalogSkinLineInfo,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectGameDataSummonerSpell {
    icon_path: String,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ElevationRequest {
    action: ElevationAction,
}
type LolInventoryRmsWalletPayload = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueNotification {
    afk_lp_penalty_level: i32,
    tier: String,
    split_points_breakdown: Value,
    rated_rating_delta: i32,
    game_id: u64,
    consolation_lp_used: i32,
    afk_lp_penalty_amount: i32,
    can_demote_from_tier: bool,
    change_reason: String,
    wins: i32,
    queue_type: String,
    rated_rating: i32,
    rank: String,
    promo_series_for_ranks_enabled: bool,
    provisional_games_remaining: i32,
    league_points: i32,
    league_points_delta: i32,
    was_afk_or_leaver: bool,
    miniseries_progress: String,
    split_points: i32,
    eligible_for_promo_helper: bool,
    win_streak: i32,
    notify_reason: String,
    rated_tier: String,
    losses: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TracingCriticalFlowEventV1 {
    when: u64,
    event_id: String,
    payload_string: String,
    succeeded: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashReadyCheckInfo {
    queue_id: i32,
    ready_check_resource: LolClashMatchmakingReadyCheckResource,
    timestamp_received: i64,
    is_accept_successful: bool,
    accept_error: String,
    timestamp_last_clash_gameflow_dodge: i64,
    timestamp_response_complete: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryItem {
    expiration_date: String,
    quantity: u64,
    item_id: i32,
    inventory_type: String,
    wins: u64,
    purchase_date: String,
    ownership_type: LolInventoryItemOwnershipType,
    uuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTeamBoost {
    puuid: String,
    unlocked: bool,
    skin_unlock_mode: String,
    price: i64,
    summoner_id: i64,
    ip_reward_for_purchaser: i64,
    ip_reward: i64,
    available_skins: Vec<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSParticipantFrame")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsParticipantFrame {
    jungle_minions_killed: u16,
    team_score: u16,
    current_gold: i32,
    position: LolReplaysGamhsPosition,
    level: u16,
    dominion_score: u16,
    participant_id: u16,
    total_gold: i32,
    xp: u32,
    minions_killed: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreRiotMessagingServiceMessage {
    service: String,
    resource: String,
    version: String,
    timestamp: i64,
    payload: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSplitPointsNotification {
    split_points_required: i32,
    split_points_breakdown: Value,
    previous_split_points_required: i32,
    next_reward_type: String,
    split_points_delta: i32,
    next_reward_id: String,
    split_points_after_game: i32,
    split_points_before_game: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSkinLineInfo {
    description_info: Vec<LolPurchaseWidgetSkinLineDescriptionInfo>,
    tile_path: String,
    uncentered_splash_path: String,
    collection_description: String,
    collection_card_path: String,
    tiers: Vec<LolPurchaseWidgetSkinLineTier>,
    splash_path: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubLoyaltyRewardsSimplified {
    #[serde(rename = "xpBoost")]
    xp_boost: i32,
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsEntitlementPayload {
    resource_operation: String,
    entitlement_type_id: String,
    item_type_id: String,
    item_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLoginSession {
    puuid: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHighlightsHighlightsSettingsData {
    highlights_folder_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolContentTargetingTargetingAttributes {
    result: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksGameCustomizationDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksGameCustomizationDto {
    content: String,
    category: String,
    queue_type: u64,
    is_teambuilder: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceSessionResource {
    is_muted: bool,
    participants: Vec<LolPremadeVoiceParticipantResource>,
    volume: u32,
    is_transmit_enabled: bool,
    id: String,
    status: LolPremadeVoiceSessionStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatLobbyPlayerStatus {
    current_lobby_status: LolChatLobbyStatus,
    last_queued_lobby_status: LolChatLobbyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootPlayerLootDelta {
    delta_count: i32,
    player_loot: LolLootPlayerLoot,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameflowLcdsPlayerCredentialsDto {
    server_ip: String,
    observer: bool,
    packet_cop_metadata: String,
    observer_encryption_key: String,
    observer_server_port: u16,
    game_id: u64,
    encryption_key: String,
    server_port: u16,
    observer_server_ip: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryParticipantStatistics {
    item_4: i32,
    total_minions_killed: i64,
    total_player_score: i64,
    player_score_4: i64,
    true_damage_taken: i64,
    unreal_kills: i64,
    player_score_1: i64,
    total_score_rank: i64,
    perk_0: i64,
    perk_1_var_1: i64,
    turret_kills: i64,
    time_c_cing_others: i64,
    player_score_6: i64,
    win: bool,
    total_heal: i64,
    neutral_minions_killed_team_jungle: i64,
    neutral_minions_killed_enemy_jungle: i64,
    perk_2_var_3: i64,
    perk_4_var_2: i64,
    caused_early_surrender: bool,
    magic_damage_dealt_to_champions: i64,
    physical_damage_taken: i64,
    inhibitor_kills: i64,
    first_blood_kill: bool,
    perk_5_var_1: i64,
    item_1: i32,
    perk_primary_style: i64,
    player_augment_3: i32,
    perk_4_var_1: i64,
    player_augment_4: i32,
    physical_damage_dealt_to_champions: i64,
    perk_5: i64,
    total_units_healed: i64,
    magical_damage_taken: i64,
    player_score_0: i64,
    killing_sprees: i64,
    triple_kills: i64,
    perk_4_var_3: i64,
    first_inhibitor_assist: bool,
    total_damage_dealt_to_champions: i64,
    vision_wards_bought_in_game: i64,
    perk_1_var_3: i64,
    total_time_crowd_control_dealt: i64,
    wards_killed: i64,
    player_score_7: i64,
    gold_earned: i64,
    perk_3_var_3: i64,
    item_5: i32,
    gold_spent: i64,
    champ_level: i64,
    perk_5_var_2: i64,
    longest_time_spent_living: i64,
    player_augment_1: i32,
    damage_dealt_to_objectives: i64,
    perk_5_var_3: i64,
    player_score_9: i64,
    player_score_5: i64,
    player_score_8: i64,
    total_damage_dealt: i64,
    objective_player_score: i64,
    team_early_surrendered: bool,
    perk_2_var_2: i64,
    participant_id: u16,
    item_3: i32,
    item_0: i32,
    assists: i64,
    largest_multi_kill: i64,
    true_damage_dealt: i64,
    first_inhibitor_kill: bool,
    early_surrender_accomplice: bool,
    perk_2: i64,
    perk_3_var_1: i64,
    sight_wards_bought_in_game: i64,
    wards_placed: i64,
    perk_3: i64,
    physical_damage_dealt: i64,
    largest_critical_strike: i64,
    vision_score: i64,
    perk_2_var_1: i64,
    perk_1_var_2: i64,
    double_kills: i64,
    item_2: i32,
    true_damage_dealt_to_champions: i64,
    deaths: i64,
    total_damage_taken: i64,
    neutral_minions_killed: i64,
    game_ended_in_surrender: bool,
    combat_player_score: i64,
    magic_damage_dealt: i64,
    first_tower_assist: bool,
    damage_self_mitigated: i64,
    damage_dealt_to_turrets: i64,
    player_score_2: i64,
    player_score_3: i64,
    kills: i64,
    perk_sub_style: i64,
    perk_0_var_2: i64,
    perk_0_var_3: i64,
    player_augment_2: i32,
    subteam_placement: i32,
    penta_kills: i64,
    player_subteam_id: i32,
    perk_1: i64,
    perk_4: i64,
    first_tower_kill: bool,
    game_ended_in_early_surrender: bool,
    perk_0_var_1: i64,
    largest_killing_spree: i64,
    quadra_kills: i64,
    item_6: i32,
    perk_3_var_2: i64,
    first_blood_assist: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassWalletDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassWalletDto {
    expires: String,
    puuid: String,
    balances: Value,
    balances_jwt: String,
    account_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSvcRewardGroup {
    reward_strategy: LolLootRewardStrategy,
    media: Value,
    localizations: Value,
    active: bool,
    celebration_type: LolLootCelebrationType,
    rewards: Vec<LolLootReward>,
    selection_strategy_config: LolLootSelectionStrategyConfig,
    child_reward_group_ids: Vec<String>,
    types: Vec<String>,
    id: String,
    product_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildInfo {
    patchline: String,
    patchline_visible_name: String,
    branch: String,
    version: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysInstallPaths {
    game_executable_path: String,
    game_install_root: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTrophiesLoadoutItem {
    item_id: i32,
    inventory_type: String,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedLeagueTierAndRankDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLeagueTierAndRankDto {
    player_or_team_id: String,
    player_or_team_name: String,
    tier: String,
    queue_type: String,
    rank: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestonesCounter {
    ready_to_claim_milestones: Vec<String>,
    loot_milestones_id: String,
    counter_value: i64,
    completed_loops: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NonTimingEventV1 {
    unit: String,
    value: String,
    event_name: String,
    when: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataTftRegionPortal {
    name_id: String,
    display_name: String,
    description: String,
    icon_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsLoadout {
    item_id: i32,
    name: String,
    scope: String,
    loadout: Value,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "TournamentPlayerInfoDTO")]
#[serde(rename_all = "camelCase")]
pub struct TournamentPlayerInfoDto {
    season_vp: i32,
    tournament_info: Vec<TournamentInfoDto>,
    roster_stats: Vec<RosterStatsDto>,
    time: i64,
    player: PlayerDto,
    theme_vps: Vec<ThemeVp>,
    tier: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatBlockedPlayerResource {
    puuid: String,
    game_tag: String,
    icon: i32,
    name: String,
    id: String,
    summoner_id: u64,
    pid: String,
    game_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpEndOfGameStats {
    game_type: String,
    previous_level: u32,
    game_mutators: Vec<String>,
    new_spells: Vec<i32>,
    rp_earned: i32,
    queue_type: String,
    game_mode: String,
    leveled_up: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsSummonerBackdrop {
    summoner_id: u64,
    backdrop_video: String,
    backdrop_mask_color: String,
    backdrop_image: String,
    champion_id: i32,
    puuid: String,
    backdrop_augments: Vec<LolCollectionsCollectionsSummonerBackdropAugments>,
    backdrop_type: LolCollectionsCollectionsSummonerBackdropType,
    account_id: u64,
    profile_icon_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsOfferPrice {
    price: u64,
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashProfileInfo {
    honor_level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEndOfGameGameClientEndOfGame {
    #[serde(rename = "gameClientEOG")]
    game_client_eog: LolEndOfGameGameClientEndOfGameStats,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolRankedLeaguesSeasonRewardConfig {
    qualification_warning_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootProgressionConfigMilestonePropertiesGdsResource {
    reward_strategy: String,
    name: String,
    description: String,
    id: String,
    rewards: Vec<LolLootProgressionConfigMilestoneRewardGdsResource>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolContentTargetingRegionLocale {
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTPlaybookGroupViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftPlaybookGroupViewModel {
    items: Vec<LolCosmeticsCosmeticsTftPlaybookViewModel>,
    group_name: String,
    num_available: u32,
    group_id: u32,
    num_owned: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameSimpleMessage {
    params: Vec<String>,
    _type: String,
    account_id: u64,
    msg_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyInvitation {
    invitation_meta_data: Value,
    from_summoner_id: u64,
    timestamp: String,
    error_type: String,
    from_summoner_name: String,
    eligibility: LolLobbyEligibility,
    id: String,
    state: LolLobbyLobbyInvitationState,
    to_summoner_name: String,
    to_summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreStoreStatus {
    storefront_is_running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventRewardsProductConfig {
    service_url: String,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsInvitePrivileges {
    can_invite: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginAccessToken {
    expiry: u64,
    token: String,
    scopes: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreAliasChangeNotificationResource {
    details: LolStoreAliasDetail,
    id: u64,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassInventoryDto {
    puuid: String,
    summoner_id: u64,
    items: Value,
    account_id: u64,
    expires: String,
    items_jwt: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatGameClientChatMessageResource {
    body: String,
    from_summoner_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassInventoryNotification {
    id: i64,
    _type: String,
    acknowledged: bool,
    item_id: i32,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationRegionLocale {
    web_region: String,
    region: String,
    web_language: String,
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPatchPatcherSelfUpdateSettings {
    #[serde(rename = "restart_delay")]
    restart_delay: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderBenchChampion {
    champion_id: i32,
    is_priority: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemDefinition {
    tags: Vec<String>,
    has_visible_loot_odds: bool,
    inventory_type: String,
    name: String,
    loyalty_unlocked: bool,
    image_path: String,
    sub_title: String,
    description: String,
    item_id: i32,
    owned: bool,
    assets: LolPurchaseWidgetCatalogPluginItemAssets,
    metadata: Vec<LolPurchaseWidgetItemMetadataEntry>,
    bundled_item_price: LolPurchaseWidgetBundledItemPricingInfo,
    sub_inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubPurchaseRequest {
    items: Vec<LolEventHubPurchaseItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatPuuidBody {
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetPriceOptionDto {
    currency_type: String,
    currency_payment_option: String,
    price: i64,
    currency_image_path: String,
    currency_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "MassCraftLootDTO")]
#[serde(rename_all = "camelCase")]
pub struct MassCraftLootDto {
    client_id: String,
    transaction_id: String,
    loot_items: Vec<CraftLootDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesPersonalBestNotification {
    summoner: LolStatstonesSummoner,
    personal_best: String,
    image_url: String,
    statstone_name: String,
    statstone_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolBannersInventoryResponse {
    items: LolBannersInventoryItemsByType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedVictoriousSkinDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedVictoriousSkinDto {
    item_instance_id: String,
    split_points_by_highest_season_end_tier: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NpeReward {
    renderer: String,
    data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsLoyaltyStatusNotification {
    reload_inventory: bool,
    status: LolChampionsLoyaltyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatcherStatus {
    successfully_installed_version: u32,
    connected_to_patch_server: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOffer {
    label: String,
    type_id: String,
    active: bool,
    created_date: String,
    product_id: String,
    merchant_id: String,
    start_date: String,
    id: String,
    payload: Vec<LolPurchaseWidgetCapOfferPayloadEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetChampionSkinEmblem {
    name: String,
    emblem_path: LolPurchaseWidgetChampionSkinEmblemPath,
    emblem_position: LolPurchaseWidgetChampionSkinEmblemPosition,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LolBannersInventoryItemsByType {
    tournament_frame: Vec<LolBannersTournamentFrameInventoryItem>,
    tournament_flag: Vec<LolBannersTournamentFlagInventoryItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopLoginSession {
    summoner_id: u64,
    state: LolYourshopLoginSessionStates,
    account_id: u64,
    id_token: String,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterPlayerAggregatedStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterPlayerAggregatedStatsDto {
    raw_stats_max: Value,
    raw_stats_sum: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSocialLeaderboardRowData {
    availability: String,
    is_giftable: bool,
    game_name: String,
    wins: i32,
    tier: String,
    division: LolSocialLeaderboardLeagueDivision,
    provisional_games_remaining: i32,
    leaderboard_position: i32,
    puuid: String,
    tag_line: String,
    summoner_name: String,
    summoner_level: i32,
    summoner_id: u64,
    is_provisional: bool,
    profile_icon_id: i32,
    league_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesLcuChallengeNotification {
    icon_path: String,
    display_type: LolChallengesNotificationDisplayType,
    id: u64,
    category_challenges: Value,
    msg_id: String,
    update_reason: String,
    challenge_name: String,
    challenge_id: i64,
    level: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHoneyfruitV1AuthenticationResponse {
    country: String,
    success: LolHoneyfruitV1SuccessResponseDetails,
    _type: LolHoneyfruitV1ResponseType,
    error: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLoyaltyLoyaltyRewards {
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "xpBoost")]
    xp_boost: Value,
    #[serde(rename = "global")]
    global: LolLoyaltyGlobalRewards,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingMatchmakingReadyCheckResource {
    player_response: LolMatchmakingMatchmakingReadyCheckResponse,
    timer: f32,
    decliner_ids: Vec<u64>,
    suppress_ux: bool,
    state: LolMatchmakingMatchmakingReadyCheckState,
    dodge_warning: LolMatchmakingMatchmakingDodgeWarning,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSettingsLoginSession {
    summoner_id: u64,
    account_id: u64,
    state: LolSettingsLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationError {
    message: String,
    response_items: Vec<String>,
    error_details: Value,
    error_code: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTrovesRiotMessagingServiceMessage {
    service: String,
    resource: String,
    version: String,
    payload: String,
    timestamp: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct CollectionsLcdsClientDynamicConfigurationNotification {
    configs: String,
    delta: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterRankedRestriction {
    punished_games_remaining: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStoreItemCost {
    cost: i64,
    discount: f32,
    currency: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServiceEntitlementsToken {
    issuer: String,
    access_token: String,
    token: String,
    entitlements: Vec<String>,
    subject: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolClashSetPositionRequest {
    position: Position,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMissionSeries {
    id: String,
    internal_name: String,
    status: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RankedScoutingDTO")]
#[serde(rename_all = "camelCase")]
pub struct RankedScoutingDto {
    puuid: String,
    top_masteries: Vec<ChampionMasteryPublicDto>,
    total_mastery_score: u64,
    top_season_champions: Vec<ChampionScoutingDto>,
    player_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathObjective {
    reward_groups: Vec<String>,
    description: String,
    _type: String,
    sequence: i32,
    progress: LolNpeTutorialPathProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationPhoneNumberResponseData {
    phone_number_obfuscated: LolAccountVerificationPhoneNumberObfuscated,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidationRequestItem {
    quantity: i32,
    item_key: LolPurchaseWidgetItemKey,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTeamBuilderBoostInfo {
    battle_boost_activated: bool,
    activator_cell_id: i64,
    allow_battle_boost: bool,
    boostable_skin_count: i32,
    cost: i64,
    unlocked_skin_ids: Vec<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyBotParams {
    champion_id: i32,
    bot_difficulty: LolLobbyLobbyBotDifficulty,
    team_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "CollectionsLcdsChampionDTO")]
#[serde(rename_all = "PascalCase")]
pub struct CollectionsLcdsChampionDto {
    #[serde(rename = "f2pRewardSources")]
    f_2_p_reward_sources: Vec<String>,
    #[serde(rename = "endDate")]
    end_date: u64,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "purchased")]
    purchased: u64,
    #[serde(rename = "botEnabled")]
    bot_enabled: bool,
    #[serde(rename = "rankedPlayEnabled")]
    ranked_play_enabled: bool,
    #[serde(rename = "purchaseDate")]
    purchase_date: u64,
    #[serde(rename = "sources")]
    sources: Vec<String>,
    #[serde(rename = "active")]
    active: bool,
    #[serde(rename = "winCountRemaining")]
    win_count_remaining: i32,
    #[serde(rename = "championId")]
    champion_id: i32,
    #[serde(rename = "freeToPlay")]
    free_to_play: bool,
    #[serde(rename = "championSkins")]
    champion_skins: Vec<CollectionsLcdsChampionSkinDto>,
    #[serde(rename = "freeToPlayReward")]
    free_to_play_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesLoginSession {
    id_token: String,
    account_id: u64,
    puuid: String,
    state: LolTftTrovesLoginSessionStates,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreGiftingFriend {
    nick: String,
    friends_since: String,
    old_friends: bool,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubExternalItemMetadataEntry {
    _type: String,
    value: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRewardsRegionLocale {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolRankedGameflowGameData {
    queue: LolRankedQueue,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2HonorInteraction {
    display_name: String,
    summoner_id: u64,
    game_id: u64,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyOpenPartyToggleAnalytics {
    party_id: String,
    elapsed_time: u64,
    player_id: String,
    platform_id: String,
    event_timestamp: u64,
    game_mode: String,
    num_of_toggles: i32,
    event_type: String,
    start_transition: String,
    event_data: Value,
    start_timestamp: u64,
    initial_state: String,
    final_state: String,
    end_transition: String,
    end_timestamp: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCatalogGameDataChampion {
    skins: Vec<LolCatalogGameDataChampionSkin>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RosterWithdraw {
    init_vote_member: u64,
    game_start_buffer_ms: i64,
    init_vote_time: i64,
    vote_withdraw_members: Vec<i64>,
    lockout_time_ms: i64,
    vote_timeout_ms: i64,
    decline_withdraw_members: Vec<i64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartyQueueEligibilityDto {
    party_restrictions: Vec<LolLobbyGatekeeperRestrictionDto>,
    available_queue_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootRequestMetadataDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolLootRequestMetadataDto {
    transaction_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "MissionRequirementDTO")]
#[serde(rename_all = "kebab-case")]
pub struct MissionRequirementDto {
    expression: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyGameflowGameDodge {
    state: LolChampSelectLegacyGameflowGameDodgeState,
    dodge_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftLolTftBattlePassHub {
    #[serde(rename = "battlePassXPBoosted")]
    battle_pass_xp_boosted: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTrovesTrovesActiveBanner {
    id: String,
    version: u8,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigDataPaths {
    user_data_root: String,
    private_settings_path: String,
    data_root: String,
    local_settings_path: String,
    config_root: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemOwnership {
    item_key: LolEventHubItemKey,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubLoyaltyRewards {
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "xpBoost")]
    xp_boost: Value,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardFriendResult {
    icon: i32,
    patchline: String,
    game_tag: String,
    note: String,
    name: String,
    availability: String,
    summoner_id: u64,
    id: String,
    game_name: String,
    score: u64,
    account_id: u64,
    puuid: String,
    status_message: String,
    product_name: String,
    platform_id: String,
    product: String,
    lol: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoginRSOPlayerCredentials")]
#[serde(rename_all = "camelCase")]
pub struct LolLoginRsoPlayerCredentials {
    username: String,
    password: String,
    platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerRewards {
    season_vp: i32,
    theme_vp: Vec<LolClashThemeVp>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreLcdsSimpleDialogMessageResponse {
    account_id: u64,
    msg_id: String,
    command: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineDescriptionInfo {
    title: String,
    icon_path: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SanitizerContainsSanitizedRequest {
    text: String,
    aggressive_scan: bool,
    level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyPlayerStatus {
    last_queued_lobby_status: LolChampSelectLegacyLobbyStatus,
    current_lobby_status: LolChampSelectLegacyLobbyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionSkinAugment {
    overlays: Vec<LolChampionsCollectionsChampionSkinAugmentOverlays>,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameSettingsgamesettingsgameflowsession {
    game_client: LolGameSettingsgamesettingsgameclient,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct LolTrophiesInventoryItemsByType {
    tournament_trophy: Vec<LolTrophiesTournamentTrophyInventoryItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolContentTargetingPlatformConfig {
    #[serde(rename = "ABTestFilterSalt")]
    ab_test_filter_salt: u64,
    level_filter_enabled: bool,
    location_filters_enabled: bool,
    rank_filter_enabled: bool,
    mastery_filter_enabled: bool,
    #[serde(rename = "ABTestFilterEnabled")]
    ab_test_filter_enabled: bool,
    entitlements_filter_enabled: bool,
    mapping: String,
    main_filter_enabled: bool,
    mastery_filter_champion_limit: u32,
    mastery_filter_days_since_last_played: u32,
    summoner_icon_filter_enabled: bool,
    #[serde(rename = "ABTestFilterGroups")]
    ab_test_filter_groups: u64,
    targeting_attribute_storage_base_uri: String,
    ranked_filter_enabled: bool,
    entitlements_prefix: String,
    asynchronous_event_handler_setup_delay_in_seconds: u32,
    mastery_filter_level_threshold: u32,
    missions_filter_enabled: bool,
    enabled: bool,
    targeting_attribute_storage_enabled: bool,
    tas_ingestion_delay_in_seconds: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerMessagingSimpleMessage {
    account_id: u64,
    title_code: String,
    body_code: String,
    msg_id: String,
    params: Vec<String>,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoadoutsItemKey {
    inventory_type: String,
    item_id: i32,
    content_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectTimer {
    total_time_in_phase: i64,
    adjusted_time_left_in_phase: i64,
    phase: String,
    is_infinite: bool,
    internal_now_in_epoch_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[doc = "Describes a log entry."]
pub struct LogEvent {
    message: String,
    severity: LogSeverityLevels,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsLoginDataPacket {
    time_until_first_win_of_day: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolServiceStatusRiotStatusResource {
    maintenances: Vec<LolServiceStatusRiotStatusMaintenance>,
    incidents: Vec<LolServiceStatusRiotStatusIncident>,
    id: String,
    locales: Vec<String>,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEsportStreamNotificationsEsportsAPI_teams")]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsEsportsApiTeams {
    logo_url: String,
    acronym: String,
    id: i64,
    slug: String,
    guid: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameEndOfGamePoints {
    points_until_next_reroll: i32,
    reroll_count: i32,
    previous_points: i32,
    total_points: i32,
    point_change_from_champions_owned: i32,
    point_change_from_gameplay: i32,
    points_used: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitSummoner {
    internal_name: String,
    xp_until_next_level: u64,
    xp_since_last_level: u64,
    account_id: u64,
    summoner_level: u32,
    percent_complete_for_next_level: u32,
    summoner_id: u64,
    puuid: String,
    unnamed: bool,
    name_change_flag: bool,
    profile_icon_id: i32,
    display_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTEndOfGamePoints")]
#[serde(rename_all = "camelCase")]
pub struct LolPftPftEndOfGamePoints {
    points_until_next_reroll: i32,
    previous_points: i32,
    point_change_from_gameplay: i32,
    reroll_count: i32,
    total_points: i32,
    point_change_from_champions_owned: i32,
    points_used: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlayerTournamentData {
    bracket_id: i64,
    state: LolClashPlayerState,
    roster_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterDynamicStateDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterDynamicStateDto {
    withdraw: RosterWithdraw,
    roster_id: i64,
    phase_checkin_states: Vec<u64>,
    tournament_id: i64,
    ticket_offers: Vec<TicketOfferDto>,
    members: Vec<PhaseInMember>,
    version: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventShop {
    localized_upsell_title: String,
    token_shop: LolEventHubTokenShop,
    header_icon_image: String,
    background_image: String,
    localized_upsell_button_text: String,
    start_date: String,
    progress_end_date: String,
    event_hub_type: String,
    end_date: String,
    localized_upsell_tooltip_description: String,
    event_pass_bundles_catalog_entry: Vec<LolEventHubCatalogEntry>,
    navbar_icon_image: String,
    reward_track: LolEventHubRewardTrack,
    upsell_tooltip_background_image_url: String,
    upsell_background_image_url: String,
    help_modal_image: String,
    localized_upsell_tooltip_title: String,
    event_id: String,
    localized_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftTrovesTrovesPCSpriteAnimation")]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesPcSpriteAnimation {
    num_frames: u32,
    delay: f32,
    start_frame: u32,
    num_rows: u32,
    fps: u32,
    max_play_count: i32,
    play_at_insert: bool,
    num_cols: u32,
    duration: f32,
    spritesheet_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetChampionSkinEmblemPosition {
    horizontal: String,
    vertical: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCustomGameDto {
    allow_spectators: String,
    max_num_players: i32,
    team_1_count: i32,
    owner_puuid: String,
    party_id: String,
    game_type: String,
    team_2_count: i32,
    map_id: i32,
    spectator_count: i32,
    lobby_name: String,
    private_game: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PendingRosterDTO")]
#[serde(rename_all = "camelCase")]
pub struct PendingRosterDto {
    ticket_offers: Vec<TicketOfferDto>,
    version: i32,
    lft: bool,
    name: String,
    logo_color: i32,
    reward_logos: Vec<RewardLogo>,
    captain_id: u64,
    members: Vec<PendingRosterMemberDto>,
    muc_jwt_dto: MucJwtDto,
    tournament_id: i64,
    logo: i32,
    short_name: String,
    invitees: Vec<PendingRosterInviteeDto>,
    invite_faileds: Vec<FailedInvite>,
    tier: i32,
    high_tier_variance: bool,
    invitation_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubActiveEventUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubActiveEventUiData {
    event_info: LolEventHubEventInfoUiData,
    event_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectMySelection {
    spell_2_id: u64,
    spell_1_id: u64,
    selected_skin_id: i32,
    ward_skin_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootResponseDTO-vector-SvcRewardGrant")]
#[serde(rename_all = "kebab-case")]
pub struct LolLootResponseDtoVectorSvcRewardGrant {
    data: Vec<LolLootSvcRewardGrant>,
    metadata: LolLootResponseMetadataDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "CraftLootRefTransactionDTO")]
#[serde(rename_all = "camelCase")]
pub struct CraftLootRefTransactionDto {
    recipe_name: String,
    loot_name_ref_ids: Vec<LootNameRefId>,
    transaction_id: String,
    client_id: String,
    repeat: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesUIChallenge")]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiChallenge {
    is_capstone: bool,
    current_threshold: f64,
    position: i32,
    current_level_achieved_time: i64,
    description_short: String,
    retire_timestamp: i64,
    players_in_level: i32,
    next_threshold: f64,
    description: String,
    id: i64,
    icon_path: String,
    name: String,
    friends_at_levels: Vec<LolChallengesFriendLevelsData>,
    capstone_group_id: i64,
    capstone_group_name: String,
    current_level: String,
    priority: f64,
    next_level: String,
    percentile: f64,
    thresholds: Value,
    completed_ids: Vec<i32>,
    game_modes: Vec<String>,
    source: String,
    children_ids: Vec<i64>,
    next_level_icon_path: String,
    points_awarded: i64,
    previous_level: String,
    parent_id: i64,
    id_list_type: LolChallengesChallengeRequirementMappingName,
    category: String,
    level_to_icon_path: Value,
    value_mapping: String,
    is_reverse_direction: bool,
    current_value: f64,
    previous_value: f64,
    is_apex: bool,
    has_leaderboard: bool,
    parent_name: String,
    available_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "TournamentInfoMinimalDTO")]
#[serde(rename_all = "camelCase")]
pub struct TournamentInfoMinimalDto {
    tournament_info: Vec<TournamentInfoDto>,
    time: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogPluginPrice {
    cost_type: String,
    cost: i64,
    sale: LolEventHubCatalogPluginSale,
    currency: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolAccountVerificationIsVerifiedResponse {
    success: bool,
    message: String,
    status: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryTimeline {
    damage_taken_diff_per_min_deltas: Value,
    damage_taken_per_min_deltas: Value,
    role: String,
    xp_diff_per_min_deltas: Value,
    gold_per_min_deltas: Value,
    creeps_per_min_deltas: Value,
    lane: String,
    participant_id: u16,
    xp_per_min_deltas: Value,
    cs_diff_per_min_deltas: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRewardTrackItemOption {
    state: LolEventHubRewardTrackItemStates,
    override_footer: String,
    thumb_icon_path: String,
    header_type: LolEventHubRewardTrackItemHeaderType,
    reward_description: String,
    reward_group_id: String,
    celebration_type: LolEventHubCelebrationType,
    splash_image_path: String,
    card_size: String,
    reward_name: String,
    selected: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterMatchmakingSearchErrorResource {
    penalty_time_remaining: f64,
    message: String,
    penalized_summoner_id: u64,
    id: i32,
    error_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubItemMetadataEntry {
    value: String,
    _type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCollectionsCollectionsSummonerBackdropAugments {
    #[serde(rename = "socialCardLCOverlayPath")]
    social_card_lc_overlay_path: String,
    #[serde(rename = "centeredLCOverlayPath")]
    centered_lc_overlay_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventoryInventoryDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryInventoryDto {
    expires: String,
    items: Value,
    puuid: String,
    account_id: u64,
    items_jwt: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "TournamentHistoryAndWinnersDTO")]
#[serde(rename_all = "camelCase")]
pub struct TournamentHistoryAndWinnersDto {
    tournament_history: Vec<TournamentDto>,
    tournament_winners_compressed: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashQueueGameTypeConfig {
    battle_boost: bool,
    exclusive_pick: bool,
    max_allowable_bans: i32,
    name: String,
    allow_trades: bool,
    cross_team_champion_pool: bool,
    duplicate_pick: bool,
    reroll: bool,
    ban_timer_duration: i32,
    do_not_remove: bool,
    onboard_coop_beginner: bool,
    main_pick_timer_duration: i32,
    pick_mode: String,
    advanced_learning_quests: bool,
    learning_quests: bool,
    post_pick_timer_duration: i32,
    team_champion_pool: bool,
    death_match: bool,
    ban_mode: String,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2ApiHonorPlayerServerRequest {
    puuid: String,
    summoner_id: u64,
    game_id: u64,
    honor_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopInventoryItem {
    item_id: i32,
    quantity: u64,
    uuid: String,
    wins: u64,
    expiration_date: String,
    purchase_date: String,
    inventory_type: String,
    ownership_type: LolYourshopItemOwnershipType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsCollectionsSummoner {
    summoner_level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPerksGameDataChampionSummary {
    name: String,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashPlaymodeRestrictedInfo {
    roster_id: String,
    presence_state: LolClashPresenceState,
    phase_id: i64,
    is_restricted: bool,
    tournament_id: i64,
    ready_for_voice: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstonesInfo {
    series_id_to_stat_stone_ids: Value,
    pack_id_to_sub_pack_ids: Value,
    pack_id_to_stat_stones_ids: Value,
    collection_id_to_stat_stone_ids: Value,
    pack_data: Vec<LolStatstonesGameDataStatstonePack>,
    pack_id_to_champ_ids: Value,
    champ_id_to_pack_ids: Value,
    statstone_data: Vec<LolStatstonesGameDataStatstoneSet>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolInventoryWalletCacheEntry {
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
    #[serde(rename = "signedBalancesJwt")]
    signed_balances_jwt: String,
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderBackwardsTransitionInfoV1 {
    backwards_transition_reason: String,
    initiator_summoner_ids: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPlayerBehaviorUserInfoBanData {
    restrictions: Vec<LolPlayerBehaviorUserInfoRestriction>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatFriendGroupUpdate {
    #[serde(rename = "new_name")]
    new_name: String,
    #[serde(rename = "collapsed")]
    collapsed: bool,
    #[serde(rename = "name")]
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatSanitizeRequest {
    level: u32,
    aggressive_scan: bool,
    texts: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventCollectionsSummoner {
    summoner_level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCapOrdersSubOrderDto {
    offer: LolPurchaseWidgetCapOrdersOfferDto,
    recipient_id: String,
    offer_context: LolPurchaseWidgetCapOrdersOfferContextDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubExternalCatalogItemCost {
    cost: i64,
    discount: f32,
    currency: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSocialLeaderboardSummoner {
    summoner_id: u64,
    account_id: u64,
    internal_name: String,
    summoner_level: u32,
    puuid: String,
    profile_icon_id: i32,
    tag_line: String,
    display_name: String,
    game_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsAcceptGameInvitationRequestDto {
    champ_select_inventory_jwt: String,
    player_gco_tokens: LcdsPlayerGcoTokens,
    game_version: String,
    invitation_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopWalletResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopWalletResponseDto {
    data: LolYourshopWalletDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSuggestedPlayersEndOfGameStats {
    teams: Vec<LolSuggestedPlayersEndOfGameTeam>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "StoreLcdsChampionSkinDTO")]
#[serde(rename_all = "PascalCase")]
pub struct StoreLcdsChampionSkinDto {
    #[serde(rename = "purchaseDate")]
    purchase_date: u64,
    #[serde(rename = "sources")]
    sources: Vec<String>,
    #[serde(rename = "winCountRemaining")]
    win_count_remaining: i32,
    #[serde(rename = "endDate")]
    end_date: u64,
    #[serde(rename = "freeToPlayReward")]
    free_to_play_reward: bool,
    #[serde(rename = "skinId")]
    skin_id: i32,
    #[serde(rename = "f2pRewardSources")]
    f_2_p_reward_sources: Vec<String>,
    #[serde(rename = "lastSelected")]
    last_selected: bool,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "stillObtainable")]
    still_obtainable: bool,
    #[serde(rename = "championId")]
    champion_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGameflowReplaysSettingsData {
    highlights_folder_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceAccountSettingsCategoryResource {
    schema_version: u32,
    data: LolPremadeVoiceAccountSettingsCategoryDataResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolClashNoShowPingDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolClashNoShowPingDto {
    dodge_time: i64,
    tournament_id: i64,
    match_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSwapContract {
    state: LolChampSelectChampSelectSwapState,
    id: i64,
    cell_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2GameflowGameData {
    queue: LolHonorV2Queue,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolActiveBoostsLoyaltyRewardsSimplified {
    xp_boost: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectMySelection {
    selected_skin_id: i32,
    ward_skin_id: i64,
    spell_2_id: u64,
    spell_1_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLootRMSPayload")]
#[serde(rename_all = "camelCase")]
pub struct LolLootRmsPayload {
    product_id: String,
    affinities: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPreEndOfGameSequenceEvent {
    name: String,
    priority: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceParticipantResource {
    is_speaking: bool,
    name: String,
    is_muted: bool,
    id: String,
    volume: u32,
    energy: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventCollectionsSummonerIcons {
    icons: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemChoices {
    choices: Vec<LolEventHubItemChoiceDetails>,
    validation_errors: Vec<LolEventHubValidationErrorEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEndOfGameGameDataQuestSkinInfo {
    tiers: Vec<LolEndOfGameGameDataChampionQuestSkin>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusServiceStatusResource {
    status: String,
    human_readable_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatParticipant {
    #[serde(rename = "muted")]
    muted: bool,
    #[serde(rename = "puuid")]
    puuid: String,
    #[serde(rename = "cid")]
    cid: String,
    #[serde(rename = "pid")]
    pid: String,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "game_name")]
    game_name: String,
    #[serde(rename = "region")]
    region: String,
    #[serde(rename = "game_tag")]
    game_tag: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPlayerPreferencesPlayerPreferencesEndpoint {
    service_endpoint: String,
    version: String,
    enabled: bool,
    enforce_https: bool,
    retries: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolStatstonesCatalogBundlePrice {
    cost: i32,
    currency: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassTftPaidBattlepassInfo {
    description: String,
    pc_purchase_requirement: String,
    title: String,
    start_date: u64,
    pass_id: String,
    media: Value,
    end_date: u64,
    premium_entitlement_id: String,
    premium: bool,
    premium_title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedEosSettingsResource {
    data: LolRankedEosSettingsData,
    schema_version: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatFriendRequest {
    name: String,
    subscription: LolChatFriendSubscriptionType,
    note: String,
    puuid: String,
    tag_line: String,
    game_name: String,
    region: String,
    pid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsObfuscatedParticipant {
    game_unique_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchPatchSieveQueryResponse {
    releases: Vec<LolPatchPatchSieveRelease>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorRestrictionNotification {
    games_remaining: i64,
    display_reform_card: bool,
    expiration_millis: u64,
    id: u64,
    source: LolPlayerBehaviorNotificationSource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderTradeV1 {
    cell_id: i32,
    state: String,
    id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingPlayerStatus {
    last_queued_lobby_status: LolMatchmakingLobbyStatus,
    current_lobby_status: LolMatchmakingLobbyStatus,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatChatWindowSettings {
    height: u32,
    left: i32,
    width: u32,
    detached: bool,
    top: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPlayerBehaviorPlayerBehavior_SimpleMessage")]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorPlayerBehaviorSimpleMessage {
    account_id: u64,
    _type: String,
    params: Vec<String>,
    msg_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LootItemListClientDTO")]
#[serde(rename_all = "camelCase")]
pub struct LootItemListClientDto {
    last_update: i64,
    loot_items: Vec<LootItemClientDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSummonerIdAvailability {
    available_for_watching: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardGrantInfo {
    grantee_id: String,
    viewed: bool,
    id: String,
    selected_ids: Vec<String>,
    status: LolMissionsGrantStatus,
    grant_elements: Vec<LolMissionsRewardGrantElement>,
    reward_group_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchProductState {
    action: LolPatchComponentStateAction,
    is_update_available: bool,
    is_corrupted: bool,
    is_up_to_date: bool,
    id: String,
    is_stopped: bool,
    components: Vec<LolPatchComponentState>,
    percent_patched: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoiceDetails {
    display_type: String,
    purchase_options: Vec<LolPurchaseWidgetPurchaseOption>,
    item: LolPurchaseWidgetCatalogPluginItem,
    contents: Vec<LolPurchaseWidgetItemDetails>,
    background_image: String,
    discount: String,
    full_price: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeTutorialPathRequirement {
    expression: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionSkin {
    chroma_path: String,
    disabled: bool,
    rarity_gem_path: String,
    ownership: LolChampSelectCollectionsOwnership,
    id: i32,
    chromas: Vec<LolChampSelectCollectionsChampionChroma>,
    splash_path: String,
    emblems: Vec<LolChampSelectCollectionsChampionSkinEmblem>,
    is_base: bool,
    name: String,
    splash_video_path: String,
    still_obtainable: bool,
    tile_path: String,
    champion_id: i32,
    skin_augments: LolChampSelectChampionSkinAugments,
    quest_skin_info: LolChampSelectChampionQuestSkinInfo,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolServiceStatusLegacyServiceStatusTranslation {
    locale: String,
    heading: String,
    content: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatFriendResource {
    summoner_id: u64,
    game_name: String,
    id: String,
    name: String,
    puuid: String,
    game_tag: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreBundledItemCost {
    cost: i64,
    cost_type: String,
    discount: f32,
    currency: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "TicketOfferDTO")]
#[serde(rename_all = "camelCase")]
pub struct TicketOfferDto {
    receive_player_id: u64,
    count: i32,
    _type: TicketType,
    offer_player_id: u64,
    ticket_offer_state: TicketOfferState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTeamPlannerQueue {
    game_mode: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLicenseAgreementPluginRegionLocaleChangedEvent {
    locale: String,
    region: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootMilestoneRepeat {
    repeat_scope: i32,
    repeat_count: i32,
    multiplier: f32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogBundledItem {
    discount_prices: Vec<LolCatalogBundledItemCost>,
    inventory_type: String,
    item_id: i32,
    quantity: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubSummonerIcon {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatTranslateResponse {
    results: Vec<LolChatTranslateResult>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashSuggestedInvite {
    suggester_summoner_id: u64,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHonorV2EndOfGamePlayer {
    leaver: bool,
    game_id: u64,
    champion_name: String,
    bot_player: bool,
    puuid: String,
    summoner_name: String,
    summoner_id: u64,
    skin_splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsSeriesOpt {
    series_id: String,
    option: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGamePlayerReport {
    offender_summoner_id: u64,
    categories: Vec<String>,
    offender_puuid: String,
    game_id: u64,
    comment: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RewardLogo {
    member_owned_count: i32,
    logo: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentPhase {
    id: i64,
    limit_tiers: Vec<i32>,
    capacity_status: CapacityEnum,
    period: i32,
    scouting_start_time: i64,
    tournament_id: i64,
    cancelled: bool,
    lockin_start_time: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolMissionsMissionsSettingsDataResource {
    #[serde(rename = "selected_series")]
    selected_series: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderGameAgnosticReportPayload {
    comment: String,
    categories: Vec<String>,
    token_type: String,
    location: String,
    token: String,
    offender_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesDropsDropTablePityInfo {
    pity_limit: i32,
    chase_content_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopCurrencyDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopCurrencyDto {
    sub_currencies: Value,
    amount: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectCollectionsChampionChroma {
    colors: Vec<String>,
    champion_id: i32,
    id: i32,
    ownership: LolChampSelectCollectionsOwnership,
    still_obtainable: bool,
    name: String,
    disabled: bool,
    chroma_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataSummonerIcon {
    title: String,
    image_path: String,
    id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashFindTeams {
    tournament_id: i64,
    page: i32,
    count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientDynamicConfig {
    delta: bool,
    compressed: bool,
    configs: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPremadeVoiceVoiceAvailability {
    #[serde(rename = "showDisconnectedState")]
    show_disconnected_state: bool,
    #[serde(rename = "disabledAfterLogin")]
    disabled_after_login: bool,
    #[serde(rename = "connectedToVoiceServer")]
    connected_to_voice_server: bool,
    #[serde(rename = "enabled")]
    enabled: bool,
    #[serde(rename = "voiceChannelAvailable")]
    voice_channel_available: bool,
    #[serde(rename = "showUI")]
    show_ui: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "VerboseLootOddsDTO")]
#[serde(rename_all = "camelCase")]
pub struct VerboseLootOddsDto {
    checks_ownership: bool,
    recipe_name: String,
    last_updated: String,
    has_pity_rules: bool,
    guaranteed_to_contain: Vec<LootOddsDto>,
    chance_to_contain: Vec<LootOddsDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsTopChampionMasteries {
    masteries: Vec<LolCollectionsCollectionsChampionMastery>,
    puuid: String,
    summoner_id: u64,
    score: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesCollectionsChampion {
    name: String,
    id: i32,
    square_portrait_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedSocialLeaderboardRankedQueueStats {
    tier: String,
    wins: i32,
    provisional_game_threshold: i32,
    league_points: i32,
    mini_series_progress: String,
    provisional_games_remaining: i32,
    queue_type: LolRankedLeagueQueueType,
    rated_rating: i32,
    is_provisional: bool,
    rated_tier: LolRankedRatedTier,
    losses: i32,
    division: LolRankedLeagueDivision,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolDropsCapDropsDropTableWithPityDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolDropsCapDropsDropTableWithPityDto {
    start_date: String,
    total_rolls_info: LolDropsTotalRollsInfoDto,
    pity_info: LolDropsCapDropsDropTablePityInfo,
    product_id: String,
    currency_id: String,
    cost: u16,
    roll_offer: String,
    display_metadata: LolDropsCapDropsDropTableDisplayMetadata,
    source_id: String,
    end_date: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsBotParticipant {
    summoner_id: u64,
    summoner_internal_name: String,
    summoner_name: String,
    bot_skill_level: i32,
    team_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatRsoAuthorization {
    subject: String,
    current_account_id: u64,
    current_platform_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksChampSelectSession {
    local_player_cell_id: i64,
    actions: Vec<Value>,
    bans: LolPerksChampSelectBannedChampions,
    chat_details: LolPerksChampSelectChatRoomDetails,
    is_spectating: bool,
    my_team: Vec<LolPerksChampSelectPlayerSelection>,
    trades: Vec<LolPerksChampSelectTradeContract>,
    timer: LolPerksChampSelectTimer,
    their_team: Vec<LolPerksChampSelectPlayerSelection>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiotMessagingServiceSession {
    token: String,
    state: RiotMessagingServiceState,
    token_type: RiotMessagingServiceTokenType,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerProfilePrivacy {
    enabled_state: LolSummonerProfilePrivacyEnabledState,
    setting: LolSummonerProfilePrivacySetting,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsQuestSkinDescriptionInfo {
    description: String,
    icon_path: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksRuneRecommendationGDSResource")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksRuneRecommendationGdsResource {
    map_id: i32,
    is_default_position: bool,
    position: String,
    min_summoner_level: u32,
    perk_ids: Vec<i32>,
    secondary_perk_style_id: i32,
    recommendation_id: String,
    primary_perk_style_id: i32,
    summoner_spell_ids: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectSwapNotification {
    requestor_index: i64,
    initiated_by_local_player: bool,
    other_summoner_index: i64,
    id: i64,
    responder_index: i64,
    state: LolChampSelectChampSelectSwapState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolBannersClashV2FlagRewardSpec {
    season_id: String,
    level: String,
    theme: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsHarassmentReport {
    comment: String,
    reporting_summoner_id: u64,
    reported_summoner_id: u64,
    offense: String,
    game_id: u64,
    report_source: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubRewardTrackXP")]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubRewardTrackXp {
    #[serde(rename = "totalLevelXP")]
    total_level_xp: i64,
    #[serde(rename = "currentLevelXP")]
    current_level_xp: i64,
    #[serde(rename = "currentLevel")]
    current_level: i64,
    #[serde(rename = "isBonusPhase")]
    is_bonus_phase: bool,
    #[serde(rename = "iteration")]
    iteration: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashBracketReadyNotification {
    tournament_id: i64,
    bracket_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubPurchaseOfferOrderStatuses {
    statuses: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLoginSimpleMessage {
    account_id: u64,
    _type: String,
    params: Vec<String>,
    msg_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampSelectLegacyCollectionsOwnership {
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "rental")]
    rental: LolChampSelectLegacyCollectionsRental,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootDropTableEntryGdsResource {
    loot_id: String,
    localized_description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubItemKey {
    item_id: i32,
    inventory_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootRecipeGdsResource {
    image_path: String,
    outputs: Vec<LolLootLootOutputGdsResource>,
    id: String,
    single_open: bool,
    description: String,
    requirement_text: String,
    has_visible_loot_odds: bool,
    intro_video_path: String,
    loop_video_path: String,
    outro_video_path: String,
    context_menu_text: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThirdPartyApiPlayer {
    summoner_id: u64,
    role: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataWardSkin {
    id: i64,
    description: String,
    ward_image_path: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashKickRequest {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameClientChatMutedPlayerInfo {
    puuid: String,
    summoner_id: u64,
    obfuscated_puuid: String,
    obfuscated_summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolRankedRankedStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedStatsDto {
    previous_season_split_points: i32,
    seasons: Value,
    highest_previous_season_end_rank: String,
    splits_progress: Value,
    current_season_split_points: i32,
    highest_previous_season_end_tier: String,
    queues: Vec<LolRankedRankedQueueStatsDto>,
    earned_regalia_reward_ids: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetCatalogPluginItemWithDetails {
    assets: LolPurchaseWidgetCatalogPluginItemAssets,
    item: LolPurchaseWidgetCatalogPluginItem,
    quantity: u32,
    bundled_items: Vec<LolPurchaseWidgetCatalogPluginItemWithDetails>,
    minimum_bundle_prices: Vec<LolPurchaseWidgetCatalogPluginPrice>,
    required_items: Vec<LolPurchaseWidgetCatalogPluginItemWithDetails>,
    bundled_discount_prices: Vec<LolPurchaseWidgetCatalogPluginPrice>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PlayerNotificationsRiotMessagingServiceMessage {
    service: String,
    payload: String,
    version: String,
    timestamp: i64,
    resource: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameQueuesQueueTranslation {
    detailed_description: String,
    name: String,
    short_name: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyPositionPreferences {
    second_preference: String,
    first_preference: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksUISettings")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUiSettings {
    show_preset_pages: bool,
    gameplay_updated_perks_seen: Vec<i32>,
    show_long_descriptions: bool,
    grid_mode_enabled: bool,
    gameplay_patch_version_seen: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedGlobalNotification {
    participant_id: String,
    queue_type: String,
    notify_reason: String,
    tier: String,
    position: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEsportStreamNotificationsLiveMatchTeam {
    guid: String,
    acronym: String,
    name: String,
    logo_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesStatstoneMasteryVignette {
    num_sets_completed: u32,
    puuid: String,
    completed_set_uuids: Vec<String>,
    mastery_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyChampSelectChatRoomDetails {
    multi_user_chat_password: String,
    muc_jwt_dto: LolChampSelectLegacyMucJwtDto,
    multi_user_chat_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderLobbyInvitation {
    invitation_meta_data: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateGameInfo {
    game_queue_type: String,
    puuid: String,
    allow_observe_mode: String,
    drop_in_spectate_game_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventCollectionsChampionSkin {
    champion_id: i32,
    id: i32,
    ownership: LolTftEventCollectionsOwnership,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolYourshopYourshopConfig {
    promotion_name: String,
    active: bool,
    themed_background: bool,
    promotion_start_date: String,
    promotion_end_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LcdsPayloadDto {
    method: String,
    path: String,
    headers: Value,
    body: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolItemSetsPreferredItemSlot {
    id: String,
    preferred_item_slot: i16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPatchPatchSieveReleaseInfo {
    product: String,
    id: String,
    labels: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerRosterHistoryDTO")]
#[serde(rename_all = "kebab-case")]
pub struct PlayerRosterHistoryDto {
    rosters: Vec<RosterMemberDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashTournamentHistoryAndWinners {
    tournament_history: Vec<LolClashTournament>,
    tournament_winners: LolClashTournamentWinnerHistory,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameSummoner {
    puuid: String,
    summoner_level: u32,
    xp_since_last_level: u64,
    xp_until_next_level: u64,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitMatchHistoryGameList {
    game_index_end: u64,
    game_begin_date: String,
    game_index_begin: u64,
    game_end_date: String,
    game_count: u64,
    games: Vec<LolHoneyfruitMatchHistoryGame>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoyaltyAccessToken {
    token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPublishingContentSystemInfo {
    operating_system: LolPublishingContentSystemInfoOperatingSystem,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplayContextData {
    component_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MissionMetadata {
    xp_reward: i32,
    order: i32,
    npe_reward_pack: NpeRewardPackMetadata,
    chain: i32,
    week_num: i32,
    chain_size: i32,
    tutorial: TutorialMetadata,
    mission_type: String,
    level: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameDataCompanion {
    color_name: String,
    species_name: String,
    content_id: String,
    loadouts_icon: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryMatchHistoryList {
    games: LolMatchHistoryMatchHistoryGameList,
    platform_id: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventLolTftHomeHub {
    override_url: String,
    enabled: bool,
    battle_pass_offer_ids: Vec<String>,
    prime_gaming_promo_offer: LolTftEventLolTftPrimeGaming,
    store_promo_offer_ids: Vec<String>,
    fallback_store_promo_offer_ids: Vec<String>,
    tactician_promo_offer_ids: Vec<String>,
    header_buttons_override_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceLocalSettingsCategoryDataResource {
    input_volume: u32,
    vad_sensitivity: u32,
    current_capture_device_handle: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitSettingCategoryResource {
    schema_version: u32,
    data: LolHoneyfruitHoneyfruitSettings,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ClientConfigConfigNamespaceUpdate {
    player: Vec<String>,
    public: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLeaverBusterAllSummonerData {
    summoner: LolLeaverBusterSummoner,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "YourshopLcdsChampionDTO")]
#[serde(rename_all = "camelCase")]
pub struct YourshopLcdsChampionDto {
    champion_id: i32,
    owned: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerAutoFillQueueDto {
    queue_id: i32,
    auto_fill_eligible: bool,
    auto_fill_protected_for_streaking: bool,
    auto_fill_protected_for_promos: bool,
    auto_fill_protected_for_remedy: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHoneyfruitHoneyfruitSettingCategoryResourceAccountClaim {
    schema_version: u32,
    data: LolHoneyfruitHoneyfruitSettingsAccountClaim,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomJoinParameters {
    as_spectator: bool,
    password: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigSession {
    product_id: String,
    version: String,
    is_internal: bool,
    patchline_id: String,
    connections: Vec<ClientConfigAuthenticatedConnection>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventPlayerUpdateResponse {
    player_series: Vec<SeriesDto>,
    player_missions: Vec<PlayerMissionDto>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerMissionObjectiveDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerMissionObjectiveDto {
    has_objective_based_reward: bool,
    requirements: Vec<String>,
    description: String,
    reward_groups: Vec<String>,
    sequence: i32,
    status: String,
    _type: String,
    progress: MissionProgressDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginSummonerCreatedResource {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubEventDetailsUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubEventDetailsUiData {
    promotion_banner_image: String,
    event_name: String,
    shop_end_date: String,
    event_icon_path: String,
    help_modal_image_path: String,
    inductee_name: String,
    event_start_date: String,
    progress_end_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowLobbyStatus {
    lobby_id: String,
    invited_summoner_ids: Vec<u64>,
    is_practice_tool: bool,
    is_leader: bool,
    allowed_play_again: bool,
    queue_id: i32,
    member_summoner_ids: Vec<u64>,
    is_custom: bool,
    is_spectator: bool,
    custom_spectator_policy: LolGameflowQueueCustomGameSpectatorPolicy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGameSettingsgamesettingsgameclient {
    running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogBundledItemCost {
    cost_type: String,
    discount: f32,
    currency: String,
    cost: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedLcuLeagueNotification {
    time_until_inactivity_status_changes: i64,
    win_streak: i32,
    promo_series_for_ranks_enabled: bool,
    losses: i32,
    reward_earned_type: String,
    afk_lp_penalty_level: i32,
    provisional_games_remaining: i32,
    was_afk_or_leaver: bool,
    queue_type: LolRankedLeagueQueueType,
    notify_reason: String,
    can_demote_from_tier: bool,
    miniseries_wins: i32,
    league_points: i32,
    afk_lp_penalty_amount: i32,
    game_id: u64,
    rated_tier: LolRankedRatedTier,
    rated_rating_delta: i32,
    msg_id: String,
    tier: String,
    number_of_promotions: u64,
    eligible_for_promo_helper: bool,
    reward_earned_id: String,
    split_points_notification: LolRankedSplitPointsNotification,
    id: u64,
    consolation_lp_used: i32,
    wins: i32,
    reward_override_image_path: String,
    display_type: LolRankedNotificationDisplayType,
    league_points_delta: i32,
    rated_rating: i32,
    change_reason: String,
    division: LolRankedLeagueDivision,
    miniseries_progress: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEsportStreamNotificationsESportStreamNotificationsConfig {
    #[serde(rename = "beappFailureLongPollMinutes")]
    beapp_failure_long_poll_minutes: i64,
    #[serde(rename = "notificationsStreamURL")]
    notifications_stream_url: String,
    #[serde(rename = "notificationsStreamGroupSlug")]
    notifications_stream_group_slug: String,
    #[serde(rename = "notificationsAssetMagickURL")]
    notifications_asset_magick_url: String,
    #[serde(rename = "notificationsLongPollMinutes")]
    notifications_long_poll_minutes: i64,
    #[serde(rename = "useServiceEndpointV2")]
    use_service_endpoint_v_2: bool,
    #[serde(rename = "notificationsEnabled")]
    notifications_enabled: bool,
    #[serde(rename = "notificationsShortPollMinutes")]
    notifications_short_poll_minutes: i64,
    #[serde(rename = "notificationsServiceEndpointV2")]
    notifications_service_endpoint_v_2: String,
    #[serde(rename = "notificationsServiceEndpoint")]
    notifications_service_endpoint: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPremadeVoiceGameflowGameClient {
    running: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPublishingContentBuildInfo {
    patchline: String,
    version: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengeSeason {
    season_id: i32,
    season_start: i64,
    season_end: i64,
    is_active: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesUIPlayerSummary")]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesUiPlayerSummary {
    is_apex: bool,
    apex_ladder_update_time: i64,
    selected_challenges_string: String,
    position_percentile: f64,
    apex_leaderboard_position: i32,
    overall_challenge_level: String,
    banner_id: String,
    title: LolChallengesUiTitle,
    crest_id: String,
    category_progress: Vec<LolChallengesUiCategoryProgress>,
    total_challenge_score: i64,
    prestige_crest_border_level: i32,
    points_until_next_rank: i64,
    top_challenges: Vec<LolChallengesUiChallenge>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticSettingsResource {
    schema_version: u32,
    data: LolCosmeticsAccountSettingsCategoryDataResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueWarnings {
    display_decay_warning: bool,
    days_until_decay: i32,
    time_until_inactivity_status_changes: i64,
    demotion_warning: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolServiceStatusRegionLocaleResource {
    region: String,
    locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PlayerFinderDTO")]
#[serde(rename_all = "camelCase")]
pub struct PlayerFinderDto {
    primary_pos: Position,
    _type: PlayerFinderEnum,
    friend_id: i64,
    player_id: u64,
    tier: i32,
    secondary_pos: Position,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsSeasonMetaData {
    public_name: String,
    current_split: i32,
    total_split: i32,
    loc_key: String,
    year: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyCustomFailedPlayer {
    summoner_name: String,
    reason: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "SeriesDTO")]
#[serde(rename_all = "camelCase")]
pub struct SeriesDto {
    media: SeriesMediaDto,
    warnings: Vec<AlertDto>,
    id: String,
    tags: Vec<String>,
    end_date: i64,
    status: String,
    parent_internal_name: String,
    description: String,
    start_date: i64,
    created_date: i64,
    eligibility_type: String,
    title: String,
    _type: String,
    display_type: String,
    opt_in_button_text: String,
    opt_out_button_text: String,
    viewed: bool,
    last_updated_timestamp: i64,
    internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLeaverBusterPlayerNotificationResource {
    id: u64,
    background_url: String,
    _type: String,
    created: String,
    data: Value,
    icon_url: String,
    critical: bool,
    source: String,
    expires: String,
    state: String,
    detail_key: String,
    title_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSuggestedPlayersEndOfGamePlayer {
    summoner_name: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventPublishingSettings {
    publishing_locale: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectMutedPlayerInfo {
    puuid: String,
    summoner_id: u64,
    obfuscated_puuid: String,
    obfuscated_summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolYourshopOfferIds {
    offers: Vec<LolYourshopOfferId>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeepLinksDeepLinksSettings {
    launch_lor_url: String,
    is_scheme_ready: bool,
    launch_lor_enabled: bool,
    external_client_scheme: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "ChampSelectLcdsTradeContractDTO")]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsTradeContractDto {
    requester_summoner_id: u64,
    responder_champion_id: i32,
    requester_champion_id: i32,
    state: String,
    responder_internal_summoner_name: String,
    responder_puuid: String,
    requester_puuid: String,
    requester_internal_summoner_name: String,
    responder_summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEventHubBundleOfferUIData")]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubBundleOfferUiData {
    discount_percentage: f64,
    bundled_items: Vec<LolEventHubBundledItemUiData>,
    details: LolEventHubBundledItemUiData,
    initial_price: i64,
    final_price: i64,
    future_balance: i64,
    is_purchasable: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LCDSPlayerMessagingSimpleMessage")]
#[serde(rename_all = "camelCase")]
pub struct LcdsPlayerMessagingSimpleMessage {
    title_code: String,
    account_id: u64,
    params: Vec<String>,
    _type: String,
    body_code: String,
    msg_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyPartiesInvitationAnalytics {
    event_type: String,
    event_data: Value,
    event_timestamp: u64,
    players: Vec<LolLobbyPartiesInvitationPlayerAnalytics>,
    platform_id: String,
    party_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolSummonerSummonerDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerSummonerDto {
    puuid: String,
    exp_to_next_level: u32,
    level: u32,
    name_change_flag: bool,
    id: u64,
    name: String,
    account_id: u64,
    exp_points: u32,
    privacy: LolSummonerProfilePrivacySetting,
    partner_id: String,
    profile_icon_id: i32,
    unnamed: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolTftEventCollectionsOwnership {
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
    #[serde(rename = "rental")]
    rental: LolTftEventCollectionsRental,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopOfferRequest {
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesEndOfGameStats {
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsMissionsNotificationResource {
    icon_url: String,
    created: String,
    critical: bool,
    state: String,
    _type: String,
    source: String,
    data: Value,
    title_key: String,
    background_url: String,
    detail_key: String,
    expires: String,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryTopChampionMasteries {
    masteries: Vec<LolChampionMasteryChampionMastery>,
    puuid: String,
    score: u64,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeRewardsMission {
    objectives: Vec<LolNpeRewardsObjective>,
    status: String,
    series_name: String,
    completed_date: i64,
    metadata: LolNpeRewardsMissionsRewardPackMetaData,
    display: LolNpeRewardsMissionDisplay,
    internal_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolVanguardVanguardMachineSpecs {
    tpm_2_enabled: bool,
    secure_boot_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPremadeVoiceFirstExperience {
    #[serde(rename = "showFirstExperienceInGame")]
    show_first_experience_in_game: bool,
    #[serde(rename = "showFirstExperienceInLCU")]
    show_first_experience_in_lcu: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventGameflowGameData {
    queue: LolTftEventQueue,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryRmsEntitlementPayload {
    item_type_id: String,
    entitlement_type_id: String,
    item_id: String,
    resource_operation: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeTutorialPathMissionMetadata {
    tutorial: LolNpeTutorialPathTutorialMetadata,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatChatPlatformLoginSession {
    state: LolChatChatPlatformLoginSessionState,
    id_token: String,
    summoner_id: u64,
    account_id: u64,
    username: String,
    puuid: String,
    is_new_player: bool,
    user_auth_token: String,
    gas_token: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLoginLcdsResponse {
    body: Value,
    type_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolLoyaltyLoyaltyRewardsSimplified {
    #[serde(rename = "global")]
    global: LolLoyaltyGlobalRewards,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Value,
    #[serde(rename = "ipBoost")]
    ip_boost: i32,
    #[serde(rename = "loyaltyTFTMapSkinCount")]
    loyalty_tft_map_skin_count: i32,
    #[serde(rename = "freeRewardedChampionsCount")]
    free_rewarded_champions_count: i32,
    #[serde(rename = "championIds")]
    champion_ids: Vec<i32>,
    #[serde(rename = "loyaltyTFTCompanionCount")]
    loyalty_tft_companion_count: i32,
    #[serde(rename = "freeRewardedSkinsCount")]
    free_rewarded_skins_count: i32,
    #[serde(rename = "skinIds")]
    skin_ids: Vec<i32>,
    #[serde(rename = "xpBoost")]
    xp_boost: i32,
    #[serde(rename = "loyaltyTFTDamageSkinCount")]
    loyalty_tft_damage_skin_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsCosmeticsTFTPlaybookAugment")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCosmeticsTftPlaybookAugment {
    name: String,
    effect_amounts: Vec<LolCosmeticsCosmeticsTftPlaybookAugmentEffectAmount>,
    icon_path: String,
    description: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPerksPerkStyleSlotResource {
    _type: String,
    slot_label: String,
    perks: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubBalance {
    currency_type: String,
    amount: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSummonerLoginSession {
    state: LolSummonerLoginSessionStates,
    connected: bool,
    account_id: u64,
    puuid: String,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyQueue {
    are_free_champions_allowed: bool,
    game_type_config: LolChampSelectLegacyQueueGameTypeConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMatchHistoryMatchHistoryTimelineFrames {
    frames: Vec<LolMatchHistoryMatchHistoryTimelineFrame>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "MissionsCompressedPayloadDTO")]
#[serde(rename_all = "camelCase")]
pub struct MissionsCompressedPayloadDto {
    missions_compressed: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolSummonerAliasLookupResponse {
    alias: LolSummonerAlias,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesPlayerNotificationResource {
    expires: String,
    background_url: String,
    icon_url: String,
    data: Value,
    source: String,
    state: String,
    title_key: String,
    _type: String,
    critical: bool,
    id: u64,
    detail_key: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectLegacyQueueGameTypeConfig {
    name: String,
    max_allowable_bans: i32,
    battle_boost: bool,
    allow_trades: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterStats {
    start_time_ms: i64,
    roster_short_name: String,
    roster_name: String,
    roster_id: i64,
    tournament_periods: i32,
    tier: i32,
    roster_icon_color_id: i32,
    tournament_name_loc_key: String,
    tournament_theme_id: i32,
    roster_icon_id: i32,
    tournament_name_loc_key_secondary: String,
    end_time_ms: i64,
    period_stats: Vec<LolClashRosterPeriodAggregatedStats>,
    player_stats: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyReceivedInvitationDto {
    from_summoner_id: u64,
    game_config: LolLobbyReceivedInvitationGameConfigDto,
    restrictions: Vec<LolLobbyEligibilityRestriction>,
    invitation_id: String,
    timestamp: String,
    from_summoner_name: String,
    invitation_type: LolLobbyInvitationType,
    can_accept_invitation: bool,
    state: LolLobbyLobbyInvitationState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderChampSelectDevPanelData {
    queue_id: i32,
    dto_index: i64,
    counter: i64,
    team_id_suffix: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootVerboseLootOddsResponse {
    recipe_name: String,
    chance_to_contain: Vec<LolLootLootOddsResponse>,
    guaranteed_to_contain: Vec<LolLootLootOddsResponse>,
    loot_item: LolLootPlayerLoot,
    has_pity_rules: bool,
    checks_ownership: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct MetricMetadataPagerDutyNotification {
    apikey: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubExternalCatalogPluginItemAssets {
    icon_path: String,
    splash_path: String,
    tile_path: String,
    colors: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPatchComponentStateProgress {
    bytes_complete: u64,
    bytes_per_second: f64,
    bytes_required: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootMilestonesProgressionCounterGdsResource {
    id: String,
    name: String,
    start_value: i32,
    direction: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RiotMessagingServiceAcknowledgeBody {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectChampSelectTradeNotification {
    state: LolChampSelectChampSelectTradeState,
    initiated_by_local_player: bool,
    responder_champion_name: String,
    requester_champion_name: String,
    id: i64,
    responder_index: i64,
    requester_champion_splash_path: String,
    other_summoner_index: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolHonorV2SequenceEvent {
    name: String,
    priority: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyGameDataChampionSummary {
    id: i32,
    alias: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubLoginSession {
    account_id: u64,
    summoner_id: u64,
    puuid: String,
    id_token: String,
    state: LolEventHubLoginSessionStates,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingGameflowGameData {
    queue: LolContentTargetingQueue,
    game_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLoginRSOConfigReadyState")]
#[serde(rename_all = "kebab-case")]
pub struct LolLoginRsoConfigReadyState {
    ready: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsCompanionsGroupViewModel {
    num_owned: u32,
    group_name: String,
    group_id: u32,
    num_available: u32,
    items: Vec<LolCosmeticsCosmeticsCompanionViewModel>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChampionMasteryUIChampionMasteryCustomReward")]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryUiChampionMasteryCustomReward {
    level: i32,
    reward_value: String,
    _type: String,
    quantity: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootRecipeWithMilestones {
    _type: String,
    outro_video_path: String,
    metadata: LolLootRecipeMetadata,
    loot_milestone_ids: Vec<String>,
    context_menu_text: String,
    description: String,
    loop_video_path: String,
    outputs: Vec<LolLootRecipeOutput>,
    single_open: bool,
    recipe_name: String,
    display_categories: String,
    intro_video_path: String,
    requirement_text: String,
    crafter_name: String,
    image_path: String,
    slots: Vec<LolLootRecipeSlot>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSeasonsAllProductSeasonQuery {
    last_n_years: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashMatchmakingDodgeData {
    state: LolClashMatchmakingDodgeState,
    dodger_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolEndOfGameTFTEndOfGameCompanionViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameTftEndOfGameCompanionViewModel {
    species_name: String,
    color_name: String,
    icon: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsInventoryItemWithPayload {
    uuid: String,
    purchase_date: String,
    item_id: i32,
    inventory_type: String,
    quantity: u64,
    ownership_type: LolCollectionsItemOwnershipType,
    payload: Value,
}
type LolEventHubResponseMetadataDto = Value;
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolFeaturedModesCollectionsSummonerIcon {
    icon_id: i32,
    ownership: LolFeaturedModesCollectionsOwnership,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolCatalogChampionSkinEmblemPath {
    small: String,
    large: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatCidBody {
    cid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRepeatGroupTrigger {
    increase_by: u16,
    multiplier: f32,
    counter_id: String,
    _type: String,
    start_trigger_value: u16,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowPlayerStatus {
    last_queued_lobby_status: LolGameflowLobbyStatus,
    current_lobby_status: LolGameflowLobbyStatus,
    can_invite_others_at_eog: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassLoginSession {
    account_id: u64,
    state: LolTftPassLoginSessionStates,
    puuid: String,
    summoner_id: u64,
    id_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftEventGameflowSession {
    phase: LolTftEventGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolHovercardAlias {
    game_name: String,
    tag_line: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubExternalCatalogItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEndOfGameGameflowSession {
    phase: LolEndOfGameGameflowPhase,
    game_data: LolEndOfGameGameflowGameData,
    game_client: LolEndOfGameGameflowClient,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "StoreLcdsChampionDTO")]
#[serde(rename_all = "PascalCase")]
pub struct StoreLcdsChampionDto {
    #[serde(rename = "rankedPlayEnabled")]
    ranked_play_enabled: bool,
    #[serde(rename = "freeToPlayReward")]
    free_to_play_reward: bool,
    #[serde(rename = "freeToPlay")]
    free_to_play: bool,
    #[serde(rename = "championSkins")]
    champion_skins: Vec<StoreLcdsChampionSkinDto>,
    #[serde(rename = "championId")]
    champion_id: i32,
    #[serde(rename = "sources")]
    sources: Vec<String>,
    #[serde(rename = "endDate")]
    end_date: u64,
    #[serde(rename = "purchaseDate")]
    purchase_date: u64,
    #[serde(rename = "active")]
    active: bool,
    #[serde(rename = "winCountRemaining")]
    win_count_remaining: u32,
    #[serde(rename = "f2pRewardSources")]
    f_2_p_reward_sources: Vec<String>,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "botEnabled")]
    bot_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PluginManagerResource {
    state: PluginManagerState,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolNpeTutorialPathTutorialReward {
    icon_url: String,
    reward_fulfilled: bool,
    sequence: i32,
    unique_name: String,
    description: String,
    reward_type: String,
    item_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyInventoryItem {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerLevelUpGameDataSummonerSpell {
    summoner_level: u32,
    id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolAccountVerificationResponseError {
    error_code: String,
    message: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolMatchHistoryAcsEndPoint {
    url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreSale {
    end_date: String,
    prices: Vec<LolStoreItemCost>,
    start_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventorySimpleInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolInventorySimpleInventoryResponseDto {
    data: LolInventorySimpleInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChallengesGameDataChallengesData {
    titles: Value,
    challenges: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchHistoryLoginSession {
    account_id: u64,
    summoner_id: u64,
    state: LolMatchHistoryLoginSessionStates,
    id_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetChampionSkinEmblemPath {
    large: String,
    small: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesGameDataStatstonePack {
    description: String,
    content_id: String,
    item_id: i32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingLowPriorityDataResource {
    penalized_summoner_ids: Vec<u64>,
    penalty_time: f64,
    busted_leaver_access_token: String,
    reason: String,
    penalty_time_remaining: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolContentTargetingGameflowSession {
    game_data: LolContentTargetingGameflowGameData,
    phase: LolContentTargetingGameflowPhase,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootProgressionConfigGdsResource {
    counters: Vec<LolLootMilestonesProgressionCounterGdsResource>,
    milestones: Vec<LolLootProgressionConfigMilestoneGdsResource>,
    id: String,
    name: String,
    repeat: LolLootMilestonesProgressionConfigRepeatGdsResource,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftPassMilestoneInstance {
    milestone_id: String,
    owner_id: String,
    triggered: bool,
    trigger_value: i64,
    instance_id: String,
    repeat_sequence: u32,
    counter_id: String,
    product_id: String,
    group_id: String,
    triggered_timestamp: String,
    triggers: Vec<LolTftPassTrigger>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatActiveConversationResource {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EndOfGameLcdsTeamInfo {
    member_status_string: String,
    team_id: EndOfGameLcdsTeamId,
    tag: String,
    seconds_until_eligible_for_deletion: i64,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyLobbyMatchmakingSearchErrorResource {
    message: String,
    penalized_summoner_id: u64,
    error_type: String,
    id: i32,
    penalty_time_remaining: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolCosmeticsTFTDamageSkinGroupedViewModel")]
#[serde(rename_all = "camelCase")]
pub struct LolCosmeticsTftDamageSkinGroupedViewModel {
    default_item_id: i32,
    groups: Vec<LolCosmeticsTftDamageSkinGroupViewModel>,
    selected_loadout_item: LolCosmeticsCosmeticsTftDamageSkinViewModel,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPftPFTSurveyV1")]
#[serde(rename_all = "kebab-case")]
pub struct LolPftPftSurveyV1 {
    url: String,
    id: u64,
    _type: String,
    caption: String,
    title: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolServiceStatusTickerMessage {
    created_at: String,
    updated_at: String,
    message: String,
    heading: String,
    severity: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolVanguardLoginSession {
    puuid: String,
    account_id: u64,
    state: LolVanguardLoginSessionState,
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootGroup {
    product_id: String,
    repeat: LolLootRepeat,
    counters: Vec<LolLootCounter>,
    name: String,
    milestones: Vec<LolLootMilestone>,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashRosterPhaseInfo {
    checkin_time: i64,
    phase_id: i64,
    is_bracket_complete: bool,
    period: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolServiceStatusLegacyServiceStatusMessage {
    #[serde(rename = "updated_at")]
    updated_at: String,
    #[serde(rename = "content")]
    content: String,
    #[serde(rename = "translations")]
    translations: Vec<LolServiceStatusLegacyServiceStatusTranslation>,
    #[serde(rename = "severity")]
    severity: String,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "heading")]
    heading: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChampSelectLcdsPlayerParticipant {
    summoner_internal_name: String,
    puuid: String,
    pick_turn: i32,
    summoner_name: String,
    summoner_id: u64,
    pick_mode: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsSimpleMessage {
    _type: String,
    msg_id: String,
    params: Vec<String>,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubItemCost {
    discount: f32,
    cost: i64,
    currency: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerReportSenderGameflowSession {
    phase: LolPlayerReportSenderGameflowPhase,
    game_data: LolPlayerReportSenderGameflowGameData,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRegaliaAsync {
    md_5: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PublicRosterDTO")]
#[serde(rename_all = "camelCase")]
pub struct PublicRosterDto {
    short_name: String,
    id: i64,
    logo: i32,
    logo_color: i32,
    tournament_id: i64,
    member_ids: Vec<u64>,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolChallengesChallengesRMSNotification")]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesChallengesRmsNotification {
    ack_required: bool,
    payload: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderGatekeeperRestricted {
    gatekeeper_restrictions: Vec<LolLobbyTeamBuilderGatekeeperRestriction>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLootMilestonesRecipeGdsResource {
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogGameDataSkinAugment {
    image: String,
    item_id: i32,
    cap_type: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventoryWalletDTO")]
#[serde(rename_all = "camelCase")]
pub struct LolInventoryWalletDto {
    puuid: String,
    balances: Value,
    account_id: i64,
    balances_jwt: String,
    expires: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsCollectionsRental {
    end_date: u64,
    purchase_date: u64,
    win_count_remaining: i32,
    rented: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubRmsStoreEntitlementPayload {
    items: Vec<LolEventHubRmsStoreEntitlementItem>,
    transaction_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueue {
    game_mode: String,
    id: i32,
    minimum_participant_list_size: i32,
    queue_rewards: LolGameflowQueueReward,
    champions_required_to_play: u32,
    map_id: i32,
    allowable_premade_sizes: Vec<i32>,
    name: String,
    description: String,
    detailed_description: String,
    is_ranked: bool,
    removal_from_game_delay_minutes: i32,
    category: LolGameflowQueueGameCategory,
    num_players_per_team: i32,
    asset_mutator: String,
    is_team_builder_managed: bool,
    last_toggled_off_time: u64,
    _type: String,
    spectator_enabled: bool,
    last_toggled_on_time: u64,
    min_level: u32,
    show_position_selector: bool,
    short_name: String,
    are_free_champions_allowed: bool,
    removal_from_game_allowed: bool,
    maximum_participant_list_size: i32,
    queue_availability: LolGameflowQueueAvailability,
    game_type_config: LolGameflowQueueGameTypeConfig,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPerksValidatePageNameData {
    id: i32,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRewardsInfo {
    current_split: LolRankedSeasonSplit,
    splits: Vec<LolRankedSeasonSplit>,
    current_season_id: i32,
    reward_info_by_reward_id: Value,
    current_split_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubValidationErrorEntry {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueGameTypeConfig {
    pick_mode: String,
    exclusive_pick: bool,
    learning_quests: bool,
    battle_boost: bool,
    do_not_remove: bool,
    main_pick_timer_duration: i32,
    reroll: bool,
    post_pick_timer_duration: i32,
    death_match: bool,
    ban_timer_duration: i32,
    cross_team_champion_pool: bool,
    advanced_learning_quests: bool,
    name: String,
    ban_mode: String,
    onboard_coop_beginner: bool,
    id: i64,
    allow_trades: bool,
    max_allowable_bans: i32,
    team_champion_pool: bool,
    duplicate_pick: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolProgressionGroup {
    repeat: LolProgressionRepeat,
    id: String,
    counters: Vec<LolProgressionCounter>,
    milestones: Vec<LolProgressionMilestone>,
    product_id: String,
    name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetSale {
    start_date: String,
    end_date: String,
    prices: Vec<LolPurchaseWidgetItemCost>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogItemChoiceDetails {
    background_image: String,
    contents: Vec<LolCatalogItemDetails>,
    discount: String,
    full_price: i64,
    display_type: String,
    metadata: Value,
    item: LolCatalogCatalogPluginItem,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEventHubCatalogPluginSale {
    cost: i64,
    discount: f32,
    end_date: String,
    start_date: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderCountdownTimer {
    timer: i64,
    phase_name: String,
    counter: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolPerksUIPerkMinimal")]
#[serde(rename_all = "camelCase")]
pub struct LolPerksUiPerkMinimal {
    name: String,
    style_id: i32,
    slot_type: String,
    id: i32,
    icon_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftTrovesTrovesPurchaseRequest {
    payment_option: String,
    quantity: u32,
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRankedRankedQueueStats {
    previous_season_end_tier: String,
    queue_type: LolRankedLeagueQueueType,
    previous_season_end_division: LolRankedLeagueDivision,
    division: LolRankedLeagueDivision,
    provisional_games_remaining: i32,
    rated_rating: i32,
    provisional_game_threshold: i32,
    highest_division: LolRankedLeagueDivision,
    tier: String,
    warnings: LolRankedRankedQueueWarnings,
    losses: i32,
    is_provisional: bool,
    previous_season_highest_division: LolRankedLeagueDivision,
    mini_series_progress: String,
    rated_tier: LolRankedRatedTier,
    highest_tier: String,
    previous_season_highest_tier: String,
    wins: i32,
    league_points: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolEmailVerificationRemoteEmailVerificationSession {
    email_verified: bool,
    email: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolKrShutdownLawRatingScreenInfo {
    shown: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampionQuestSkin {
    stage: u64,
    still_obtainable: bool,
    tile_path: String,
    splash_video_path: String,
    description: String,
    collection_splash_video_path: String,
    chroma_path: String,
    name: String,
    splash_path: String,
    short_name: String,
    disabled: bool,
    id: i32,
    is_base: bool,
    champion_id: i32,
    uncentered_splash_path: String,
    last_selected: bool,
    load_screen_path: String,
    ownership: LolChampionsCollectionsOwnership,
    skin_augments: LolChampionsCollectionsChampionSkinAugments,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsPlayerParticipant {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CrashReportingEnvironment {
    environment: String,
    user_id: String,
    user_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetItemChoices {
    choices: Vec<LolPurchaseWidgetItemChoiceDetails>,
    validation_errors: Vec<LolPurchaseWidgetValidationErrorEntry>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsRewardsProductConfig {
    service_url: String,
    enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventCollectionsChampion {
    free_to_play: bool,
    skins: Vec<LolTftEventCollectionsChampionSkin>,
    id: i32,
    ownership: LolTftEventCollectionsOwnership,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampSelectChampGridChampion {
    #[serde(rename = "roles")]
    roles: Vec<String>,
    #[serde(rename = "squarePortraitPath")]
    square_portrait_path: String,
    #[serde(rename = "positionsFavorited")]
    positions_favorited: Vec<String>,
    #[serde(rename = "id")]
    id: i32,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "selectionStatus")]
    selection_status: LolChampSelectChampionSelection,
    #[serde(rename = "masteryLevel")]
    mastery_level: i32,
    #[serde(rename = "freeToPlay")]
    free_to_play: bool,
    #[serde(rename = "freeToPlayForQueue")]
    free_to_play_for_queue: bool,
    #[serde(rename = "masteryPoints")]
    mastery_points: i32,
    #[serde(rename = "rented")]
    rented: bool,
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "disabled")]
    disabled: bool,
    #[serde(rename = "masteryChestGranted")]
    mastery_chest_granted: bool,
    #[serde(rename = "xboxGPReward")]
    xbox_gp_reward: bool,
    #[serde(rename = "loyaltyReward")]
    loyalty_reward: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetWallet {
    account_id: u64,
    version: i32,
    balances: Vec<LolPurchaseWidgetBalance>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashAlias {
    game_name: String,
    tag_line: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolGeoinfoWhereAmIResponse {
    region: String,
    country: String,
    city: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopRmsXboxSubscriptionChange {
    active: String,
    subscription_id: String,
    puuid: String,
    identity_provider: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCollectionsSummonerProfile {
    background_skin_id: i32,
    background_skin_augments: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStoreItemKey {
    inventory_type: String,
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolCollectionsGameDataSplashMetadata {
    calculated_color: String,
    override_color: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysReplaysConfiguration {
    is_replays_for_end_of_game_enabled: bool,
    is_logged_in: bool,
    minutes_until_replay_considered_lost: f64,
    min_server_version: String,
    is_replays_for_match_history_enabled: bool,
    is_playing_replay: bool,
    game_version: String,
    is_playing_game: bool,
    is_patching: bool,
    is_in_tournament: bool,
    is_replays_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolPlayerBehaviorPlayerBehaviorConfig {
    code_of_conduct_enabled: bool,
    is_loaded: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolYourshopUIOffer")]
#[serde(rename_all = "camelCase")]
pub struct LolYourshopUiOffer {
    owned: bool,
    expiration_date: String,
    skin_id: i32,
    original_price: i64,
    id: String,
    _type: String,
    purchasing: bool,
    skin_name: String,
    champion_id: i32,
    revealed: bool,
    discount_price: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasteryAllChampionMasterySetReward {
    season_milestone_require_and_rewards: Value,
    new_player_next_level: LolChampionMasteryLevelMark,
    champion_set: LolChampionMasteryChampionSet,
    champion_set_rewards: Value,
    champion_masteries: Vec<LolChampionMasteryChampionMastery>,
    total_score: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerParticipant {
    summoner_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPurchaseWidgetValidateOfferRequestV3 {
    offer_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPftSummoner {
    summoner_id: u64,
    unnamed: bool,
    account_id: u64,
    puuid: String,
    summoner_level: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolEventHubRewardTrackProgress {
    #[serde(rename = "futureLevelProgress")]
    future_level_progress: u16,
    #[serde(rename = "level")]
    level: i16,
    #[serde(rename = "totalLevelXP")]
    total_level_xp: i64,
    #[serde(rename = "totalLevels")]
    total_levels: i16,
    #[serde(rename = "levelProgress")]
    level_progress: u16,
    #[serde(rename = "currentLevelXP")]
    current_level_xp: i64,
    #[serde(rename = "iteration")]
    iteration: u32,
    #[serde(rename = "passProgress")]
    pass_progress: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampSelectTeamBoost {
    skin_unlock_mode: String,
    ip_reward: i64,
    price: i64,
    unlocked: bool,
    puuid: String,
    available_skins: Vec<i64>,
    ip_reward_for_purchaser: i64,
    summoner_id: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPlayerBehaviorReformCardChatLogs {
    post_game_chat_logs: Vec<String>,
    in_game_chat_logs: Vec<String>,
    pre_game_chat_logs: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolGameflowQueueReward {
    is_ip_enabled: bool,
    is_champion_points_enabled: bool,
    is_xp_enabled: bool,
    party_size_ip_rewards: Vec<i32>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolSpectatorSpectateResource {
    available_for_watching: Vec<String>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerBehaviorSimpleMessageResponse {
    msg_id: String,
    command: String,
    account_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChatEndOfGamePlayer {
    summoner_id: u64,
    is_local_player: bool,
    puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatDebugResource {
    #[serde(rename = "enableChatFiltering")]
    enable_chat_filtering: bool,
    #[serde(rename = "keepAliveInterval")]
    keep_alive_interval: u32,
    #[serde(rename = "minReconnectInterval")]
    min_reconnect_interval: u32,
    #[serde(rename = "failAllChatLogin")]
    fail_all_chat_login: bool,
    #[serde(rename = "failNextChatLogout")]
    fail_next_chat_logout: bool,
    #[serde(rename = "silenceChatWhileInGame")]
    silence_chat_while_in_game: bool,
    #[serde(rename = "asyncWaitInterval")]
    async_wait_interval: u32,
    #[serde(rename = "failNextChatLogin")]
    fail_next_chat_login: bool,
    #[serde(rename = "triggerChatDisconnect")]
    trigger_chat_disconnect: bool,
    #[serde(rename = "maxReconnectInterval")]
    max_reconnect_interval: u32,
    #[serde(rename = "failNextKeepAlive")]
    fail_next_keep_alive: bool,
    #[serde(rename = "isXMPPLoggingEnabled")]
    is_xmpp_logging_enabled: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLoadoutsLoadout {
    items: Value,
    name: String,
    id: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsCreatePracticeGameRequestDto {
    practice_game_config: LcdsPracticeGameConfig,
    simple_inventory_jwt: String,
    player_gco_tokens: LcdsPlayerGcoTokens,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolInventoryInventoryResponseDTO")]
#[serde(rename_all = "kebab-case")]
pub struct LolInventoryInventoryResponseDto {
    data: LolInventoryInventoryDto,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolClashThemeVp {
    vp: i32,
    theme_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolEventHubChampionSkinEmblemPath {
    large: String,
    small: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootSvcRewardGrantElement {
    status: LolLootRewardStatus,
    localizations: Value,
    media: Value,
    fulfillment_source: String,
    element_id: String,
    item_type: String,
    quantity: i32,
    item_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootLootItem {
    rarity: String,
    _type: String,
    upgrade_loot_name: String,
    rental_seconds: i64,
    rental_games: i32,
    loot_name: String,
    value: i32,
    expiry_time: i64,
    asset: String,
    store_item_id: i32,
    tags: String,
    display_categories: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolNpeRewardsObjective {
    progress: LolNpeRewardsProgress,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesLoadout {
    loadout: Value,
    scope: String,
    item_id: i32,
    name: String,
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "PaymentsPMCStartUrlResult")]
#[serde(rename_all = "camelCase")]
pub struct PaymentsPmcStartUrlResult {
    id: String,
    created_at: String,
    locale_id: String,
    summoner_level: i16,
    player_facing_id: String,
    user_id: String,
    pmc_start_url: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChampionsInventoryItemWithPayload {
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "uuid")]
    uuid: String,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "owned")]
    owned: bool,
    #[serde(rename = "payload")]
    payload: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolReplaysReplaysDynamicConfig {
    min_supported_game_server_version: String,
    minutes_until_replay_considered_lost: f64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolYourshopWalletCacheEntry {
    #[serde(rename = "signedBalancesJwt")]
    signed_balances_jwt: String,
    #[serde(rename = "receivedAtMS")]
    received_at_ms: u64,
    #[serde(rename = "expirationMS")]
    expiration_ms: u64,
    #[serde(rename = "valid")]
    valid: bool,
    #[serde(rename = "issuedAtMS")]
    issued_at_ms: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCareerStatsPositionStatsQueryRequest {
    rank_tier: String,
    position: LolCareerStatsSummonersRiftPosition,
    queue_type: LolCareerStatsCareerStatsQueueType,
    season: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolPremadeVoiceKeyCombo {
    key_bindings: Vec<LolPremadeVoicePushToTalkKey>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "RosterMatchAggregatedStatsDTO")]
#[serde(rename_all = "camelCase")]
pub struct RosterMatchAggregatedStatsDto {
    player_champion_ids: Value,
    opponent_logo: i32,
    kills: i32,
    duration: i64,
    loser_bracket: bool,
    opponent_logo_color: i32,
    round: i32,
    win: bool,
    game_id: i64,
    opponent_short_name: String,
    opponent_kills: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetPurchaseRequest {
    items: Vec<LolPurchaseWidgetPurchaseItem>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogBundled {
    items: Vec<LolCatalogBundledItem>,
    flexible: bool,
    minimum_prices: Vec<LolCatalogBundledItemCost>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChallengesChallengesPlayerPreferences {
    #[serde(rename = "title")]
    title: String,
    #[serde(rename = "challengeIds")]
    challenge_ids: Vec<i64>,
    #[serde(rename = "crestBorder")]
    crest_border: String,
    #[serde(rename = "prestigeCrestBorderLevel")]
    prestige_crest_border_level: i32,
    #[serde(rename = "signedJWTPayload")]
    signed_jwt_payload: LolChallengesChallengeSignedUpdatePayload,
    #[serde(rename = "bannerAccent")]
    banner_accent: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootCollectionsChampionSkinMinimal {
    name: String,
    champion_id: i32,
    id: i32,
    ownership: LolLootCollectionsOwnership,
    splash_path: String,
    tile_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyFriendAvailabilityAnalytics {
    num_friends_in_queue: u32,
    num_friends_available: u32,
    event_type: String,
    event_data: Value,
    num_friends_in_champ_select: u32,
    num_friends_mobile: u32,
    platform_id: String,
    puuid: String,
    summoner_id: u64,
    event_timestamp: u64,
    num_friends: u32,
    party_type: String,
    num_friends_in_game: u32,
    num_friends_online: u32,
    num_friends_away: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthUserInfo {
    user_info: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolReplaysGAMHSGame")]
#[serde(rename_all = "camelCase")]
pub struct LolReplaysGamhsGame {
    game_creation: u64,
    game_id: u64,
    game_name: String,
    game_type: String,
    platform_id: String,
    game_duration: u32,
    queue_id: i32,
    end_of_game_result: String,
    game_start_timestamp: u64,
    game_version: String,
    season_id: u16,
    tournament_code: String,
    game_end_timestamp: u64,
    map_id: u16,
    game_mode: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChallengesLoginPlayerData {
    seasons: Vec<LolChallengesChallengeSeason>,
    client_player_data_list: Vec<LolChallengesChallengePlayerData>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionMasterySeasonMilestoneRequireAndRewards {
    reward_marks: u16,
    reward_config: LolChampionMasteryRewardConfigurationEntry,
    bonus: bool,
    require_grade_counts: Value,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRsoAuthDeviceId {
    session_id: String,
    collector_server_name: String,
    install_id: String,
    frame_url: String,
    merchant_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolChatChatSettings {
    #[serde(rename = "roster-group-collapsed")]
    roster_group_collapsed: Value,
    #[serde(rename = "chatGroupOffline")]
    chat_group_offline: bool,
    #[serde(rename = "friendRequestToastsDisabled")]
    friend_request_toasts_disabled: bool,
    #[serde(rename = "bounceDockIconEnabled")]
    bounce_dock_icon_enabled: bool,
    #[serde(rename = "chatGBG")]
    chat_gbg: bool,
    #[serde(rename = "chat-status-message")]
    chat_status_message: String,
    #[serde(rename = "closed-conversations")]
    closed_conversations: Value,
    #[serde(rename = "showWhenTypingEnabled")]
    show_when_typing_enabled: bool,
    #[serde(rename = "chatGroupMobile")]
    chat_group_mobile: bool,
    #[serde(rename = "chatWindow")]
    chat_window: LolChatChatWindowSettings,
    #[serde(rename = "muted-conversations")]
    muted_conversations: Value,
    #[serde(rename = "chatFilterDisabled")]
    chat_filter_disabled: bool,
    #[serde(rename = "usePlayerPreferences")]
    use_player_preferences: bool,
    #[serde(rename = "recentlyPlayedOpen")]
    recently_played_open: bool,
    #[serde(rename = "moreUnreadsEnabled")]
    more_unreads_enabled: bool,
    #[serde(rename = "recentlyPlayedFirstOpen")]
    recently_played_first_open: bool,
    #[serde(rename = "messageNotificationsEnabled")]
    message_notifications_enabled: bool,
    #[serde(rename = "sortBy")]
    sort_by: String,
    #[serde(rename = "hidden-conversations")]
    hidden_conversations: Value,
    #[serde(rename = "linkClickWarningEnabled")]
    link_click_warning_enabled: bool,
    #[serde(rename = "chatWindowDockedHeight")]
    chat_window_docked_height: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolStatstonesMilestoneNotificationDto {
    threshold: i32,
    is_completed: bool,
    level: i32,
    statstone_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolLobbyTFTNewPlayerDto")]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTftNewPlayerDto {
    tft_games_played: i32,
    tft_games_won: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "AlertDTO")]
#[serde(rename_all = "camelCase")]
pub struct AlertDto {
    alert_time: i64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolChampionsCollectionsChampion {
    ban_vo_path: String,
    name: String,
    stinger_sfx_path: String,
    passive: LolChampionsCollectionsChampionSpell,
    id: i32,
    purchased: u64,
    choose_vo_path: String,
    ownership: LolChampionsCollectionsOwnership,
    roles: Vec<String>,
    ranked_play_enabled: bool,
    disabled_queues: Vec<String>,
    alias: String,
    active: bool,
    square_portrait_path: String,
    base_splash_path: String,
    spells: Vec<LolChampionsCollectionsChampionSpell>,
    bot_enabled: bool,
    free_to_play: bool,
    title: String,
    skins: Vec<LolChampionsCollectionsChampionSkin>,
    tactical_info: LolChampionsCollectionsChampionTacticalInfo,
    base_load_screen_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolPurchaseWidgetValidationErrorEntry {
    id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolLobbyTeamBuilderGameModeSpellList {
    spells: Vec<u64>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesEventBeginV1 {
    when: u64,
    event_name: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatParticipantList {
    participants: Vec<LolChatParticipant>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyTeamBuilderCellV1 {
    spell_2_id: i32,
    skin_id: i32,
    champion_id: i32,
    team_id: i32,
    champion_pick_intent: i32,
    assigned_position: String,
    puuid: String,
    spell_1_id: i32,
    summoner_id: u64,
    name_visibility_type: String,
    cell_id: i32,
    obfuscated_summoner_id: u64,
    obfuscated_puuid: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLicenseAgreementLicenseAgreement {
    id: String,
    license_type: LolLicenseAgreementLicenseAgreementType,
    text: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMatchmakingLobbyStatus {
    is_leader: bool,
    lobby_id: String,
    queue_id: i32,
    is_spectator: bool,
    member_summoner_ids: Vec<u64>,
    is_custom: bool,
    allowed_play_again: bool,
    custom_spectator_policy: LolMatchmakingQueueCustomGameSpectatorPolicy,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolTftEventInventoryItemWithPayload {
    item_id: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolChatErrorList {
    errors: Vec<LolChatError>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LolTftTrovesTrovesRewards {
    standard: Vec<LolTftTrovesTrovesReward>,
    highlight: Vec<LolTftTrovesTrovesReward>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LolServiceStatusRiotStatusMaintenance {
    #[serde(rename = "archive_at")]
    archive_at: String,
    #[serde(rename = "id")]
    id: i64,
    #[serde(rename = "created_at")]
    created_at: String,
    #[serde(rename = "updated_at")]
    updated_at: String,
    #[serde(rename = "updates")]
    updates: Vec<LolServiceStatusRiotStatusUpdate>,
    #[serde(rename = "platforms")]
    platforms: Vec<String>,
    #[serde(rename = "maintenance_status")]
    maintenance_status: String,
    #[serde(rename = "titles")]
    titles: Vec<LolServiceStatusRiotStatusTitle>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLobbyCollectionsChampionMinimal {
    id: i32,
    disabled_queues: Vec<String>,
    ownership: LolLobbyCollectionsOwnership,
    free_to_play: bool,
    active: bool,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientConfigClientConfig {
    update_time: i64,
    data: Value,
    params: ClientConfigConfigParams,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolCatalogSkinLineTier {
    splash_video_path: String,
    uncentered_splash_path: String,
    id: i64,
    name: String,
    collection_splash_video_path: String,
    description: String,
    stage: i64,
    load_screen_path: String,
    short_name: String,
    tile_path: String,
    splash_path: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolLootContextMenu {
    name: String,
    action_type: String,
    recipe_description: String,
    recipe_context_menu_action: String,
    essence_quantity: i32,
    required_others: String,
    required_others_count: i32,
    required_others_name: String,
    enabled: bool,
    essence_type: String,
    required_tokens: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolRegaliaRankedQueueStats {
    queue_type: LolRegaliaLeagueQueueType,
    division: LolRegaliaLeagueDivision,
    is_provisional: bool,
    tier: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "LolTftPassInventoryItemDTO")]
#[serde(rename_all = "PascalCase")]
pub struct LolTftPassInventoryItemDto {
    #[serde(rename = "loyalty")]
    loyalty: bool,
    #[serde(rename = "lsb")]
    lsb: bool,
    #[serde(rename = "rental")]
    rental: bool,
    #[serde(rename = "ownedQuantity")]
    owned_quantity: u64,
    #[serde(rename = "f2p")]
    f_2_p: bool,
    #[serde(rename = "purchaseDate")]
    purchase_date: String,
    #[serde(rename = "quantity")]
    quantity: u64,
    #[serde(rename = "expirationDate")]
    expiration_date: String,
    #[serde(rename = "instanceId")]
    instance_id: String,
    #[serde(rename = "instanceTypeId")]
    instance_type_id: String,
    #[serde(rename = "inventoryType")]
    inventory_type: String,
    #[serde(rename = "itemId")]
    item_id: i32,
    #[serde(rename = "loyaltySources")]
    loyalty_sources: Vec<String>,
    #[serde(rename = "usedInGameDate")]
    used_in_game_date: String,
    #[serde(rename = "entitlementTypeId")]
    entitlement_type_id: String,
    #[serde(rename = "wins")]
    wins: u64,
    #[serde(rename = "payload")]
    payload: Value,
    #[serde(rename = "entitlementId")]
    entitlement_id: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename = "ChampionScoutingDTO")]
#[serde(rename_all = "camelCase")]
pub struct ChampionScoutingDto {
    champion_id: i32,
    kda: f32,
    game_count: i32,
    win_count: i32,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[doc = "Describes a function."]
pub struct BindingFullFunctionHelp {
    help: String,
    thread_safe: bool,
    _async: String,
    description: String,
    tags: Vec<String>,
    name_space: String,
    name: String,
    returns: BindingFullTypeIdentifier,
    arguments: Vec<BindingFullArgumentHelp>,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchmakingLcdsSummoner {
    name: String,
    sum_id: u64,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LcdsPlayerGcoTokens {
    id_token: String,
    user_info_jwt: String,
    summoner_token: String,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LolMissionsTftPaidBattlepass {
    progress_mission_id: String,
    total_points_earned: i32,
    active_milestone: LolMissionsTftPaidBattlepassMilestone,
    milestones: Vec<LolMissionsTftPaidBattlepassMilestone>,
    last_viewed_progress: i32,
    info: LolMissionsTftPaidBattlepassInfo,
    last_viewed_milestone: LolMissionsTftPaidBattlepassMilestone,
    bonuses: Vec<LolMissionsTftPaidBattlepassMilestone>,
    current_level: i32,
}

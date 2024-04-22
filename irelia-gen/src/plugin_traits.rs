use crate::generated_enums::*;
use crate::generated_structs::*;
use serde_json::Value;
pub trait Async {
    fn cancel(&self, body: u32) -> Value;
    fn async_result(&self, body: u32) -> Value;
    fn async_delete(&self, body: u32) -> Value;
    fn async_status(&self, body: u32) -> Value;
    fn http_async_result(&self, async_token: u32) -> Value;
    fn http_async_status(&self, async_token: u32) -> Value;
    fn http_async_delete(&self, async_token: u32) -> Value;
}
pub trait Builtin {
    fn exit(&self) -> Value;
    fn help(&self, target: String, format: RemotingHelpFormat) -> Value;
    fn cancel(&self, body: u32) -> Value;
    fn async_result(&self, body: u32) -> Value;
    fn subscribe(&self, event_name: String, format: RemotingSerializedFormat) -> Value;
    fn unsubscribe(&self, body: String) -> Value;
    fn async_delete(&self, body: u32) -> Value;
    fn async_status(&self, body: u32) -> Value;
    fn web_socket_format(&self) -> Value;
    fn http_async_result(&self, async_token: u32) -> Value;
    fn http_async_status(&self, async_token: u32) -> Value;
    fn http_async_delete(&self, async_token: u32) -> Value;
}
pub trait CookieJar {
    fn get_cookie_jar_v_1_cookies(&self) -> Vec<Cookie>;
    fn post_cookie_jar_v_1_cookies(&self, body: Vec<Cookie>) -> Value;
}
pub trait Core {
    fn post_riotclient_addorupdatemetric(
        &self,
        group: String,
        object: String,
        name: String,
        value: u64,
    );
    fn logging_metrics_metadata(&self) -> Value;
    fn logging_start(&self, buffered: bool, severity: LogSeverityLevels);
    fn logging_get_entries(&self) -> Vec<LogEvent>;
    fn logging_stop(&self);
    fn logging_metrics(&self) -> Value;
}
pub trait DataStore {
    fn post_data_store_v_1_install_settings_by_path(&self, _path: String, body: Value);
    fn get_data_store_v_1_install_settings_by_path(&self, _path: String) -> Value;
    fn get_data_store_v_1_install_dir(&self) -> String;
    fn get_data_store_v_1_system_settings_by_path(&self, _path: String) -> Value;
}
pub trait Http {
    fn http_async_result(&self, async_token: u32) -> Value;
    fn http_async_status(&self, async_token: u32) -> Value;
    fn http_async_delete(&self, async_token: u32) -> Value;
}
pub trait Logging {
    fn post_riotclient_addorupdatemetric(
        &self,
        group: String,
        object: String,
        name: String,
        value: u64,
    );
    fn logging_metrics_metadata(&self) -> Value;
    fn logging_start(&self, buffered: bool, severity: LogSeverityLevels);
    fn logging_get_entries(&self) -> Vec<LogEvent>;
    fn logging_stop(&self);
    fn logging_metrics(&self) -> Value;
}
pub trait Memory {
    fn get_memory_v_1_fe_processes_ready(&self) -> bool;
    fn post_memory_v_1_notify_fe_processes_ready(&self);
    fn post_memory_v_1_snapshot(&self, process_ids: Vec<u32>, context: String);
}
pub trait Performance {
    fn get_performance_v_1_memory(&self) -> Value;
    fn post_performance_v_1_process_by_process_id(&self, process_id: u32);
    fn post_performance_v_1_report_restart(
        &self,
        sample_length: i32,
        sample_count: i32,
    ) -> Vec<Value>;
    fn get_performance_v_1_report(&self) -> Vec<Value>;
    fn get_performance_v_1_system_info(&self, full: i32) -> Value;
}
pub trait ProcessControl {
    fn get_process_control_v_1_process(&self) -> ProcessControlProcess;
    fn post_process_control_v_1_process_quit(&self);
}
pub trait Riotclient {
    fn post_riotclient_v_1_crash_reporting_logs(&self, body: String);
    fn post_riotclient_v_1_elevation_requests(&self);
    fn post_riotclient_unload(&self);
    fn delete_riotclient_v_1_auth_tokens_by_auth_token(&self, auth_token: String) -> Value;
    fn put_riotclient_v_1_auth_tokens_by_auth_token(&self, auth_token: String) -> Value;
    fn post_riotclient_zoom_scale(&self, body: f64);
    fn get_riotclient_zoom_scale(&self) -> f64;
    fn post_riotclient_kill_ux(&self);
    fn get_riotclient_machine_id(&self) -> String;
    fn get_riotclient_ux_crash_count(&self) -> u32;
    fn post_riotclient_show_swagger(&self);
    fn post_riotclient_ux_minimize(&self);
    fn post_riotclient_launch_ux(&self);
    fn get_riotclient_app_port(&self) -> u16;
    fn post_riotclient_ux_flash(&self);
    fn post_riotclient_new_args(&self, body: Vec<String>);
    fn post_riotclient_ux_allow_foreground(&self);
    fn get_riotclient_region_locale(&self) -> LolL10NRegionLocale;
    fn put_riotclient_ux_state_ack(&self, body: u32);
    fn get_riotclient_ux_state(&self) -> String;
    fn post_riotclient_kill_and_restart_ux(&self);
    fn get_riotclient_trace(&self) -> Value;
    fn post_riotclient_open_url_in_browser(&self, body: String);
    fn get_riotclient_command_line_args(&self) -> Vec<String>;
    fn delete_riotclient_splash(&self);
    fn put_riotclient_splash(&self, body: String);
    fn get_riotclient_app_name(&self) -> String;
    fn put_riotclient_v_1_crash_reporting_environment(&self);
    fn get_riotclient_v_1_crash_reporting_environment(&self) -> CrashReportingEnvironment;
    fn get_riotclient_system_info_v_1_basic_info(&self) -> BasicSystemInfo;
    fn get_riotclient_auth_token(&self) -> String;
    fn delete_riotclient_affinity(&self);
    fn post_riotclient_affinity(&self, body: String);
    fn get_riotclient_affinity(&self) -> Value;
    fn put_riotclient_ux_load_complete(&self);
    fn post_riotclient_ux_show(&self);
}
pub trait Telemetry {
    fn post_telemetry_v_3_events_by_event_type(&self, event_type: String, body: Value);
    fn patch_telemetry_v_3_slis_add_bool_diagnostic(&self);
    fn patch_telemetry_v_3_slis_add_string_diagnostic(&self);
    fn post_telemetry_v_1_events_by_event_type(&self, event_type: String, body: Value);
    fn post_telemetry_v_1_common_data_by_key(&self, key: String, body: String);
    fn post_telemetry_v_3_slis_counts(&self);
    fn post_telemetry_v_3_events_once_by_event_type_by_once_tag(
        &self,
        event_type: String,
        once_tag: String,
        body: Value,
    );
    fn post_telemetry_v_1_events_with_perf_info_by_event_type(
        &self,
        event_type: String,
        body: Value,
    );
    fn get_telemetry_v_1_application_start_time(&self) -> u64;
    fn post_telemetry_v_3_uptime_tracking_notify_failure(&self);
    fn patch_telemetry_v_3_slis_add_int_diagnostic(&self);
    fn patch_telemetry_v_3_slis_add_double_diagnostic(&self);
    fn post_telemetry_v_3_uptime_tracking_notify_success(&self);
    fn patch_telemetry_v_3_slis_add_label(&self);
}
pub trait Tracing {
    fn delete_tracing_v_1_trace_time_series_event_by_event_name(
        &self,
        event_name: String,
        when: u64,
        suffix: String,
    );
    fn post_tracing_v_1_trace_time_series_event_by_event_name(&self, event_name: String, body: u64);
    fn post_tracing_v_1_trace_step_event(&self, body: String);
    fn post_tracing_v_1_trace_event(&self);
    fn post_tracing_v_1_trace_module(&self);
    fn post_tracing_v_1_trace_phase_end(&self);
    fn get_tracing_v_1_trace_payloads_enabled(&self) -> bool;
    fn post_tracing_v_1_trace_phase_begin(&self);
    fn post_tracing_v_1_trace_time_series_event_by_event_name_marker_by_marker_name(
        &self,
        event_name: String,
        marker_name: String,
        body: u64,
    );
    fn post_tracing_v_1_trace_critical_flow(&self);
    fn post_tracing_v_1_trace_non_timing_event_by_event_name(
        &self,
        event_name: String,
        when: u64,
        value: String,
        unit: String,
    );
    fn post_tracing_v_1_performance_by_name(&self, name: String);
    fn delete_tracing_v_1_performance_by_name(&self, name: String);
}
pub trait Other {
    fn get_config_v_1_config(&self) -> Value;
    fn get_system_v_1_builds(&self) -> BuildInfo;
    fn get_crash_reporting_v_1_crash_status(&self) -> bool;
}
pub trait PluginAssetServing {
    fn head_by_plugin_assets_by_path(
        &self,
        plugin: String,
        _path: String,
        if_none_match: String,
    ) -> Value;
    fn get_by_plugin_assets_by_path(
        &self,
        plugin: String,
        _path: String,
        if_none_match: String,
    ) -> Value;
}
pub trait PluginClientConfig {
    fn get_client_config_v_1_status_by_type(
        &self,
        _type: ClientConfigConfigType,
    ) -> ClientConfigConfigStatus;
    fn get_client_config_v_2_config_by_name(&self, name: String) -> Value;
    fn get_client_config_v_2_namespace_by_namespace_public(&self, namespace: String) -> Value;
    fn get_client_config_v_1_config_by_name(
        &self,
        name: String,
        _type: ClientConfigConfigType,
        app: String,
        version: String,
        patchline: String,
        region: String,
    ) -> Value;
    fn put_client_config_v_1_refresh_config_status(&self);
    fn get_client_config_v_1_config(
        &self,
        _type: ClientConfigConfigType,
        app: String,
        version: String,
        patchline: String,
        region: String,
        namespace: String,
    ) -> Value;
    fn get_client_config_v_2_namespace_by_namespace(&self, namespace: String) -> Value;
    fn get_client_config_v_2_namespace_by_namespace_player(&self, namespace: String) -> Value;
    fn put_client_config_v_2_namespace_changes(&self);
    fn put_client_config_v_1_entitlements_token(&self);
}
pub trait PluginDeepLinks {
    fn get_deep_links_v_1_settings(&self) -> DeepLinksDeepLinksSettings;
    fn post_deep_links_v_1_launch_lor_link(&self) -> String;
}
pub trait PluginEntitlements {
    fn get_entitlements_v_1_token(&self) -> EntitlementsToken;
}
pub trait PluginManager {
    fn get_plugin_manager_v_1_external_plugins_availability(&self) -> ExternalPluginsResource;
    fn head_by_plugin_assets_by_path(
        &self,
        plugin: String,
        _path: String,
        if_none_match: String,
    ) -> Value;
    fn get_by_plugin_assets_by_path(
        &self,
        plugin: String,
        _path: String,
        if_none_match: String,
    ) -> Value;
    fn get_plugin_manager_v_3_plugins_manifest(&self) -> Value;
    fn get_plugin_manager_v_2_descriptions(&self) -> Vec<PluginDescriptionResource>;
    fn get_plugin_manager_v_1_status(&self) -> PluginManagerResource;
    fn get_plugin_manager_v_2_descriptions_by_plugin(
        &self,
        plugin: String,
    ) -> PluginDescriptionResource;
    fn get_plugin_manager_v_2_plugins(&self) -> Vec<PluginResource>;
    fn get_plugin_manager_v_2_plugins_by_plugin(&self, plugin: String) -> PluginResource;
}
pub trait PluginManagerDiagnostics {
    fn get_plugin_manager_v_1_external_plugins_availability(&self) -> ExternalPluginsResource;
    fn get_plugin_manager_v_1_status(&self) -> PluginManagerResource;
    fn get_plugin_manager_v_2_plugins(&self) -> Vec<PluginResource>;
    fn get_plugin_manager_v_2_plugins_by_plugin(&self, plugin: String) -> PluginResource;
}
pub trait PluginManagerInfo {
    fn get_plugin_manager_v_3_plugins_manifest(&self) -> Value;
    fn get_plugin_manager_v_2_descriptions(&self) -> Vec<PluginDescriptionResource>;
    fn get_plugin_manager_v_2_descriptions_by_plugin(
        &self,
        plugin: String,
    ) -> PluginDescriptionResource;
}
pub trait PluginPatcher {
    fn post_patcher_v_1_products_by_product_id_partial_repair_request(
        &self,
        product_id: String,
    ) -> Value;
    fn put_patcher_v_1_ux(&self);
    fn post_patcher_v_1_products_by_product_id_detect_corruption_request(
        &self,
        product_id: String,
    ) -> PatcherProductState;
    fn post_patcher_v_1_products_by_product_id_start_checking_request(
        &self,
        product_id: String,
    ) -> Value;
    fn post_patcher_v_1_products_by_product_id_start_patching_request(
        &self,
        product_id: String,
    ) -> Value;
    fn get_patcher_v_1_products_by_product_id_tags(&self, product_id: String) -> Value;
    fn post_patcher_v_1_products_by_product_id_signal_start_patching_delayed(
        &self,
        product_id: String,
    ) -> Value;
    fn get_patcher_v_1_products_by_product_id_state(
        &self,
        product_id: String,
    ) -> PatcherProductState;
    fn delete_patcher_v_1_notifications_by_id(&self, id: String) -> Value;
    fn get_patcher_v_1_status(&self) -> PatcherStatus;
    fn get_patcher_v_1_products_by_product_id_paths(&self, product_id: String) -> Value;
    fn post_patcher_v_1_products_by_product_id_stop_patching_request(
        &self,
        product_id: String,
    ) -> Value;
    fn delete_patcher_v_1_products_by_product_id(&self, product_id: String) -> Value;
    fn get_patcher_v_1_products(&self) -> Vec<String>;
    fn post_patcher_v_1_products_by_product_id_stop_checking_request(
        &self,
        product_id: String,
    ) -> Value;
    fn patch_patcher_v_1_p_2_p_status(&self) -> Value;
    fn get_patcher_v_1_p_2_p_status(&self) -> PatcherP2PStatus;
    fn get_patcher_v_1_notifications(&self) -> Vec<PatcherNotification>;
    fn post_patcher_v_1_notifications(&self);
}
pub trait PluginPayments {
    fn post_payments_v_1_pmc_start_url(&self) -> PaymentsFrontEndResult;
    fn post_payments_v_1_update_payment_telemetry_state(&self);
}
pub trait PluginPlayerNotifications {
    fn get_player_notifications_v_1_notifications_by_id(
        &self,
        id: u64,
    ) -> PlayerNotificationsPlayerNotificationResource;
    fn delete_player_notifications_v_1_notifications_by_id(&self, id: u64) -> Value;
    fn put_player_notifications_v_1_notifications_by_id(
        &self,
        id: u64,
        body: Value,
    ) -> PlayerNotificationsPlayerNotificationResource;
    fn get_player_notifications_v_1_config(
        &self,
    ) -> PlayerNotificationsPlayerNotificationConfigResource;
    fn put_player_notifications_v_1_config(
        &self,
    ) -> PlayerNotificationsPlayerNotificationConfigResource;
    fn post_player_notifications_v_1_notifications(
        &self,
    ) -> PlayerNotificationsPlayerNotificationResource;
    fn get_player_notifications_v_1_notifications(
        &self,
    ) -> Vec<PlayerNotificationsPlayerNotificationResource>;
}
pub trait PluginRiotMessagingService {
    fn post_riot_messaging_service_v_1_connect(&self, body: String);
    fn delete_riot_messaging_service_v_1_connect(&self);
    fn get_riot_messaging_service_v_1_state(&self) -> RiotMessagingServiceState;
    fn delete_riot_messaging_service_v_1_session(&self);
    fn get_riot_messaging_service_v_1_session(&self) -> RiotMessagingServiceSession;
    fn delete_riot_messaging_service_v_1_entitlements(&self);
    fn post_riot_messaging_service_v_1_entitlements(&self);
    fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c_by_d_by_e(
        &self,
        a: String,
        b: String,
        c: String,
        d: String,
        e: String,
    ) -> RmsMessage;
    fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c(
        &self,
        a: String,
        b: String,
        c: String,
    ) -> RmsMessage;
    fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c_by_d(
        &self,
        a: String,
        b: String,
        c: String,
        d: String,
    ) -> RmsMessage;
    fn get_riot_messaging_service_v_1_message_by_a_by_b(&self, a: String, b: String) -> RmsMessage;
    fn get_riot_messaging_service_v_1_message_by_a_by_b_by_c_by_d_by_e_by_f(
        &self,
        a: String,
        b: String,
        c: String,
        d: String,
        e: String,
        f: String,
    ) -> RmsMessage;
    fn get_riot_messaging_service_v_1_message_by_a(&self, a: String) -> RmsMessage;
}
pub trait PluginSanitizer {
    fn get_sanitizer_v_1_status(&self) -> SanitizerSanitizerStatus;
    fn post_sanitizer_v_1_sanitize(&self) -> SanitizerSanitizeResponse;
    fn post_sanitizer_v_1_contains_sanitized(&self) -> SanitizerContainsSanitizedResponse;
}
pub trait PluginLolAccountVerification {
    fn post_lol_account_verification_v_1_send_deactivation_pin(&self);
    fn post_lol_account_verification_v_1_confirm_activation_pin(&self);
    fn post_lol_account_verification_v_1_confirm_deactivation_pin(&self);
    fn get_lol_account_verification_v_1_is_verified(
        &self,
    ) -> LolAccountVerificationIsVerifiedResponse;
    fn get_lol_account_verification_v_1_phone_number(
        &self,
    ) -> LolAccountVerificationPhoneNumberResponse;
    fn post_lol_account_verification_v_1_send_activation_pin(&self);
}
pub trait PluginLolActiveBoosts {
    fn get_lol_active_boosts_v_1_active_boosts(&self) -> LolActiveBoostsActiveBoosts;
}
pub trait PluginLolAntiAddiction {
    fn get_lol_anti_addiction_v_1_anti_addiction_token(&self)
        -> LolAntiAddictionAntiAddictionToken;
    fn get_anti_addiction_v_1_policies_by_policy_type_anti_addiction_state(
        &self,
        policy_type: LolAntiAddictionPolicyType,
    ) -> LolAntiAddictionAntiAddictionState;
}
pub trait PluginLolBanners {
    fn get_lol_banners_v_1_players_by_puuid_flags_equipped(
        &self,
        puuid: String,
    ) -> LolBannersBannerFlag;
    fn get_lol_banners_v_1_current_summoner_flags(&self) -> Vec<LolBannersBannerFlag>;
    fn get_lol_banners_v_1_current_summoner_frames_equipped(&self) -> LolBannersBannerFrame;
    fn get_lol_banners_v_1_current_summoner_flags_equipped(&self) -> LolBannersBannerFlag;
    fn put_lol_banners_v_1_current_summoner_flags_equipped(&self) -> LolBannersBannerFlag;
}
pub trait PluginLolCareerStats {
    fn get_lol_career_stats_v_1_position_experts_by_position(
        &self,
        position: LolCareerStatsSummonersRiftPosition,
    ) -> Vec<LolCareerStatsExpertPlayer>;
    fn get_lol_career_stats_v_1_summoner_stats_by_puuid_by_season_by_queue_by_position(
        &self,
        puuid: String,
        season: u32,
        queue: LolCareerStatsCareerStatsQueueType,
        position: LolCareerStatsSummonersRiftPosition,
        champion_id: i32,
    ) -> Value;
    fn get_lol_career_stats_v_1_summoner_games_by_puuid(&self, puuid: String) -> Value;
    fn get_lol_career_stats_v_1_position_averages_by_position_by_tier_by_queue(
        &self,
        position: LolCareerStatsSummonersRiftPosition,
        tier: String,
        queue: LolCareerStatsCareerStatsQueueType,
    ) -> LolCareerStatsChampionQueueStatsResponse;
    fn post_lol_career_stats_v_1_position_stats_percentiles(
        &self,
        body: Vec<LolCareerStatsPositionStatsQueryRequest>,
    ) -> Vec<LolCareerStatsStatisticsPercentilesResponse>;
    fn get_lol_career_stats_v_1_champion_experts_season_by_season_by_champion_id_by_position(
        &self,
        season: u32,
        champion_id: i32,
        position: LolCareerStatsSummonersRiftPosition,
    ) -> Vec<LolCareerStatsExpertPlayer>;
    fn get_lol_career_stats_v_1_position_experts_season_by_season_by_position(
        &self,
        season: u32,
        position: LolCareerStatsSummonersRiftPosition,
    ) -> Vec<LolCareerStatsExpertPlayer>;
    fn post_lol_career_stats_v_1_champion_stats_percentiles(
        &self,
        body: Vec<LolCareerStatsStatsQueryRequest>,
    ) -> Vec<LolCareerStatsStatisticsPercentilesResponse>;
    fn get_lol_career_stats_v_1_champion_experts_by_champion_id_by_position(
        &self,
        champion_id: i32,
        position: LolCareerStatsSummonersRiftPosition,
    ) -> Vec<LolCareerStatsExpertPlayer>;
    fn get_lol_career_stats_v_1_position_averages_season_by_season_by_position_by_tier_by_queue(
        &self,
        season: u32,
        position: LolCareerStatsSummonersRiftPosition,
        tier: String,
        queue: LolCareerStatsCareerStatsQueueType,
    ) -> LolCareerStatsChampionQueueStatsResponse;
    fn get_lol_career_stats_v_1_champion_averages_by_champion_id_by_position_by_tier_by_queue(
        &self,
        champion_id: i32,
        position: LolCareerStatsSummonersRiftPosition,
        tier: String,
        queue: LolCareerStatsCareerStatsQueueType,
    ) -> LolCareerStatsChampionQueueStatsResponse;
    fn get_lol_career_stats_v_1_summoner_games_by_puuid_season_by_season(
        &self,
        puuid: String,
        season: u32,
    ) -> Value;
    fn get_lol_career_stats_v_1_champion_averages_season_by_season_by_champion_id_by_position_by_tier_by_queue(
        &self,
        season: u32,
        champion_id: i32,
        position: LolCareerStatsSummonersRiftPosition,
        tier: String,
        queue: LolCareerStatsCareerStatsQueueType,
    ) -> LolCareerStatsChampionQueueStatsResponse;
}
pub trait PluginLolCatalog {
    fn get_lol_catalog_v_1_items_by_inventory_type(
        &self,
        inventory_type: String,
    ) -> Vec<LolCatalogCatalogPluginItem>;
    fn get_lol_catalog_v_1_items(
        &self,
        inventory_type: String,
        item_ids: Vec<i64>,
    ) -> Vec<LolCatalogItemChoiceDetails>;
    fn get_lol_catalog_v_1_items_list_details(
        &self,
        catalog_items_keys: Vec<LolCatalogItemKey>,
    ) -> Vec<LolCatalogCatalogPluginItemWithDetails>;
    fn get_lol_catalog_v_1_item_details(
        &self,
        inventory_type: String,
        item_id: i64,
    ) -> LolCatalogCatalogPluginItemWithDetails;
    fn get_lol_catalog_v_1_items_list_details_skip_cache(
        &self,
        catalog_items_keys: Vec<LolCatalogItemKey>,
    ) -> Vec<LolCatalogCatalogPluginItemWithDetails>;
}
pub trait PluginLolChallenges {
    fn get_lol_challenges_v_1_summary_player_data_player_by_puuid(
        &self,
        puuid: String,
    ) -> LolChallengesUiPlayerSummary;
    fn get_lol_challenges_v_1_summary_players_data_players(&self, puuids: Vec<String>) -> Value;
    fn get_lol_challenges_v_1_available_queue_ids(&self) -> Vec<i32>;
    fn get_lol_challenges_v_1_summary_player_data_local_player(
        &self,
    ) -> LolChallengesUiPlayerSummary;
    fn get_lol_challenges_v_2_titles_all(&self) -> Value;
    fn get_lol_challenges_v_1_seasons(&self) -> Vec<LolChallengesChallengeSeason>;
    fn get_lol_challenges_v_1_challenges_category_data(&self) -> Value;
    fn get_lol_challenges_v_1_client_state(&self) -> LolChallengesClientState;
    fn get_lol_challenges_v_2_titles_local_player(&self) -> Vec<LolChallengesUiTitle>;
    fn get_lol_challenges_v_1_updated_challenges_by_game_id_by_puuid(
        &self,
        game_id: u64,
        puuid: String,
    ) -> Value;
    fn get_lol_challenges_v_1_my_updated_challenges_by_game_id(&self, game_id: u64) -> Value;
    fn get_lol_challenges_v_1_challenges_local_player(&self) -> Value;
    fn get_lol_challenges_v_1_level_points(&self) -> Value;
    fn get_lol_challenges_v_1_penalty(&self) -> LolChallengesUiChallengePenalty;
    fn post_lol_challenges_v_1_ack_challenge_update_by_id(&self, id: u64);
    fn post_lol_challenges_v_1_update_player_preferences(&self);
}
pub trait PluginLolChampSelect {
    fn get_lol_champ_select_v_1_team_boost(&self) -> LolChampSelectTeamBoost;
    fn post_lol_champ_select_v_1_session_trades_by_id_decline(&self, id: i64) -> Value;
    fn get_lol_champ_select_v_1_session_swaps_by_id(
        &self,
        id: i64,
    ) -> LolChampSelectChampSelectSwapContract;
    fn get_lol_champ_select_v_1_session_timer(&self) -> LolChampSelectChampSelectTimer;
    fn post_lol_champ_select_v_1_session_trades_by_id_request(
        &self,
        id: i64,
    ) -> LolChampSelectChampSelectTradeContract;
    fn get_lol_champ_select_v_1_disabled_champion_ids(&self) -> Vec<i32>;
    fn patch_lol_champ_select_v_1_session_my_selection(&self) -> Value;
    fn get_lol_champ_select_v_1_session_my_selection(
        &self,
    ) -> LolChampSelectChampSelectPlayerSelection;
    fn get_lol_champ_select_v_1_pickable_skin_ids(&self) -> Vec<i32>;
    fn post_lol_champ_select_v_1_session_trades_by_id_accept(&self, id: i64) -> Value;
    fn get_lol_champ_select_v_1_bannable_champion_ids(&self) -> Vec<i32>;
    fn get_lol_champ_select_v_1_session_trades(
        &self,
    ) -> Vec<LolChampSelectChampSelectTradeContract>;
    fn post_lol_champ_select_v_1_session_simple_inventory(&self) -> Value;
    fn get_lol_champ_select_v_1_muted_players(&self) -> Vec<LolChampSelectMutedPlayerInfo>;
    fn post_lol_champ_select_v_1_battle_training_launch(&self) -> Value;
    fn post_lol_champ_select_v_1_session_trades_by_id_cancel(&self, id: i64) -> Value;
    fn post_lol_champ_select_v_1_session_swaps_by_id_accept(&self, id: i64) -> Value;
    fn post_lol_champ_select_v_1_toggle_player_muted(&self) -> Value;
    fn post_lol_champ_select_v_1_session_my_selection_reroll(&self) -> Value;
    fn post_lol_champ_select_v_1_team_boost_purchase(&self) -> Value;
    fn post_lol_champ_select_v_1_session_actions_by_id_complete(&self, id: i64) -> Value;
    fn get_lol_champ_select_v_1_summoners_by_slot_id(
        &self,
        slot_id: u64,
    ) -> LolChampSelectChampSelectSummoner;
    fn post_lol_champ_select_v_1_ongoing_swap_by_id_clear(&self, id: i64) -> Value;
    fn post_lol_champ_select_v_1_toggle_favorite_by_champion_id_by_position(
        &self,
        champion_id: i64,
        position: String,
    ) -> Value;
    fn get_lol_champ_select_v_1_skin_selector_info(&self) -> LolChampSelectSkinSelectorInfo;
    fn patch_lol_champ_select_v_1_session_actions_by_id(&self, id: i64) -> Value;
    fn get_lol_champ_select_v_1_session(&self) -> LolChampSelectChampSelectSession;
    fn post_lol_champ_select_v_1_retrieve_latest_game_dto(&self) -> Value;
    fn get_lol_champ_select_v_1_all_grid_champions(&self) -> Vec<LolChampSelectChampGridChampion>;
    fn get_lol_champ_select_v_1_session_trades_by_id(
        &self,
        id: i64,
    ) -> LolChampSelectChampSelectTradeContract;
    fn get_lol_champ_select_v_1_sfx_notifications(&self) -> Vec<LolChampSelectSfxNotification>;
    fn post_lol_champ_select_v_1_session_swaps_by_id_cancel(&self, id: i64) -> Value;
    fn get_lol_champ_select_v_1_ongoing_trade(&self) -> LolChampSelectChampSelectTradeNotification;
    fn get_lol_champ_select_v_1_ongoing_swap(&self) -> LolChampSelectChampSelectSwapNotification;
    fn get_lol_champ_select_v_1_session_swaps(&self) -> Vec<LolChampSelectChampSelectSwapContract>;
    fn get_lol_champ_select_v_1_pickable_champion_ids(&self) -> Vec<i32>;
    fn post_lol_champ_select_v_1_ongoing_trade_by_id_clear(&self, id: i64) -> Value;
    fn post_lol_champ_select_v_1_session_swaps_by_id_request(
        &self,
        id: i64,
    ) -> LolChampSelectChampSelectSwapContract;
    fn get_lol_champ_select_v_1_grid_champions_by_champion_id(
        &self,
        champion_id: i32,
    ) -> LolChampSelectChampGridChampion;
    fn get_lol_champ_select_v_1_current_champion(&self) -> i32;
    fn get_lol_champ_select_v_1_skin_carousel_skins(&self) -> Vec<LolChampSelectSkinSelectorSkin>;
    fn post_lol_champ_select_v_1_session_swaps_by_id_decline(&self, id: i64) -> Value;
    fn post_lol_champ_select_v_1_session_bench_swap_by_champion_id(
        &self,
        champion_id: i32,
    ) -> Value;
    fn get_lol_champ_select_v_1_pin_drop_notification(
        &self,
    ) -> LolChampSelectChampSelectPinDropNotification;
}
pub trait PluginLolChampSelectLegacy {
    fn post_lol_champ_select_legacy_v_1_battle_training_launch(&self) -> Value;
    fn get_lol_champ_select_legacy_v_1_session_trades(
        &self,
    ) -> Vec<LolChampSelectLegacyChampSelectTradeContract>;
    fn get_lol_champ_select_legacy_v_1_pickable_skin_ids(&self) -> Vec<i32>;
    fn post_lol_champ_select_legacy_v_1_session_actions_by_id_complete(&self, id: i64) -> Value;
    fn get_lol_champ_select_legacy_v_1_current_champion(&self) -> i32;
    fn get_lol_champ_select_legacy_v_1_bannable_champion_ids(&self) -> Vec<i32>;
    fn get_lol_champ_select_legacy_v_1_disabled_champion_ids(&self) -> Vec<i32>;
    fn get_lol_champ_select_legacy_v_1_pickable_champion_ids(&self) -> Vec<i32>;
    fn get_lol_champ_select_legacy_v_1_implementation_active(&self) -> bool;
    fn post_lol_champ_select_legacy_v_1_session_trades_by_id_request(
        &self,
        id: i64,
    ) -> LolChampSelectLegacyChampSelectTradeContract;
    fn get_lol_champ_select_legacy_v_1_session(&self) -> LolChampSelectLegacyChampSelectSession;
    fn get_lol_champ_select_legacy_v_1_session_trades_by_id(
        &self,
        id: i64,
    ) -> LolChampSelectLegacyChampSelectTradeContract;
    fn post_lol_champ_select_legacy_v_1_session_my_selection_reroll(&self) -> Value;
    fn get_lol_champ_select_legacy_v_1_session_timer(&self)
        -> LolChampSelectLegacyChampSelectTimer;
    fn get_lol_champ_select_legacy_v_1_team_boost(&self) -> LolChampSelectLegacyTeamBoost;
    fn post_lol_champ_select_legacy_v_1_session_trades_by_id_decline(&self, id: i64) -> Value;
    fn post_lol_champ_select_legacy_v_1_session_trades_by_id_accept(&self, id: i64) -> Value;
    fn post_lol_champ_select_legacy_v_1_session_trades_by_id_cancel(&self, id: i64) -> Value;
    fn get_lol_champ_select_legacy_v_1_session_my_selection(
        &self,
    ) -> LolChampSelectLegacyChampSelectPlayerSelection;
    fn patch_lol_champ_select_legacy_v_1_session_my_selection(&self) -> Value;
    fn patch_lol_champ_select_legacy_v_1_session_actions_by_id(&self, id: u64) -> Value;
}
pub trait PluginLolChampionMastery {
    fn post_lol_champion_mastery_v_1_by_puuid_champion_mastery_top(
        &self,
        puuid: String,
        body: u32,
    ) -> LolChampionMasteryTopChampionMasteries;
    fn get_lol_champion_mastery_v_1_by_puuid_champion_mastery(
        &self,
        puuid: String,
    ) -> Vec<LolChampionMasteryChampionMastery>;
    fn get_lol_champion_mastery_v_1_local_player_champion_mastery_sets_and_rewards(
        &self,
    ) -> LolChampionMasteryUiAllChampionMasteryWithSets;
    fn get_lol_champion_mastery_v_1_notifications(
        &self,
    ) -> LolChampionMasteryChampionMasteryChangeNotification;
    fn post_lol_champion_mastery_v_1_scouting(&self, body: Vec<String>) -> Vec<RankedScoutingDto>;
    fn get_lol_champion_mastery_v_1_local_player_champion_mastery(
        &self,
    ) -> Vec<LolChampionMasteryChampionMastery>;
    fn post_lol_champion_mastery_v_1_notifications_ack(&self);
    fn get_lol_champion_mastery_v_1_local_player_champion_mastery_score(&self) -> u64;
}
pub trait PluginLolChampions {
    fn get_lol_champions_v_1_inventories_by_summoner_id_champions_by_champion_id_skins_by_skin_id_chromas(
        &self,
        summoner_id: u64,
        champion_id: i32,
        skin_id: i32,
    ) -> Vec<LolChampionsCollectionsChampionChroma>;
    fn get_lol_champions_v_1_inventories_by_summoner_id_champions_by_champion_id_skins(
        &self,
        summoner_id: u64,
        champion_id: i32,
    ) -> Vec<LolChampionsCollectionsChampionSkin>;
    fn get_lol_champions_v_1_inventories_by_summoner_id_champions(
        &self,
        summoner_id: u64,
    ) -> Vec<LolChampionsCollectionsChampion>;
    fn get_lol_champions_v_1_inventories_by_summoner_id_champions_by_champion_id_skins_by_champion_skin_id(
        &self,
        summoner_id: u64,
        champion_id: i32,
        champion_skin_id: i32,
    ) -> LolChampionsCollectionsChampionSkin;
    fn get_lol_champions_v_1_inventories_by_summoner_id_champions_by_champion_id(
        &self,
        summoner_id: u64,
        champion_id: i32,
    ) -> LolChampionsCollectionsChampion;
    fn get_lol_champions_v_1_inventories_by_summoner_id_champions_playable_count(
        &self,
        summoner_id: u64,
    ) -> LolChampionsCollectionsChampionPlayableCounts;
    fn get_lol_champions_v_1_owned_champions_minimal(
        &self,
    ) -> Vec<LolChampionsCollectionsChampionMinimal>;
    fn get_lol_champions_v_1_inventories_by_summoner_id_champions_minimal(
        &self,
        summoner_id: u64,
    ) -> Vec<LolChampionsCollectionsChampionMinimal>;
    fn get_lol_champions_v_1_inventories_by_summoner_id_skins_minimal(
        &self,
        summoner_id: u64,
    ) -> Vec<LolChampionsCollectionsChampionSkinMinimal>;
}
pub trait PluginLolChat {
    fn get_lol_chat_v_1_conversations_by_id(&self, id: String) -> LolChatConversationResource;
    fn put_lol_chat_v_1_conversations_by_id(&self, id: String) -> LolChatConversationResource;
    fn delete_lol_chat_v_1_conversations_by_id(&self, id: String) -> Value;
    fn delete_lol_chat_v_1_errors_by_id(&self, id: u64) -> Value;
    fn delete_lol_chat_v_1_player_mutes(&self) -> Value;
    fn post_lol_chat_v_1_player_mutes(&self) -> Value;
    fn get_lol_chat_v_1_player_mutes(&self) -> Value;
    fn post_lol_chat_v_1_friend_groups(&self) -> Value;
    fn get_lol_chat_v_1_friend_groups(&self) -> Vec<LolChatGroupResource>;
    fn get_lol_chat_v_1_friend_counts(&self) -> LolChatFriendCountsResource;
    fn put_lol_chat_v_1_friend_groups_order(&self) -> Value;
    fn get_lol_chat_v_1_conversations_by_id_participants(
        &self,
        id: String,
    ) -> Vec<LolChatUserResource>;
    fn get_lol_chat_v_1_errors(&self) -> Vec<LolChatErrorResource>;
    fn get_lol_chat_v_1_conversations_by_id_participants_by_pid(
        &self,
        id: String,
        pid: String,
    ) -> LolChatUserResource;
    fn put_lol_chat_v_1_settings(&self, data: Value, do_async: bool) -> Value;
    fn get_lol_chat_v_1_settings(&self) -> Value;
    fn get_lol_chat_v_1_session(&self) -> LolChatSessionResource;
    fn delete_lol_chat_v_1_friend_groups_by_id(&self, id: u32) -> Value;
    fn get_lol_chat_v_1_friend_groups_by_id(&self, id: u32) -> LolChatGroupResource;
    fn put_lol_chat_v_1_friend_groups_by_id(&self, id: u32) -> Value;
    fn get_lol_chat_v_2_friend_requests(&self) -> Vec<LolChatFriendRequestResource>;
    fn post_lol_chat_v_2_friend_requests(&self) -> Value;
    fn get_lol_chat_v_1_me(&self) -> LolChatUserResource;
    fn put_lol_chat_v_1_me(&self) -> LolChatUserResource;
    fn post_lol_chat_v_1_system_mutes(&self) -> Value;
    fn get_lol_chat_v_1_blocked_players_by_id(&self, id: String) -> LolChatBlockedPlayerResource;
    fn delete_lol_chat_v_1_blocked_players_by_id(&self, id: String) -> Value;
    fn put_lol_chat_v_1_friends_by_id(&self, id: String) -> Value;
    fn delete_lol_chat_v_1_friends_by_id(&self, id: String) -> Value;
    fn get_lol_chat_v_1_friends_by_id(&self, id: String) -> LolChatFriendResource;
    fn get_lol_chat_v_1_blocked_players(&self) -> Vec<LolChatBlockedPlayerResource>;
    fn post_lol_chat_v_1_blocked_players(&self) -> Value;
    fn get_lol_chat_v_1_conversations_notify(&self) -> String;
    fn put_lol_chat_v_2_friend_requests_by_id(&self, id: String) -> Value;
    fn delete_lol_chat_v_2_friend_requests_by_id(&self, id: String) -> Value;
    fn get_lol_chat_v_1_resources(&self) -> LolChatProductMetadataMap;
    fn get_lol_chat_v_1_conversations_active(&self) -> LolChatActiveConversationResource;
    fn put_lol_chat_v_1_conversations_active(&self) -> Value;
    fn delete_lol_chat_v_1_conversations_active(&self) -> Value;
    fn get_lol_chat_v_1_conversations(&self) -> Vec<LolChatConversationResource>;
    fn post_lol_chat_v_1_conversations(&self) -> LolChatConversationResource;
    fn post_lol_chat_v_1_conversations_eog_chat_toggle(&self, body: bool) -> Value;
    fn get_lol_chat_v_1_friend_exists_by_summoner_id(&self, summoner_id: u64) -> bool;
    fn delete_lol_chat_v_1_conversations_by_id_messages(&self, id: String) -> Value;
    fn post_lol_chat_v_1_conversations_by_id_messages(
        &self,
        id: String,
    ) -> LolChatConversationMessageResource;
    fn get_lol_chat_v_1_conversations_by_id_messages(
        &self,
        id: String,
    ) -> Vec<LolChatConversationMessageResource>;
    fn get_lol_chat_v_1_config(&self) -> LolChatChatServiceDynamicClientConfig;
    fn get_lol_chat_v_1_friends(&self) -> Vec<LolChatFriendResource>;
}
pub trait PluginLolClash {
    fn get_lol_clash_v_1_iconconfig(&self) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_suggest_by_summoner_id_decline(
        &self,
        roster_id: String,
        summoner_id: u64,
    ) -> Value;
    fn get_lol_clash_v_1_player_chat_rosters(&self) -> Vec<LolClashPlayerChatRoster>;
    fn post_lol_clash_v_1_roster_by_roster_id_set_ticket(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_unlockin(&self, roster_id: String) -> Value;
    fn get_lol_clash_v_1_tournament_get_all_player_tiers(&self) -> Vec<PlayerTierDto>;
    fn post_lol_clash_v_1_roster_by_roster_id_ticket_offer_by_summoner_id_accept(
        &self,
        roster_id: String,
        summoner_id: u64,
    ) -> Value;
    fn get_lol_clash_v_1_tournament_by_tournament_id_get_player_tiers(
        &self,
        tournament_id: i64,
        summoner_ids: Vec<u64>,
    ) -> Vec<PlayerTierDto>;
    fn get_lol_clash_v_1_tournament_by_tournament_id(
        &self,
        tournament_id: i64,
    ) -> LolClashTournament;
    fn post_lol_clash_v_1_roster_by_roster_id_lockin(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_suggest_by_summoner_id_revoke(
        &self,
        roster_id: String,
        summoner_id: u64,
    ) -> Value;
    fn get_lol_clash_v_1_rewards(&self) -> LolClashPlayerRewards;
    fn post_lol_clash_v_1_roster_by_roster_id_change_all_details(&self, roster_id: String)
        -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_transfer_captain(
        &self,
        roster_id: String,
        body: u64,
    ) -> Value;
    fn post_lol_clash_v_1_update_logos(&self) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_disband(&self, roster_id: String) -> Value;
    fn get_lol_clash_v_1_voice_enabled(&self) -> bool;
    fn post_lol_clash_v_1_roster_by_roster_id_change_short_name(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_lft_team_by_roster_id_request(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_game_end_acknowledge(&self) -> Value;
    fn get_lol_clash_v_1_tournament_summary(&self) -> Vec<LolClashTournamentSummary>;
    fn post_lol_clash_v_1_roster_by_roster_id_invite(
        &self,
        roster_id: String,
        body: Vec<u64>,
    ) -> Vec<LolClashClientFailedInvite>;
    fn post_lol_clash_v_1_roster_by_roster_id_ticket_offer_by_summoner_id_offer(
        &self,
        roster_id: String,
        summoner_id: u64,
    ) -> Value;
    fn get_lol_clash_v_1_tournament_cancelled(&self) -> Vec<i64>;
    fn get_lol_clash_v_1_visible(&self) -> bool;
    fn delete_lol_clash_v_1_voice(&self) -> Value;
    fn post_lol_clash_v_1_voice(&self) -> Value;
    fn get_lol_clash_v_1_historyandwinners(&self) -> LolClashTournamentHistoryAndWinners;
    fn post_lol_clash_v_1_roster_by_roster_id_withdraw(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_simple_state_flags_by_id_acknowledge(&self, id: String) -> Value;
    fn get_lol_clash_v_1_simple_state_flags(&self) -> Vec<LolClashSimpleStateFlag>;
    fn get_lol_clash_v_1_eog_player_update(&self) -> LolClashEogPlayerUpdateDto;
    fn post_lol_clash_v_1_roster_by_roster_id_change_icon(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_refresh(&self) -> Value;
    fn post_lol_clash_v_1_tournament_by_tournament_id_create_roster(
        &self,
        tournament_id: i64,
    ) -> Value;
    fn get_lol_clash_v_1_roster_by_roster_id_stats(&self, roster_id: i64) -> LolClashRosterStats;
    fn get_lol_clash_v_1_thirdparty_team_data(&self) -> LolClashThirdPartyApiRoster;
    fn post_lol_clash_v_1_lft_player(&self) -> Value;
    fn get_lol_clash_v_1_lft_team_requests(&self) -> Vec<PendingOpenedTeamDto>;
    fn get_lol_clash_v_1_game_end(&self) -> LolClashTournamentGameEnd;
    fn get_lol_clash_v_1_time(&self) -> i64;
    fn get_lol_clash_v_1_disabled_config(&self) -> LolClashClashDisabledConfig;
    fn delete_lol_clash_v_1_voice_delay_by_delay_seconds(&self, delay_seconds: f64) -> Value;
    fn post_lol_clash_v_1_voice_delay_by_delay_seconds(&self, delay_seconds: f64) -> Value;
    fn post_lol_clash_v_1_events(&self, body: Vec<String>) -> Value;
    fn get_lol_clash_v_1_playmode_restricted(&self) -> bool;
    fn post_lol_clash_v_1_lft_team(&self) -> Value;
    fn post_lol_clash_v_1_eog_player_update_acknowledge(&self) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_leave(&self, roster_id: String) -> Value;
    fn get_lol_clash_v_1_current_tournament_ids(&self) -> Vec<i64>;
    fn post_lol_clash_v_1_roster_by_roster_id_set_position(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_ticket_offer_by_summoner_id_revoke(
        &self,
        roster_id: String,
        summoner_id: u64,
    ) -> Value;
    fn post_lol_clash_v_1_lft_team_find(&self) -> Vec<OpenedTeamDto>;
    fn post_lol_clash_v_1_roster_by_roster_id_cancel_withdraw(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_kick(&self, roster_id: String) -> Value;
    fn get_lol_clash_v_1_invited_roster_ids(&self) -> Vec<String>;
    fn post_lol_clash_v_1_notifications_acknowledge(&self) -> Value;
    fn get_lol_clash_v_1_tournament_by_tournament_id_state_info(
        &self,
        tournament_id: i64,
    ) -> LolClashTournamentStateInfo;
    fn get_lol_clash_v_1_notifications(&self) -> LolClashPlayerNotificationData;
    fn post_lol_clash_v_1_roster_by_roster_id_decline(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_unwithdraw(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_lft_player_find(&self) -> Vec<PlayerFinderDto>;
    fn get_lol_clash_v_1_awaiting_resent_eog(&self) -> bool;
    fn get_lol_clash_v_1_ping(&self) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_accept(&self, roster_id: String) -> Value;
    fn get_lol_clash_v_1_tournament_by_tournament_id_winners(
        &self,
        tournament_id: i64,
    ) -> LolClashTournamentWinnerHistory;
    fn post_lol_clash_v_1_roster_by_roster_id_ticket_offer_by_summoner_id_decline(
        &self,
        roster_id: String,
        summoner_id: u64,
    ) -> Value;
    fn get_lol_clash_v_1_player_history(&self) -> Vec<LolClashRosterStats>;
    fn post_lol_clash_v_1_roster_by_roster_id_suggest(
        &self,
        roster_id: String,
        body: Vec<u64>,
    ) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_update_logos(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_lft_team_fetch_requests(&self, body: i64) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_change_name(&self, roster_id: String) -> Value;
    fn post_lol_clash_v_1_roster_by_roster_id_suggest_by_summoner_id_accept(
        &self,
        roster_id: String,
        summoner_id: u64,
    ) -> Value;
    fn get_lol_clash_v_1_scouting_champions(
        &self,
        puuids: Vec<String>,
    ) -> Vec<LolClashScoutingChampions>;
    fn get_lol_clash_v_2_playmode_restricted(&self) -> LolClashPlaymodeRestrictedInfo;
    fn get_lol_clash_v_1_checkin_allowed(&self) -> bool;
    fn get_lol_clash_v_1_all_tournaments(&self) -> Vec<TournamentDto>;
    fn get_lol_clash_v_1_bracket_by_bracket_id(&self, bracket_id: i64) -> LolClashBracket;
    fn get_lol_clash_v_1_tournament_state_info(&self) -> Vec<LolClashTournamentStateInfo>;
    fn get_lol_clash_v_1_scouting_matchhistory(&self, summoner_ids: Vec<u64>) -> Value;
    fn get_lol_clash_v_1_ready(&self) -> bool;
    fn get_lol_clash_v_1_enabled(&self) -> bool;
    fn get_lol_clash_v_1_tournament_by_tournament_id_player(
        &self,
        tournament_id: i64,
    ) -> LolClashPlayerTournamentData;
    fn get_lol_clash_v_1_event_by_uuid(&self, uuid: String) -> ClashEventData;
    fn get_lol_clash_v_1_player(&self) -> LolClashPlayerData;
    fn get_lol_clash_v_1_roster_by_roster_id(&self, roster_id: String) -> LolClashRoster;
    fn get_lol_clash_v_1_season_rewards_by_season_id(
        &self,
        season_id: i32,
    ) -> ClashSeasonRewardResult;
    fn get_lol_clash_v_1_tournament_by_tournament_id_player_honor_restricted(
        &self,
        tournament_id: i64,
    ) -> bool;
}
pub trait PluginLolClientConfig {
    fn get_lol_client_config_v_3_client_config_operational_by_name(&self, name: String) -> Value;
    fn get_lol_client_config_v_3_client_config_by_name(&self, name: String) -> Value;
}
pub trait PluginLolCollections {
    fn get_lol_collections_v_1_inventories_by_summoner_id_ward_skins_by_ward_skin_id(
        &self,
        summoner_id: u64,
        ward_skin_id: i64,
    ) -> LolCollectionsCollectionsWardSkin;
    fn get_lol_collections_v_1_inventories_by_summoner_id_backdrop(
        &self,
        summoner_id: u64,
    ) -> LolCollectionsCollectionsSummonerBackdrop;
    fn get_lol_collections_v_1_inventories_by_puuid_champion_mastery_top(
        &self,
        puuid: String,
        limit: u64,
        sort_rule: String,
    ) -> LolCollectionsCollectionsTopChampionMasteries;
    fn get_lol_collections_v_1_inventories_by_summoner_id_ward_skins(
        &self,
        summoner_id: u64,
    ) -> Vec<LolCollectionsCollectionsWardSkin>;
    fn get_lol_collections_v_1_inventories_local_player_champion_mastery_score(&self) -> u64;
    fn get_lol_collections_v_1_inventories_by_summoner_id_spells(
        &self,
        summoner_id: u64,
    ) -> LolCollectionsCollectionsSummonerSpells;
    fn get_lol_collections_v_1_inventories_scouting(
        &self,
        puuids: Vec<String>,
    ) -> Vec<RankedScoutingDto>;
    fn get_lol_collections_v_1_inventories_by_puuid_champion_mastery(
        &self,
        puuid: String,
    ) -> Vec<LolCollectionsCollectionsChampionMastery>;
    fn get_lol_collections_v_1_inventories_chest_eligibility(
        &self,
    ) -> LolCollectionsCollectionsChestEligibility;
}
pub trait PluginLolContentTargeting {
    fn get_lol_content_targeting_v_1_filters(
        &self,
    ) -> LolContentTargetingContentTargetingFilterResponse;
    fn get_lol_content_targeting_v_1_protected_filters(
        &self,
    ) -> LolContentTargetingContentTargetingFilterResponse;
    fn get_lol_content_targeting_v_1_locale(
        &self,
    ) -> LolContentTargetingContentTargetingLocaleResponse;
}
pub trait PluginLolCosmetics {
    fn delete_lol_cosmetics_v_1_selection_tft_map_skin(&self) -> Value;
    fn put_lol_cosmetics_v_1_selection_tft_map_skin(&self, body: i32) -> Value;
    fn get_lol_cosmetics_v_1_favorites_tft_map_skins(
        &self,
    ) -> LolCosmeticsTftMapSkinFavoritesViewModel;
    fn get_lol_cosmetics_v_1_inventories_by_set_name_damage_skins(
        &self,
        set_name: String,
    ) -> LolCosmeticsTftDamageSkinGroupedViewModel;
    fn get_lol_cosmetics_v_1_inventories_by_set_name_map_skins(
        &self,
        set_name: String,
    ) -> LolCosmeticsTftMapSkinGroupedViewModel;
    fn delete_lol_cosmetics_v_1_selection_tft_damage_skin(&self) -> Value;
    fn put_lol_cosmetics_v_1_selection_tft_damage_skin(&self, body: i32) -> Value;
    fn put_lol_cosmetics_v_1_selection_companion(&self, body: i32) -> Value;
    fn delete_lol_cosmetics_v_1_selection_companion(&self) -> Value;
    fn patch_lol_cosmetics_v_1_recent_by_type(&self, _type: String, body: Vec<String>) -> Value;
    fn get_lol_cosmetics_v_1_inventories_by_set_name_playbooks(
        &self,
        set_name: String,
    ) -> LolCosmeticsTftPlaybookGroupedViewModel;
    fn delete_lol_cosmetics_v_1_favorites_tft_by_cosmetic_type_by_content_id(
        &self,
        cosmetic_type: String,
        content_id: String,
    ) -> Value;
    fn put_lol_cosmetics_v_1_favorites_tft_by_cosmetic_type_by_content_id(
        &self,
        cosmetic_type: String,
        content_id: String,
    ) -> Value;
    fn get_lol_cosmetics_v_1_inventories_by_set_name_companions(
        &self,
        set_name: String,
    ) -> LolCosmeticsCompanionsGroupedViewModel;
    fn get_lol_cosmetics_v_1_favorites_tft_companions(
        &self,
    ) -> LolCosmeticsCompanionsFavoritesViewModel;
    fn put_lol_cosmetics_v_1_selection_playbook(&self, body: i32) -> Value;
    fn delete_lol_cosmetics_v_1_selection_playbook(&self) -> Value;
    fn put_lol_cosmetics_v_1_favorites_tft_save(&self) -> Value;
    fn get_lol_cosmetics_v_1_favorites_tft_damage_skins(
        &self,
    ) -> LolCosmeticsTftDamageSkinFavoritesViewModel;
}
pub trait PluginLolDrops {
    fn get_lol_drops_v_1_drop_tables_by_drop_table_id_players_by_player_id_pity_count(
        &self,
        drop_table_id: String,
        player_id: String,
    ) -> LolDropsCapDropTableCounterDto;
    fn get_lol_drops_v_1_ready(&self) -> bool;
    fn get_lol_drops_v_1_drop_tables(&self) -> Vec<LolDropsCapDropsDropTableWithPityDto>;
    fn get_lol_drops_v_1_drop_tables_by_drop_table_id_odds_list(
        &self,
        drop_table_id: String,
    ) -> Vec<LolDropsCapDropsOddsListEntryDto>;
    fn get_lol_drops_v_1_players_by_player_id_pity_counts(
        &self,
        player_id: String,
    ) -> Vec<LolDropsCapDropTableCounterDto>;
    fn get_lol_drops_v_1_drop_tables_by_drop_table_id(
        &self,
        drop_table_id: String,
    ) -> LolDropsCapDropsDropTableWithPityDto;
    fn get_lol_drops_v_1_drop_tables_by_drop_table_id_odds_tree(
        &self,
        drop_table_id: String,
    ) -> LolDropsCapDropsOddsTreeNodeDto;
    fn get_lol_drops_v_1_players_by_player_id_total_rolls_counts(
        &self,
        player_id: String,
    ) -> Vec<LolDropsCapDropTableCounterDto>;
}
pub trait PluginLolDx9Deprecation {
    fn post_dx_9_deprecation_legacy_mode_notification_ack(&self);
    fn get_dx_9_deprecation_notification_type(
        &self,
    ) -> LolDx9DeprecationDx9DeprecationNotificationType;
}
pub trait PluginLolEmailVerification {
    fn post_lol_email_verification_v_1_confirm_email(&self) -> Value;
    fn get_lol_email_verification_v_1_email(&self) -> LolEmailVerificationEmailVerificationSession;
    fn put_lol_email_verification_v_1_email(&self) -> Value;
}
pub trait PluginLolEndOfGame {
    fn get_lol_end_of_game_v_1_tft_eog_stats(&self) -> LolEndOfGameTftEndOfGameViewModel;
    fn get_lol_end_of_game_v_1_eog_stats_block(&self) -> LolEndOfGameEndOfGameStats;
    fn get_lol_end_of_game_v_1_champion_mastery_updates(&self)
        -> LolEndOfGameChampionMasteryUpdate;
    fn post_lol_end_of_game_v_1_state_dismiss_stats(&self) -> Value;
    fn get_lol_end_of_game_v_1_gameclient_eog_stats_block(
        &self,
    ) -> LolEndOfGameGameClientEndOfGameStats;
    fn post_lol_end_of_game_v_1_gameclient_eog_stats_block(&self) -> Value;
}
pub trait PluginLolEsportStreamNotifications {
    fn post_lol_esport_stream_notifications_v_1_send_stats(
        &self,
        event_type: String,
        match_id: String,
    );
    fn get_lol_esport_stream_notifications_v_1_stream_url(&self) -> String;
    fn get_lol_esport_stream_notifications_v_1_live_streams(
        &self,
    ) -> LolEsportStreamNotificationsESportsLiveStreams;
}
pub trait PluginLolEventHub {
    fn get_lol_event_hub_v_1_events_by_event_id_event_details_data(
        &self,
        event_id: String,
    ) -> LolEventHubEventDetailsUiData;
    fn get_lol_event_hub_v_1_events_by_event_id_progress_info_data(
        &self,
        event_id: String,
    ) -> LolEventHubProgressInfoUiData;
    fn get_lol_event_hub_v_1_skins(&self) -> Value;
    fn get_lol_event_hub_v_1_events_by_event_id_pass_bundles(
        &self,
        event_id: String,
    ) -> Vec<LolEventHubBundleOfferUiData>;
    fn get_lol_event_hub_v_1_token_upsell(&self) -> Vec<LolEventHubTokenUpsell>;
    fn get_lol_event_hub_v_1_events(&self) -> Vec<LolEventHubActiveEventUiData>;
    fn get_lol_event_hub_v_1_events_by_event_id_token_shop_token_balance(
        &self,
        event_id: String,
    ) -> u32;
    fn get_lol_event_hub_v_1_events_by_event_id_reward_track_items(
        &self,
        event_id: String,
    ) -> Vec<LolEventHubRewardTrackItem>;
    fn post_lol_event_hub_v_1_purchase_item(&self) -> LolEventHubPurchaseOrderResponseDto;
    fn get_lol_event_hub_v_1_events_by_event_id_reward_track_xp(
        &self,
        event_id: String,
    ) -> LolEventHubRewardTrackXp;
    fn get_lol_event_hub_v_1_events_by_event_id_pass_background_data(
        &self,
        event_id: String,
    ) -> LolEventHubEventBackgroundUiData;
    fn get_lol_event_hub_v_1_events_by_event_id_is_grace_period(&self, event_id: String) -> bool;
    fn get_lol_event_hub_v_1_events_by_event_id_narrative(
        &self,
        event_id: String,
    ) -> Vec<LolEventHubNarrativeElement>;
    fn get_lol_event_hub_v_1_events_by_event_id_reward_track_failure(
        &self,
        event_id: String,
    ) -> LolEventHubEventHubError;
    fn get_lol_event_hub_v_1_events_by_event_id_reward_track_progress(
        &self,
        event_id: String,
    ) -> LolEventHubRewardTrackProgress;
    fn get_lol_event_hub_v_1_events_by_event_id_token_shop_categories_offers(
        &self,
        event_id: String,
    ) -> Vec<LolEventHubCategoryOffersUiData>;
    fn get_lol_event_hub_v_1_events_by_event_id_info(
        &self,
        event_id: String,
    ) -> LolEventHubEventInfoUiData;
    fn post_lol_event_hub_v_1_events_by_event_id_reward_track_claim_all(&self, event_id: String);
    fn get_lol_event_hub_v_1_events_by_event_id_progression_purchase_data(
        &self,
        event_id: String,
    ) -> LolEventHubProgressionPurchaseUiData;
    fn get_lol_event_hub_v_1_events_by_event_id_token_shop(
        &self,
        event_id: String,
    ) -> LolEventHubTokenShopUiData;
    fn get_lol_event_hub_v_1_navigation_button_data(&self) -> LolEventHubNavigationButtonUiData;
    fn get_lol_event_hub_v_1_events_by_event_id_reward_track_bonus_items(
        &self,
        event_id: String,
    ) -> Vec<LolEventHubRewardTrackItem>;
    fn get_lol_event_hub_v_1_events_by_event_id_reward_track_bonus_progress(
        &self,
        event_id: String,
    ) -> LolEventHubRewardTrackProgress;
    fn post_lol_event_hub_v_1_events_by_event_id_purchase_offer(
        &self,
        event_id: String,
    ) -> LolEventHubPurchaseOfferResponseV3;
    fn get_lol_event_hub_v_1_events_by_event_id_reward_track_unclaimed_rewards(
        &self,
        event_id: String,
    ) -> LolEventHubUnclaimedRewardsUiData;
}
pub trait PluginLolGameClientChat {
    fn get_lol_game_client_chat_v_1_buddies(&self) -> Vec<String>;
    fn get_lol_game_client_chat_v_2_ignored_players(&self) -> Vec<u64>;
    fn get_lol_game_client_chat_v_1_ignored_summoners(&self) -> Vec<u64>;
    fn post_lol_game_client_chat_v_2_instant_messages(&self);
    fn post_lol_game_client_chat_v_1_instant_messages(
        &self,
        summoner_name: String,
        message: String,
    );
    fn get_lol_game_client_chat_v_1_muted_summoners(&self) -> Vec<u64>;
    fn get_lol_game_client_chat_v_2_muted_players(&self) -> Vec<u64>;
    fn get_lol_game_client_chat_v_2_buddies(&self) -> Vec<LolGameClientChatBuddy>;
}
pub trait PluginLolGameQueues {
    fn get_lol_game_queues_v_1_game_type_config_by_game_type_config_id(
        &self,
        game_type_config_id: u32,
    ) -> LolGameQueuesQueueGameTypeConfig;
    fn get_lol_game_queues_v_1_custom(&self) -> LolGameQueuesQueueCustomGame;
    fn get_lol_game_queues_v_1_queues_by_id(&self, id: i32) -> LolGameQueuesQueue;
    fn get_lol_game_queues_v_1_queues_type_by_queue_type(
        &self,
        queue_type: String,
    ) -> LolGameQueuesQueue;
    fn get_lol_game_queues_v_1_game_type_config_by_game_type_config_id_map_by_map_id(
        &self,
        game_type_config_id: u32,
        map_id: i32,
    ) -> LolGameQueuesQueueGameTypeConfig;
    fn get_lol_game_queues_v_1_queues(&self) -> Vec<LolGameQueuesQueue>;
    fn get_lol_game_queues_v_1_custom_non_default(&self) -> LolGameQueuesQueueCustomGame;
}
pub trait PluginLolGameSettings {
    fn post_lol_game_settings_v_1_save(&self) -> bool;
    fn post_lol_game_settings_v_1_reload_post_game(&self);
    fn get_lol_game_settings_v_1_game_settings(&self) -> Value;
    fn patch_lol_game_settings_v_1_game_settings(&self, body: Value) -> Value;
    fn patch_lol_game_settings_v_1_input_settings(&self, body: Value) -> Value;
    fn get_lol_game_settings_v_1_input_settings(&self) -> Value;
    fn get_lol_game_settings_v_1_didreset(&self) -> bool;
    fn get_lol_game_settings_v_1_ready(&self) -> bool;
    fn get_lol_game_settings_v_1_input_settings_schema(&self) -> Value;
    fn get_lol_game_settings_v_1_game_settings_schema(&self) -> Value;
}
pub trait PluginLolGameflow {
    fn get_lol_gameflow_v_1_basic_tutorial(&self) -> bool;
    fn post_lol_gameflow_v_1_basic_tutorial_start(&self) -> Value;
    fn post_lol_gameflow_v_1_spectate_launch(&self, body: String) -> Value;
    fn post_lol_gameflow_v_1_watch_launch(&self, body: Vec<String>) -> Value;
    fn post_lol_gameflow_v_1_session_request_lobby(&self) -> bool;
    fn delete_lol_gameflow_v_1_early_exit_notifications_eog_by_key(&self, key: i32);
    fn get_lol_gameflow_v_1_session_per_position_summoner_spells_required_as_string(
        &self,
    ) -> String;
    fn get_lol_gameflow_v_1_session_per_position_summoner_spells_disallowed(&self) -> Value;
    fn get_lol_gameflow_v_1_watch(&self) -> LolGameflowGameflowWatchPhase;
    fn delete_lol_gameflow_v_1_early_exit_notifications_missions(&self);
    fn get_lol_gameflow_v_1_early_exit_notifications_missions(&self) -> Vec<Value>;
    fn post_lol_gameflow_v_1_gameflow_metadata_registration_status(&self);
    fn get_lol_gameflow_v_1_gameflow_metadata_registration_status(
        &self,
    ) -> LolGameflowRegistrationStatus;
    fn post_lol_gameflow_v_1_gameflow_metadata_player_status(&self);
    fn get_lol_gameflow_v_1_gameflow_metadata_player_status(&self) -> LolGameflowPlayerStatus;
    fn get_lol_gameflow_v_1_session_per_position_summoner_spells_required(&self) -> Value;
    fn get_lol_gameflow_v_1_active_patcher_lock(&self) -> bool;
    fn post_lol_gameflow_v_1_session_dodge(&self);
    fn post_lol_gameflow_v_1_session_request_tournament_checkin(&self) -> bool;
    fn post_lol_gameflow_v_1_early_exit(&self) -> Value;
    fn post_lol_gameflow_v_1_battle_training_start(&self) -> Value;
    fn delete_lol_gameflow_v_1_early_exit_notifications_missions_by_key(&self, key: i32);
    fn get_lol_gameflow_v_1_spectate_delayed_launch(&self);
    fn get_lol_gameflow_v_1_early_exit_quit_enabled(&self) -> bool;
    fn post_lol_gameflow_v_1_session_champ_select_phase_time_remaining(&self, body: u64);
    fn get_lol_gameflow_v_1_extra_game_client_args(&self) -> Vec<String>;
    fn post_lol_gameflow_v_1_extra_game_client_args(&self, body: Vec<String>);
    fn post_lol_gameflow_v_1_reconnect(&self) -> Value;
    fn post_lol_gameflow_v_1_battle_training_stop(&self) -> Value;
    fn get_lol_gameflow_v_1_session_per_position_summoner_spells_disallowed_as_string(
        &self,
    ) -> String;
    fn post_lol_gameflow_v_1_session_game_configuration(&self);
    fn post_lol_gameflow_v_1_spectate_quit(&self) -> Value;
    fn get_lol_gameflow_v_1_early_exit_notifications_eog(&self) -> Vec<Value>;
    fn delete_lol_gameflow_v_1_early_exit_notifications_eog(&self);
    fn get_lol_gameflow_v_1_game_exit_early_vanguard(&self) -> u64;
    fn post_lol_gameflow_v_1_session_tournament_ended(&self);
    fn post_lol_gameflow_v_1_tick(&self);
    fn post_lol_gameflow_v_1_ack_failed_to_launch(&self);
    fn post_lol_gameflow_v_2_spectate_launch(&self) -> Value;
    fn get_lol_gameflow_v_1_gameflow_phase(&self) -> LolGameflowGameflowPhase;
    fn get_lol_gameflow_v_1_early_exit_enabled(&self) -> bool;
    fn post_lol_gameflow_v_1_pre_end_game_transition(&self, body: bool);
    fn get_lol_gameflow_v_1_player_kicked_vanguard(&self) -> bool;
    fn get_lol_gameflow_v_1_spectate(&self) -> bool;
    fn post_lol_gameflow_v_1_session_request_enter_gameflow(&self, body: String) -> bool;
    fn get_lol_gameflow_v_1_battle_training(&self) -> bool;
    fn post_lol_gameflow_v_1_session_event(&self, body: String);
    fn get_lol_gameflow_v_1_session(&self) -> LolGameflowGameflowSession;
    fn get_lol_gameflow_v_1_availability(&self) -> LolGameflowGameflowAvailability;
}
pub trait PluginLolGeoinfo {
    fn get_lol_geoinfo_v_1_getlocation(&self, ip_address: String) -> LolGeoinfoGeoInfo;
    fn get_lol_geoinfo_v_1_whereami(&self) -> LolGeoinfoGeoInfoResponse;
}
pub trait PluginLolHighlights {
    fn get_lol_highlights_v_1_highlights_folder_path_default(&self) -> String;
    fn post_lol_highlights_v_1_highlights(&self) -> Vec<LolHighlightsHighlight>;
    fn get_lol_highlights_v_1_highlights(&self) -> Vec<LolHighlightsHighlight>;
    fn post_lol_highlights_v_1_file_browser_by_highlight_id(&self, highlight_id: u64) -> Value;
    fn get_lol_highlights_v_1_config(&self) -> LolHighlightsHighlightsConfig;
    fn put_lol_highlights_v_1_highlights_by_id(&self, id: u64) -> LolHighlightsHighlight;
    fn delete_lol_highlights_v_1_highlights_by_id(&self, id: u64) -> LolHighlightsHighlight;
    fn get_lol_highlights_v_1_highlights_by_id(&self, id: u64) -> LolHighlightsHighlight;
    fn get_lol_highlights_v_1_highlights_folder_path(&self) -> String;
}
pub trait PluginLolHoneyfruit {
    fn get_lol_honeyfruit_v_1_account_claim_migration(&self) -> String;
    fn delete_lol_honeyfruit_v_1_account_claim_migration(&self) -> Value;
    fn post_lol_honeyfruit_v_1_account_claim_migration(&self) -> String;
    fn get_lol_honeyfruit_v_1_linking_settings_button_available(&self) -> bool;
    fn get_lol_honeyfruit_v_1_account_claim_account_status_by_puuid(
        &self,
        puuid: String,
    ) -> LolHoneyfruitAccountClaimStatus;
    fn post_lol_honeyfruit_v_1_account_claim_linking_redirect(&self);
    fn post_lol_honeyfruit_v_1_vng_publisher_settings(&self) -> Value;
    fn get_lol_honeyfruit_v_1_vng_publisher_settings(
        &self,
    ) -> LolHoneyfruitHoneyfruitVngPublisherSettings;
    fn put_lol_honeyfruit_v_1_account_claim_auto_dismiss(&self, body: bool) -> Value;
    fn get_lol_honeyfruit_v_1_account_claim_auto_dismiss(&self) -> bool;
}
pub trait PluginLolHonorV2 {
    fn get_lol_honor_v_2_v_1_mutual_honor(&self) -> LolHonorV2MutualHonor;
    fn get_lol_honor_v_2_v_1_late_recognition(&self) -> Vec<LolHonorV2Honor>;
    fn post_lol_honor_v_2_v_1_level_change_ack(&self);
    fn post_lol_honor_v_2_v_1_mutual_honor_ack(&self);
    fn get_lol_honor_v_2_v_1_vote_completion(&self) -> LolHonorV2VoteCompletion;
    fn post_lol_honor_v_2_v_1_reward_granted_ack(&self);
    fn get_lol_honor_v_2_v_1_latest_eligible_game(&self) -> u64;
    fn get_lol_honor_v_2_v_1_profile(&self) -> LolHonorV2ProfileInfo;
    fn post_lol_honor_v_2_v_1_honor_player(&self) -> String;
    fn get_lol_honor_v_2_v_1_config(&self) -> LolHonorV2HonorConfig;
    fn get_lol_honor_v_2_v_1_recognition_history(&self) -> Vec<LolHonorV2HonorInteraction>;
    fn get_lol_honor_v_2_v_1_recognition(&self) -> Vec<LolHonorV2Honor>;
    fn get_lol_honor_v_2_v_1_reward_granted(&self) -> LolHonorV2VendedReward;
    fn get_lol_honor_v_2_v_1_team_choices(&self) -> Vec<String>;
    fn post_lol_honor_v_2_v_1_late_recognition_ack(&self);
    fn get_lol_honor_v_2_v_1_ballot(&self) -> LolHonorV2Ballot;
    fn get_lol_honor_v_2_v_1_level_change(&self) -> LolHonorV2VendedHonorChange;
}
pub trait PluginLolHovercard {
    fn get_lol_hovercard_v_1_friend_info_by_puuid(
        &self,
        puuid: String,
    ) -> LolHovercardHovercardUserInfo;
    fn get_lol_hovercard_v_1_friend_info_by_summoner_by_summoner_id(
        &self,
        summoner_id: u64,
    ) -> LolHovercardHovercardUserInfo;
}
pub trait PluginLolInventory {
    fn get_lol_inventory_v_1_wallet_by_currency_type(&self, currency_type: String) -> Value;
    fn get_lol_inventory_v_1_signed_inventory(&self, inventory_types: Vec<String>) -> Value;
    fn get_lol_inventory_v_1_initial_configuration_complete(&self) -> bool;
    fn get_lol_inventory_v_1_strawberry_inventory(&self) -> String;
    fn get_lol_inventory_v_1_notifications_by_inventory_type(
        &self,
        inventory_type: String,
    ) -> Vec<LolInventoryInventoryNotification>;
    fn get_lol_inventory_v_1_signed_wallet_by_currency_type(&self, currency_type: String) -> Value;
    fn get_lol_inventory_v_1_signed_inventory_simple(
        &self,
        inventory_types: Vec<String>,
        query_params: Value,
    ) -> String;
    fn get_lol_inventory_v_1_inventory(
        &self,
        inventory_types: Vec<String>,
    ) -> Vec<LolInventoryInventoryItemWithPayload>;
    fn post_lol_inventory_v_1_notification_acknowledge(&self, body: i64);
    fn get_lol_inventory_v_1_signed_wallet(&self, currency_types: Vec<String>) -> Value;
    fn get_lol_inventory_v_1_signed_inventory_cache(&self) -> Value;
    fn get_lol_inventory_v_1_inventory_with_f_2_p(
        &self,
        inventory_types: Vec<String>,
    ) -> Vec<LolInventoryInventoryItemWithPayload>;
    fn get_lol_inventory_v_1_champ_select_inventory(&self) -> String;
    fn get_lol_inventory_v_1_inventory_emotes(&self) -> Vec<LolInventoryInventoryItemWithPayload>;
    fn get_lol_inventory_v_1_signed_inventory_tournamentlogos(&self) -> Value;
    fn get_lol_inventory_v_1_players_by_puuid_inventory(
        &self,
        puuid: String,
        inventory_types: Vec<String>,
    ) -> Vec<LolInventoryInventoryItemWithPayload>;
    fn get_lol_inventory_v_2_inventory_by_inventory_type(
        &self,
        inventory_type: String,
    ) -> Vec<LolInventoryInventoryItemWithPayload>;
    fn get_lol_inventory_v_1_wallet(&self, currency_types: Vec<String>) -> Value;
    fn get_lol_inventory_v_1_xbox_subscription_status(&self) -> LolInventoryXboxSubscriptionStatus;
}
pub trait PluginLolItemSets {
    fn get_lol_item_sets_v_1_item_sets_by_summoner_id_sets(
        &self,
        summoner_id: u64,
    ) -> LolItemSetsItemSets;
    fn post_lol_item_sets_v_1_item_sets_by_summoner_id_sets(&self, summoner_id: u64);
    fn put_lol_item_sets_v_1_item_sets_by_summoner_id_sets(&self, summoner_id: u64);
    fn post_lol_item_sets_v_1_item_sets_validate(&self) -> LolItemSetsValidateItemSetNameResponse;
}
pub trait PluginLolKickout {
    fn get_lol_kickout_v_1_notification(&self) -> LolKickoutKickoutMessage;
}
pub trait PluginLolKrPlaytimeReminder {
    fn get_lol_kr_playtime_reminder_v_1_hours_played(&self) -> u32;
}
pub trait PluginLolKrShutdownLaw {
    fn get_lol_kr_shutdown_law_v_1_status(&self) -> LolKrShutdownLawAllQueueShutdownStatus;
    fn get_lol_kr_shutdown_law_v_1_rating_screen(&self) -> LolKrShutdownLawRatingScreenInfo;
    fn get_lol_kr_shutdown_law_v_1_queue_status_by_queue_id(
        &self,
        queue_id: i32,
    ) -> LolKrShutdownLawQueueShutdownStatus;
    fn get_lol_kr_shutdown_law_v_1_disabled_queues(&self) -> Vec<i32>;
    fn get_lol_kr_shutdown_law_v_1_is_enabled(&self) -> bool;
    fn post_lol_kr_shutdown_law_v_1_rating_screen_acknowledge(&self);
    fn get_lol_kr_shutdown_law_v_1_custom_status(&self) -> LolKrShutdownLawQueueShutdownStatus;
    fn get_lol_kr_shutdown_law_v_1_notification(&self) -> LolKrShutdownLawShutdownLawNotification;
}
pub trait PluginLolLeagueSession {
    fn get_lol_league_session_v_1_league_session_token(&self) -> String;
}
pub trait PluginLolLeaverBuster {
    fn get_lol_leaver_buster_v_1_ranked_restriction(&self) -> LolLeaverBusterRankedRestrictionInfo;
    fn delete_lol_leaver_buster_v_1_notifications_by_id(&self, id: u32) -> Value;
    fn get_lol_leaver_buster_v_1_notifications_by_id(
        &self,
        id: u32,
    ) -> LolLeaverBusterLeaverBusterNotificationResource;
    fn get_lol_leaver_buster_v_1_notifications(
        &self,
    ) -> Vec<LolLeaverBusterLeaverBusterNotificationResource>;
}
pub trait PluginLolLicenseAgreement {
    fn get_lol_license_agreement_v_1_agreement(&self) -> String;
    fn post_lol_license_agreement_v_1_agreements_by_id_accept(&self, id: String) -> Value;
    fn get_lol_license_agreement_v_1_agreements(&self) -> Vec<LolLicenseAgreementLicenseAgreement>;
    fn get_lol_license_agreement_v_1_privacy_policy(&self) -> String;
    fn get_lol_license_agreement_v_1_serve_location(
        &self,
    ) -> LolLicenseAgreementLicenseServeLocation;
    fn post_lol_license_agreement_v_1_agreements_by_id_decline(&self, id: String) -> Value;
    fn get_lol_license_agreement_v_1_all_agreements(
        &self,
    ) -> Vec<LolLicenseAgreementLicenseAgreement>;
}
pub trait PluginLolLoadouts {
    fn put_lol_loadouts_v_4_loadouts_by_id(&self, id: String) -> LolLoadoutsScopedLoadout;
    fn patch_lol_loadouts_v_4_loadouts_by_id(&self, id: String) -> LolLoadoutsScopedLoadout;
    fn delete_lol_loadouts_v_4_loadouts_by_id(&self, id: String);
    fn post_lol_loadouts_v_4_loadouts(&self) -> LolLoadoutsScopedLoadout;
    fn get_lol_loadouts_v_1_loadouts_ready(&self) -> bool;
    fn get_lol_loadouts_v_4_loadouts_by_loadout_id(
        &self,
        loadout_id: String,
    ) -> LolLoadoutsScopedLoadout;
    fn get_lol_loadouts_v_4_loadouts_scope_account(&self) -> Vec<LolLoadoutsScopedLoadout>;
    fn get_lol_loadouts_v_4_loadouts_scope_by_scope_by_scope_item_id(
        &self,
        scope: String,
        scope_item_id: u32,
    ) -> Vec<LolLoadoutsScopedLoadout>;
}
pub trait PluginLolLobby {
    fn delete_lol_lobby_v_2_notifications_by_notification_id(
        &self,
        notification_id: String,
    ) -> Value;
    fn get_lol_lobby_v_2_eligibility_initial_configuration_complete(&self) -> bool;
    fn get_lol_lobby_v_2_comms_token(&self) -> String;
    fn get_lol_lobby_v_1_autofill_displayed(&self) -> bool;
    fn put_lol_lobby_v_1_autofill_displayed(&self) -> bool;
    fn get_lol_lobby_v_2_lobby_members(&self) -> Vec<LolLobbyLobbyParticipantDto>;
    fn post_lol_lobby_v_2_play_again(&self) -> Value;
    fn get_lol_lobby_v_2_received_invitations(&self) -> Vec<LolLobbyReceivedInvitationDto>;
    fn post_lol_lobby_v_2_lobby_team_by_team(&self, team: String);
    fn post_lol_lobby_v_2_eligibility_party(&self) -> Vec<LolLobbyEligibility>;
    fn delete_lol_lobby_v_1_lobby_custom_bots_by_summoner_internal_name(
        &self,
        summoner_internal_name: String,
    ) -> Value;
    fn post_lol_lobby_v_1_lobby_custom_bots_by_summoner_internal_name(
        &self,
        summoner_internal_name: String,
    ) -> Value;
    fn post_lol_lobby_v_2_matchmaking_quick_search(&self) -> Value;
    fn put_lol_lobby_v_2_lobby_party_type(&self, body: String) -> Value;
    fn put_lol_lobby_v_2_lobby_members_local_member_position_preferences(&self) -> Value;
    fn post_lol_lobby_v_1_lobby_custom_switch_teams(&self, body: String) -> Value;
    fn get_lol_lobby_v_1_lobby_countdown(&self) -> i64;
    fn get_lol_lobby_v_2_lobby_custom_bots_enabled(&self) -> bool;
    fn post_lol_lobby_v_1_custom_games_refresh(&self) -> Value;
    fn get_lol_lobby_v_2_lobby_matchmaking_search_state(
        &self,
    ) -> LolLobbyLobbyMatchmakingSearchResource;
    fn get_lol_lobby_v_2_lobby_invitations(&self) -> Vec<LolLobbyLobbyInvitationDto>;
    fn post_lol_lobby_v_2_lobby_invitations(
        &self,
        body: Vec<LolLobbyLobbyInvitationDto>,
    ) -> Vec<LolLobbyLobbyInvitationDto>;
    fn post_lol_lobby_v_2_lobby_matchmaking_search(&self) -> Value;
    fn delete_lol_lobby_v_2_lobby_matchmaking_search(&self) -> Value;
    fn get_lol_lobby_v_2_comms_members(&self) -> LolLobbyPremadePartyDto;
    fn put_lol_lobby_v_1_parties_queue(&self, body: i32);
    fn post_lol_lobby_v_2_eligibility_self(&self) -> Vec<LolLobbyEligibility>;
    fn post_lol_lobby_v_2_lobby_members_by_summoner_id_grant_invite(&self, summoner_id: u64)
        -> u64;
    fn post_lol_lobby_v_2_received_invitations_by_invitation_id_decline(
        &self,
        invitation_id: String,
    );
    fn get_lol_lobby_v_1_lobby_members_local_member_player_slots(
        &self,
    ) -> Vec<LolLobbyQuickPlayPresetSlotDto>;
    fn put_lol_lobby_v_1_lobby_members_local_member_player_slots(
        &self,
        body: Vec<LolLobbyQuickPlayPresetSlotDto>,
    ) -> Value;
    fn post_lol_lobby_v_1_lobby_invitations(&self) -> LolLobbyLobbyInvitation;
    fn get_lol_lobby_v_1_lobby_invitations(&self) -> Vec<LolLobbyLobbyInvitation>;
    fn post_lol_lobby_v_2_lobby_members_by_summoner_id_promote(&self, summoner_id: u64) -> u64;
    fn post_lol_lobby_v_2_parties_overrides_enabled_for_team_builder_queues(&self, body: bool);
    fn delete_lol_lobby_v_1_clash(&self) -> Value;
    fn post_lol_lobby_v_1_clash(&self, body: String) -> Value;
    fn put_lol_lobby_v_1_parties_ready(&self, body: i32);
    fn post_lol_lobby_v_1_lobby_custom_start_champ_select(
        &self,
    ) -> LolLobbyLobbyCustomChampSelectStartResponse;
    fn get_lol_lobby_v_1_lobby_availability(&self) -> LolLobbyQueueAvailability;
    fn post_lol_lobby_v_1_lobby_members_local_member_player_slots_by_slots_index_by_perks_string(
        &self,
        slots_index: u64,
        perks_string: String,
    ) -> Value;
    fn put_lol_lobby_v_2_lobby_subteam_data(&self);
    fn post_lol_lobby_v_1_lobby_custom_cancel_champ_select(&self) -> Value;
    fn get_lol_lobby_v_1_lobby_tft_ranked_history(&self) -> bool;
    fn get_lol_lobby_v_2_lobby(&self) -> LolLobbyLobbyDto;
    fn delete_lol_lobby_v_2_lobby(&self);
    fn post_lol_lobby_v_2_lobby(&self) -> LolLobbyLobbyDto;
    fn get_lol_lobby_v_1_lobby_invitations_by_id(&self, id: String) -> LolLobbyLobbyInvitation;
    fn put_lol_lobby_v_1_lobby_members_local_member_quickplay_player_state(
        &self,
        body: String,
    ) -> Value;
    fn get_lol_lobby_v_2_registration_status(&self) -> Value;
    fn post_lol_lobby_v_2_eog_invitations(
        &self,
        body: Vec<LolLobbyLobbyInvitationDto>,
    ) -> Vec<LolLobbyLobbyInvitationDto>;
    fn get_lol_lobby_v_2_party_active(&self) -> bool;
    fn get_lol_lobby_v_2_notifications(&self) -> Vec<LolLobbyLobbyNotification>;
    fn post_lol_lobby_v_2_notifications(&self);
    fn post_lol_lobby_v_1_custom_games_by_id_join(&self, id: u64) -> Value;
    fn post_lol_lobby_v_2_party_by_party_id_join(&self, party_id: String) -> Value;
    fn put_lol_lobby_v_1_parties_by_party_id_members_by_puuid_role(
        &self,
        party_id: String,
        puuid: String,
        body: String,
    );
    fn put_lol_lobby_v_1_lobby_members_local_member_position_preferences(&self) -> Value;
    fn get_lol_lobby_v_1_parties_player(&self) -> LolLobbyPlayerDto;
    fn get_lol_lobby_v_2_lobby_custom_available_bots(&self) -> Vec<LolLobbyLobbyBotChampion>;
    fn post_lol_lobby_v_2_lobby_members_by_summoner_id_kick(&self, summoner_id: u64) -> u64;
    fn get_lol_lobby_v_1_party_rewards(&self) -> LolLobbyLobbyPartyRewards;
    fn post_lol_lobby_v_1_lobby_custom_bots(&self) -> Value;
    fn post_lol_lobby_v_2_received_invitations_by_invitation_id_accept(
        &self,
        invitation_id: String,
    );
    fn get_lol_lobby_v_1_custom_games(&self) -> Vec<LolLobbyLobbyCustomGame>;
    fn get_lol_lobby_v_1_custom_games_by_id(&self, id: i32) -> LolLobbyLobbyCustomGame;
    fn put_lol_lobby_v_1_parties_active(&self, body: i32);
    fn get_lol_lobby_v_2_party_eog_status(&self) -> LolLobbyPartyStatusDto;
    fn post_lol_lobby_v_1_tournaments_by_id_join(&self, id: String) -> Value;
    fn put_lol_lobby_v_1_parties_metadata(&self);
    fn get_lol_lobby_v_2_eligibility_game_select_eligibility_hash(&self) -> i64;
    fn post_lol_lobby_v_2_lobby_members_by_summoner_id_revoke_invite(
        &self,
        summoner_id: u64,
    ) -> u64;
    fn get_lol_lobby_v_1_parties_gamemode(&self) -> LolLobbyGameModeDto;
    fn post_lol_lobby_v_2_play_again_decline(&self) -> Value;
}
pub trait PluginLolLobbyTeamBuilder {
    fn post_lol_lobby_team_builder_champ_select_v_1_session_swaps_by_id_decline(
        &self,
        id: i32,
    ) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_implementation_active(&self) -> bool;
    fn get_lol_lobby_team_builder_champ_select_v_1_session_timer(
        &self,
    ) -> LolLobbyTeamBuilderChampSelectTimer;
    fn get_lol_lobby_team_builder_champ_select_v_1_team_boost(
        &self,
    ) -> LolLobbyTeamBuilderTeamBoost;
    fn post_lol_lobby_team_builder_v_1_ready_check_decline(&self) -> Value;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_trades_by_id_request(
        &self,
        id: i32,
    ) -> LolLobbyTeamBuilderChampSelectTradeContract;
    fn get_lol_lobby_team_builder_champ_select_v_1_sending_loadouts_gcos_enabled(&self) -> bool;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_swaps_by_id_accept(
        &self,
        id: i32,
    ) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_session_swaps(
        &self,
    ) -> Vec<LolLobbyTeamBuilderChampSelectSwapContract>;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_actions_by_id_complete(
        &self,
        id: i32,
    ) -> Value;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_trades_by_id_cancel(
        &self,
        id: i32,
    ) -> Value;
    fn patch_lol_lobby_team_builder_champ_select_v_1_session_actions_by_id(&self, id: i32)
        -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_session_swaps_by_id(
        &self,
        id: i64,
    ) -> LolLobbyTeamBuilderChampSelectSwapContract;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_bench_swap_by_champion_id(
        &self,
        champion_id: i32,
    ) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_disabled_champion_ids(&self) -> Vec<i32>;
    fn get_lol_lobby_team_builder_champ_select_v_1_bannable_champion_ids(&self) -> Vec<i32>;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_swaps_by_id_request(
        &self,
        id: i32,
    ) -> LolLobbyTeamBuilderChampSelectSwapContract;
    fn get_lol_lobby_team_builder_champ_select_v_1_has_auto_assigned_smite(&self) -> bool;
    fn patch_lol_lobby_team_builder_champ_select_v_1_session_my_selection(&self) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_session_my_selection(
        &self,
    ) -> LolLobbyTeamBuilderChampSelectPlayerSelection;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_my_selection_reroll(&self) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_current_champion(&self) -> i32;
    fn get_lol_lobby_team_builder_champ_select_v_1_session_trades(
        &self,
    ) -> Vec<LolLobbyTeamBuilderChampSelectTradeContract>;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_trades_by_id_accept(
        &self,
        id: i32,
    ) -> Value;
    fn post_lol_lobby_team_builder_champ_select_v_1_retrieve_latest_game_dto(&self) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_preferences(
        &self,
    ) -> LolLobbyTeamBuilderChampionSelectPreferences;
    fn get_lol_lobby_team_builder_champ_select_v_1_session_obfuscated_summoner_ids(
        &self,
    ) -> Vec<u64>;
    fn get_lol_lobby_team_builder_champ_select_v_1_session_trades_by_id(
        &self,
        id: i64,
    ) -> LolLobbyTeamBuilderChampSelectTradeContract;
    fn post_lol_lobby_team_builder_champ_select_v_1_simple_inventory(&self);
    fn get_lol_lobby_team_builder_champ_select_v_1_pickable_skin_ids(&self) -> Vec<i32>;
    fn post_lol_lobby_team_builder_champ_select_v_1_team_boost_purchase(&self) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_session(
        &self,
    ) -> LolLobbyTeamBuilderChampSelectSession;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_trades_by_id_decline(
        &self,
        id: i32,
    ) -> Value;
    fn get_lol_lobby_team_builder_champ_select_v_1_pickable_champion_ids(&self) -> Vec<i32>;
    fn post_lol_lobby_team_builder_v_1_ready_check_accept(&self) -> Value;
    fn get_lol_lobby_team_builder_v_1_matchmaking(
        &self,
    ) -> LolLobbyTeamBuilderMatchmakingSearchResource;
    fn post_lol_lobby_team_builder_champ_select_v_1_session_swaps_by_id_cancel(
        &self,
        id: i32,
    ) -> Value;
}
pub trait PluginLolLogin {
    fn post_lol_login_v_1_session_invoke(
        &self,
        destination: String,
        method: String,
        args: Vec<Value>,
    ) -> LolLoginLcdsResponse;
    fn post_lol_login_v_1_delete_rso_on_close(&self) -> Value;
    fn get_lol_login_v_1_wallet(&self) -> LolLoginLoginSessionWallet;
    fn get_lol_login_v_1_login_in_game_creds(&self) -> Value;
    fn get_lol_login_v_1_login_data_packet(&self) -> Value;
    fn post_lol_login_v_1_service_proxy_async_requests_by_service_name_by_method_name(
        &self,
        service_name: String,
        method_name: String,
        body: u32,
    );
    fn delete_lol_login_v_1_service_proxy_async_requests_by_service_name_by_method_name(
        &self,
        service_name: String,
        method_name: String,
        plugin_id: u32,
    );
    fn get_lol_login_v_1_session(&self) -> LolLoginLoginSession;
    fn delete_lol_login_v_1_session(&self);
    fn post_lol_login_v_1_session(&self) -> LolLoginLoginSession;
    fn post_lol_login_v_1_service_proxy_uuid_requests(
        &self,
        service_name: String,
        method_name: String,
        plugin_id: u32,
        timeout_millis: u64,
        payload: String,
    ) -> String;
    fn get_lol_login_v_1_login_platform_credentials(&self) -> LolLoginPlatformGeneratedCredentials;
    fn delete_lol_login_v_1_shutdown_locks_by_lock_name(&self, lock_name: String);
    fn put_lol_login_v_1_shutdown_locks_by_lock_name(&self, lock_name: String);
    fn post_lol_login_v_1_summoner_session_failed(&self, body: i32) -> Value;
    fn post_lol_login_v_1_change_summoner_name(&self, body: String) -> Value;
    fn get_lol_login_v_2_league_session_init_token(&self) -> LolLoginLeagueSessionTokenEnvelope;
    fn post_lol_login_v_1_summoner_session(&self) -> Value;
    fn post_lol_login_v_1_account_state(&self);
    fn get_lol_login_v_1_account_state(&self) -> LolLoginAccountStateResource;
    fn get_lol_login_v_1_login_connection_state(&self) -> LolLoginLoginConnectionState;
    fn post_lol_login_v_1_league_session_status(&self);
    fn get_lol_login_v_1_login_queue_state(&self) -> LolLoginLoginQueue;
}
pub trait PluginLolLoot {
    fn get_lol_loot_v_1_ready(&self) -> bool;
    fn get_lol_loot_v_1_player_loot_notifications(&self) -> Vec<LolLootPlayerLootNotification>;
    fn post_lol_loot_v_1_craft_mass(&self, body: Vec<CraftLootDto>) -> LolLootPlayerLootUpdate;
    fn get_lol_loot_v_1_player_loot_map(&self) -> Value;
    fn get_lol_loot_v_1_loot_odds_by_recipe_name_visibility(&self, recipe_name: String) -> bool;
    fn get_lol_loot_v_1_mass_disenchant_recipes(&self) -> Vec<LootLcdsRecipeClientDto>;
    fn post_lol_loot_v_1_player_loot_by_loot_id_context_menu(
        &self,
        loot_id: String,
    ) -> Vec<LolLootContextMenu>;
    fn get_lol_loot_v_1_player_loot_by_loot_id_context_menu(
        &self,
        loot_id: String,
    ) -> Vec<LolLootContextMenu>;
    fn get_lol_loot_v_1_loot_grants(&self) -> Vec<LolLootLootGrantNotification>;
    fn get_lol_loot_v_1_mass_disenchant_configuration(&self) -> LolLootMassDisenchantClientConfig;
    fn get_lol_loot_v_1_loot_items(&self) -> Vec<LolLootLootItem>;
    fn get_lol_loot_v_1_enabled(&self) -> bool;
    fn post_lol_loot_v_1_recipes_initial_item_by_loot_id(
        &self,
        loot_id: String,
    ) -> Vec<LolLootRecipeWithMilestones>;
    fn get_lol_loot_v_1_recipes_initial_item_by_loot_id(
        &self,
        loot_id: String,
    ) -> Vec<LolLootRecipeWithMilestones>;
    fn get_lol_loot_v_2_player_loot_map(&self) -> LolLootPlayerLootMap;
    fn get_lol_loot_v_1_milestones_by_loot_milestones_id(
        &self,
        loot_milestones_id: String,
    ) -> LolLootLootMilestones;
    fn post_lol_loot_v_1_milestones_by_loot_milestones_id_claim(&self, loot_milestones_id: String);
    fn get_lol_loot_v_1_player_loot_by_loot_id(&self, loot_id: String) -> LolLootPlayerLoot;
    fn get_lol_loot_v_1_player_display_categories(&self) -> Vec<String>;
    fn get_lol_loot_v_1_milestones_items(&self) -> Vec<String>;
    fn get_lol_loot_v_1_milestones_by_loot_milestones_id_counter(
        &self,
        loot_milestones_id: String,
    ) -> LolLootLootMilestonesCounter;
    fn get_lol_loot_v_1_milestones_by_loot_milestones_id_claim_progress(
        &self,
        loot_milestones_id: String,
    ) -> LolLootLootMilestonesClaimResponse;
    fn get_lol_loot_v_1_milestones(&self, minimize_response: bool) -> Vec<LolLootLootMilestones>;
    fn get_lol_loot_v_1_recipes_configuration(&self);
    fn post_lol_loot_v_1_player_loot_by_loot_name_redeem(
        &self,
        loot_name: String,
    ) -> LolLootPlayerLootUpdate;
    fn get_lol_loot_v_1_milestones_counters(&self) -> Vec<LolLootLootMilestonesCounter>;
    fn put_lol_loot_v_1_loot_odds_evaluate_query(&self) -> Vec<LolLootQueryEvaluatedLootItem>;
    fn delete_lol_loot_v_1_player_loot_by_loot_id_new_notification(&self, loot_id: String)
        -> Value;
    fn get_lol_loot_v_1_loot_odds_by_recipe_name(
        &self,
        recipe_name: String,
    ) -> LolLootVerboseLootOddsResponse;
    fn delete_lol_loot_v_1_loot_grants_by_id(&self, id: i64) -> Value;
    fn get_lol_loot_v_1_player_loot(&self) -> Vec<LolLootPlayerLoot>;
    fn get_lol_loot_v_1_currency_configuration(&self);
    fn post_lol_loot_v_1_player_loot_notifications_by_id_acknowledge(&self, id: String) -> String;
    fn post_lol_loot_v_1_refresh(&self, body: bool) -> String;
    fn post_lol_loot_v_1_recipes_by_recipe_name_craft(
        &self,
        recipe_name: String,
        player_loot_list: Vec<String>,
        repeat: i32,
    ) -> LolLootPlayerLootUpdate;
}
pub trait PluginLolLoyalty {
    fn get_lol_loyalty_v_1_status_notification(&self) -> LolLoyaltyLoyaltyStatusNotification;
    fn post_lol_loyalty_v_1_update_loyalty_inventory(&self);
}
pub trait PluginLolMaps {
    fn get_lol_maps_v_2_map_by_id_by_game_mode_by_game_mutator(
        &self,
        id: i64,
        game_mode: String,
        game_mutator: String,
    ) -> LolMapsMaps;
    fn get_lol_maps_v_2_map_by_id_by_game_mode(&self, id: i64, game_mode: String) -> LolMapsMaps;
    fn post_lol_maps_v_1_map(&self);
    fn get_lol_maps_v_1_map_by_id(&self, id: i64) -> LolMapsMaps;
    fn get_lol_maps_v_2_maps(&self) -> Vec<LolMapsMaps>;
    fn get_lol_maps_v_1_maps(&self) -> Vec<LolMapsMaps>;
}
pub trait PluginLolMarketingPreferences {
    fn post_lol_marketing_preferences_v_1_partition_by_partition_key(
        &self,
        partition_key: String,
        body: Value,
    ) -> Value;
    fn get_lol_marketing_preferences_v_1_partition_by_partition_key(
        &self,
        partition_key: String,
    ) -> Value;
    fn get_lol_marketing_preferences_v_1_ready(&self) -> bool;
}
pub trait PluginLolMatchHistory {
    fn get_lol_match_history_v_1_recently_played_summoners(
        &self,
    ) -> Vec<LolMatchHistoryRecentlyPlayedSummoner>;
    fn get_lol_match_history_v_1_products_lol_current_summoner_matches(
        &self,
        beg_index: u32,
        end_index: u32,
    ) -> LolMatchHistoryMatchHistoryList;
    fn get_lol_match_history_v_1_web_url(&self) -> String;
    fn get_lol_match_history_v_1_products_tft_by_puuid_matches(
        &self,
        puuid: String,
        begin: u32,
        count: u32,
        tag: String,
    ) -> LolMatchHistoryGamhsMatchHistoryList;
    fn post_lol_match_history_v_1_acs_endpoint_override(&self) -> Value;
    fn get_lol_match_history_v_1_delta(&self) -> LolMatchHistoryMatchHistoryPlayerDelta;
    fn get_lol_match_history_v_1_game_timelines_by_game_id(
        &self,
        game_id: u64,
    ) -> LolMatchHistoryMatchHistoryTimelineFrames;
    fn get_lol_match_history_v_1_games_by_game_id(
        &self,
        game_id: u64,
    ) -> LolMatchHistoryMatchHistoryGame;
    fn get_lol_match_history_v_3_matchlist_account_by_account_id(
        &self,
        account_id: u64,
        beg_index: u32,
        end_index: u32,
    ) -> LolMatchHistoryMatchHistoryList;
    fn get_lol_match_history_v_1_products_lol_by_puuid_matches(
        &self,
        puuid: String,
        beg_index: u32,
        end_index: u32,
    ) -> LolMatchHistoryMatchHistoryList;
}
pub trait PluginLolMatchmaking {
    fn post_lol_matchmaking_v_1_ready_check_decline(&self) -> Value;
    fn get_lol_matchmaking_v_1_search(&self) -> LolMatchmakingMatchmakingSearchResource;
    fn post_lol_matchmaking_v_1_search(&self) -> Value;
    fn put_lol_matchmaking_v_1_search(&self) -> Value;
    fn delete_lol_matchmaking_v_1_search(&self) -> Value;
    fn post_lol_matchmaking_v_1_ready_check_accept(&self) -> Value;
    fn get_lol_matchmaking_v_1_search_errors(
        &self,
    ) -> Vec<LolMatchmakingMatchmakingSearchErrorResource>;
    fn get_lol_matchmaking_v_1_ready_check(&self) -> LolMatchmakingMatchmakingReadyCheckResource;
    fn get_lol_matchmaking_v_1_search_errors_by_id(
        &self,
        id: i32,
    ) -> LolMatchmakingMatchmakingSearchErrorResource;
}
pub trait PluginLolMissions {
    fn get_lol_tft_v_2_tft_battlepass(&self) -> LolMissionsTftPaidBattlepass;
    fn get_lol_missions_v_1_series(&self) -> Vec<SeriesDto>;
    fn put_lol_missions_v_1_player(&self);
    fn put_lol_missions_v_2_player_opt(&self);
    fn get_lol_missions_v_1_data(&self) -> PlayerMissionEligibilityData;
    fn put_lol_missions_v_1_player_by_mission_id(&self, mission_id: String);
    fn post_lol_missions_v_1_force(&self);
    fn get_lol_missions_v_1_missions(&self) -> Vec<PlayerMissionDto>;
}
pub trait PluginLolNpeRewards {
    fn get_lol_npe_rewards_v_1_login_rewards(&self) -> LolNpeRewardsRewardSeries;
    fn post_lol_npe_rewards_v_1_challenges_opt(&self);
    fn get_lol_npe_rewards_v_1_challenges_progress(&self) -> LolNpeRewardsChallengesProgress;
    fn get_lol_npe_rewards_v_1_level_rewards_state(&self) -> LolNpeRewardsRewardSeriesState;
    fn get_lol_npe_rewards_v_1_login_rewards_state(&self) -> LolNpeRewardsRewardSeriesState;
    fn get_lol_npe_rewards_v_1_level_rewards(&self) -> LolNpeRewardsRewardSeries;
}
pub trait PluginLolNpeTutorialPath {
    fn patch_lol_npe_tutorial_path_v_1_tutorials_init(&self);
    fn get_lol_npe_tutorial_path_v_1_settings(&self) -> LolNpeTutorialPathAccountSettingsTutorial;
    fn put_lol_npe_tutorial_path_v_1_settings(&self);
    fn get_lol_npe_tutorial_path_v_1_rewards_champ(&self) -> LolNpeTutorialPathCollectionsChampion;
    fn get_lol_npe_tutorial_path_v_1_tutorials(&self) -> Vec<LolNpeTutorialPathTutorial>;
    fn put_lol_npe_tutorial_path_v_1_tutorials_by_tutorial_id_view(&self, tutorial_id: String);
}
pub trait PluginLolPatch {
    fn get_lol_patch_v_1_environment(&self) -> LolPatchChunkingPatcherEnvironment;
    fn get_lol_patch_v_1_products_league_of_legends_supported_game_releases(
        &self,
    ) -> LolPatchSupportedGameReleases;
    fn post_lol_patch_v_1_products_league_of_legends_partial_repair_request(&self);
    fn get_lol_patch_v_1_game_version(&self) -> String;
    fn get_lol_patch_v_1_products_league_of_legends_install_location(&self)
        -> LolPatchInstallPaths;
    fn post_lol_patch_v_1_products_league_of_legends_stop_patching_request(&self, body: bool);
    fn put_lol_patch_v_1_game_patch_url(&self, body: String);
    fn delete_lol_patch_v_1_notifications_by_id(&self, id: String);
    fn get_lol_patch_v_1_products_league_of_legends_state(&self) -> LolPatchProductState;
    fn post_lol_patch_v_1_products_league_of_legends_start_patching_request(&self);
    fn get_lol_patch_v_1_notifications(&self) -> Vec<LolPatchNotification>;
    fn get_lol_patch_v_1_checking_enabled(&self) -> bool;
    fn post_lol_patch_v_1_products_league_of_legends_detect_corruption_request(&self);
    fn post_lol_patch_v_1_products_league_of_legends_start_checking_request(&self);
    fn post_lol_patch_v_1_products_league_of_legends_stop_checking_request(&self);
    fn get_lol_patch_v_1_status(&self) -> LolPatchStatus;
    fn put_lol_patch_v_1_ux(&self);
}
pub trait PluginLolPerks {
    fn get_lol_perks_v_1_quick_play_selections_champion_by_champion_id_position_by_position(
        &self,
        champion_id: i32,
        position: String,
    ) -> String;
    fn put_lol_perks_v_1_currentpage(&self, body: i32) -> Value;
    fn get_lol_perks_v_1_currentpage(&self) -> LolPerksPerkPageResource;
    fn post_lol_perks_v_1_recommended_pages_position_champion_by_champion_id_position_by_position(
        &self,
        champion_id: i32,
        position: String,
    ) -> Value;
    fn get_lol_perks_v_1_inventory(&self) -> LolPerksPlayerInventory;
    fn get_lol_perks_v_1_settings(&self) -> LolPerksUiSettings;
    fn put_lol_perks_v_1_settings(&self) -> Value;
    fn delete_lol_perks_v_1_pages_by_id_auto_modified_selections(&self, id: i32) -> Value;
    fn put_lol_perks_v_1_perks_ack_gameplay_updated(&self, body: Vec<i32>) -> Value;
    fn get_lol_perks_v_1_recommended_pages_position_champion_by_champion_id(
        &self,
        champion_id: i32,
    ) -> String;
    fn get_lol_perks_v_1_perks_disabled(&self) -> Vec<i32>;
    fn get_lol_perks_v_1_perks_gameplay_updated(&self) -> Vec<i32>;
    fn delete_lol_perks_v_1_pages_by_id(&self, id: i32) -> Value;
    fn get_lol_perks_v_1_pages_by_id(&self, id: i32) -> LolPerksPerkPageResource;
    fn put_lol_perks_v_1_pages_by_id(&self, id: i32) -> LolPerksPerkPageResource;
    fn get_lol_perks_v_1_recommended_pages_champion_by_champion_id_position_by_position_map_by_map_id(
        &self,
        champion_id: i32,
        position: String,
        map_id: i32,
    ) -> Vec<LolPerksPerkUiRecommendedPage>;
    fn put_lol_perks_v_1_pages_validate(&self) -> LolPerksValidateItemSetNameResponse;
    fn get_lol_perks_v_1_pages(&self) -> Vec<LolPerksPerkPageResource>;
    fn delete_lol_perks_v_1_pages(&self) -> Value;
    fn post_lol_perks_v_1_pages(&self) -> LolPerksPerkPageResource;
    fn get_lol_perks_v_1_rune_recommender_auto_select(&self) -> bool;
    fn post_lol_perks_v_1_rune_recommender_auto_select(&self) -> Value;
    fn get_lol_perks_v_1_show_auto_modified_pages_notification(&self) -> bool;
    fn post_lol_perks_v_1_show_auto_modified_pages_notification(&self) -> Value;
    fn get_lol_perks_v_1_styles(&self) -> Vec<LolPerksPerkUiStyle>;
    fn get_lol_perks_v_1_perks(&self) -> Vec<LolPerksPerkUiPerk>;
    fn post_lol_perks_v_1_update_page_order(&self) -> Value;
}
pub trait PluginLolPft {
    fn post_lol_pft_v_2_events(&self) -> Value;
    fn get_lol_pft_v_2_survey(&self) -> LolPftPftSurvey;
    fn post_lol_pft_v_2_survey(&self);
}
pub trait PluginLolPlatformConfig {
    fn get_lol_platform_config_v_1_namespaces(&self) -> Value;
    fn get_lol_platform_config_v_1_initial_configuration_complete(&self) -> bool;
    fn get_lol_platform_config_v_1_namespaces_by_ns_by_key(&self, ns: String, key: String)
        -> Value;
    fn get_lol_platform_config_v_1_namespaces_by_ns(&self, ns: String) -> Value;
}
pub trait PluginLolPlayerBehavior {
    fn get_lol_player_behavior_v_2_reporter_feedback(
        &self,
    ) -> Vec<LolPlayerBehaviorReporterFeedbackMessage>;
    fn get_lol_player_behavior_v_1_ranked_restriction(
        &self,
    ) -> LolPlayerBehaviorRestrictionNotification;
    fn get_lol_player_behavior_v_1_config(&self) -> LolPlayerBehaviorPlayerBehaviorConfig;
    fn get_lol_player_behavior_v_1_reporter_feedback(
        &self,
    ) -> Vec<LolPlayerBehaviorReporterFeedback>;
    fn put_lol_player_behavior_v_3_reform_card_by_id(&self, id: String);
    fn put_lol_player_behavior_v_1_ack_credibility_behavior_warning_by_mail_id(
        &self,
        mail_id: String,
    );
    fn get_lol_player_behavior_v_2_reform_card(&self) -> LolPlayerBehaviorReformCardV2;
    fn get_lol_player_behavior_v_3_reform_cards(&self) -> Value;
    fn get_lol_player_behavior_v_1_ban(&self) -> LolPlayerBehaviorBanNotification;
    fn delete_lol_player_behavior_v_1_reporter_feedback_by_id(
        &self,
        id: String,
    ) -> LolPlayerBehaviorReporterFeedback;
    fn get_lol_player_behavior_v_1_reporter_feedback_by_id(
        &self,
        id: String,
    ) -> LolPlayerBehaviorReporterFeedback;
    fn get_lol_player_behavior_v_1_reform_card(&self) -> LolPlayerBehaviorReformCard;
    fn post_lol_player_behavior_v_2_reporter_feedback_by_key(&self, key: String);
    fn get_lol_player_behavior_v_1_credibility_behavior_warnings(&self);
    fn get_lol_player_behavior_v_1_chat_restriction(
        &self,
    ) -> LolPlayerBehaviorRestrictionNotification;
    fn get_lol_player_behavior_v_1_code_of_conduct_notification(
        &self,
    ) -> LolPlayerBehaviorCodeOfConductNotification;
    fn delete_lol_player_behavior_v_1_code_of_conduct_notification(&self) -> Value;
}
pub trait PluginLolPlayerLevelUp {
    fn get_lol_player_level_up_v_1_level_up(&self) -> LolPlayerLevelUpPlayerLevelUpEvent;
    fn post_lol_player_level_up_v_1_level_up_notifications_by_plugin_name(
        &self,
        plugin_name: String,
    );
    fn get_lol_player_level_up_v_1_level_up_notifications_by_plugin_name(
        &self,
        plugin_name: String,
    ) -> LolPlayerLevelUpPlayerLevelUpEventAck;
}
pub trait PluginLolPlayerMessaging {
    fn get_lol_player_messaging_v_1_notification(
        &self,
    ) -> LolPlayerMessagingPlayerMessagingNotificationResource;
    fn delete_lol_player_messaging_v_1_notification_by_id_acknowledge(&self, id: u32) -> Value;
    fn delete_lol_player_messaging_v_1_celebration_notification_by_id_acknowledge(
        &self,
        id: u32,
    ) -> Value;
    fn get_lol_player_messaging_v_1_celebration_notification(
        &self,
    ) -> LolPlayerMessagingDynamicCelebrationMessagingNotificationResource;
}
pub trait PluginLolPlayerPreferences {
    fn get_lol_player_preferences_v_1_player_preferences_ready(&self) -> bool;
    fn post_lol_player_preferences_v_1_player_preferences_endpoint_override(&self) -> Value;
    fn get_lol_player_preferences_v_1_preference_by_type(&self, _type: String) -> Value;
    fn put_lol_player_preferences_v_1_preference(&self) -> Value;
}
pub trait PluginLolPlayerReportSender {
    fn post_lol_player_report_sender_v_1_champ_select_reports(&self) -> Value;
    fn get_lol_player_report_sender_v_1_game_ids_with_verbal_abuse_report(&self) -> Vec<u64>;
    fn delete_lol_player_report_sender_v_1_reported_players_game_id_by_game_id(
        &self,
        game_id: u64,
    ) -> Value;
    fn get_lol_player_report_sender_v_1_reported_players_game_id_by_game_id(
        &self,
        game_id: u64,
    ) -> Vec<String>;
    fn post_lol_player_report_sender_v_1_match_history_reports(&self) -> Value;
    fn post_lol_player_report_sender_v_1_end_of_game_reports(&self) -> Value;
    fn get_lol_player_report_sender_v_1_in_game_reports(
        &self,
    ) -> Vec<LolPlayerReportSenderPlayerReport>;
    fn post_lol_player_report_sender_v_1_in_game_reports(&self) -> Value;
}
pub trait PluginLolPreEndOfGame {
    fn post_lol_pre_end_of_game_v_1_complete_by_sequence_event_name(
        &self,
        sequence_event_name: String,
    );
    fn get_lol_pre_end_of_game_v_1_current_sequence_event(&self) -> LolPreEndOfGameSequenceEvent;
    fn post_lol_pre_end_of_game_v_1_registration_by_sequence_event_name_by_priority(
        &self,
        sequence_event_name: String,
        priority: i32,
    );
    fn delete_lol_pre_end_of_game_v_1_registration_by_sequence_event_name(
        &self,
        sequence_event_name: String,
    );
}
pub trait PluginLolPremadeVoice {
    fn post_lol_premade_voice_v_1_first_experience_reset(&self);
    fn put_lol_premade_voice_v_1_participants_by_puuid_mute(&self, puuid: String, body: i32);
    fn post_lol_premade_voice_v_1_first_experience_lcu(&self);
    fn put_lol_premade_voice_v_1_self_activation_sensitivity(&self, body: i32);
    fn get_lol_premade_voice_v_1_participants(
        &self,
    ) -> Vec<LolPremadeVoicePremadeVoiceParticipantDto>;
    fn post_lol_premade_voice_v_1_first_experience_game(&self);
    fn post_lol_premade_voice_v_1_settings_reset(&self);
    fn post_lol_premade_voice_v_1_game_client_updated_ptt_key(&self, body: String);
    fn put_lol_premade_voice_v_1_self_mic_level(&self, body: i32);
    fn get_lol_premade_voice_v_1_capturedevices(&self) -> Vec<LolPremadeVoiceDeviceResource>;
    fn put_lol_premade_voice_v_1_capturedevices(&self, body: String);
    fn post_lol_premade_voice_v_1_push_to_talk_check_available(&self, body: i32) -> bool;
    fn post_lol_premade_voice_v_1_session(&self);
    fn delete_lol_premade_voice_v_1_session(&self);
    fn get_lol_premade_voice_v_1_first_experience(&self) -> LolPremadeVoiceFirstExperience;
    fn put_lol_premade_voice_v_1_participants_by_puuid_volume(&self, puuid: String, body: i32);
    fn put_lol_premade_voice_v_1_self_input_mode(&self);
    fn get_lol_premade_voice_v_1_mic_test(&self) -> LolPremadeVoiceAudioPropertiesResource;
    fn delete_lol_premade_voice_v_1_mic_test(&self) -> Value;
    fn post_lol_premade_voice_v_1_mic_test(&self) -> Value;
    fn get_lol_premade_voice_v_1_settings(&self) -> LolPremadeVoiceSettingsResource;
    fn put_lol_premade_voice_v_1_self_mute(&self, body: i32);
    fn get_lol_premade_voice_v_1_availability(&self) -> LolPremadeVoiceVoiceAvailability;
    fn get_lol_premade_voice_v_1_participant_records(
        &self,
    ) -> Vec<LolPremadeVoicePremadeVoiceParticipantDto>;
}
pub trait PluginLolProgression {
    fn get_lol_progression_v_1_ready(&self) -> bool;
    fn get_lol_progression_v_1_groups_by_group_id_instance_data(
        &self,
        group_id: String,
    ) -> LolProgressionEntityInstance;
    fn get_lol_progression_v_1_groups_configuration(&self) -> Vec<LolProgressionGroup>;
    fn get_lol_progression_v_1_groups_by_group_id_configuration(
        &self,
        group_id: String,
    ) -> LolProgressionGroup;
}
pub trait PluginLolPublishingContent {
    fn get_lol_publishing_content_v_1_settings(&self) -> LolPublishingContentPublishingSettings;
    fn get_lol_publishing_content_v_1_listeners_pubhub_config(
        &self,
    ) -> LolPublishingContentPubHubConfig;
    fn get_lol_publishing_content_v_1_listeners_allow_list_by_region(
        &self,
        region: String,
    ) -> Vec<String>;
    fn get_lol_publishing_content_v_1_tft_hub_cards(&self) -> Value;
    fn get_lol_publishing_content_v_1_listeners_client_data(
        &self,
    ) -> LolPublishingContentClientData;
    fn get_lol_publishing_content_v_1_ready(&self) -> bool;
}
pub trait PluginLolPurchaseWidget {
    fn post_lol_purchase_widget_v_1_purchasable_items_by_inventory_type(
        &self,
        inventory_type: String,
        body: Vec<i64>,
    ) -> LolPurchaseWidgetItemChoices;
    fn post_lol_purchase_widget_v_2_purchase_items(&self) -> Value;
    fn get_lol_purchase_widget_v_3_base_skin_line_data_by_offer_id(
        &self,
        offer_id: String,
    ) -> LolPurchaseWidgetBaseSkinLineDto;
    fn post_lol_purchase_widget_v_3_purchase_offer(
        &self,
    ) -> LolPurchaseWidgetPurchaseOfferResponseV3;
    fn post_lol_purchase_widget_v_3_validate_offer(
        &self,
    ) -> LolPurchaseWidgetValidateOfferResponseV3;
    fn post_lol_purchase_widget_v_3_purchase_offer_via_cap(
        &self,
    ) -> LolPurchaseWidgetPurchaseOfferResponseV3;
    fn get_lol_purchase_widget_v_1_purchasable_item(
        &self,
        inventory_type: String,
        item_id: i64,
    ) -> LolPurchaseWidgetPurchasableItem;
    fn get_lol_purchase_widget_v_1_configuration(&self) -> LolPurchaseWidgetPurchaseWidgetConfig;
    fn get_lol_purchase_widget_v_1_order_notifications(
        &self,
    ) -> Vec<LolPurchaseWidgetOrderNotificationResource>;
    fn get_lol_purchase_widget_v_3_purchase_offer_order_statuses(
        &self,
    ) -> LolPurchaseWidgetPurchaseOfferOrderStatuses;
}
pub trait PluginLolRanked {
    fn get_lol_ranked_v_1_current_lp_change_notification(&self) -> LolRankedLcuLeagueNotification;
    fn get_lol_ranked_v_1_league_ladders_by_puuid(
        &self,
        puuid: String,
    ) -> Vec<LolRankedLeagueLadderInfo>;
    fn get_lol_ranked_v_1_ranked_stats_by_puuid(&self, puuid: String) -> LolRankedRankedStats;
    fn get_lol_ranked_v_1_rated_ladder_by_queue_type(
        &self,
        queue_type: LolRankedLeagueQueueType,
    ) -> LolRankedRatedLadderInfo;
    fn get_lol_ranked_v_2_tiers(
        &self,
        summoner_ids: Vec<u64>,
        queue_types: Vec<LolRankedLeagueQueueType>,
    ) -> Vec<LolRankedParticipantTiers>;
    fn get_lol_ranked_v_1_eos_rewards(&self) -> LolRankedEosRewardsConfig;
    fn get_lol_ranked_v_1_social_leaderboard_ranked_queue_stats_for_puuids(
        &self,
        queue_type: LolRankedLeagueQueueType,
        puuids: Vec<String>,
    ) -> Value;
    fn get_lol_ranked_v_1_signed_ranked_stats(&self) -> LolRankedSignedRankedStatsDto;
    fn get_lol_ranked_v_1_challenger_ladders_enabled(&self) -> Vec<String>;
    fn get_lol_ranked_v_1_eligible_tiers_queue_type_by_queue_type(
        &self,
        queue_type: LolRankedLeagueQueueType,
    ) -> Vec<String>;
    fn get_lol_ranked_v_1_apex_leagues_by_queue_type_by_tier(
        &self,
        queue_type: LolRankedLeagueQueueType,
        tier: String,
    ) -> LolRankedLeagueLadderInfo;
    fn get_lol_ranked_v_1_current_ranked_stats(&self) -> LolRankedRankedStats;
    fn get_lol_ranked_v_1_splits_config(&self) -> LolRankedRewardsInfo;
    fn get_lol_ranked_v_1_eos_notifications(&self) -> Vec<LolRankedEosNotificationResource>;
    fn post_lol_ranked_v_1_eos_notifications_by_id_acknowledge(&self, id: String) -> Value;
    fn post_lol_ranked_v_1_notifications_by_id_acknowledge(&self, id: u64) -> Value;
    fn get_lol_ranked_v_1_notifications(&self) -> Vec<LolRankedLcuLeagueNotification>;
    fn get_lol_ranked_v_1_top_rated_ladders_enabled(&self) -> Vec<String>;
}
pub trait PluginLolRegalia {
    fn get_lol_regalia_v_2_summoners_by_summoner_id_regalia_async(
        &self,
        summoner_id: u64,
    ) -> LolRegaliaRegaliaAsync;
    fn get_lol_regalia_v_2_config(&self) -> LolRegaliaRegaliaFrontendConfig;
    fn get_lol_regalia_v_3_summoners_by_summoner_id_regalia(
        &self,
        summoner_id: u64,
    ) -> LolRegaliaRegalia;
    fn get_lol_regalia_v_2_summoners_by_summoner_id_regalia(
        &self,
        summoner_id: u64,
        hovercard: bool,
    ) -> LolRegaliaRegalia;
    fn put_lol_regalia_v_2_current_summoner_regalia(&self) -> LolRegaliaRegaliaWithPreferences;
    fn get_lol_regalia_v_2_current_summoner_regalia(&self) -> LolRegaliaRegaliaWithPreferences;
    fn get_lol_regalia_v_2_summoners_by_summoner_id_queues_by_queue_regalia(
        &self,
        summoner_id: u64,
        queue: String,
    ) -> LolRegaliaRegalia;
    fn get_lol_regalia_v_3_inventory_by_inventory_type(&self, inventory_type: String) -> Value;
    fn get_lol_regalia_v_2_summoners_by_summoner_id_queues_by_queue_positions_by_position_regalia(
        &self,
        summoner_id: u64,
        queue: String,
        position: String,
    ) -> LolRegaliaRegalia;
}
pub trait PluginLolRemedy {
    fn get_lol_remedy_v_1_remedy_notifications(&self) -> Vec<LolRemedyMail>;
    fn put_lol_remedy_v_1_ack_remedy_notification_by_mail_id(&self, mail_id: String);
    fn get_lol_remedy_v_1_config_is_verbal_abuse_remedy_modal_enabled(&self) -> bool;
}
pub trait PluginLolReplays {
    fn post_lol_replays_v_1_rofls_by_game_id_download(&self, game_id: u64);
    fn post_lol_replays_v_1_rofls_by_game_id_download_graceful(&self, game_id: u64);
    fn get_lol_replays_v_1_rofls_path(&self) -> String;
    fn post_lol_replays_v_2_metadata_by_game_id_create(&self, game_id: u64);
    fn post_lol_replays_v_1_rofls_by_game_id_watch(&self, game_id: u64);
    fn post_lol_replays_v_1_metadata_by_game_id_create_game_version_by_game_version_game_type_by_game_type_queue_id_by_queue_id(
        &self,
        game_id: u64,
        game_version: String,
        game_type: String,
        queue_id: i32,
    );
    fn post_lol_replays_v_1_rofls_scan(&self);
    fn get_lol_replays_v_1_rofls_path_default(&self) -> String;
    fn get_lol_replays_v_1_configuration(&self) -> LolReplaysReplaysConfiguration;
    fn get_lol_replays_v_1_metadata_by_game_id(&self, game_id: u64) -> LolReplaysReplayMetadata;
}
pub trait PluginLolRewards {
    fn get_lol_rewards_v_1_groups(&self, types: Vec<String>) -> Vec<LolRewardsSvcRewardGroup>;
    fn patch_lol_rewards_v_1_grants_view(&self, body: Vec<String>);
    fn post_lol_rewards_v_1_reward_replay(&self, body: String);
    fn post_lol_rewards_v_1_grants_by_grant_id_select(
        &self,
        grant_id: String,
    ) -> LolRewardsRewardGrant;
    fn post_lol_rewards_v_1_select_bulk(&self, body: Vec<LolRewardsSelectionRequestDto>) -> Value;
    fn get_lol_rewards_v_1_grants(
        &self,
        status: LolRewardsGrantStatus,
    ) -> Vec<LolRewardsRewardGrant>;
}
pub trait PluginLolRiotMessagingService {
    fn get_lol_rms_v_1_champion_mastery_leaveup_update(
        &self,
    ) -> Vec<LolRiotMessagingServiceChampionMasteryLevelUp>;
    fn delete_lol_rms_v_1_champion_mastery_leaveup_update_by_id(&self, id: u64) -> Value;
}
pub trait PluginLolRsoAuth {
    fn get_lol_rso_auth_v_1_authorization(&self) -> LolRsoAuthAuthorization;
    fn delete_lol_rso_auth_v_1_authorization(&self);
    fn post_lol_rso_auth_v_1_authorization_refresh(&self) -> LolRsoAuthAuthorization;
    fn get_lol_rso_auth_configuration_v_3_ready_state(&self) -> LolRsoAuthRsoConfigReadyState;
    fn post_lol_rso_auth_v_1_authorization_userinfo(&self) -> LolRsoAuthUserInfo;
    fn get_lol_rso_auth_v_1_authorization_userinfo(&self) -> LolRsoAuthUserInfo;
    fn get_lol_rso_auth_v_1_authorization_access_token(&self) -> LolRsoAuthAccessToken;
    fn delete_lol_rso_auth_v_2_config(&self);
    fn post_lol_rso_auth_v_2_config(&self);
    fn get_lol_rso_auth_v_1_authorization_id_token(&self) -> LolRsoAuthIdToken;
    fn post_lol_rso_auth_v_1_device_id(&self) -> LolRsoAuthDeviceId;
    fn post_lol_rso_auth_v_1_external_session_config(&self, body: Value) -> Value;
    fn get_lol_rso_auth_v_1_authorization_error(&self) -> LolRsoAuthAuthError;
    fn get_lol_rso_auth_v_1_status_by_platform_id(
        &self,
        platform_id: String,
    ) -> LolRsoAuthRegionStatus;
    fn post_lol_rso_auth_v_1_authorization_gas(&self) -> LolRsoAuthAuthorization;
    fn delete_lol_rso_auth_v_1_session(&self);
}
pub trait PluginLolSeasons {
    fn post_lol_seasons_v_1_all_seasons_product_by_product(
        &self,
        product: String,
    ) -> Vec<LolSeasonsAllSeasonsProduct>;
    fn get_lol_seasons_v_1_season_product_by_product(
        &self,
        product: String,
    ) -> LolSeasonsAllSeasonsProduct;
    fn get_lol_seasons_v_1_season_recent_final_split(&self) -> LolSeasonsAllSeasonsProduct;
    fn get_lol_seasons_v_1_season_lol_current_split_seasons(&self) -> Vec<i64>;
}
pub trait PluginLolServiceStatus {
    fn get_lol_service_status_v_1_ticker_messages(&self) -> Vec<LolServiceStatusTickerMessage>;
    fn get_lol_service_status_v_1_lcu_status(&self) -> LolServiceStatusServiceStatusResource;
}
pub trait PluginLolSettings {
    fn get_lol_settings_v_2_config(&self) -> LolSettingsSettingsConfig;
    fn get_lol_settings_v_2_didreset_by_pp_type(&self, pp_type: String) -> bool;
    fn patch_lol_settings_v_1_local_by_category(&self, category: String) -> Value;
    fn get_lol_settings_v_1_local_by_category(&self, category: String) -> Value;
    fn post_lol_settings_v_1_account_save(&self);
    fn patch_lol_settings_v_1_account_by_category(&self, category: String) -> Value;
    fn get_lol_settings_v_1_account_by_category(&self, category: String) -> Value;
    fn put_lol_settings_v_1_account_by_category(&self, category: String) -> Value;
    fn patch_lol_settings_v_2_local_by_category(&self, category: String) -> Value;
    fn get_lol_settings_v_2_local_by_category(&self, category: String) -> Value;
    fn post_lol_settings_v_2_reload_by_pp_type(&self, pp_type: String);
    fn get_lol_settings_v_1_account_didreset(&self) -> bool;
    fn patch_lol_settings_v_2_account_by_pp_type_by_category(
        &self,
        pp_type: String,
        category: String,
    ) -> Value;
    fn put_lol_settings_v_2_account_by_pp_type_by_category(
        &self,
        pp_type: String,
        category: String,
    ) -> Value;
    fn get_lol_settings_v_2_account_by_pp_type_by_category(
        &self,
        pp_type: String,
        category: String,
    ) -> Value;
    fn get_lol_settings_v_2_ready(&self) -> bool;
}
pub trait PluginLolShutdown {
    fn get_lol_shutdown_v_1_notification(&self) -> LolShutdownShutdownNotification;
}
pub trait PluginLolSimpleDialogMessages {
    fn delete_lol_simple_dialog_messages_v_1_messages_by_message_id(
        &self,
        message_id: i64,
    ) -> Value;
    fn get_lol_simple_dialog_messages_v_1_messages(&self) -> Vec<LolSimpleDialogMessagesMessage>;
    fn post_lol_simple_dialog_messages_v_1_messages(&self) -> Value;
}
pub trait PluginLolSocialLeaderboard {
    fn get_lol_social_leaderboard_v_1_social_leaderboard_data(
        &self,
        queue_type: LolSocialLeaderboardLeagueQueueType,
    ) -> LolSocialLeaderboardSocialLeaderboardData;
    fn get_lol_social_leaderboard_v_1_leaderboard_next_update_time(
        &self,
        queue_type: LolSocialLeaderboardLeagueQueueType,
    ) -> i64;
}
pub trait PluginLolSpectator {
    fn post_lol_spectator_v_2_buddy_spectate(
        &self,
        body: Vec<u64>,
    ) -> LolSpectatorSummonerIdAvailability;
    fn post_lol_spectator_v_3_buddy_spectate(&self, body: Vec<String>);
    fn get_lol_spectator_v_3_buddy_spectate(&self) -> LolSpectatorSpectateResource;
    fn post_lol_spectator_v_1_spectate_launch(&self) -> Value;
    fn get_lol_spectator_v_1_spectate_config(&self) -> LolSpectatorSpectatorConfig;
    fn get_lol_spectator_v_1_spectate(&self) -> LolSpectatorSpectateGameInfo;
    fn post_lol_spectator_v_1_buddy_spectate(
        &self,
        body: Vec<String>,
    ) -> LolSpectatorSummonerOrTeamAvailabilty;
}
pub trait PluginLolStatstones {
    fn delete_lol_statstones_v_1_vignette_notifications_by_key(&self, key: i32);
    fn get_lol_statstones_v_1_statstones_enabled_queue_ids(&self) -> Vec<u32>;
    fn get_lol_statstones_v_1_eog_notifications_by_game_id(
        &self,
        game_id: u64,
    ) -> LolStatstonesEogNotificationEnvelope;
    fn get_lol_statstones_v_1_profile_summary_by_puuid(
        &self,
        puuid: String,
    ) -> Vec<LolStatstonesProfileStatstoneSummary>;
    fn get_lol_statstones_v_1_statstone_by_content_id_owned(&self, content_id: String) -> bool;
    fn get_lol_statstones_v_2_player_statstones_self_by_champion_item_id(
        &self,
        champion_item_id: i32,
    ) -> Vec<LolStatstonesStatstoneSet>;
    fn post_lol_statstones_v_1_featured_champion_statstones_by_champion_item_id_by_statstone_id(
        &self,
        champion_item_id: i32,
        statstone_id: String,
    ) -> Value;
    fn get_lol_statstones_v_2_player_summary_self(
        &self,
    ) -> Vec<LolStatstonesChampionStatstoneSummary>;
    fn get_lol_statstones_v_1_featured_champion_statstones_by_champion_item_id(
        &self,
        champion_item_id: i32,
    ) -> Vec<LolStatstonesStatstone>;
    fn delete_lol_statstones_v_1_vignette_notifications(&self);
    fn get_lol_statstones_v_1_vignette_notifications(&self) -> Vec<Value>;
}
pub trait PluginLolStore {
    fn post_lol_store_v_1_notifications_acknowledge(&self, body: String) -> Value;
    fn get_lol_store_v_1_catalog_by_inventory_type(
        &self,
        inventory_type: String,
        item_ids: Vec<i32>,
    ) -> Vec<LolStoreCatalogItem>;
    fn get_lol_store_v_1_status(&self) -> LolStoreStoreStatus;
    fn get_lol_store_v_1_order_notifications(&self) -> Vec<LolStoreOrderNotificationResource>;
    fn get_lol_store_v_1_by_page_type(&self, page_type: String) -> Value;
    fn get_lol_store_v_2_offers(&self, type_id: String) -> Vec<LolStoreCapOffer>;
    fn get_lol_store_v_1_order_notifications_by_id(
        &self,
        id: u64,
    ) -> LolStoreOrderNotificationResource;
    fn get_lol_store_v_1_last_page(&self) -> String;
    fn post_lol_store_v_1_last_page(&self, body: String);
    fn get_lol_store_v_1_catalog_items_skip_cache(
        &self,
        catalog_item_keys: Vec<LolStoreItemKey>,
    ) -> Vec<LolStoreCatalogItem>;
    fn get_lol_store_v_1_get_store_url(&self) -> String;
    fn get_lol_store_v_1_catalog_sales(&self) -> Vec<LolStoreItemSale>;
    fn get_lol_store_v_1_skins_by_skin_id(&self, skin_id: i32) -> LolStoreCatalogItem;
    fn get_lol_store_v_1_offers_by_offer_id(&self, offer_id: String) -> LolStoreCapOffer;
    fn get_lol_store_v_1_catalog(
        &self,
        inventory_type: Vec<String>,
        item_id: Vec<i32>,
    ) -> Vec<LolStoreCatalogItem>;
    fn get_lol_store_v_1_payment_details(
        &self,
        action: String,
        gift_recipient_account_id: u64,
        gift_message: String,
    ) -> Value;
    fn get_lol_store_v_1_catalog_by_instance_ids(
        &self,
        instance_ids: Vec<String>,
    ) -> Vec<LolStoreCatalogItem>;
    fn get_lol_store_v_1_alias_change_notifications(
        &self,
    ) -> Vec<LolStoreAliasChangeNotificationResource>;
    fn get_lol_store_v_1_giftablefriends(&self) -> Vec<LolStoreGiftingFriend>;
    fn get_lol_store_v_1_item_keys_from_offer_ids(&self, offer_ids: Vec<String>) -> Value;
    fn post_lol_store_v_3_purchase(
        &self,
        body: Vec<LolStoreItemOrderDto>,
    ) -> LolStorePurchaseOrderResponseDto;
    fn get_lol_store_v_1_store_ready(&self) -> bool;
    fn get_lol_store_v_1_item_keys_from_instance_ids(&self, instance_ids: Vec<String>) -> Value;
    fn get_lol_store_v_1_offers(&self, inventory_type_uui_ds: Vec<String>)
        -> Vec<LolStoreCapOffer>;
}
pub trait PluginLolSuggestedPlayers {
    fn get_lol_suggested_players_v_1_suggested_players(
        &self,
    ) -> Vec<LolSuggestedPlayersSuggestedPlayersSuggestedPlayer>;
    fn post_lol_suggested_players_v_1_victorious_comrade(&self);
    fn post_lol_suggested_players_v_1_reported_player(&self);
    fn delete_lol_suggested_players_v_1_suggested_players_by_summoner_id(
        &self,
        summoner_id: u64,
    ) -> Value;
}
pub trait PluginLolSummoner {
    fn get_lol_summoner_v_1_summoners(&self, name: String) -> LolSummonerSummoner;
    fn post_lol_summoner_v_1_summoners(&self) -> LolSummonerSummoner;
    fn post_lol_summoner_v_2_summoners_puuid(&self, body: Vec<String>) -> Vec<LolSummonerSummoner>;
    fn get_lol_summoner_v_1_check_name_availability_new_summoners_by_name(
        &self,
        name: String,
    ) -> bool;
    fn get_lol_summoner_v_2_summoners_puuid_by_puuid(&self, puuid: String) -> LolSummonerSummoner;
    fn get_lol_summoner_v_1_status(&self) -> LolSummonerStatus;
    fn get_lol_summoner_v_1_summoner_profile(&self, puuid: String) -> Value;
    fn get_lol_summoner_v_1_player_name_mode(&self) -> LolSummonerPlayerNameMode;
    fn post_lol_summoner_v_1_summoner_aliases_by_puuids(&self, body: Vec<String>) -> Value;
    fn get_lol_summoner_v_1_current_summoner_account_and_summoner_ids(
        &self,
    ) -> LolSummonerAccountIdAndSummonerId;
    fn post_lol_summoner_v_1_save_alias(&self) -> LolSummonerAliasAvailability;
    fn get_lol_summoner_v_1_player_alias_state(&self) -> LolSummonerPlayerNameState;
    fn get_lol_summoner_v_2_summoner_names(
        &self,
        ids: Vec<u64>,
    ) -> Vec<LolSummonerSummonerIdAndName>;
    fn get_lol_summoner_v_1_current_summoner(&self) -> LolSummonerSummoner;
    fn get_lol_summoner_v_1_riot_alias_free_eligibility(&self) -> bool;
    fn put_lol_summoner_v_1_current_summoner_icon(&self) -> LolSummonerSummoner;
    fn post_lol_summoner_v_1_current_summoner_name(&self, body: String) -> LolSummonerSummoner;
    fn post_lol_summoner_v_1_summoner_aliases_by_ids(&self, body: Vec<u64>) -> Value;
    fn get_lol_summoner_v_1_current_summoner_autofill(&self) -> Vec<LolSummonerAutoFillQueueDto>;
    fn get_lol_summoner_v_1_summoners_by_puuid_cached_by_puuid(
        &self,
        puuid: String,
    ) -> LolSummonerSummoner;
    fn put_lol_summoner_v_1_current_summoner_profile_privacy(&self) -> Value;
    fn get_lol_summoner_v_1_current_summoner_profile_privacy(&self) -> LolSummonerProfilePrivacy;
    fn post_lol_summoner_v_1_summoners_aliases(
        &self,
        body: Vec<LolSummonerAlias>,
    ) -> Vec<LolSummonerSummoner>;
    fn post_lol_summoner_v_1_current_summoner_summoner_profile(&self) -> Value;
    fn get_lol_summoner_v_1_current_summoner_summoner_profile(&self) -> Value;
    fn get_lol_summoner_v_1_profile_privacy_enabled(&self)
        -> LolSummonerProfilePrivacyEnabledState;
    fn post_lol_summoner_v_1_validate_alias(&self) -> LolSummonerAliasAvailability;
    fn post_lol_summoner_v_2_summoners_names(&self, body: Vec<String>) -> Vec<LolSummonerSummoner>;
    fn get_lol_summoner_v_1_check_name_availability_by_name(&self, name: String) -> bool;
    fn get_lol_summoner_v_2_summoner_icons(
        &self,
        ids: Vec<u64>,
    ) -> Vec<LolSummonerSummonerIdAndIcon>;
    fn get_lol_summoner_v_1_summoner_requests_ready(&self) -> bool;
    fn get_lol_summoner_v_1_summoners_by_id(&self, id: u64) -> LolSummonerSummoner;
    fn get_lol_summoner_v_1_riot_alias_purchase_eligibility(&self) -> bool;
    fn get_lol_summoner_v_1_current_summoner_reroll_points(
        &self,
    ) -> LolSummonerSummonerRerollPoints;
    fn get_lol_summoner_v_1_alias_lookup(
        &self,
        game_name: String,
        tag_line: String,
    ) -> LolSummonerAliasLookupResponse;
    fn get_lol_summoner_v_1_current_summoner_jwt(&self) -> String;
    fn get_lol_summoner_v_2_summoners(&self, ids: Vec<u64>) -> Vec<LolSummonerSummoner>;
}
pub trait PluginLolTastes {
    fn get_lol_tastes_v_1_skins_model(&self) -> LolTastesDataModelResponse;
    fn get_lol_tastes_v_1_tft_overview_model(&self) -> LolTastesDataModelResponse;
    fn get_lol_tastes_v_1_ready(&self) -> bool;
}
pub trait PluginLolTft {
    fn get_lol_tft_v_1_tft_news_hub(&self) -> LolTftLolTftNewsHub;
    fn get_lol_tft_v_1_tft_promo_buttons(&self) -> LolTftLolTftPromoButtons;
    fn get_lol_tft_v_1_tft_direct_to_hub(&self) -> bool;
    fn get_lol_tft_v_1_tft_events(&self) -> LolTftLolTftEvents;
    fn put_lol_tft_v_1_tft_experiment_bucket(&self, body: u8) -> Value;
    fn get_lol_tft_v_1_tft_home_hub(&self) -> LolTftLolTftHomeHub;
    fn get_lol_tft_v_1_tft_battle_pass_hub(&self) -> LolTftLolTftBattlePassHub;
    fn post_lol_tft_v_1_tft_home_hub_redirect(&self);
    fn get_lol_tft_v_1_tft_tencent_eventhub_configs(&self) -> LolTftLolTftTencentEventHubConfigs;
}
pub trait PluginLolTftEvent {
    fn get_lol_event_mission_v_1_event_mission(&self) -> Vec<LolTftEventTftEventMissionChain>;
}
pub trait PluginLolTftPass {
    fn get_lol_tft_pass_v_1_battle_pass(&self) -> LolTftPassTftPaidBattlepass;
    fn get_lol_tft_pass_v_1_enabled(&self) -> bool;
    fn post_lol_tft_pass_v_1_passes(&self);
    fn get_lol_tft_pass_v_1_reward_notification(&self) -> LolTftPassTftPassRewardNotification;
    fn get_lol_tft_pass_v_1_daily_login_pass(&self) -> LolTftPassTftPaidBattlepass;
    fn post_lol_tft_pass_v_1_pass_by_id(&self, id: String);
    fn put_lol_tft_pass_v_1_pass_by_id_milestone_by_milestone_id_reward(
        &self,
        id: String,
        milestone_id: String,
    );
    fn get_lol_tft_pass_v_1_event_pass(&self) -> LolTftPassTftPaidBattlepass;
}
pub trait PluginLolTftTeamPlanner {
    fn patch_lol_tft_team_planner_v_1_team_reminders(&self, body: bool) -> Value;
    fn get_lol_tft_team_planner_v_1_team_reminders(&self) -> bool;
    fn delete_lol_tft_team_planner_v_1_team_champions(&self) -> Value;
    fn patch_lol_tft_team_planner_v_1_team_champions(&self, body: Vec<u64>) -> Value;
    fn get_lol_tft_team_planner_v_1_config(&self) -> LolTftTeamPlannerTftTeamPlannerConfig;
    fn put_lol_tft_team_planner_v_1_team(&self) -> Value;
    fn delete_lol_tft_team_planner_v_1_team_champions_by_index(&self, index: u64) -> Value;
    fn post_lol_tft_team_planner_v_1_team_champions_by_index(
        &self,
        index: u64,
        body: String,
    ) -> Value;
    fn get_lol_tft_team_planner_v_1_team_local(&self) -> LolTftTeamPlannerTeamSettings;
    fn get_lol_tft_team_planner_v_1_team_dirty(&self) -> LolTftTeamPlannerTeamPlan;
    fn delete_lol_tft_team_planner_v_1_team_dirty(&self);
    fn delete_lol_tft_team_planner_v_1_team_champions_by_id_by_champion_name(
        &self,
        champion_name: String,
    ) -> Value;
    fn post_lol_tft_team_planner_v_1_team_champions_by_id_by_champion_name(
        &self,
        champion_name: String,
    ) -> Value;
    fn get_lol_tft_team_planner_v_1_ftue_has_viewed(&self) -> bool;
    fn patch_lol_tft_team_planner_v_1_ftue_has_viewed(&self, body: bool) -> Value;
}
pub trait PluginLolTftTroves {
    fn get_lol_tft_troves_v_1_roll_rewards(&self);
    fn delete_lol_tft_troves_v_1_roll_rewards(&self);
    fn get_lol_tft_troves_v_1_banners(&self) -> Vec<LolTftTrovesTrovesBanner>;
    fn post_lol_tft_troves_v_1_roll(&self) -> LolTftTrovesCapOrdersResponseDto;
    fn get_lol_tft_troves_v_1_config(&self) -> LolTftTrovesTroves;
    fn post_lol_tft_troves_v_1_purchase(&self) -> LolTftTrovesCapOrdersResponseDto;
    fn get_lol_tft_troves_v_1_loot_odds_by_drop_table_id(
        &self,
        drop_table_id: String,
    ) -> LolTftTrovesVerboseLootOddsResponse;
    fn get_lol_tft_troves_v_1_status_notifications(&self);
}
pub trait PluginLolTrophies {
    fn get_lol_trophies_v_1_current_summoner_trophies_profile(
        &self,
    ) -> LolTrophiesTrophyProfileData;
    fn get_lol_trophies_v_1_players_by_puuid_trophies_profile(
        &self,
        puuid: String,
    ) -> LolTrophiesTrophyProfileData;
}
pub trait PluginLolVanguard {
    fn get_lol_vanguard_v_1_config_enabled(&self) -> bool;
    fn get_lol_vanguard_v_1_session(&self) -> LolVanguardVanguardSession;
    fn get_lol_vanguard_v_1_config_days_to_reshow_modal(&self) -> f32;
    fn get_lol_vanguard_v_1_machine_specs(&self) -> LolVanguardVanguardMachineSpecs;
    fn get_lol_vanguard_v_1_is_playing_in_pcb(&self) -> bool;
    fn post_lol_vanguard_v_1_telemetry_system_check(&self) -> Value;
}
pub trait PluginLolYourshop {
    fn get_lol_yourshop_v_1_themed(&self) -> bool;
    fn post_lol_yourshop_v_1_offers_by_id_purchase(
        &self,
        id: String,
    ) -> LolYourshopPurchaseResponse;
    fn get_lol_yourshop_v_1_ready(&self) -> bool;
    fn get_lol_yourshop_v_1_status(&self) -> LolYourshopUiStatus;
    fn post_lol_yourshop_v_1_offers_by_id_reveal(&self, id: String) -> Vec<LolYourshopUiOffer>;
    fn get_lol_yourshop_v_1_offers(&self) -> Vec<LolYourshopUiOffer>;
    fn get_lol_yourshop_v_1_has_permissions(&self) -> bool;
    fn post_lol_yourshop_v_1_permissions(&self) -> LolYourshopPlayerPermissions;
    fn get_lol_yourshop_v_1_modal(&self) -> bool;
}

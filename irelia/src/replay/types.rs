pub use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AVContainer {
    Webm,
    Png,
    PngAndDepth,
}

/// Ability Resource
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum AbilityResource {
    Mana,
    Energy,
    None,
    Shield,
    BattleFury,
    DragonFury,
    Rage,
    Heat,
    GnarFury,
    Ferocity,
    BloodWell,
    Wind,
    Ammo,
    MoonLight,
    Other,
    Max
}

#[derive(Serialize, Deserialize)]
pub struct Color {
    a: u8,
    b: u8,
    g: u8,
    r: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ColorValue {
    /// Alpha channel value (0-255)
    a: f32,
    /// Blue channel value (0-255)
    b: f32,
    /// Green channel value (0-255)
    g: f32,
    /// Red channel value (0-255)
    r: f32,
}

/// Blending options for interpolating time between keyframes
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EasingType {
    Linear,
    Snap,
    SmoothStep,
    SmootherStep,
    QuadraticEaseIn,
    QuadraticEaseOut,
    QuadraticEaseInOut,
    CubicEaseIn,
    CubicEaseOut,
    CubicEaseInOut,
    QuarticEaseIn,
    QuarticEaseOut,
    QuarticEaseInOut,
    QuinticEaseIn,
    QuinticEaseOut,
    QuinticEaseInOut,
    SineEaseIn,
    SineEaseOut,
    SineEaseInOut,
    CircularEaseIn,
    CircularEaseOut,
    CircularEaseInOut,
    ExponentialEaseIn,
    ExponentialEaseOut,
    ExponentialEaseInOut,
    ElasticEaseIn,
    ElasticEaseOut,
    ElasticEaseInOut,
    BackEaseIn,
    BackEaseOut,
    BackEaseInOut,
    BounceEaseIn,
    BounceEaseOut,
    BounceEaseInOut,
}

/// Process identifier for this game
#[derive(Serialize, Deserialize)]
pub struct Game {
    process_id: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HudCameraMode {
    Top,
    Fps,
    Tps,
    Focus,
    Path,
}

#[derive(Serialize, Deserialize)]
pub struct KeyFrameAString {
    blend: EasingType,
    time: f32,
    value: String,
}

#[derive(Serialize, Deserialize)]
pub struct KeyFrameBool {
    blend: EasingType,
    time: f32,
    value: bool,
}

#[derive(Serialize, Deserialize)]
pub struct KeyFrameColor {
    blend: EasingType,
    time: f32,
    value: ColorValue
}

#[derive(Serialize, Deserialize)]
pub struct KeyFrameFloat {
    blend: EasingType,
    time: f32,
    value: f32,
}

#[derive(Serialize, Deserialize)]
pub struct KeyFrameVector3 {
    blend: EasingType,
    time: f32,
    value: Vector3f
}

#[derive(Serialize, Deserialize)]
pub struct Playback {
    /// Total length of the replay in seconds
    length: f32,
    /// True if the replay is paused
    paused: bool,
    /// True if the replay is fast forwarding or rewinding
    seeking: bool,
    /// Replay playback speed (0.5 is half speed, 2.0 is double speed etc.)
    speed: f32,
    /// Current time of the replay in seconds since the beginning of the game.
    time: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recording {
    /// Indicates the output format of the recording (for example webm or png)
    codec: AVContainer,
    /// Current time of the recording, indicating progress of the render
    current_time: f32,
    /// Game time in seconds where the recording ends
    end_time: f32,
    /// True if the recording should match the target frames per second exactly by slowing down the recording if required
    enforce_frame_rate: bool,
    /// Target number of frames per second to record in the video
    frames_per_second: i32,
    /// Height of the output video in pixels (same as the game window size)
    height: i32,
    /// True if the recording is being output in a lossless codec (no compression)
    lossless: bool,
    /// File path or directory where the recording should be saved
    path: String,
    #[allow(clippy::struct_field_names)]
    /// True if we are currently recording a replay
    recording: bool,
    /// Playback speed used when recording
    replay_speed: f32,
    /// Game time in seconds where the recording starts
    start_time: f32,
    /// Width of the output video in pixels (same as the game window size)
    width: i32,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Render {
    banners: bool,
    camera_attached: bool,
    camera_look_speed: f32,
    camera_mod: HudCameraMode,
    camera_move_speed: f32,
    camera_position: Vector3f,
    camera_rotation: Vector3f,
    characters: bool,
    depth_fog_color: ColorValue,
    depth_fog_enabled: bool,
    depth_fog_end: f32,
    depth_fog_intensity: f32,
    depth_fog_start: f32,
    depth_of_field_circle: f32,
    depth_of_field_debug: bool,
    depth_of_field_enabled: bool,
    depth_of_field_far: f32,
    depth_of_field_mid: f32,
    depth_of_field_near: f32,
    depth_of_field_width: f32,
    environment: bool,
    far_clip: f32,
    field_of_view: f32,
    floating_text: f32,
    fog_of_war: bool,
    health_bar_champions: bool,
    health_bar_minions: bool,
    health_bar_pets: bool,
    health_bar_structures: bool,
    health_bar_wards: bool,
    height_fog_color: ColorValue,
    height_fog_enabled: bool,
    height_fog_end: f32,
    height_fog_intensity: f32,
    height_fog_start: f32,
    interface_all: bool,
    interface_announce: bool,
    interface_chat: bool,
    interface_frames: bool,
    interface_kill_callouts: bool,
    interface_minimap: bool,
    interface_neutral_timers: bool,
    interface_quests: bool,
    interface_replay: bool,
    interface_score: bool,
    interface_scoreboard: bool,
    interface_target: bool,
    interface_timeline: bool,
    nav_grid_offset: f32,
    near_clip: f32,
    outline_hover: bool,
    outline_select: bool,
    particles: bool,
    selection_name: String,
    selection_offset: Vector3f,
    skybox_offset: f32,
    skybox_path: String,
    skybox_radius: f32,
    skybox_rotation: f32,
    sun_direction: Vector3f,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    camera_position: KeyFrameVector3,
    camera_rotation: KeyFrameVector3,
    depth_fog_color: KeyFrameColor,
    depth_fog_enabled: KeyFrameBool,
    depth_fog_end: KeyFrameFloat,
    depth_fog_intensity: KeyFrameFloat,
    depth_fog_start: KeyFrameFloat,
    depth_of_field_circle: KeyFrameFloat,
    depth_of_field_debug: KeyFrameBool,
    depth_of_field_enabled: KeyFrameBool,
    depth_of_field_far: KeyFrameFloat,
    depth_of_field_mid: KeyFrameFloat,
    depth_of_field_near: KeyFrameFloat,
    depth_of_field_width: KeyFrameFloat,
    far_clip: KeyFrameFloat,
    field_of_view: KeyFrameFloat,
    height_fog_color: KeyFrameColor,
    height_fog_enabled: KeyFrameBool,
    height_fog_end: KeyFrameFloat,
    height_fog_intensity: KeyFrameFloat,
    height_fog_start: KeyFrameFloat,
    nav_grid_offset: KeyFrameFloat,
    near_clip: KeyFrameFloat,
    playback_speed: KeyFrameFloat,
    selection_name: KeyFrameAString,
    selection_offset: KeyFrameVector3,
    skybox_offset: KeyFrameFloat,
    skybox_radius: KeyFrameFloat,
    skybox_rotation: KeyFrameFloat,
    sun_direction: KeyFrameVector3,
}

#[derive(Serialize, Deserialize)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}

use serde::{Deserialize, Serialize};
pub use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AVContainer {
    Webm,
    Png,
    PngAndDepth,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// Ability Resource
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
    Max,
}

#[derive(Serialize, Deserialize)]
pub struct ColorValue {
    /// Red channel value (0-255)
    r: f32,
    /// Green channel value (0-255)
    g: f32,
    /// Blue channel value (0-255)
    b: f32,
    /// Alpha channel value (0-255)
    a: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Blending options for interpolating time between keyframes
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
pub struct KeyFrameT<T: Serialize + Deserialize> {
    blend: EasingType,
    time: f32,
    value: T,
}

pub type KeyFrameAString = KeyFrameT<String>;
pub type KeyFrameBool = KeyFrameT<bool>;
pub type KeyFrameColor = KeyFrameT<ColorValue>;
pub type KeyFrameFloat = KeyFrameT<f32>;
pub type KeyFrameVector3 = KeyFrameT<Vector3f>;

#[derive(Serialize, Deserialize)]
/// Playback state
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
/// Recording State
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
/// Render State
pub struct Render {
    /// Display banners on the map
    banners: bool,
    /// True if the camera is attached to an object in the game
    camera_attached: bool,
    /// Mouse look speed of the camera when in FPS mode (higher is faster)
    camera_look_speed: f32,
    /// Camera movement mode such as first person or third person
    camera_mode: HudCameraMode,
    /// Movement speed of the camera (higher is faster)
    camera_move_speed: f32,
    /// Position of the camera in world coordinates
    camera_position: Vector3f,
    /// Rotation of the camera in Euler degrees (yaw, pitch, roll)
    camera_rotation: Vector3f,
    /// Display champions and minions
    characters: bool,
    /// Depth fog color specified in RGBA
    depth_fog_color: ColorValue,
    /// Display depth based fog
    depth_fog_enabled: bool,
    /// Distance from the camera to the end of the fog
    depth_fog_end: f32,
    /// Depth fog intensity (opacity from 0.0 to 1.0
    depth_fog_intensity: f32,
    /// Distance from the camera to the start of the fog
    depth_fog_start: f32,
    /// Adjusts the shape and strength of the blur effect
    depth_of_field_circle: f32,
    /// Render a debug display to visualize depth of field distances
    depth_of_field_debug: bool,
    /// Display depth of field post-processing
    depth_of_field_enabled: bool,
    /// Furthest distance from the camera in full blur
    depth_of_field_far: f32,
    /// Distance to the center of the depth of field effect, the point that will be the most in focus
    depth_of_field_mid: f32,
    /// Closest distance from the camera in full blur
    depth_of_field_near: f32,
    /// Distance around the middle point that should be in focus
    depth_of_field_width: f32,
    /// Display the level environment
    environment: bool,
    /// Far camera clipping distance
    far_clip: f32,
    /// Camera field of view in degrees (default 45)
    field_of_view: f32,
    /// Display text notifications over the top of champions
    floating_text: f32,
    /// Display fog of war
    fog_of_war: bool,
    /// Display health bars on champions
    health_bar_champions: bool,
    /// Display health bars on minions
    health_bar_minions: bool,
    /// Display health bars on champion pets
    health_bar_pets: bool,
    /// Display health bars on structure and towers
    health_bar_structures: bool,
    /// Display health bars on wards
    health_bar_wards: bool,
    /// Height fog color specified in RGBA
    height_fog_color: ColorValue,
    /// Display height based fog
    height_fog_enabled: bool,
    /// Vertical height at the end of the fog
    height_fog_end: f32,
    /// Height fog intensity (opacity from 0.0 to 1.0)
    height_fog_intensity: f32,
    /// Vertical height at the start of the fog
    height_fog_start: f32,
    /// Display all the user interface
    interface_all: bool,
    /// Display game announcements (center of the window)
    interface_announce: bool,
    /// Display the chat window
    interface_chat: bool,
    /// Display the champion frames (sides of the window)
    interface_frames: bool,
    /// Display kill callouts
    interface_kill_callouts: bool,
    /// Display the game minimap (bottom right corner)
    interface_minimap: bool,
    /// Display neutral objective timers
    interface_neutral_timers: bool,
    /// Display quests
    interface_quests: bool,
    /// Display the replay HUD with camera options
    interface_replay: bool,
    /// Display the replay score interface (top of the window)
    interface_score: bool,
    /// Display the replay scoreboard (bottom of the window)
    interface_scoreboard: bool,
    /// Display the target selection window
    interface_target: bool,
    /// Display the replay timeline (bottom of the window)
    interface_timeline: bool,
    /// Adjusts the height that champions and minions walk over the environment
    nav_grid_offset: f32,
    /// Near camera clipping distance
    near_clip: f32,
    /// Display outlines on champions when the mouse is hovered over
    outline_hover: bool,
    /// Display outlines on champions when selected
    outline_select: bool,
    /// Display particles
    particles: bool,
    /// Sets the selection to the given name, case-insensitive
    selection_name: String,
    /// Sets the camera location to the selection's location adding the given offset
    selection_offset: Vector3f,
    /// Y-Axis offset of the skybox from the camera position
    skybox_offset: f32,
    /// Filepath for a cube mapped skybox in DDS format
    skybox_path: String,
    /// Radius from the camera position to the edge of the skybox
    skybox_radius: f32,
    /// Y-Axis rotation of the skybox in degrees
    skybox_rotation: f32,
    /// Vector indicating the direction of the sun for shadows
    sun_direction: Vector3f,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    /// Keyframe track for Render.cameraPosition
    camera_position: KeyFrameVector3,
    /// Keyframe track for Render.cameraRotation
    camera_rotation: KeyFrameVector3,
    /// Keyframe track for Render.depthFogColor
    depth_fog_color: KeyFrameColor,
    /// Keyframe track for Render.depthFogEnabled
    depth_fog_enabled: KeyFrameBool,
    /// Keyframe track for Render.depthFogEnd
    depth_fog_end: KeyFrameFloat,
    /// Keyframe track for Render.depthFogIntensity
    depth_fog_intensity: KeyFrameFloat,
    /// Keyframe track for Render.depthFogStart
    depth_fog_start: KeyFrameFloat,
    /// Keyframe track for Render.depthOfFieldCircle
    depth_of_field_circle: KeyFrameFloat,
    /// Keyframe track for Render.depthOfFieldEnabled
    depth_of_field_enabled: KeyFrameBool,
    /// Keyframe track for Render.depthOfFieldFar
    depth_of_field_far: KeyFrameFloat,
    /// Keyframe track for Render.depthOfFieldMid
    depth_of_field_mid: KeyFrameFloat,
    /// Keyframe track for Render.depthOfFieldNear
    depth_of_field_near: KeyFrameFloat,
    /// Keyframe track for Render.depthOfFieldWidth
    depth_of_field_width: KeyFrameFloat,
    /// Keyframe track for Render.farClip
    far_clip: KeyFrameFloat,
    /// Keyframe track for Render.fieldOfView
    field_of_view: KeyFrameFloat,
    /// Keyframe track for Render.heightFogColor
    height_fog_color: KeyFrameColor,
    /// Keyframe track for Render.heightFogEnabled
    height_fog_enabled: KeyFrameBool,
    /// Keyframe track for Render.heightFogEnd
    height_fog_end: KeyFrameFloat,
    /// Keyframe track for Render.heightFogIntensity
    height_fog_intensity: KeyFrameFloat,
    /// Keyframe track for Render.heightFogStart
    height_fog_start: KeyFrameFloat,
    /// Keyframe track for Render.navGridOffset
    nav_grid_offset: KeyFrameFloat,
    /// Keyframe track for Render.nearClip
    near_clip: KeyFrameFloat,
    /// Keyframe track for Playback.speed
    playback_speed: KeyFrameFloat,
    /// Keyframe track for Render.selectionName
    selection_name: KeyFrameAString,
    /// Keyframe track for Render.selectionOffset
    selection_offset: KeyFrameVector3,
    /// Keyframe track for Render.skyboxOffset
    skybox_offset: KeyFrameFloat,
    /// Keyframe track for Render.skyboxRadius
    skybox_radius: KeyFrameFloat,
    /// Keyframe track for Render.skyboxRotation
    skybox_rotation: KeyFrameFloat,
    /// Keyframe track for Render.sunDirection
    sun_direction: KeyFrameVector3,
}

#[derive(Serialize, Deserialize)]
pub struct Vector3f {
    x: f32,
    y: f32,
    z: f32,
}

use serde::Serialize;
pub use serde_derive::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum AVContainer {
    Webm,
    Png,
    PngAndDepth,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorValue {
    /// Red channel value (0-255)
    pub r: f32,
    /// Green channel value (0-255)
    pub g: f32,
    /// Blue channel value (0-255)
    pub b: f32,
    /// Alpha channel value (0-255)
    pub a: f32,
}

#[derive(Serialize, Deserialize, Debug)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    #[serde(rename = "processID")]
    pub process_id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum HudCameraMode {
    Top,
    Fps,
    Tps,
    Focus,
    Path,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyFrameT<T: Serialize + Debug> {
    pub blend: EasingType,
    pub time: f32,
    #[serde(bound(deserialize = "T: serde::Deserialize<'de>"))]
    pub value: T,
}

pub type KeyFrameAString = KeyFrameT<String>;
pub type KeyFrameBool = KeyFrameT<bool>;
pub type KeyFrameColor = KeyFrameT<ColorValue>;
pub type KeyFrameFloat = KeyFrameT<f32>;
pub type KeyFrameVector3 = KeyFrameT<Vector3f>;

#[derive(Serialize, Deserialize, Debug)]
/// Playback state
pub struct Playback {
    /// Total length of the replay in seconds
    pub length: f32,
    /// True if the replay is paused
    pub paused: bool,
    /// True if the replay is fast forwarding or rewinding
    pub seeking: bool,
    /// Replay playback speed (0.5 is half speed, 2.0 is double speed etc.)
    pub speed: f32,
    /// Current time of the replay in seconds since the beginning of the game.
    pub time: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Recording State
pub struct Recording {
    /// Indicates the output format of the recording (for example webm or png)
    pub codec: AVContainer,
    /// Current time of the recording, indicating progress of the render
    pub current_time: f32,
    /// Game time in seconds where the recording ends
    pub end_time: f32,
    /// True if the recording should match the target frames per second exactly by slowing down the recording if required
    pub enforce_frame_rate: bool,
    /// Target number of frames per second to record in the video
    pub frames_per_second: i32,
    /// Height of the output video in pixels (same as the game window size)
    pub height: i32,
    /// True if the recording is being output in a lossless codec (no compression)
    pub lossless: bool,
    /// File path or directory where the recording should be saved
    pub path: String,
    #[allow(clippy::struct_field_names)]
    /// True if we are currently recording a replay
    pub recording: bool,
    /// Playback speed used when recording
    pub replay_speed: f32,
    /// Game time in seconds where the recording starts
    pub start_time: f32,
    /// Width of the output video in pixels (same as the game window size)
    pub width: i32,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Render State
pub struct Render {
    /// Display banners on the map
    pub banners: Option<bool>,
    /// True if the camera is attached to an object in the game
    pub camera_attached: Option<bool>,
    /// Mouse look speed of the camera when in FPS mode (higher is faster)
    pub camera_look_speed: f32,
    /// Camera movement mode such as first person or third person
    pub camera_mode: HudCameraMode,
    /// Movement speed of the camera (higher is faster)
    pub camera_move_speed: f32,
    /// Position of the camera in world coordinates
    pub camera_position: Vector3f,
    /// Rotation of the camera in Euler degrees (yaw, pitch, roll)
    pub camera_rotation: Vector3f,
    /// Display champions and minions
    pub characters: Option<bool>,
    /// Depth fog color specified in RGBA
    pub depth_fog_color: ColorValue,
    /// Display depth based fog
    pub depth_fog_enabled: Option<bool>,
    /// Distance from the camera to the end of the fog
    pub depth_fog_end: f32,
    /// Depth fog intensity (opacity from 0.0 to 1.0
    pub depth_fog_intensity: f32,
    /// Distance from the camera to the start of the fog
    pub depth_fog_start: f32,
    /// Adjusts the shape and strength of the blur effect
    pub depth_of_field_circle: f32,
    /// Render a debug display to visualize depth of field distances
    pub depth_of_field_debug: Option<bool>,
    /// Display depth of field post-processing
    pub depth_of_field_enabled: Option<bool>,
    /// Furthest distance from the camera in full blur
    pub depth_of_field_far: f32,
    /// Distance to the center of the depth of field effect, the point that will be the most in focus
    pub depth_of_field_mid: f32,
    /// Closest distance from the camera in full blur
    pub depth_of_field_near: f32,
    /// Distance around the middle point that should be in focus
    pub depth_of_field_width: f32,
    /// Display the level environment
    pub environment: Option<bool>,
    /// Far camera clipping distance
    pub far_clip: f32,
    /// Camera field of view in degrees (default 45)
    pub field_of_view: f32,
    /// Display text notifications over the top of champions
    pub floating_text: Option<bool>,
    /// Display fog of war
    pub fog_of_war: Option<bool>,
    /// Display health bars on champions
    pub health_bar_champions: Option<bool>,
    /// Display health bars on minions
    pub health_bar_minions: Option<bool>,
    /// Display health bars on champion pets
    pub health_bar_pets: Option<bool>,
    /// Display health bars on structure and towers
    pub health_bar_structures: Option<bool>,
    /// Display health bars on wards
    pub health_bar_wards: Option<bool>,
    /// Height fog color specified in RGBA
    pub height_fog_color: ColorValue,
    /// Display height based fog
    pub height_fog_enabled: Option<bool>,
    /// Vertical height at the end of the fog
    pub height_fog_end: f32,
    /// Height fog intensity (opacity from 0.0 to 1.0)
    pub height_fog_intensity: f32,
    /// Vertical height at the start of the fog
    pub height_fog_start: f32,
    /// Display all the user interface
    pub interface_all: Option<bool>,
    /// Display game announcements (center of the window)
    pub interface_announce: Option<bool>,
    /// Display the chat window
    pub interface_chat: Option<bool>,
    /// Display the champion frames (sides of the window)
    pub interface_frames: Option<bool>,
    /// Display kill callouts
    pub interface_kill_callouts: Option<bool>,
    /// Display the game minimap (bottom right corner)
    pub interface_minimap: Option<bool>,
    /// Display neutral objective timers
    pub interface_neutral_timers: Option<bool>,
    /// Display quests
    pub interface_quests: Option<bool>,
    /// Display the replay HUD with camera options
    pub interface_replay: Option<bool>,
    /// Display the replay score interface (top of the window)
    pub interface_score: Option<bool>,
    /// Display the replay scoreboard (bottom of the window)
    pub interface_scoreboard: Option<bool>,
    /// Display the target selection window
    pub interface_target: Option<bool>,
    /// Display the replay timeline (bottom of the window)
    pub interface_timeline: Option<bool>,
    /// Adjusts the height that champions and minions walk over the environment
    pub nav_grid_offset: f32,
    /// Near camera clipping distance
    pub near_clip: f32,
    /// Display outlines on champions when the mouse is hovered over
    pub outline_hover: Option<bool>,
    /// Display outlines on champions when selected
    pub outline_select: Option<bool>,
    /// Display particles
    pub particles: Option<bool>,
    /// Sets the selection to the given name, case-insensitive
    pub selection_name: String,
    /// Sets the camera location to the selection's location adding the given offset
    pub selection_offset: Vector3f,
    /// Y-Axis offset of the skybox from the camera position
    pub skybox_offset: f32,
    /// Filepath for a cube mapped skybox in DDS format
    pub skybox_path: String,
    /// Radius from the camera position to the edge of the skybox
    pub skybox_radius: f32,
    /// Y-Axis rotation of the skybox in degrees
    pub skybox_rotation: f32,
    /// Vector indicating the direction of the sun for shadows
    pub sun_direction: Vector3f,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Sequence {
    /// Keyframe track for Render.cameraPosition
    pub camera_position: Option<Vec<KeyFrameVector3>>,
    /// Keyframe track for Render.cameraRotation
    pub camera_rotation: Option<Vec<KeyFrameVector3>>,
    /// Keyframe track for Render.depthFogColor
    pub depth_fog_color: Option<Vec<KeyFrameColor>>,
    /// Keyframe track for Render.depthFogEnabled
    pub depth_fog_enabled: Option<Vec<KeyFrameBool>>,
    /// Keyframe track for Render.depthFogEnd
    pub depth_fog_end: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.depthFogIntensity
    pub depth_fog_intensity: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.depthFogStart
    pub depth_fog_start: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.depthOfFieldCircle
    pub depth_of_field_circle: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.depthOfFieldEnabled
    pub depth_of_field_enabled: Option<Vec<KeyFrameBool>>,
    /// Keyframe track for Render.depthOfFieldFar
    pub depth_of_field_far: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.depthOfFieldMid
    pub depth_of_field_mid: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.depthOfFieldNear
    pub depth_of_field_near: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.depthOfFieldWidth
    pub depth_of_field_width: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.farClip
    pub far_clip: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.fieldOfView
    pub field_of_view: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.heightFogColor
    pub height_fog_color: Option<Vec<KeyFrameColor>>,
    /// Keyframe track for Render.heightFogEnabled
    pub height_fog_enabled: Option<Vec<KeyFrameBool>>,
    /// Keyframe track for Render.heightFogEnd
    pub height_fog_end: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.heightFogIntensity
    pub height_fog_intensity: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.heightFogStart
    pub height_fog_start: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.navGridOffset
    pub nav_grid_offset: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.nearClip
    pub near_clip: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Playback.speed
    pub playback_speed: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.selectionName
    pub selection_name: Option<Vec<KeyFrameAString>>,
    /// Keyframe track for Render.selectionOffset
    pub selection_offset: Option<Vec<KeyFrameVector3>>,
    /// Keyframe track for Render.skyboxOffset
    pub skybox_offset: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.skyboxRadius
    pub skybox_radius: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.skyboxRotation
    pub skybox_rotation: Option<Vec<KeyFrameFloat>>,
    /// Keyframe track for Render.sunDirection
    pub sun_direction: Option<Vec<KeyFrameVector3>>,
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            camera_position: Some(Vec::default()),
            camera_rotation: Some(Vec::default()),
            depth_fog_color: Some(Vec::default()),
            depth_fog_enabled: Some(Vec::default()),
            depth_fog_end: Some(Vec::default()),
            depth_fog_intensity: Some(Vec::default()),
            depth_fog_start: Some(Vec::default()),
            depth_of_field_enabled: Some(Vec::default()),
            depth_of_field_width: Some(Vec::default()),
            depth_of_field_circle: Some(Vec::default()),
            depth_of_field_far: Some(Vec::default()),
            depth_of_field_mid: Some(Vec::default()),
            depth_of_field_near: Some(Vec::default()),
            far_clip: Some(Vec::default()),
            field_of_view: Some(Vec::default()),
            height_fog_color: Some(Vec::default()),
            height_fog_enabled: Some(Vec::default()),
            height_fog_end: Some(Vec::default()),
            height_fog_intensity: Some(Vec::default()),
            height_fog_start: Some(Vec::default()),
            nav_grid_offset: Some(Vec::default()),
            near_clip: Some(Vec::default()),
            playback_speed: Some(Vec::default()),
            selection_name: Some(Vec::default()),
            selection_offset: Some(Vec::default()),
            skybox_offset: Some(Vec::default()),
            skybox_radius: Some(Vec::default()),
            skybox_rotation: Some(Vec::default()),
            sun_direction: Some(Vec::default()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Vector3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

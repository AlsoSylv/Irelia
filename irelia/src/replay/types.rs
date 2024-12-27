use crate::in_game::types::duration;
use crate::replay::types::sealed::KeyFrameValue;
use serde_derive::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::default::Default;
use std::fmt::Debug;
use sysinfo::Pid;
use time::Duration;

mod sealed {
    use crate::replay::types::{ColorValue, Vector3f};

    /// This is a specific bound, only applied to valid schema types
    pub trait KeyFrameValue {}
    impl KeyFrameValue for String {}
    impl KeyFrameValue for f64 {}
    impl KeyFrameValue for ColorValue {}
    impl KeyFrameValue for Vector3f {}
    impl KeyFrameValue for bool {}
}

/// Video container
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AVContainer {
    #[default]
    Webm,
    Png,
    PngAndDepth,
}

/// `ColorValue` from the API, all channels use floats
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ColorValue {
    /// Red channel value (0-255)
    pub r: f64,
    /// Green channel value (0-255)
    pub g: f64,
    /// Blue channel value (0-255)
    pub b: f64,
    /// Alpha channel value (0-255)
    pub a: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Blending options for interpolating time between keyframes
pub enum EasingType {
    #[default]
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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    #[serde(rename = "processID")]
    #[serde(with = "pid")]
    process_id: Pid,
}

impl Game {
    #[must_use]
    pub fn process_id(&self) -> Pid {
        self.process_id
    }
}

/// Note: There is no sane default for the camera position, as sometimes
/// Top just does not work, and sometimes it will just force FPS camera mode
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HudCameraMode {
    /// This is the top-down camera
    Top,
    /// This camera can be controlled like an FPS camera in the client
    /// Using the arrow keys
    Fps,
    /// This always crashes
    Tps,
    /// This always crashes
    Focus,
    /// This always crashes
    Path,
}

/// Two keyframes are considered equal, assuming they both have the same value and that both use `Linear` blending,
/// or they use the same `EasingType`, and take place at the same time
///
/// As for ordering, keyframes are ordered based on time
///
/// Supported `KeyFrameValue`'s are:
/// - `String`
/// - `ColorValue`
/// - `Vector3f`
/// - `f64`
/// - `bool`
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct KeyFrameT<T: KeyFrameValue> {
    pub blend: EasingType,
    #[serde(with = "duration")]
    pub time: Duration,
    #[serde(bound(deserialize = "T: serde::Deserialize<'de>"))]
    pub value: T,
}

impl<T: KeyFrameValue> KeyFrameT<T> {
    #[must_use]
    pub fn new(value: T, time: Duration, blend: EasingType) -> Self {
        Self { blend, time, value }
    }

    #[must_use]
    pub fn new_default_blending(value: T, time: Duration) -> Self {
        Self {
            value,
            time,
            blend: EasingType::Linear,
        }
    }
}

impl<T: KeyFrameValue + PartialEq> PartialEq<KeyFrameT<T>> for KeyFrameT<T> {
    fn eq(&self, other: &KeyFrameT<T>) -> bool {
        self.value == other.value
            && ((self.blend == EasingType::Linear && other.blend == EasingType::Linear)
                || (self.blend == other.blend && self.time == other.time))
    }
}

impl<T: KeyFrameValue + PartialEq> PartialOrd<Self> for KeyFrameT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

#[test]
fn test_deserialize() {
    use serde::Deserialize;

    let json = serde_json::json!({
        "blend": "linear",
        "value": 10.0,
        "time": 1.1001,
    });

    let keyframe = KeyFrameF64::deserialize(json).unwrap();

    println!("{keyframe:?}");

    let json = serde_json::to_value(keyframe).unwrap();

    println!("{json}");
}

type KeyFrameString = KeyFrameT<String>;
type KeyFrameBool = KeyFrameT<bool>;
type KeyFrameColor = KeyFrameT<ColorValue>;
type KeyFrameF64 = KeyFrameT<f64>;
type KeyFrameVector3 = KeyFrameT<Vector3f>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// Playback state
pub struct Playback {
    #[serde(with = "duration")]
    /// Total length of the replay in seconds
    pub length: Duration,
    /// True if the replay is paused
    pub paused: bool,
    /// True if the replay is fast forwarding or rewinding
    pub seeking: bool,
    /// Replay playback speed (0.5 is half speed, 2.0 is double speed etc.)
    pub speed: f64,
    #[serde(with = "duration")]
    /// Current time of the replay in seconds since the beginning of the game.
    pub time: Duration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Recording State
pub struct RecordingState {
    /// Indicates the output format of the recording (for example webm or png)
    pub codec: AVContainer,
    #[serde(with = "duration")]
    /// Current time of the recording, indicating progress of the render
    pub current_time: Duration,
    #[serde(with = "duration")]
    /// Game time in seconds where the recording ends
    pub end_time: Duration,
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
    /// True if we are currently recording a replay
    pub recording: bool,
    /// Playback speed used when recording
    pub replay_speed: f64,
    #[serde(with = "duration")]
    /// Game time in seconds where the recording starts
    pub start_time: Duration,
    /// Width of the output video in pixels (same as the game window size)
    pub width: i32,
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Render State
pub struct Render {
    /// Display text notifications over the top of champions
    pub floating_text: bool,
    /// Display fog of war
    pub fog_of_war: bool,
    /// Display health bars on champions
    pub health_bar_champions: bool,
    /// Display health bars on minions
    pub health_bar_minions: bool,
    /// Display health bars on champion pets
    pub health_bar_pets: bool,
    /// Display health bars on structure and towers
    pub health_bar_structures: bool,
    /// Display health bars on wards
    pub health_bar_wards: bool,
    /// Display all the user interface
    pub interface_all: bool,
    /// Display height based fog
    pub height_fog_enabled: bool,
    /// Display game announcements (center of the window)
    pub interface_announce: bool,
    /// Display the chat window
    pub interface_chat: bool,
    /// Display the champion frames (sides of the window)
    pub interface_frames: bool,
    /// Display kill callouts
    pub interface_kill_callouts: bool,
    /// Display the game minimap (bottom right corner)
    pub interface_minimap: bool,
    /// Display the replay HUD with camera options
    pub interface_replay: bool,
    /// Display the replay score interface (top of the window)
    pub interface_score: bool,
    /// Display the replay scoreboard (bottom of the window)
    pub interface_scoreboard: bool,
    /// Display the target selection window
    pub interface_target: bool,
    /// Display the replay timeline (bottom of the window)
    pub interface_timeline: bool,
    /// Display outlines on champions when the mouse is hovered over
    pub outline_hover: bool,
    /// Display outlines on champions when selected
    pub outline_select: bool,
    /// Display particles
    pub particles: bool,
    /// True if the camera is attached to an object in the game
    pub camera_attached: bool,
    /// Display depth based fog
    pub depth_fog_enabled: bool,
    /// Display champions and minions
    pub characters: bool,
    /// Render a debug display to visualize depth of field distances
    pub depth_of_field_debug: bool,
    /// Display depth of field post-processing
    pub depth_of_field_enabled: bool,
    /// Display the level environment
    pub environment: bool,
    /// Display banners on the map
    pub banners: Option<bool>,
    /// Display neutral objective timers
    pub interface_neutral_timers: Option<bool>,
    /// Display quests
    pub interface_quests: Option<bool>,
    /// Movement speed of the camera (higher is faster)
    pub camera_move_speed: f64,
    /// Mouse look speed of the camera when in FPS mode (higher is faster)
    pub camera_look_speed: f64,
    /// Distance from the camera to the end of the fog
    pub depth_fog_end: f64,
    /// Depth fog intensity (opacity from 0.0 to 1.0)
    pub depth_fog_intensity: f64,
    /// Distance from the camera to the start of the fog
    pub depth_fog_start: f64,
    /// Adjusts the shape and strength of the blur effect
    pub depth_of_field_circle: f64,
    /// Furthest distance from the camera in full blur
    pub depth_of_field_far: f64,
    /// Distance to the center of the depth of field effect, the point that will be the most in focus
    pub depth_of_field_mid: f64,
    /// Closest distance from the camera in full blur
    pub depth_of_field_near: f64,
    /// Distance around the middle point that should be in focus
    pub depth_of_field_width: f64,
    /// Far camera clipping distance
    pub far_clip: f64,
    /// Camera field of view in degrees (default 45)
    pub field_of_view: f64,
    /// Vertical height at the end of the fog
    pub height_fog_end: f64,
    /// Height fog intensity (opacity from 0.0 to 1.0)
    pub height_fog_intensity: f64,
    /// Vertical height at the start of the fog
    pub height_fog_start: f64,
    /// Adjusts the height that champions and minions walk over the environment
    pub nav_grid_offset: f64,
    /// Near camera clipping distance
    pub near_clip: f64,
    /// Y-Axis offset of the skybox from the camera position
    pub skybox_offset: f64,
    /// Radius from the camera position to the edge of the skybox
    pub skybox_radius: f64,
    /// Y-Axis rotation of the skybox in degrees
    pub skybox_rotation: f64,
    /// Vector indicating the direction of the sun for shadows
    pub sun_direction: Vector3f,
    /// Position of the camera in world coordinates
    pub camera_position: Vector3f,
    /// Rotation of the camera in Euler degrees (yaw, pitch, roll)
    pub camera_rotation: Vector3f,
    /// Sets the camera location to the selection's location adding the given offset
    pub selection_offset: Vector3f,
    /// Height fog color specified in RGBA
    pub height_fog_color: ColorValue,
    /// Depth fog color specified in RGBA
    pub depth_fog_color: ColorValue,
    /// Filepath for a cube mapped skybox in DDS format
    pub skybox_path: String,
    /// Sets the selection to the given name, case-insensitive
    pub selection_name: String,
    /// Camera movement mode such as first person or third person
    pub camera_mode: HudCameraMode,
}

/// Sequence of `KeyFrames` to be executed, controls settings, FOV, offsets, etc
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Sequence {
    /// Keyframe track for Render.cameraPosition
    pub camera_position: Vec<KeyFrameVector3>,
    /// Keyframe track for Render.cameraRotation
    pub camera_rotation: Vec<KeyFrameVector3>,
    /// Keyframe track for Render.depthFogColor
    pub depth_fog_color: Vec<KeyFrameColor>,
    /// Keyframe track for Render.depthFogEnabled
    pub depth_fog_enabled: Vec<KeyFrameBool>,
    /// Keyframe track for Render.depthFogEnd
    pub depth_fog_end: Vec<KeyFrameF64>,
    /// Keyframe track for Render.depthFogIntensity
    pub depth_fog_intensity: Vec<KeyFrameF64>,
    /// Keyframe track for Render.depthFogStart
    pub depth_fog_start: Vec<KeyFrameF64>,
    /// Keyframe track for Render.depthOfFieldCircle
    pub depth_of_field_circle: Vec<KeyFrameF64>,
    /// Keyframe track for Render.depthOfFieldEnabled
    pub depth_of_field_enabled: Vec<KeyFrameBool>,
    /// Keyframe track for Render.depthOfFieldFar
    pub depth_of_field_far: Vec<KeyFrameF64>,
    /// Keyframe track for Render.depthOfFieldMid
    pub depth_of_field_mid: Vec<KeyFrameF64>,
    /// Keyframe track for Render.depthOfFieldNear
    pub depth_of_field_near: Vec<KeyFrameF64>,
    /// Keyframe track for Render.depthOfFieldWidth
    pub depth_of_field_width: Vec<KeyFrameF64>,
    /// Keyframe track for Render.farClip
    pub far_clip: Vec<KeyFrameF64>,
    /// Keyframe track for Render.fieldOfView
    pub field_of_view: Vec<KeyFrameF64>,
    /// Keyframe track for Render.heightFogColor
    pub height_fog_color: Vec<KeyFrameColor>,
    /// Keyframe track for Render.heightFogEnabled
    pub height_fog_enabled: Vec<KeyFrameBool>,
    /// Keyframe track for Render.heightFogEnd
    pub height_fog_end: Vec<KeyFrameF64>,
    /// Keyframe track for Render.heightFogIntensity
    pub height_fog_intensity: Vec<KeyFrameF64>,
    /// Keyframe track for Render.heightFogStart
    pub height_fog_start: Vec<KeyFrameF64>,
    /// Keyframe track for Render.navGridOffset
    pub nav_grid_offset: Vec<KeyFrameF64>,
    /// Keyframe track for Render.nearClip
    pub near_clip: Vec<KeyFrameF64>,
    /// Keyframe track for Playback.speed
    pub playback_speed: Vec<KeyFrameF64>,
    /// Keyframe track for Render.selectionName
    pub selection_name: Vec<KeyFrameString>,
    /// Keyframe track for Render.selectionOffset
    pub selection_offset: Vec<KeyFrameVector3>,
    /// Keyframe track for Render.skyboxOffset
    pub skybox_offset: Vec<KeyFrameF64>,
    /// Keyframe track for Render.skyboxRadius
    pub skybox_radius: Vec<KeyFrameF64>,
    /// Keyframe track for Render.skyboxRotation
    pub skybox_rotation: Vec<KeyFrameF64>,
    /// Keyframe track for Render.sunDirection
    pub sun_direction: Vec<KeyFrameVector3>,
}

impl Sequence {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[must_use]
    pub fn from_render(render: &Render, playback_speed: f64, time: Duration) -> Self {
        let mut sequence = Self::default();

        sequence.push_render(render, playback_speed, time);

        sequence
    }

    #[rustfmt::skip]
    pub fn push_render(&mut self, render: &Render, playback_speed: f64, time: Duration) {
        self.camera_position.push(KeyFrameT::new_default_blending(render.camera_position, time));
        self.camera_rotation.push(KeyFrameT::new_default_blending(render.camera_rotation, time));
        self.depth_fog_color.push(KeyFrameT::new_default_blending(render.depth_fog_color, time));
        self.depth_fog_enabled.push(KeyFrameT::new_default_blending(render.depth_fog_enabled, time));
        self.depth_fog_end.push(KeyFrameT::new_default_blending(render.depth_fog_end, time));
        self.depth_fog_intensity.push(KeyFrameT::new_default_blending(render.depth_fog_intensity, time));
        self.depth_fog_start.push(KeyFrameT::new_default_blending(render.depth_fog_start, time));
        self.depth_of_field_circle.push(KeyFrameT::new_default_blending(render.depth_of_field_circle, time));
        self.depth_of_field_enabled.push(KeyFrameT::new_default_blending(render.depth_of_field_enabled, time));
        self.depth_of_field_far.push(KeyFrameT::new_default_blending(render.depth_of_field_far, time));
        self.depth_of_field_mid.push(KeyFrameT::new_default_blending(render.depth_of_field_mid, time));
        self.depth_of_field_near.push(KeyFrameT::new_default_blending(render.depth_of_field_near, time));
        self.depth_of_field_width.push(KeyFrameT::new_default_blending(render.depth_of_field_width, time));
        self.far_clip.push(KeyFrameT::new_default_blending(render.far_clip, time));
        self.field_of_view.push(KeyFrameT::new_default_blending(render.field_of_view, time));
        self.height_fog_color.push(KeyFrameT::new_default_blending(render.height_fog_color, time));
        self.height_fog_enabled.push(KeyFrameT::new_default_blending(render.height_fog_enabled, time));
        self.height_fog_end.push(KeyFrameT::new_default_blending(render.height_fog_end, time));
        self.height_fog_intensity.push(KeyFrameT::new_default_blending(render.height_fog_intensity, time));
        self.height_fog_start.push(KeyFrameT::new_default_blending(render.height_fog_start, time));
        self.nav_grid_offset.push(KeyFrameT::new_default_blending(render.nav_grid_offset, time));
        self.near_clip.push(KeyFrameT::new_default_blending(render.near_clip, time));
        self.playback_speed.push(KeyFrameT::new_default_blending(playback_speed, time));

        if let Some(selection) = self.selection_name.last() {
            if selection.value != render.selection_name {
                self.selection_name.push(KeyFrameT::new_default_blending(render.selection_name.clone(), time));
            }
        }

        self.selection_offset.push(KeyFrameT::new_default_blending(render.selection_offset, time));
        self.skybox_offset.push(KeyFrameT::new_default_blending(render.skybox_offset, time));
        self.skybox_radius.push(KeyFrameT::new_default_blending(render.skybox_radius, time));
        self.skybox_rotation.push(KeyFrameT::new_default_blending(render.skybox_rotation, time));
        self.sun_direction.push(KeyFrameT::new_default_blending(render.sun_direction, time));
    }
}

/// Basic Vec3 for the API, all fields are floats
#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

mod pid {
    use serde::de::{Error, Visitor};
    use serde::{Deserializer, Serializer};
    use std::fmt::Formatter;
    use sysinfo::Pid;

    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub(crate) fn serialize<S: Serializer>(pid: &Pid, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u32(pid.as_u32())
    }

    pub(crate) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Pid, D::Error> {
        struct PidVisitor;

        impl Visitor<'_> for PidVisitor {
            type Value = Pid;

            fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                formatter.write_str("A number corresponding to the PID of league")
            }

            fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Pid::from_u32(v))
            }

            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
            where
                E: Error,
            {
                Ok(Pid::from(usize::try_from(v).map_err(E::custom)?))
            }
        }

        deserializer.deserialize_u32(PidVisitor)
    }
}

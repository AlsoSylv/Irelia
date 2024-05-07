use crate::in_game::types::{AllPlayer, Events, GameData};
use crate::replay::types::hidden::KeyFrameValue;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use time::Duration;

mod hidden {
    use crate::replay::types::{ColorValue, Vector3f};

    /// This is a specific bound, only applied to valid schema types
    pub trait KeyFrameValue {}
    impl KeyFrameValue for String {}
    impl KeyFrameValue for f64 {}
    impl KeyFrameValue for ColorValue {}
    impl KeyFrameValue for Vector3f {}
    impl KeyFrameValue for bool {}
}

#[path = "impls.rs"]
mod impls;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayGameData {
    all_players: Box<[AllPlayer]>,
    events: Events,
    game_data: GameData,
}

impl ReplayGameData {
    #[must_use]
    pub fn all_players(&self) -> &[AllPlayer] {
        &self.all_players
    }
    #[must_use]
    pub fn events(&self) -> &Events {
        &self.events
    }
    #[must_use]
    pub fn game_data(&self) -> &GameData {
        &self.game_data
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AVContainer {
    #[default]
    Webm,
    Png,
    PngAndDepth,
}

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
    pub process_id: u32,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum HudCameraMode {
    #[default]
    Top,
    Fps,
    Tps,
    Focus,
    Path,
}

/// Two keyframes are considered equal, assuming they both have the same value
/// and that both use `Linear` blending
///
/// As for ordering, keyframes are ordered based on time
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
struct KeyFrameT<T: KeyFrameValue> {
    pub blend: EasingType,
    #[serde(
        serialize_with = "duration_to_f64",
        deserialize_with = "duration_from_f64"
    )]
    pub time: Duration,
    #[serde(bound(deserialize = "T: serde::Deserialize<'de>"))]
    pub value: T,
}

fn duration_to_f64<S: Serializer>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
    let seconds = duration.as_seconds_f64();
    serializer.serialize_f64(seconds)
}

fn duration_from_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Duration, D::Error> {
    struct F64Visitor;

    impl<'a> Visitor<'a> for F64Visitor {
        type Value = Duration;

        fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
            formatter.write_str("An f64, representing time in seconds")
        }

        fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
        where
            E: Error,
        {
            Ok(Duration::seconds_f64(v))
        }
    }

    deserializer.deserialize_any(F64Visitor)
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
    #[serde(
        serialize_with = "duration_to_f64",
        deserialize_with = "duration_from_f64"
    )]
    /// Total length of the replay in seconds
    pub length: Duration,
    /// True if the replay is paused
    pub paused: bool,
    /// True if the replay is fast forwarding or rewinding
    pub seeking: bool,
    /// Replay playback speed (0.5 is half speed, 2.0 is double speed etc.)
    pub speed: f64,
    #[serde(
        serialize_with = "duration_to_f64",
        deserialize_with = "duration_from_f64"
    )]
    /// Current time of the replay in seconds since the beginning of the game.
    pub time: Duration,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
/// Recording State
pub struct Recording {
    /// Indicates the output format of the recording (for example webm or png)
    pub codec: AVContainer,
    #[serde(
        serialize_with = "duration_to_f64",
        deserialize_with = "duration_from_f64"
    )]
    /// Current time of the recording, indicating progress of the render
    pub current_time: Duration,
    #[serde(
        serialize_with = "duration_to_f64",
        deserialize_with = "duration_from_f64"
    )]
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
    #[allow(clippy::struct_field_names)]
    /// True if we are currently recording a replay
    pub recording: bool,
    /// Playback speed used when recording
    pub replay_speed: f64,
    #[serde(
        serialize_with = "duration_to_f64",
        deserialize_with = "duration_from_f64"
    )]
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
    /// Display banners on the map
    pub banners: Option<bool>,
    /// True if the camera is attached to an object in the game
    pub camera_attached: bool,
    /// Mouse look speed of the camera when in FPS mode (higher is faster)
    pub camera_look_speed: f64,
    /// Camera movement mode such as first person or third person
    pub camera_mode: HudCameraMode,
    /// Movement speed of the camera (higher is faster)
    pub camera_move_speed: f64,
    /// Position of the camera in world coordinates
    pub camera_position: Vector3f,
    /// Rotation of the camera in Euler degrees (yaw, pitch, roll)
    pub camera_rotation: Vector3f,
    /// Display champions and minions
    pub characters: bool,
    /// Depth fog color specified in RGBA
    pub depth_fog_color: ColorValue,
    /// Display depth based fog
    pub depth_fog_enabled: bool,
    /// Distance from the camera to the end of the fog
    pub depth_fog_end: f64,
    /// Depth fog intensity (opacity from 0.0 to 1.0)
    pub depth_fog_intensity: f64,
    /// Distance from the camera to the start of the fog
    pub depth_fog_start: f64,
    /// Adjusts the shape and strength of the blur effect
    pub depth_of_field_circle: f64,
    /// Render a debug display to visualize depth of field distances
    pub depth_of_field_debug: bool,
    /// Display depth of field post-processing
    pub depth_of_field_enabled: bool,
    /// Furthest distance from the camera in full blur
    pub depth_of_field_far: f64,
    /// Distance to the center of the depth of field effect, the point that will be the most in focus
    pub depth_of_field_mid: f64,
    /// Closest distance from the camera in full blur
    pub depth_of_field_near: f64,
    /// Distance around the middle point that should be in focus
    pub depth_of_field_width: f64,
    /// Display the level environment
    pub environment: bool,
    /// Far camera clipping distance
    pub far_clip: f64,
    /// Camera field of view in degrees (default 45)
    pub field_of_view: f64,
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
    /// Height fog color specified in RGBA
    pub height_fog_color: ColorValue,
    /// Display height based fog
    pub height_fog_enabled: bool,
    /// Vertical height at the end of the fog
    pub height_fog_end: f64,
    /// Height fog intensity (opacity from 0.0 to 1.0)
    pub height_fog_intensity: f64,
    /// Vertical height at the start of the fog
    pub height_fog_start: f64,
    /// Display all the user interface
    pub interface_all: bool,
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
    /// Display neutral objective timers
    pub interface_neutral_timers: Option<bool>,
    /// Display quests
    pub interface_quests: Option<bool>,
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
    /// Adjusts the height that champions and minions walk over the environment
    pub nav_grid_offset: f64,
    /// Near camera clipping distance
    pub near_clip: f64,
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
    pub skybox_offset: f64,
    /// Filepath for a cube mapped skybox in DDS format
    pub skybox_path: String,
    /// Radius from the camera position to the edge of the skybox
    pub skybox_radius: f64,
    /// Y-Axis rotation of the skybox in degrees
    pub skybox_rotation: f64,
    /// Vector indicating the direction of the sun for shadows
    pub sun_direction: Vector3f,
}

#[derive(Debug, Clone)]
pub struct Frame {
    /// Keyframe track for Render.cameraPosition
    pub camera_position: Option<FrameVector3>,
    /// Keyframe track for Render.cameraRotation
    pub camera_rotation: Option<FrameVector3>,
    /// Keyframe track for Render.depthFogColor
    pub depth_fog_color: Option<FrameColor>,
    /// Keyframe track for Render.depthFogEnabled
    pub depth_fog_enabled: Option<FrameBool>,
    /// Keyframe track for Render.depthFogEnd
    pub depth_fog_end: Option<FrameFloat>,
    /// Keyframe track for Render.depthFogIntensity
    pub depth_fog_intensity: Option<FrameFloat>,
    /// Keyframe track for Render.depthFogStart
    pub depth_fog_start: Option<FrameFloat>,
    /// Keyframe track for Render.depthOfFieldCircle
    pub depth_of_field_circle: Option<FrameFloat>,
    /// Keyframe track for Render.depthOfFieldEnabled
    pub depth_of_field_enabled: Option<FrameBool>,
    /// Keyframe track for Render.depthOfFieldFar
    pub depth_of_field_far: Option<FrameFloat>,
    /// Keyframe track for Render.depthOfFieldMid
    pub depth_of_field_mid: Option<FrameFloat>,
    /// Keyframe track for Render.depthOfFieldNear
    pub depth_of_field_near: Option<FrameFloat>,
    /// Keyframe track for Render.depthOfFieldWidth
    pub depth_of_field_width: Option<FrameFloat>,
    /// Keyframe track for Render.farClip
    pub far_clip: Option<FrameFloat>,
    /// Keyframe track for Render.fieldOfView
    pub field_of_view: Option<FrameFloat>,
    /// Keyframe track for Render.heightFogColor
    pub height_fog_color: Option<FrameColor>,
    /// Keyframe track for Render.heightFogEnabled
    pub height_fog_enabled: Option<FrameBool>,
    /// Keyframe track for Render.heightFogEnd
    pub height_fog_end: Option<FrameFloat>,
    /// Keyframe track for Render.heightFogIntensity
    pub height_fog_intensity: Option<FrameFloat>,
    /// Keyframe track for Render.heightFogStart
    pub height_fog_start: Option<FrameFloat>,
    /// Keyframe track for Render.navGridOffset
    pub nav_grid_offset: Option<FrameFloat>,
    /// Keyframe track for Render.nearClip
    pub near_clip: Option<FrameFloat>,
    /// Keyframe track for Playback.speed
    pub playback_speed: Option<FrameFloat>,
    /// Keyframe track for Render.selectionName
    pub selection_name: Option<FrameString>,
    /// Keyframe track for Render.selectionOffset
    pub selection_offset: Option<FrameVector3>,
    /// Keyframe track for Render.skyboxOffset
    pub skybox_offset: Option<FrameFloat>,
    /// Keyframe track for Render.skyboxRadius
    pub skybox_radius: Option<FrameFloat>,
    /// Keyframe track for Render.skyboxRotation
    pub skybox_rotation: Option<FrameFloat>,
    /// Keyframe track for Render.sunDirection
    pub sun_direction: Option<FrameVector3>,
    pub current_time: Duration,
}

#[derive(Default, Debug, Clone)]
pub struct FrameList(Vec<Frame>);

impl FrameList {
    #[must_use]
    pub fn new() -> Self {
        FrameList(Vec::default())
    }

    pub fn push(&mut self, frame: Frame) {
        self.0.push(frame);
    }

    /// Sorts the frames in time order
    pub fn sort(&mut self) {
        self.0.sort();
    }

    /// Deduplicates the frames based on if all the values of keyframes are the same, regardless of their time
    /// You probably want the list to be sorted before calling this, as it is destructive
    pub fn dedup(&mut self) {
        self.0.dedup();
    }
}

impl std::ops::Index<usize> for FrameList {
    type Output = Frame;

    fn index(&self, index: usize) -> &Frame {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for FrameList {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

use affix::paste;

macro_rules! count {
    () => (0);
    ( $x:tt $($xs:tt)* ) => (1 + count!($($xs)*));
}

// I don't want have to write so much repeated code
macro_rules! frame_list_collectors {
    ($($field:ident: $type:ty)+) => {
        impl FrameList {
            paste! {
                $(fn [<collect_ $field s>](&self) -> Vec<$type> {
                    self.0
                        .iter()
                        .filter_map(|frame| {
                            Some($type::from_frame_value(
                                frame.current_time,
                                frame.$field.clone()?,
                            ))
                        })
                        .collect()
                })+
            }
        }

        impl PartialEq<Self> for Frame {
            fn eq(&self, other: &Self) -> bool {
                $(self.$field == other.$field)&&*
            }
        }

        impl Serialize for FrameList {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                let mut sequence = serializer.serialize_struct("Sequence", count!($($field)*))?;
                paste! {
                    $(
                        let vec = self.[< collect_ $field s>]();
                        let field_name = stringify!([< $field:camel >]);
                        if vec.is_empty() {
                            sequence.skip_field(field_name)?;
                        } else {
                            sequence.serialize_field(field_name, &vec)?;
                        }
                    )+
                }

                sequence.end()
            }
        }

        impl<'de> Deserialize<'de> for FrameList {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                paste! {
                    #[derive(Deserialize)]
                    #[serde(field_identifier, rename_all = "camelCase")]
                    enum Fields {
                        $([<$field:pascal>]),*
                    }
                }
                struct SequenceVisitor;
                impl<'de> Visitor<'de> for SequenceVisitor {
                    type Value = FrameList;
                    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                        formatter.write_str("Expecting a sequence from the replay API")
                    }
                    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        let mut hashmap: HashMap<Duration, Frame> = HashMap::new();

                        $(let mut $field: Option<Vec<$type>> = None;)*
                        while let Some(key) = map.next_key()? {
                            paste! {
                                match key {
                                    $(Fields::[<$field:pascal>] => {
                                        if $field.is_some() {
                                            return Err(Error::duplicate_field(stringify!($field)))?;
                                        }

                                        $field = Some(map.next_value()?);
                                    })*
                                }
                            }
                        }

                        $(
                            if let Some($field) = $field {
                                for $field in $field {
                                    let frame_value = Some(FrameValue::new($field.value, $field.blend));
                                    if let Some(frame) = hashmap.get_mut(&$field.time) {
                                        frame.$field = frame_value;
                                    } else {
                                        let mut frame = Frame::empty($field.time.as_seconds_f64());
                                        frame.$field = frame_value;
                                        hashmap.insert($field.time, frame);
                                    }
                                }
                            }
                        )*

                        let mut frame_list = Vec::with_capacity(hashmap.len());

                        for frames in hashmap.into_values() {
                            frame_list.push(frames);
                        }

                        frame_list.sort();

                        Ok(FrameList(frame_list))
                    }
                }
                const FIELDS: &[&str] = &[$(paste!(stringify!([< $field:camel >]))),*];
                deserializer.deserialize_struct("Sequence", FIELDS, SequenceVisitor)
            }
        }
    }
}

// This is what a final sequence should look like
frame_list_collectors! {
    camera_position: KeyFrameVector3
    camera_rotation: KeyFrameVector3
    depth_fog_color: KeyFrameColor
    depth_fog_enabled: KeyFrameBool
    depth_fog_end: KeyFrameF64
    depth_fog_intensity: KeyFrameF64
    depth_fog_start: KeyFrameF64
    depth_of_field_circle: KeyFrameF64
    depth_of_field_enabled: KeyFrameBool
    depth_of_field_far: KeyFrameF64
    depth_of_field_mid: KeyFrameF64
    depth_of_field_near: KeyFrameF64
    depth_of_field_width: KeyFrameF64
    far_clip: KeyFrameF64
    field_of_view: KeyFrameF64
    height_fog_color: KeyFrameColor
    height_fog_enabled: KeyFrameBool
    height_fog_end: KeyFrameF64
    height_fog_intensity: KeyFrameF64
    height_fog_start: KeyFrameF64
    nav_grid_offset: KeyFrameF64
    near_clip: KeyFrameF64
    playback_speed: KeyFrameF64
    selection_name: KeyFrameString
    selection_offset: KeyFrameVector3
    skybox_offset: KeyFrameF64
    skybox_radius: KeyFrameF64
    skybox_rotation: KeyFrameF64
    sun_direction: KeyFrameVector3
}

#[derive(Clone, Debug, PartialEq)]
pub struct FrameValue<T: KeyFrameValue> {
    pub value: T,
    pub blending_mode: EasingType,
}

pub type FrameString = FrameValue<String>;
pub type FrameBool = FrameValue<bool>;
pub type FrameColor = FrameValue<ColorValue>;
pub type FrameFloat = FrameValue<f64>;
pub type FrameVector3 = FrameValue<Vector3f>;

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

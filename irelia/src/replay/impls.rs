use super::hidden::KeyFrameValue;
use super::{
    EasingType, Frame, FrameBool, FrameColor, FrameFloat, FrameString, FrameValue, FrameVector3,
    KeyFrameBool, KeyFrameColor, KeyFrameF64, KeyFrameString, KeyFrameT, KeyFrameVector3,
    Recording, Render, Sequence,
};
use serde::de::DeserializeOwned;
use std::cmp::Ordering;
use time::Duration;

impl Eq for Frame {}

impl PartialOrd<Self> for Frame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.current_time.partial_cmp(&other.current_time)
    }
}

impl Ord for Frame {
    fn cmp(&self, other: &Self) -> Ordering {
        self.current_time.cmp(&other.current_time)
    }
}

impl<T: KeyFrameValue + PartialEq> PartialEq<KeyFrameT<T>> for KeyFrameT<T> {
    fn eq(&self, other: &KeyFrameT<T>) -> bool {
        self.value == other.value
            && self.blend == EasingType::Linear
            && other.blend == EasingType::Linear
    }
}

impl<T: KeyFrameValue + PartialEq> PartialOrd<Self> for KeyFrameT<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.time.partial_cmp(&other.time)
    }
}

impl<T: KeyFrameValue + DeserializeOwned> KeyFrameT<T> {
    /// Creates a new generic keyframe with the time and value
    /// Defaults to `EasingType::Linear`
    fn new(time: f64, value: T) -> Self {
        Self {
            blend: EasingType::Linear,
            time: Duration::seconds_f64(time),
            value,
        }
    }

    pub fn from_frame_value(time: Duration, value: FrameValue<T>) -> Self {
        Self {
            time,
            value: value.value,
            blend: value.blending_mode,
        }
    }
}

impl<T: KeyFrameValue> FrameValue<T> {
    #[must_use]
    pub fn new(value: T, blending_mode: EasingType) -> Self {
        Self {
            value,
            blending_mode,
        }
    }

    #[must_use]
    pub fn new_default_blending(value: T) -> Self {
        Self::new(value, EasingType::default())
    }
}

impl Frame {
    #[must_use]
    pub fn empty(time: f64) -> Self {
        Self {
            current_time: Duration::seconds_f64(time),
            camera_position: None,
            camera_rotation: None,
            depth_fog_color: None,
            depth_fog_enabled: None,
            depth_fog_end: None,
            depth_fog_intensity: None,
            depth_fog_start: None,
            depth_of_field_circle: None,
            depth_of_field_enabled: None,
            depth_of_field_far: None,
            depth_of_field_mid: None,
            depth_of_field_near: None,
            depth_of_field_width: None,
            far_clip: None,
            field_of_view: None,
            height_fog_color: None,
            height_fog_enabled: None,
            height_fog_end: None,
            height_fog_intensity: None,
            height_fog_start: None,
            nav_grid_offset: None,
            near_clip: None,
            playback_speed: None,
            selection_name: None,
            selection_offset: None,
            skybox_offset: None,
            skybox_radius: None,
            skybox_rotation: None,
            sun_direction: None,
        }
    }

    #[must_use]
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_render_time(name: impl ToString, render: &Render, current_time: f64) -> Self {
        Self {
            current_time: Duration::seconds_f64(current_time),
            camera_position: Some(FrameVector3::new_default_blending(render.camera_position)),
            camera_rotation: Some(FrameVector3::new_default_blending(render.camera_rotation)),
            depth_fog_color: Some(FrameColor::new_default_blending(render.depth_fog_color)),
            depth_fog_enabled: Some(FrameBool::new_default_blending(render.depth_fog_enabled)),
            sun_direction: Some(FrameVector3::new_default_blending(render.sun_direction)),
            depth_fog_end: Some(FrameFloat::new_default_blending(render.depth_fog_end)),
            depth_fog_intensity: Some(FrameFloat::new_default_blending(render.depth_fog_intensity)),
            depth_fog_start: Some(FrameFloat::new_default_blending(render.depth_fog_start)),
            depth_of_field_circle: Some(FrameFloat::new_default_blending(
                render.depth_of_field_circle,
            )),
            depth_of_field_enabled: Some(FrameBool::new_default_blending(
                render.depth_of_field_enabled,
            )),
            depth_of_field_far: Some(FrameFloat::new_default_blending(render.depth_of_field_far)),
            depth_of_field_mid: Some(FrameFloat::new_default_blending(render.depth_of_field_mid)),
            depth_of_field_near: Some(FrameFloat::new_default_blending(render.depth_of_field_near)),
            depth_of_field_width: Some(FrameFloat::new_default_blending(
                render.depth_of_field_width,
            )),
            far_clip: Some(FrameFloat::new_default_blending(render.far_clip)),
            field_of_view: Some(FrameFloat::new_default_blending(render.field_of_view)),
            height_fog_color: Some(FrameColor::new_default_blending(render.height_fog_color)),
            height_fog_enabled: Some(FrameBool::new_default_blending(render.height_fog_enabled)),
            height_fog_end: Some(FrameFloat::new_default_blending(render.height_fog_end)),
            height_fog_intensity: Some(FrameFloat::new_default_blending(
                render.height_fog_intensity,
            )),
            height_fog_start: Some(FrameFloat::new_default_blending(render.height_fog_start)),
            nav_grid_offset: Some(FrameFloat::new_default_blending(render.nav_grid_offset)),
            near_clip: Some(FrameFloat::new_default_blending(render.near_clip)),
            selection_name: Some(FrameString::new_default_blending(name.to_string())),
            selection_offset: Some(FrameVector3::new_default_blending(render.selection_offset)),
            skybox_offset: Some(FrameFloat::new_default_blending(render.skybox_offset)),
            playback_speed: Some(FrameFloat::new_default_blending(1.0)),
            skybox_rotation: Some(FrameFloat::new_default_blending(render.skybox_rotation)),
            skybox_radius: Some(FrameFloat::new_default_blending(render.skybox_radius)),
        }
    }

    #[must_use]
    pub fn from_render_recording(
        name: impl ToString,
        render: &Render,
        recording: &Recording,
    ) -> Self {
        let mut frame =
            Self::from_render_time(name, render, recording.current_time.as_seconds_f64());
        let Some(playback_speed) = &mut frame.playback_speed else {
            unreachable!()
        };

        playback_speed.value = recording.replay_speed;

        frame
    }
}

impl Sequence {
    #[must_use]
    /// This is intended for clearing the sequence entirely
    pub fn none() -> Self {
        Self {
            camera_position: None,
            camera_rotation: None,
            depth_fog_color: None,
            depth_fog_enabled: None,
            depth_fog_end: None,
            depth_fog_intensity: None,
            depth_fog_start: None,
            depth_of_field_circle: None,
            depth_of_field_enabled: None,
            depth_of_field_far: None,
            depth_of_field_mid: None,
            depth_of_field_near: None,
            depth_of_field_width: None,
            far_clip: None,
            field_of_view: None,
            height_fog_color: None,
            height_fog_enabled: None,
            height_fog_end: None,
            height_fog_intensity: None,
            height_fog_start: None,
            nav_grid_offset: None,
            near_clip: None,
            playback_speed: None,
            selection_name: None,
            selection_offset: None,
            skybox_offset: None,
            skybox_radius: None,
            skybox_rotation: None,
            sun_direction: None,
        }
    }

    #[must_use]
    /// This creates a new empty sequence
    pub fn empty() -> Self {
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

    #[allow(clippy::needless_pass_by_value)]
    pub fn from_render_time(name: impl ToString, render: &Render, current_time: f64) -> Self {
        Self {
            camera_position: Some(vec![KeyFrameVector3::new(
                current_time,
                render.camera_position,
            )]),
            camera_rotation: Some(vec![KeyFrameVector3::new(
                current_time,
                render.camera_rotation,
            )]),
            depth_fog_color: Some(vec![KeyFrameColor::new(
                current_time,
                render.depth_fog_color,
            )]),
            depth_fog_enabled: Some(vec![KeyFrameBool::new(
                current_time,
                render.depth_fog_enabled,
            )]),
            depth_fog_end: Some(vec![KeyFrameF64::new(current_time, render.depth_fog_end)]),
            depth_fog_intensity: Some(vec![KeyFrameF64::new(
                current_time,
                render.depth_fog_intensity,
            )]),
            depth_fog_start: Some(vec![KeyFrameF64::new(current_time, render.depth_fog_start)]),
            depth_of_field_circle: Some(vec![KeyFrameF64::new(
                current_time,
                render.depth_of_field_circle,
            )]),
            depth_of_field_enabled: Some(vec![KeyFrameBool::new(
                current_time,
                render.depth_of_field_enabled,
            )]),
            depth_of_field_far: Some(vec![KeyFrameF64::new(
                current_time,
                render.depth_of_field_far,
            )]),
            depth_of_field_mid: Some(vec![KeyFrameF64::new(
                current_time,
                render.depth_of_field_mid,
            )]),
            depth_of_field_near: Some(vec![KeyFrameF64::new(
                current_time,
                render.depth_of_field_near,
            )]),
            depth_of_field_width: Some(vec![KeyFrameF64::new(
                current_time,
                render.depth_of_field_width,
            )]),
            far_clip: Some(vec![KeyFrameF64::new(current_time, render.far_clip)]),
            field_of_view: Some(vec![KeyFrameF64::new(current_time, render.field_of_view)]),
            height_fog_color: Some(vec![KeyFrameColor::new(
                current_time,
                render.height_fog_color,
            )]),
            height_fog_enabled: Some(vec![KeyFrameBool::new(
                current_time,
                render.height_fog_enabled,
            )]),
            height_fog_end: Some(vec![KeyFrameF64::new(current_time, render.height_fog_end)]),
            height_fog_intensity: Some(vec![KeyFrameF64::new(
                current_time,
                render.height_fog_intensity,
            )]),
            height_fog_start: Some(vec![KeyFrameF64::new(
                current_time,
                render.height_fog_start,
            )]),
            nav_grid_offset: Some(vec![KeyFrameF64::new(current_time, render.nav_grid_offset)]),
            near_clip: Some(vec![KeyFrameF64::new(current_time, render.nav_grid_offset)]),
            playback_speed: Some(vec![KeyFrameF64::new(current_time, 1.0)]),
            selection_name: Some(vec![KeyFrameString::new(current_time, name.to_string())]),
            selection_offset: Some(vec![KeyFrameVector3::new(
                current_time,
                render.selection_offset,
            )]),
            skybox_offset: Some(vec![KeyFrameF64::new(current_time, render.skybox_offset)]),
            skybox_radius: Some(vec![KeyFrameF64::new(current_time, render.skybox_radius)]),
            skybox_rotation: Some(vec![KeyFrameF64::new(current_time, render.skybox_rotation)]),
            sun_direction: Some(vec![KeyFrameVector3::new(
                current_time,
                render.sun_direction,
            )]),
        }
    }

    pub fn from_render_recording(
        name: impl ToString,
        render: &Render,
        recording: &Recording,
    ) -> Self {
        let current_time = recording.current_time;
        let mut sequence = Self::from_render_time(name, render, current_time.as_seconds_f64());
        if let Some(value) = sequence.playback_speed.as_mut() {
            value[0].value = current_time.as_seconds_f64();
        }
        sequence
    }

    pub fn from_render(name: impl ToString, render: &Render) -> Self {
        Self::from_render_time(name, render, 0.0)
    }

    pub fn push_from_render_recording(&mut self, render: &Render, recording: &Recording) {
        let current_time = recording.current_time.as_seconds_f64();
        self.push_from_render_time(render, current_time);
        if let Some(vec) = &mut self.playback_speed {
            let place = vec.len() - 1;
            vec[place] = KeyFrameT::new(current_time, recording.replay_speed);
        }
    }

    pub fn push_from_render_time(&mut self, render: &Render, time: f64) {
        if let Some(vec) = &mut self.camera_position {
            vec.push(KeyFrameT::new(time, render.camera_position));
        }
        if let Some(vec) = &mut self.camera_rotation {
            vec.push(KeyFrameT::new(time, render.camera_rotation));
        }
        if let Some(vec) = &mut self.depth_fog_color {
            vec.push(KeyFrameT::new(time, render.depth_fog_color));
        }
        if let Some(vec) = &mut self.depth_fog_enabled {
            vec.push(KeyFrameT::new(time, render.depth_fog_enabled));
        }
        if let Some(vec) = &mut self.depth_fog_end {
            vec.push(KeyFrameT::new(time, render.depth_fog_end));
        }
        if let Some(vec) = &mut self.depth_fog_intensity {
            vec.push(KeyFrameT::new(time, render.depth_fog_intensity));
        }
        if let Some(vec) = &mut self.depth_fog_start {
            vec.push(KeyFrameT::new(time, render.depth_fog_start));
        }
        if let Some(vec) = &mut self.depth_of_field_circle {
            vec.push(KeyFrameT::new(time, render.depth_of_field_circle));
        }
        if let Some(vec) = &mut self.depth_of_field_enabled {
            vec.push(KeyFrameT::new(time, render.depth_of_field_enabled));
        }
        if let Some(vec) = &mut self.depth_of_field_far {
            vec.push(KeyFrameT::new(time, render.depth_of_field_far));
        }
        if let Some(vec) = &mut self.depth_of_field_mid {
            vec.push(KeyFrameT::new(time, render.depth_of_field_mid));
        }
        if let Some(vec) = &mut self.depth_of_field_near {
            vec.push(KeyFrameT::new(time, render.depth_of_field_near));
        }
        if let Some(vec) = &mut self.depth_of_field_width {
            vec.push(KeyFrameT::new(time, render.depth_of_field_width));
        }
        if let Some(vec) = &mut self.far_clip {
            vec.push(KeyFrameT::new(time, render.far_clip));
        }
        if let Some(vec) = &mut self.field_of_view {
            vec.push(KeyFrameT::new(time, render.field_of_view));
        }
        if let Some(vec) = &mut self.height_fog_color {
            vec.push(KeyFrameT::new(time, render.height_fog_color));
        }
        if let Some(vec) = &mut self.height_fog_enabled {
            vec.push(KeyFrameT::new(time, render.height_fog_enabled));
        }
        if let Some(vec) = &mut self.height_fog_end {
            vec.push(KeyFrameT::new(time, render.height_fog_end));
        }
        if let Some(vec) = &mut self.height_fog_intensity {
            vec.push(KeyFrameT::new(time, render.height_fog_intensity));
        }
        if let Some(vec) = &mut self.height_fog_start {
            vec.push(KeyFrameT::new(time, render.height_fog_start));
        }
        if let Some(vec) = &mut self.nav_grid_offset {
            vec.push(KeyFrameT::new(time, render.nav_grid_offset));
        }
        if let Some(vec) = &mut self.near_clip {
            vec.push(KeyFrameT::new(time, render.near_clip));
        }
        if let Some(vec) = &mut self.playback_speed {
            vec.push(KeyFrameT::new(time, vec[vec.len() - 2].value));
        }
        if let Some(vec) = &mut self.selection_offset {
            vec.push(KeyFrameT::new(time, render.selection_offset));
        }
        if let Some(vec) = &mut self.skybox_offset {
            vec.push(KeyFrameT::new(time, render.skybox_offset));
        }
        if let Some(vec) = &mut self.skybox_radius {
            vec.push(KeyFrameT::new(time, render.skybox_radius));
        }
        if let Some(vec) = &mut self.skybox_rotation {
            vec.push(KeyFrameT::new(time, render.skybox_rotation));
        }
        if let Some(vec) = &mut self.sun_direction {
            vec.push(KeyFrameT::new(time, render.sun_direction));
        }
    }
}

impl Sequence {
    #[allow(clippy::too_many_lines)]
    #[allow(clippy::needless_pass_by_value)]
    pub fn from_frames(frames: Vec<Frame>, name: impl ToString) -> Self {
        let mut sequence = Self::empty();
        let mut first = true;

        for frame in frames {
            let time = frame.current_time;

            if first {
                if let Some(section_name) = &mut sequence.selection_name {
                    section_name.push(KeyFrameString::from_frame_value(
                        time,
                        FrameValue::new_default_blending(name.to_string()),
                    ));
                }
            }

            first = false;

            if let Some(vec) = &mut sequence.camera_position {
                if let Some(value) = frame.camera_position {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.camera_rotation {
                if let Some(value) = frame.camera_rotation {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_fog_color {
                if let Some(value) = frame.depth_fog_color {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_fog_enabled {
                if let Some(value) = frame.depth_fog_enabled {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_fog_end {
                if let Some(value) = frame.depth_fog_end {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_fog_intensity {
                if let Some(value) = frame.depth_fog_intensity {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_fog_start {
                if let Some(value) = frame.depth_fog_start {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_of_field_circle {
                if let Some(value) = frame.depth_of_field_circle {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_of_field_enabled {
                if let Some(value) = frame.depth_of_field_enabled {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_of_field_far {
                if let Some(value) = frame.depth_of_field_far {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_of_field_mid {
                if let Some(value) = frame.depth_of_field_mid {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_of_field_near {
                if let Some(value) = frame.depth_of_field_near {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.depth_of_field_width {
                if let Some(value) = frame.depth_of_field_width {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.far_clip {
                if let Some(value) = frame.far_clip {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.field_of_view {
                if let Some(value) = frame.field_of_view {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.height_fog_color {
                if let Some(value) = frame.height_fog_color {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.height_fog_enabled {
                if let Some(value) = frame.height_fog_enabled {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.height_fog_end {
                if let Some(value) = frame.height_fog_end {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.height_fog_intensity {
                if let Some(value) = frame.height_fog_intensity {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.height_fog_start {
                if let Some(value) = frame.height_fog_start {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.nav_grid_offset {
                if let Some(value) = frame.nav_grid_offset {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.near_clip {
                if let Some(value) = frame.near_clip {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.playback_speed {
                if let Some(value) = frame.playback_speed {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.selection_offset {
                if let Some(value) = frame.selection_offset {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.skybox_offset {
                if let Some(value) = frame.skybox_offset {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.skybox_radius {
                if let Some(value) = frame.skybox_radius {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.skybox_rotation {
                if let Some(value) = frame.skybox_rotation {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
            if let Some(vec) = &mut sequence.sun_direction {
                if let Some(value) = frame.sun_direction {
                    vec.push(KeyFrameT::from_frame_value(time, value));
                }
            }
        }

        sequence
    }
}

use super::hidden::KeyFrameValue;
use super::{
    EasingType, Frame, FrameBool, FrameColor, FrameFloat, FrameString, FrameValue, FrameVector3,
    KeyFrameT, Recording, Render,
};
use std::cmp::Ordering;
use time::Duration;

impl Eq for Frame {}

impl PartialOrd<Self> for Frame {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

impl<T: KeyFrameValue> KeyFrameT<T> {
    pub(super) fn from_frame_value(time: Duration, value: FrameValue<T>) -> Self {
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

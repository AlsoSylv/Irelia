use super::hidden::KeyFrameValue;
use super::{
    EasingType, Frame, FrameAString, FrameBool, FrameColor, FrameFloat, FrameValue, FrameVector3,
    KeyFrameAString, KeyFrameBool, KeyFrameColor, KeyFrameFloat, KeyFrameT, KeyFrameVector3,
    Recording, Render, Sequence,
};
use serde::de::DeserializeOwned;

impl<T: KeyFrameValue + PartialEq> PartialEq<KeyFrameT<T>> for KeyFrameT<T> {
    fn eq(&self, other: &KeyFrameT<T>) -> bool {
        self.value == other.value
            && self.blend == EasingType::Linear
            && other.blend == EasingType::Linear
    }
}

impl<T: KeyFrameValue + DeserializeOwned> KeyFrameT<T> {
    /// Creates a new generic keyframe with the time and value
    /// Defaults to `EasingType::Linear`
    fn new(time: f32, value: T) -> Self {
        Self {
            blend: EasingType::Linear,
            time,
            value,
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
    pub fn empty(time: f32) -> Self {
        Self {
            current_time: time,
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
    pub fn from_render_time(name: impl ToString, render: &Render, current_time: f32) -> Self {
        Self {
            current_time,
            camera_position: Some(FrameVector3::new_default_blending(render.camera_position)),
            camera_rotation: Some(FrameVector3::new_default_blending(render.camera_rotation)),
            depth_fog_color: Some(FrameColor::new_default_blending(
                render.depth_fog_color.clone(),
            )),
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
            height_fog_color: Some(FrameColor::new_default_blending(
                render.height_fog_color.clone(),
            )),
            height_fog_enabled: Some(FrameBool::new_default_blending(render.height_fog_enabled)),
            height_fog_end: Some(FrameFloat::new_default_blending(render.height_fog_end)),
            height_fog_intensity: Some(FrameFloat::new_default_blending(
                render.height_fog_intensity,
            )),
            height_fog_start: Some(FrameFloat::new_default_blending(render.height_fog_start)),
            nav_grid_offset: Some(FrameFloat::new_default_blending(render.nav_grid_offset)),
            near_clip: Some(FrameFloat::new_default_blending(render.near_clip)),
            selection_name: Some(FrameAString::new_default_blending(name.to_string())),
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
        let mut frame = Self::from_render_time(name, render, recording.current_time);
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
    pub fn from_render_time(name: impl ToString, render: &Render, current_time: f32) -> Self {
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
                render.depth_fog_color.clone(),
            )]),
            depth_fog_enabled: Some(vec![KeyFrameBool::new(
                current_time,
                render.depth_fog_enabled,
            )]),
            depth_fog_end: Some(vec![KeyFrameFloat::new(current_time, render.depth_fog_end)]),
            depth_fog_intensity: Some(vec![KeyFrameFloat::new(
                current_time,
                render.depth_fog_intensity,
            )]),
            depth_fog_start: Some(vec![KeyFrameFloat::new(
                current_time,
                render.depth_fog_start,
            )]),
            depth_of_field_circle: Some(vec![KeyFrameFloat::new(
                current_time,
                render.depth_of_field_circle,
            )]),
            depth_of_field_enabled: Some(vec![KeyFrameBool::new(
                current_time,
                render.depth_of_field_enabled,
            )]),
            depth_of_field_far: Some(vec![KeyFrameFloat::new(
                current_time,
                render.depth_of_field_far,
            )]),
            depth_of_field_mid: Some(vec![KeyFrameFloat::new(
                current_time,
                render.depth_of_field_mid,
            )]),
            depth_of_field_near: Some(vec![KeyFrameFloat::new(
                current_time,
                render.depth_of_field_near,
            )]),
            depth_of_field_width: Some(vec![KeyFrameFloat::new(
                current_time,
                render.depth_of_field_width,
            )]),
            far_clip: Some(vec![KeyFrameFloat::new(current_time, render.far_clip)]),
            field_of_view: Some(vec![KeyFrameFloat::new(current_time, render.field_of_view)]),
            height_fog_color: Some(vec![KeyFrameColor::new(
                current_time,
                render.height_fog_color.clone(),
            )]),
            height_fog_enabled: Some(vec![KeyFrameBool::new(
                current_time,
                render.height_fog_enabled,
            )]),
            height_fog_end: Some(vec![KeyFrameFloat::new(
                current_time,
                render.height_fog_end,
            )]),
            height_fog_intensity: Some(vec![KeyFrameFloat::new(
                current_time,
                render.height_fog_intensity,
            )]),
            height_fog_start: Some(vec![KeyFrameFloat::new(
                current_time,
                render.height_fog_start,
            )]),
            nav_grid_offset: Some(vec![KeyFrameFloat::new(
                current_time,
                render.nav_grid_offset,
            )]),
            near_clip: Some(vec![KeyFrameFloat::new(
                current_time,
                render.nav_grid_offset,
            )]),
            playback_speed: Some(vec![KeyFrameFloat::new(current_time, 1.0)]),
            selection_name: Some(vec![KeyFrameAString::new(current_time, name.to_string())]),
            selection_offset: Some(vec![KeyFrameVector3::new(
                current_time,
                render.selection_offset,
            )]),
            skybox_offset: Some(vec![KeyFrameFloat::new(current_time, render.skybox_offset)]),
            skybox_radius: Some(vec![KeyFrameFloat::new(current_time, render.skybox_radius)]),
            skybox_rotation: Some(vec![KeyFrameFloat::new(
                current_time,
                render.skybox_rotation,
            )]),
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
        let mut sequence = Self::from_render_time(name, render, current_time);
        if let Some(value) = sequence.playback_speed.as_mut() {
            value[0].value = current_time;
        }
        sequence
    }

    pub fn from_render(name: impl ToString, render: &Render) -> Self {
        Self::from_render_time(name, render, 0.0)
    }
}

impl From<Vec<Frame>> for Sequence {
	fn from(frames: Vec<Frame>) -> Self {
		let mut sequence = Self::empty();
		
		for frame in frames {
			todo!("There needs to be a giant if let Some() chain here")
		}
		
		sequence
	}
}
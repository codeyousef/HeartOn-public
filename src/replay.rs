// SPDX-License-Identifier: MIT
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraFrame {
    pub frame: u32,
    pub position: [f32; 3],
    pub rotation: [f32; 4],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputEvent {
    pub frame: u32,
    pub event_type: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameTiming {
    pub frame: u32,
    pub gpu_ms: f32,
    pub cpu_ms: f32,
}

#[derive(Debug, Serialize, Deserialize, Resource)]
pub struct Replay {
    pub name: String,
    pub total_frames: u32,
    pub camera_frames: Vec<CameraFrame>,
    pub input_events: Vec<InputEvent>,
    pub gpu_timings: Vec<FrameTiming>,
}

impl Replay {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            total_frames: 0,
            camera_frames: Vec::new(),
            input_events: Vec::new(),
            gpu_timings: Vec::new(),
        }
    }

    pub fn add_camera_frame(&mut self, frame: u32, position: [f32; 3], rotation: [f32; 4]) {
        self.camera_frames.push(CameraFrame {
            frame,
            position,
            rotation,
        });
        self.total_frames = self.total_frames.max(frame);
    }

    pub fn add_input_event(&mut self, frame: u32, event_type: &str, data: &str) {
        self.input_events.push(InputEvent {
            frame,
            event_type: event_type.to_string(),
            data: data.to_string(),
        });
    }

    pub fn add_timing(&mut self, frame: u32, gpu_ms: f32, cpu_ms: f32) {
        self.gpu_timings.push(FrameTiming {
            frame,
            gpu_ms,
            cpu_ms,
        });
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let ron = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| format!("Failed to serialize replay: {}", e))?;

        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        fs::write(path, ron).map_err(|e| format!("Failed to write replay file: {}", e))?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self, String> {
        let content =
            fs::read_to_string(path).map_err(|e| format!("Failed to read replay file: {}", e))?;

        bevy::scene::ron::from_str(&content).map_err(|e| format!("Failed to parse replay file: {}", e))
    }
}

// System to record camera path
pub fn record_camera_path(
    frame_count: Res<bevy::core::FrameCount>,
    mut replay: ResMut<Replay>,
    query: Query<(&Transform, &Camera)>,
) {
    if let Ok((transform, _)) = query.get_single() {
        let frame = frame_count.0;
        replay.add_camera_frame(
            frame,
            transform.translation.to_array(),
            transform.rotation.to_array(),
        );
    }
}

// System to playback camera path
pub fn playback_camera_path(
    frame_count: Res<bevy::core::FrameCount>,
    replay: Res<Replay>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let frame = frame_count.0;
    // Simple linear search for now, optimize later
    if let Some(frame_data) = replay.camera_frames.iter().find(|f| f.frame == frame) {
        if let Ok(mut transform) = query.get_single_mut() {
            transform.translation = Vec3::from(frame_data.position);
            transform.rotation = Quat::from_array(frame_data.rotation);
        }
    }
}

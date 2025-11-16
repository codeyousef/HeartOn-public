use bevy_app::{App, Plugin, Update};
use bevy_ecs::prelude::{Res, ResMut, Resource, With, Query};
use bevy_input::keyboard::KeyCode;
use bevy_input::ButtonInput;
use bevy_render::camera::Camera;
use bevy_transform::components::Transform;
use bevy_math::{Vec3, Quat};
use bevy_log::{info, error};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraPathFrame {
    pub frame: u32,
    pub position: Vec3,
    pub rotation: Quat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputRecord {
    pub frame: u32,
    pub input_type: InputType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    KeyPress(String),
    KeyRelease(String),
    MouseButton { button: u8, pressed: bool },
    MouseMotion { delta_x: f32, delta_y: f32 },
    GamepadButton { id: usize, button: u8, pressed: bool },
}

#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct CameraPathRecording {
    pub frames: Vec<CameraPathFrame>,
    pub inputs: Vec<InputRecord>,
    pub recording: bool,
    pub current_frame: u32,
}

impl Default for CameraPathRecording {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            inputs: Vec::new(),
            recording: false,
            current_frame: 0,
        }
    }
}

impl CameraPathRecording {
    pub fn start_recording(&mut self) {
        self.recording = true;
        self.current_frame = 0;
        self.frames.clear();
        self.inputs.clear();
    }

    pub fn stop_recording(&mut self) {
        self.recording = false;
    }

    pub fn record_camera_frame(&mut self, position: Vec3, rotation: Quat) {
        if self.recording {
            self.frames.push(CameraPathFrame {
                frame: self.current_frame,
                position,
                rotation,
            });
            self.current_frame += 1;
        }
    }

    pub fn record_input(&mut self, input_type: InputType) {
        if self.recording {
            self.inputs.push(InputRecord {
                frame: self.current_frame,
                input_type,
            });
        }
    }

    pub fn save_to_ron(&self, path: &str) -> Result<(), String> {
        let ron = ron::ser::to_string_pretty(self, ron::ser::PrettyConfig::default())
            .map_err(|e| format!("Failed to serialize recording: {}", e))?;

        std::fs::create_dir_all(std::path::Path::new(path).parent().unwrap())
            .map_err(|e| format!("Failed to create directory: {}", e))?;

        std::fs::write(path, ron).map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    pub fn load_from_ron(path: &str) -> Result<Self, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        ron::from_str(&content).map_err(|e| format!("Failed to parse RON: {}", e))
    }
}

#[derive(Resource)]
pub struct CameraPathPlayback {
    pub recording: CameraPathRecording,
    pub current_frame: u32,
    pub playing: bool,
}

impl CameraPathPlayback {
    pub fn new(recording: CameraPathRecording) -> Self {
        Self {
            recording,
            current_frame: 0,
            playing: false,
        }
    }

    pub fn start(&mut self) {
        self.playing = true;
        self.current_frame = 0;
    }

    pub fn stop(&mut self) {
        self.playing = false;
    }

    pub fn get_current_frame(&self) -> Option<&CameraPathFrame> {
        if self.playing {
            self.recording.frames.iter().find(|f| f.frame == self.current_frame)
        } else {
            None
        }
    }

    pub fn advance_frame(&mut self) {
        if self.playing {
            self.current_frame += 1;
            if self.current_frame >= self.recording.current_frame {
                self.stop();
            }
        }
    }
}

pub fn camera_recorder_system(
    mut recording: ResMut<CameraPathRecording>,
    camera_query: Query<(&Transform, &Camera), With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F9) {
        if recording.recording {
            recording.stop_recording();
            if let Err(e) = recording.save_to_ron("assets/replays/latest.ron") {
                error!("Failed to save recording: {}", e);
            } else {
                info!("Recording saved to assets/replays/latest.ron");
            }
        } else {
            recording.start_recording();
            info!("Started recording");
        }
    }

    if recording.recording {
        if let Ok((transform, _)) = camera_query.get_single() {
            recording.record_camera_frame(transform.translation, transform.rotation);
        }
    }

    if recording.recording {
        for key in keyboard.get_just_pressed() {
            recording.record_input(InputType::KeyPress(format!("{:?}", key)));
        }
        for key in keyboard.get_just_released() {
            recording.record_input(InputType::KeyRelease(format!("{:?}", key)));
        }
    }
}

pub fn camera_playback_system(
    mut playback: ResMut<CameraPathPlayback>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::F10) {
        if playback.playing {
            playback.stop();
            info!("Playback stopped");
        } else {
            playback.start();
            info!("Playback started");
        }
    }

    if let Some(frame) = playback.get_current_frame() {
        if let Ok(mut transform) = camera_query.get_single_mut() {
            transform.translation = frame.position;
            transform.rotation = frame.rotation;
        }
    }

    playback.advance_frame();
}

pub struct ReplayPlugin;

impl Plugin for ReplayPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CameraPathRecording>()
            .add_systems(Update, camera_recorder_system);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recording_start_stop() {
        let mut recording = CameraPathRecording::default();
        assert!(!recording.recording);

        recording.start_recording();
        assert!(recording.recording);
        assert_eq!(recording.current_frame, 0);
        assert_eq!(recording.frames.len(), 0);

        recording.stop_recording();
        assert!(!recording.recording);
    }

    #[test]
    fn test_record_camera_frame() {
        let mut recording = CameraPathRecording::default();
        recording.start_recording();

        let pos = Vec3::new(1.0, 2.0, 3.0);
        let rot = Quat::from_rotation_y(0.5);

        recording.record_camera_frame(pos, rot);

        assert_eq!(recording.frames.len(), 1);
        assert_eq!(recording.frames[0].frame, 0);
        assert_eq!(recording.frames[0].position, pos);
        assert_eq!(recording.frames[0].rotation, rot);
        assert_eq!(recording.current_frame, 1);
    }

    #[test]
    fn test_record_input() {
        let mut recording = CameraPathRecording::default();
        recording.start_recording();

        recording.record_input(InputType::KeyPress(KeyCode::KeyW));

        assert_eq!(recording.inputs.len(), 1);
        assert_eq!(recording.inputs[0].frame, 0);
    }

    #[test]
    fn test_playback_creation() {
        let recording = CameraPathRecording::default();
        let playback = CameraPathPlayback::new(recording);

        assert!(!playback.playing);
        assert_eq!(playback.current_frame, 0);
    }

    #[test]
    fn test_playback_start_stop() {
        let recording = CameraPathRecording::default();
        let mut playback = CameraPathPlayback::new(recording);

        playback.start();
        assert!(playback.playing);

        playback.stop();
        assert!(!playback.playing);
    }

    #[test]
    fn test_playback_advance_frame() {
        let mut recording = CameraPathRecording::default();
        recording.start_recording();
        recording.record_camera_frame(Vec3::ZERO, Quat::IDENTITY);
        recording.record_camera_frame(Vec3::ONE, Quat::IDENTITY);

        let mut playback = CameraPathPlayback::new(recording);
        playback.start();

        assert_eq!(playback.current_frame, 0);
        playback.advance_frame();
        assert_eq!(playback.current_frame, 1);
    }

    #[test]
    fn test_save_load_ron() {
        let mut recording = CameraPathRecording::default();
        recording.start_recording();
        recording.record_camera_frame(Vec3::new(1.0, 2.0, 3.0), Quat::IDENTITY);
        recording.record_input(InputType::KeyPress(KeyCode::KeyW));

        let path = "/tmp/test_replay.ron";
        recording.save_to_ron(path).unwrap();

        let loaded = CameraPathRecording::load_from_ron(path).unwrap();
        assert_eq!(loaded.frames.len(), 1);
        assert_eq!(loaded.inputs.len(), 1);
        assert_eq!(loaded.frames[0].position, Vec3::new(1.0, 2.0, 3.0));
    }
}

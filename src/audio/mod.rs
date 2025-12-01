// SPDX-License-Identifier: MIT
use bevy::prelude::*;

pub struct HeartOnAudioPlugin;

impl Plugin for HeartOnAudioPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AudioEmitter>()
            .register_type::<BiomeAudio>()
            .add_systems(Update, update_spatial_audio);
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AudioEmitter {
    pub volume: f32,
    pub pitch: f32,
    pub is_spatial: bool,
    pub max_distance: f32,
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct BiomeAudio {
    pub ambient_track: String,
    pub reverb_intensity: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ReverbZone {
    pub intensity: f32,
    pub radius: f32,
}

pub fn update_spatial_audio(
    mut query: Query<(&GlobalTransform, &mut AudioSink), With<AudioEmitter>>,
    listener: Query<&GlobalTransform, With<Camera3d>>,
    reverb_zones: Query<(&GlobalTransform, &ReverbZone)>,
) {
    let Ok(listener_transform) = listener.get_single() else {
        return;
    };

    // Calculate reverb based on listener position
    let mut total_reverb = 0.0;
    for (zone_tf, zone) in reverb_zones.iter() {
        let dist = zone_tf.translation().distance(listener_transform.translation());
        if dist < zone.radius {
            total_reverb += zone.intensity * (1.0 - dist / zone.radius);
        }
    }
    
    // Clamp reverb
    total_reverb = total_reverb.min(1.0);

    for (transform, sink) in query.iter_mut() {
        // Simple distance-based volume attenuation
        let distance = transform.translation().distance(listener_transform.translation());
        
        if distance > 50.0 {
            sink.set_volume(0.0);
        } else {
            sink.set_volume(1.0 - (distance / 50.0));
            // In a real engine, we'd set the reverb effect on the sink here
            // sink.set_effect_mix(total_reverb); 
        }
    }
}

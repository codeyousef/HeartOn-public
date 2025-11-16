// Simple Voxel World Example (Community Edition)
// Demonstrates basic HeartOn Public API usage
// MIT Licensed

use bevy::prelude::*;
use hearton_public::{
    HeartOnPublicPlugin, HeartOnPublicSettings,
    voxel::{DummyVoxelWorld, Voxel},
    budget::HeartOnBudget,
    capabilities::HeartOnCapabilities,
};

fn main() {
    println!("HeartOn Community Edition - Simple Voxel World");
    println!("===============================================\n");

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "HeartOn - Simple Voxel World".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(HeartOnPublicPlugin::new(HeartOnPublicSettings {
            enable_hud: true,
            enable_budget_tracking: true,
            budget_history_size: 100,
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_voxels, print_stats))
        .run();
}

fn setup(
    mut commands: Commands,
    caps: Res<HeartOnCapabilities>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(15.0, 15.0, 15.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 5000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(10.0, 20.0, 10.0),
        ..default()
    });

    let mut world = DummyVoxelWorld::new();
    world.generate_test_world(20);

    println!("Voxel world created:");
    println!("  Voxels: {}", world.voxel_count());
    println!("  At limit: {}", world.is_at_limit());
    println!("  Max voxels (Community): {}\n", caps.max_voxels);
    println!("Note: Voxels are rendered using Bevy's PBR system");
    println!("      This is the Community Edition renderer\n");

    commands.spawn(world);
}

fn update_voxels(
    mut budget: ResMut<HeartOnBudget>,
) {
    budget.begin_frame();
    budget.end_frame();
}

fn print_stats(
    time: Res<Time>,
    budget: Res<HeartOnBudget>,
    mut last_print: Local<f32>,
) {
    let elapsed = time.elapsed_seconds();
    
    if elapsed - *last_print >= 5.0 {
        println!("Performance stats:");
        println!("  FPS: {:.1}", budget.average_fps());
        println!("  Frame time: {:.2}ms", budget.average_cpu_ms());
        println!("  Frame count: {}\n", budget.frame_number());
        
        *last_print = elapsed;
    }
}

// SPDX-License-Identifier: MIT
//! Dummy voxel renderer using instanced cubes (Community Edition)

use bevy::prelude::*;

/// Marker component for voxel instances
#[derive(Component)]
pub struct VoxelInstance {
    /// Parent entity with `VoxelScene` handle
    pub parent: Entity,
}

/// Marker component for voxel scene root
#[derive(Component)]
pub struct VoxelSceneRoot;

/// Render voxel scenes as instanced cubes (Community Edition renderer)
pub fn render_dummy_voxels(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    voxel_scenes: Query<(Entity, &Handle<crate::voxel::VoxelScene>), Added<Handle<crate::voxel::VoxelScene>>>,
    scenes: Res<Assets<crate::voxel::VoxelScene>>,
    mut metrics: ResMut<crate::metrics::PerformanceMetrics>,
) {
    for (entity, scene_handle) in &voxel_scenes {
        let Some(scene) = scenes.get(scene_handle) else { 
            continue;
        };
        
        let voxel_count = scene.voxel_count();
        
        // Performance warning for large scenes
        if voxel_count > 100_000 {
            warn!(
                "Scene has {} voxels - Community renderer may struggle. \
                Consider upgrading to Professional Edition for GPU-driven pipeline.",
                voxel_count
            );
        }
        
        info!("Rendering {} voxels as instanced cubes", voxel_count);
        
        // Update metrics
        metrics.voxel_count = voxel_count;
        
        // Mark entity as voxel scene root
        commands.entity(entity).insert(VoxelSceneRoot);
        
        // Create shared cube mesh
        let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        
        // Get voxels from scene
        let voxels = match &scene.voxel_data {
            crate::voxel::VoxelData::Community(data) => &data.voxels,
            crate::voxel::VoxelData::Professional(_) => {
                warn!("Professional voxel data not yet supported in Community renderer");
                continue;
            }
        };
        
        // Spawn a cube for each voxel (instanced rendering)
        for voxel in voxels {
            let color = Color::rgba_u8(
                voxel.color[0],
                voxel.color[1],
                voxel.color[2],
                voxel.color[3],
            );
            
            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        perceptual_roughness: 0.8,
                        metallic: 0.0,
                        ..default()
                    }),
                    transform: Transform::from_translation(Vec3::new(
                        voxel.position[0] as f32,
                        voxel.position[1] as f32,
                        voxel.position[2] as f32,
                    )),
                    ..default()
                },
                VoxelInstance { parent: entity },
            ));
        }
        
        info!("Spawned {} voxel instances", voxels.len());
    }
}

/// Clean up voxel instances when scene is removed
pub fn cleanup_voxel_instances(
    mut commands: Commands,
    mut removed_scenes: RemovedComponents<Handle<crate::voxel::VoxelScene>>,
    instance_query: Query<(Entity, &VoxelInstance)>,
) {
    for removed_entity in removed_scenes.read() {
        // Remove all instances for this scene
        for (instance_entity, instance) in &instance_query {
            if instance.parent == removed_entity {
                commands.entity(instance_entity).despawn();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_voxel_instance_component() {
        let parent = Entity::from_raw(42);
        let instance = VoxelInstance { parent };
        
        assert_eq!(instance.parent, parent);
    }
}

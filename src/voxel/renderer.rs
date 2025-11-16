// HeartOn Voxel Renderer (Community Edition)
// MIT Licensed
// Uses Bevy's built-in PBR rendering for simple voxel visualization

use bevy_app::{App, Plugin, Update};
use bevy_asset::{Assets, Handle};
use bevy_ecs::prelude::*;
use bevy_pbr::{PbrBundle, StandardMaterial};
use bevy_render::{mesh::Mesh, color::Color};
use bevy_transform::components::Transform;
use bevy_math::Vec3;
use bevy_hierarchy::BuildChildren;

use super::dummy::{DummyVoxelWorld, Voxel};

#[derive(Component)]
pub struct VoxelMesh {
    pub voxel_index: usize,
}

#[derive(Component)]
pub struct VoxelWorldEntity;

pub struct VoxelRendererPlugin;

impl Plugin for VoxelRendererPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            spawn_voxel_meshes,
            update_voxel_meshes,
        ));
    }
}

fn spawn_voxel_meshes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &DummyVoxelWorld), (Without<VoxelWorldEntity>, Changed<DummyVoxelWorld>)>,
) {
    for (entity, world) in query.iter() {
        commands.entity(entity).insert(VoxelWorldEntity);
        
        let cube_mesh = meshes.add(Mesh::from(bevy_render::mesh::shape::Cube { size: 1.0 }));
        
        for (index, voxel) in world.voxels().iter().enumerate() {
            let material = materials.add(StandardMaterial {
                base_color: voxel_color(voxel),
                perceptual_roughness: 0.8,
                metallic: 0.0,
                ..Default::default()
            });
            
            let transform = Transform::from_translation(Vec3::new(
                voxel.x as f32,
                voxel.y as f32,
                voxel.z as f32,
            ));
            
            let voxel_entity = commands.spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material,
                    transform,
                    ..Default::default()
                },
                VoxelMesh { voxel_index: index },
            )).id();
            
            commands.entity(entity).add_child(voxel_entity);
        }
    }
}

fn update_voxel_meshes(
    mut materials: ResMut<Assets<StandardMaterial>>,
    world_query: Query<&DummyVoxelWorld, Changed<DummyVoxelWorld>>,
    mut voxel_query: Query<(&VoxelMesh, &Handle<StandardMaterial>, &mut Transform)>,
) {
    for world in world_query.iter() {
        for (voxel_mesh, material_handle, mut transform) in voxel_query.iter_mut() {
            if let Some(voxel) = world.voxels().get(voxel_mesh.voxel_index) {
                transform.translation = Vec3::new(
                    voxel.x as f32,
                    voxel.y as f32,
                    voxel.z as f32,
                );
                
                if let Some(material) = materials.get_mut(material_handle) {
                    material.base_color = voxel_color(voxel);
                }
            }
        }
    }
}

fn voxel_color(voxel: &Voxel) -> Color {
    let r = ((voxel.color >> 24) & 0xFF) as f32 / 255.0;
    let g = ((voxel.color >> 16) & 0xFF) as f32 / 255.0;
    let b = ((voxel.color >> 8) & 0xFF) as f32 / 255.0;
    let a = (voxel.color & 0xFF) as f32 / 255.0;
    Color::rgba(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn voxel_color_conversion() {
        let voxel = Voxel::from_rgb(0, 0, 0, 255, 128, 64);
        
        let color = voxel_color(&voxel);
        assert_eq!(color.r(), 1.0);
        assert!((color.g() - 0.502).abs() < 0.01);
        assert!((color.b() - 0.251).abs() < 0.01);
        assert_eq!(color.a(), 1.0);
    }
}

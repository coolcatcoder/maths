pub use crate::bevy_prelude::*;
use bevy::{
    app::HierarchyPropagatePlugin, core_pipeline::tonemapping::Tonemapping, light::NotShadowCaster,
    post_process::bloom::Bloom,
};

pub mod outlines;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        outlines::plugin,
        HierarchyPropagatePlugin::<SceneNotShadowCaster>::new(Update),
        HierarchyPropagatePlugin::<ComesFromRootEntity>::new(Update),
    ))
    .add_systems(Startup, spawn_camera)
    .add_systems(
        PostUpdate,
        camera_follow.before(TransformSystems::Propagate),
    )
    .insert_resource(AmbientLight {
        brightness: 0.0,
        ..default()
    });
}

/// Stops a gltf scene from casting shadows.
#[derive(PartialEq, Clone, Component)]
#[require(NotShadowCaster)]
pub struct SceneNotShadowCaster;

#[derive(PartialEq, Clone, Component)]
pub struct ComesFromRootEntity(pub Entity);

/// Camera's offset from the controlled character.
const CAMERA_OFFSET: Vec3 = Vec3::new(0., 3., 3.);

#[derive(Component)]
pub struct CameraFollow;

pub fn spawn_camera(mut commands: Commands, mut clear_colour: ResMut<ClearColor>) {
    clear_colour.0 = Color::BLACK;
    commands.spawn((
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
        Camera { ..default() },
        Camera3d { ..default() },
        Tonemapping::TonyMcMapface,
        Bloom::NATURAL,
    ));
}

pub fn camera_follow(
    follow: Query<&Transform, With<CameraFollow>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<CameraFollow>)>,
    time: Res<Time>,
) {
    let follow = follow.single().else_return()?;
    let mut camera = camera.single_mut().else_error("Could not get camera.")?;

    let camera_no_offset = camera.translation.xz() - CAMERA_OFFSET.xz();

    let vector_from_camera_to_follow = follow.translation.xz() - camera_no_offset;
    let amount_to_translate = vector_from_camera_to_follow * (6. * time.delta_secs());

    let new_xz_translation = camera_no_offset + amount_to_translate + CAMERA_OFFSET.xz();

    camera.translation.x = new_xz_translation.x;
    camera.translation.z = new_xz_translation.y;
}


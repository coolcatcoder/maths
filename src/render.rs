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
const CAMERA_OFFSET: Vec3 = Vec3::new(0., 1., 1.3);

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

    let mut camera_translation =
        camera.translation.xz() - Vec2::new(CAMERA_OFFSET.x, CAMERA_OFFSET.z);
    camera_translation.smooth_nudge(&follow.translation.xz(), 10., time.delta_secs());
    camera.translation.x = camera_translation.x + CAMERA_OFFSET.x;
    camera.translation.z = camera_translation.y + CAMERA_OFFSET.z;
}


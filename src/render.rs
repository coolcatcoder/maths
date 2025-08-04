use std::ops::{ControlFlow, FromResidual, Try};

use crate::{
    error_handling::ToUnwrapResult, mind_control::Controlled, propagate::HierarchyPropagatePlugin,
};
use bevy::{pbr::NotShadowCaster, prelude::*};

pub mod outlines;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        outlines::plugin,
        HierarchyPropagatePlugin::<SceneNotShadowCaster>::default(),
        HierarchyPropagatePlugin::<ComesFromRootEntity>::default(),
    ))
    .add_systems(Startup, spawn_camera)
    .add_systems(
        PostUpdate,
        move_camera_to_controlled.before(TransformSystem::TransformPropagate),
    )
    .insert_resource(AmbientLight {
        brightness: 0.0,
        ..default()
    }).init_resource::<CameraFollow>();
}

/// Stops a gltf scene from casting shadows.
#[derive(PartialEq, Clone, Component)]
#[require(NotShadowCaster)]
pub struct SceneNotShadowCaster;

#[derive(PartialEq, Clone, Component)]
pub struct ComesFromRootEntity(pub Entity);

/// Camera's offset from the controlled character.
const CAMERA_OFFSET: Vec3 = Vec3::new(0., 10., 13.);

#[derive(Default, Resource)]
pub struct CameraFollow(Option<Entity>);

pub fn spawn_camera(mut commands: Commands, mut clear_colour: ResMut<ClearColor>) {
    clear_colour.0 = Color::BLACK;
    commands.spawn((
        Transform::from_translation(CAMERA_OFFSET).looking_at(Vec3::ZERO, Vec3::Y),
        Camera { ..default() },
        Camera3d { ..default() },
    ));
}

struct MaybeBad<T>(Option<T>);
struct Bad;

impl<T> FromResidual for MaybeBad<T> {
    fn from_residual(_: Bad) -> Self {
        MaybeBad(None)
    }
}

impl FromResidual<Bad> for () {
    fn from_residual(_: Bad) -> Self {
        
    }
}

impl<T> Try for MaybeBad<T> {
    type Output = T;
    type Residual = Bad;

    fn from_output(output: Self::Output) -> Self {
        MaybeBad(Some(output))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Some(value) => ControlFlow::Continue(value),
            None => ControlFlow::Break(Bad),
        }
    }
}

pub fn camera_follow(camera_follow: Res<CameraFollow>) {
    let blah = MaybeBad(Some(0_i32));
    let dah = blah?;
}

pub fn move_camera_to_controlled(
    controlled: Option<Single<&Transform, With<Controlled>>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Controlled>)>,
    time: Res<Time>,
) {
    let controlled_translation = controlled.else_return()?.translation.xz();
    let mut camera_translation =
        camera.translation.xz() - Vec2::new(CAMERA_OFFSET.x, CAMERA_OFFSET.z);
    camera_translation.smooth_nudge(&controlled_translation, 10., time.delta_secs());
    camera.translation.x = camera_translation.x + CAMERA_OFFSET.x;
    camera.translation.z = camera_translation.y + CAMERA_OFFSET.z;
}

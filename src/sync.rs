use crate::error_handling::ToUnwrapResult;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        PostUpdate,
        (sync_rotation, sync_translation)
            .chain()
            .before(TransformSystem::TransformPropagate),
    );
}

/// Syncs a translation to another entity, if it exists and has the component.
/// Will not sync to a translation which is also syncing.
#[derive(Component)]
#[require(Transform)]
pub struct SyncTranslation {
    pub target: Entity,
    pub offset: Vec3,
}

fn sync_translation(
    sync: Query<(&SyncTranslation, &mut Transform)>,
    target: Query<&Transform, Without<SyncTranslation>>,
) {
    for (sync, mut transform) in sync {
        let target = target.get(sync.target).else_return()?;
        transform.translation = target.translation + sync.offset;
    }
}

/// Syncs a rotation to another entity, if it exists and has the component.
/// Will not sync to a rotation which is also syncing.
#[derive(Component)]
#[require(Transform)]
pub struct SyncRotation {
    pub target: Entity,
}

fn sync_rotation(
    sync: Query<(&SyncRotation, &mut Transform)>,
    target: Query<&Transform, Without<SyncRotation>>,
) {
    for (sync, mut transform) in sync {
        let target = target.get(sync.target).else_return()?;
        transform.rotation = target.rotation;
    }
}
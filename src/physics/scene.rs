use avian3d::prelude::*;
use crate::areas::LoadedFromArea;
pub use crate::bevy_prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, load);
}

fn load(
    names: Query<(Entity, &Name), Added<LoadedFromArea>>,
    mut commands: Commands,
) {
    for (entity, name) in names {
        name.starts_with("collider").else_return()?;

        commands.entity(entity).insert((
            ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
            RigidBody::Static
        ));

        info!("Decomposed");
    }
}

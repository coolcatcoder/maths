pub use crate::bevy_prelude::*;
use crate::render::ComesFromRootEntity;
use avian3d::prelude::*;

#[derive(Component)]
#[component(on_add = Self::on_add)]
pub struct Dragged(pub bool);

impl Dragged {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        world
            .commands()
            .entity(context.entity)
            .observe(drag_start)
            .observe(drag_end);
    }
}

pub fn drag_start(on: On<Pointer<DragStart>>, mut dragged: Query<&mut Dragged>) {
    dragged
        .get_mut(on.entity)
        .else_error("No Dragged component.")?
        .0 = true;
}

pub fn drag_end(on: On<Pointer<DragEnd>>, mut dragged: Query<&mut Dragged>) {
    dragged
        .get_mut(on.entity)
        .else_error("No Dragged component.")?
        .0 = true;
}

pub fn drag(
    drag: On<Pointer<Drag>>,
    mut velocity: Query<(&mut LinearVelocity, &Transform)>,
    camera: Query<(&Camera, &GlobalTransform)>,
    window: Query<&Window>,
    mut ray_cast: MeshRayCast,
    time: Res<Time>,
    comes_from_root_entity: Query<&ComesFromRootEntity>,
) {
    let (mut velocity, transform) = velocity
        .get_mut(drag.event().event_target())
        .else_error("No linear velocity when dragging entity.")?;

    let target = drag.event().event_target();

    let window = window.single().else_error("Not a single window.")?;
    let cursor_translation = window.cursor_position().else_return()?;

    let (camera, camera_transform) = camera.single().else_error("Not a single camera.")?;
    let cursor_ray = camera
        .viewport_to_world(camera_transform, cursor_translation)
        .else_error("Viewport to world failed.")?;

    let (_, hit) = ray_cast
        .cast_ray(
            cursor_ray,
            &MeshRayCastSettings {
                visibility: RayCastVisibility::VisibleInView,
                filter: &|entity| {
                    if entity == target {
                        return false;
                    }
                    let Ok(comes_from_root_entity) = comes_from_root_entity.get(entity) else {
                        return true;
                    };

                    comes_from_root_entity.0 != target
                },
                ..default()
            },
        )
        .first()
        .else_return()?;

    let cursor_translation = hit.point;
    let desired_translation = cursor_translation + Vec3::new(0., 1., 0.);

    let displacement = desired_translation - transform.translation;
    let time = time.delta_secs();
    **velocity = displacement * time * 1000.;
    **velocity = velocity.min(Vec3::splat(10.));
}

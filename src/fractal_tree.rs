use crate::l_systems::{Lsystem, Next, Start};
pub use crate::prelude::*;
pub use start_2d as start;

pub fn plugin(app: &mut App) {
    app.add_plugins(Symbol::plugin).add_systems(Startup, create_starting_symbols);
}

fn create_starting_symbols(mut commands: Commands) {
    commands.spawn((Symbol::Branch, Next(None), Start));
}

#[derive(Component, Clone, Copy)]
enum Symbol {
    Branch,
    Stem,
    PushTransform,
    PopTransform,
}
use Symbol::*;

impl Lsystem for Symbol {
    fn update(&self) -> &'static [Self] {
        match self {
            Branch => &[Stem, PushTransform, Branch, PopTransform, Branch],
            Stem => &[Stem, Stem],
            PushTransform => &[PushTransform],
            PopTransform => &[PopTransform],
        }
    }

    fn draw(&self, translation: &mut Vec2, rotation: &mut f32, saved_transforms: &mut Vec<(Vec2, f32)>, draw: &mut Draw) {
        match self {
            Branch | Stem => {
                let theta = rotation.to_radians();
                let cs = theta.cos();
                let sn = theta.sin();
                let length = 0.025;
                let rotated_movement = Vec2::new(0. * cs - length * sn, 0. * sn + length * cs);

                draw.0.line_2d(*translation, *translation + rotated_movement, RED);
                *translation += rotated_movement;
            }
            PushTransform => {
                saved_transforms.push((*translation, *rotation));
                *rotation += 45.;
            }
            PopTransform => {
                let restored = saved_transforms.pop().unwrap();
                *translation = restored.0;
                *rotation = restored.1;

                *rotation -= 45.;
            }
        }
    }
}
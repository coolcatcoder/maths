use crate::l_systems::{Lsystem, Next, Start};
pub use crate::prelude::*;
pub use start_2d as start;

pub fn plugin(app: &mut App) {
    app.add_plugins(Symbol::plugin)
        .add_systems(Startup, create_starting_symbols);
}

fn create_starting_symbols(mut commands: Commands) {
    commands.spawn((Symbol::Base, Next(None), Start));
}

#[derive(Component, Clone, Copy)]
enum Symbol {
    Base,
    Leaf,
    Line,
    Turn(f32),
    Rotate(f32),
    PushTransform,
    PopTransform,
}
use Symbol::*;

impl Lsystem for Symbol {
    fn update(&self, replace: impl FnOnce(&[Self])) {
        match self {
            Base => replace(&[
                PushTransform,
                Turn(-10.),
                Leaf,
                Leaf,
                Rotate(-30.),
                Leaf,
                PopTransform,
                PushTransform,
                Turn(10.),
                Leaf,
                Leaf,
                Rotate(30.),
                Leaf,
                PopTransform,
                Base,
            ]),
            Leaf => replace(&[Leaf, Line]),
            Turn(degrees) => replace(&[Turn(*degrees), Rotate(*degrees)]),
            other => replace(&[*other]),
        }
    }

    fn draw(
        &self,
        translation: &mut Vec2,
        rotation: &mut f32,
        saved_transforms: &mut Vec<(Vec2, f32)>,
        draw: &mut Draw,
    ) {
        match self {
            Line => {
                let theta = (-*rotation).to_radians();
                let cs = theta.cos();
                let sn = theta.sin();
                let length = 0.1;
                let rotated_movement = Vec2::new(0. * cs - length * sn, 0. * sn + length * cs);

                draw.0
                    .line_2d(*translation, *translation + rotated_movement, RED);
                *translation += rotated_movement;
            }
            Rotate(degrees) => {
                *rotation += degrees;
            }
            PushTransform => {
                saved_transforms.push((*translation, *rotation));
            }
            PopTransform => {
                let restored = saved_transforms.pop().unwrap();
                *translation = restored.0;
                *rotation = restored.1;
            }
            _ => (),
        }
    }
}

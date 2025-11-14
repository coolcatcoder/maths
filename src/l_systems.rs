pub use crate::prelude::*;
use bevy::ecs::query::QueryData;
pub use start_2d as start;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, create_starting_symbols).add_systems(Update, (draw_symbols, update_symbols).chain());
}

fn create_starting_symbols(mut commands: Commands) {
    commands.spawn((Symbol::Branch, Next(None), Start));
}

#[derive(Component)]
pub struct Next(Option<Entity>);
#[derive(Component)]
struct Start;

#[derive(Debug, Component)]
pub enum Symbol {
    Branch,
    Stem,
    PushTransform,
    PopTransform,
}
pub use Symbol::*;

const ZERO_NEXT: [Symbol; 5] = [Stem, PushTransform, Branch, PopTransform, Branch];
const ONE_NEXT: [Symbol; 2] = [Stem, Stem];

fn update_symbols(mut symbols: Query<(&mut Symbol, &mut Next)>, mut iteration: Local<u8>, mut commands: Commands) {
    if *iteration == 8 {
        return;
    }
    *iteration += 1;

    symbols.iter_mut().for_each(|(mut symbol, mut next)| {
        match *symbol {
            Symbol::Branch => {
                create_chain((&mut symbol, &mut next), ZERO_NEXT, &mut commands);
            }
            Symbol::Stem => {
                create_chain((&mut symbol, &mut next), ONE_NEXT, &mut commands);
            }
            _ => (),
        }
    });
}

fn create_chain<const N: usize>(replace: (&mut Symbol, &mut Next), symbols: [Symbol; N], commands: &mut Commands) {
    let mut symbols_iterator = symbols.into_iter();
    let Some(start) = symbols_iterator.next() else {
        error!("Why did you enter an empty array? We don't account for this possibility at all!");
        return;
    };

    let mut next = replace.1.0.take();
    for symbol in symbols_iterator.rev() {
        next = Some(commands.spawn((symbol, Next(next))).id());
    }

    replace.1.0 = next;
    *replace.0 = start;
}

fn draw_symbols(mut draw: Draw, start: Query<(&Symbol, &Next), With<Start>>, symbols: Query<(&Symbol, &Next)>) {
    let mut translation = Vec2::new(0., -4.);
    let mut saved = vec![];
    let mut rotation = 0.;
    let mut debug_string = String::new();

    fn recursion(symbol: &Symbol, next: &Next, query: &Query<(&Symbol, &Next)>, translation: &mut Vec2, saved: &mut Vec<(Vec2, f32)>, rotation: &mut f32, draw: &mut Draw, debug_string: &mut String) {
        let to_print = match symbol {
            Branch => "0",
            Stem => "1",
            PushTransform => "[",
            PopTransform => "]",
        };
        debug_string.push_str(to_print);

        match symbol {
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
                saved.push((*translation, *rotation));
                *rotation += 45.;
            }
            PopTransform => {
                let restored = saved.pop().unwrap();
                *translation = restored.0;
                *rotation = restored.1;

                *rotation -= 45.;
            }
        }

        if let Some(next) = next.0 && let Ok((symbol, next)) = query.get(next) {
            recursion(symbol, next, query, translation, saved, rotation, draw, debug_string);
        }
    }

    start.iter().for_each(|(symbol, next)| {
        //warn!("Start.");
        recursion(symbol, next, &symbols, &mut translation, &mut saved, &mut rotation, &mut draw, &mut debug_string);
        //warn!("Stop.");
    });

    //info!("{}", debug_string);

    // draw.grid_2d(
    //     Isometry2d::IDENTITY,
    //     UVec2::new(100, 100),
    //     Vec2::splat(1.),
    //     GRAY,
    // );
}

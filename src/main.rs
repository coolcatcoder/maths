#![feature(try_trait_v2)]
#![feature(macro_attr)]
#![warn(clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![allow(clippy::type_complexity)]
#![warn(clippy::unwrap_used)]
#![allow(clippy::needless_for_each)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use bevy::{
    dev_tools::fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin},
    prelude::*,
};

/// The prelude for bevy, but slightly modified.
mod bevy_prelude {
    pub use bevy::prelude::*;
}

mod gather;

plugin_module!(
    sync
);

mod areas;
mod controls;
mod creatures;
mod error_handling;
mod instantiate;
mod lost;
mod machines;
mod mind_control;
mod mouse;
mod physics;
mod propagate;
mod render;
//mod sync;

type FromSync = gather_types!(sync);

trait Is<T> {}
impl<T> Is<T> for T {}

const fn tester<A, B: Is<A>>() {}
//const _:() = tester::<FromSync, (i32, (f32, sync::Wow))>();

const FPS_DEBUG: bool = true;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins,
        render::plugin,
        controls::plugin,
        lost::plugin,
        creatures::plugin,
        mind_control::plugin,
        machines::plugin,
        mouse::plugin,
        physics::plugin,
        areas::plugin,
        instantiate::plugin,
        plugins_in_modules,
    ));

    if FPS_DEBUG {
        app.add_plugins(FpsOverlayPlugin {
            config: FpsOverlayConfig {
                text_config: TextFont {
                    font_size: 42.0,
                    ..default()
                },
                text_color: Srgba::GREEN.into(),
                refresh_interval: core::time::Duration::from_millis(100),
                enabled: true,
            },
        });
    }

    app.run();
}

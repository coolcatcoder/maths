use std::{
    fs::OpenOptions,
    io::{Read, Seek, Write},
    ops::Index,
};

use avian3d::prelude::RigidBody;
use bevy::{
    feathers::{
        FeathersPlugins,
        controls::{ButtonProps, button},
        dark_theme::create_dark_theme,
        theme::{ThemeBackgroundColor, ThemedText, UiTheme},
        tokens,
    },
    input_focus::tab_navigation::TabGroup,
    ui_widgets::{Activate, observe},
};
use bevy_mod_outline::OutlineVolume;

pub use crate::bevy_prelude::*;
use crate::mouse::selection::{OutlineWhileSelected, Selected};

pub fn editor(file_path: &'static str) -> impl Fn(&mut App) {
    move |app: &mut App| {
        app.add_plugins(FeathersPlugins)
            .insert_resource(UiTheme(create_dark_theme()))
            .add_systems(Update, make_develop_selectable)
            .add_systems(Startup, create_load(file_path));
    }
}

fn make_develop_selectable(
    physics: Query<Entity, (Added<RigidBody>, With<Name>)>,
    mut commands: Commands,
) {
    for entity in physics {
        // commands.entity(entity).insert((
        //     Selected::<true>(false),
        //     OutlineWhileSelected::<true> {
        //         colour: Color::srgb(1., 0., 0.),
        //         width: 3.,
        //     },
        // ));
    }
}

fn create_load(file_path: &'static str) -> impl Fn(Commands) {
    move |mut commands: Commands| {
        commands.spawn((
            Node {
                width: percent(20),
                height: percent(100),
                align_items: AlignItems::Start,
                justify_content: JustifyContent::Start,
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                row_gap: px(10),
                ..default()
            },
            TabGroup::default(),
            ThemeBackgroundColor(tokens::WINDOW_BG),
            children![(
                Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Stretch,
                    justify_content: JustifyContent::Start,
                    padding: UiRect::all(px(8)),
                    row_gap: px(8),
                    width: percent(100),
                    ..default()
                },
                children![
                    (
                        button(
                            ButtonProps::default(),
                            (),
                            Spawn((Text::new("Highlight Selected"), ThemedText))
                        ),
                        observe(
                            |_: On<Activate>,
                             selected: Query<(
                                &mut OutlineVolume,
                                &Selected<true>,
                                &OutlineWhileSelected<true>
                            )>| {
                                for (mut outline, selected, outline_while_selected) in selected {
                                    if selected.0 {
                                        outline.visible = true;
                                        outline.colour = outline_while_selected.colour;
                                        outline.width = outline_while_selected.width;
                                    }
                                }
                            }
                        )
                    ),
                    (
                        button(
                            ButtonProps::default(),
                            (),
                            Spawn((Text::new("Patch Selected"), ThemedText))
                        ),
                        observe(create_apply_patches(file_path))
                    ),
                ]
            ),],
        ));
    }
}

pub fn create_apply_patches(
    module_path: &'static str,
) -> impl Fn(On<Activate>, Query<(&Selected<true>, &Name, &Transform)>) {
    move |_: On<Activate>, selected: Query<(&Selected<true>, &Name, &Transform)>| {
        let mut patches = vec![];
        for (selected, name, transform) in selected {
            if selected.0 {
                info!("{}", name);
                let patch = (
                    name.to_string(),
                    format!(
                        "            {:?} => {{
                *transform = Transform {{
                    translation: Vec3::new({:?}, {:?}, {:?}),
                    rotation: Quat::from_array([{:?}, {:?}, {:?}, {:?}]),
                    scale: Vec3::ONE,
                }}
            }}",
                        name.as_str(),
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z,
                        transform.rotation.x,
                        transform.rotation.y,
                        transform.rotation.z,
                        transform.rotation.w,
                    ),
                );
                patches.push(patch);
            }
        }

        let path = module_path;

        let mut file = OpenOptions::new()
            .write(true)
            .read(true)
            .open(path)
            .else_error(format!("Could not open file. Path: {path}"))?;

        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .else_error("Could not read file.")?;

        let start = contents.find("fn patch");
        match start {
            None => {
                contents.push_str(
                    "
pub fn patch(names: Query<(&Name, &mut Transform), Added<Name>>) {
    for (name, mut transform) in names {
        #[allow(clippy::match_same_arms)]
        #[allow(clippy::unreadable_literal)]
        #[allow(clippy::single_match)]
        match name.as_str() {",
                );

                for (_, patch) in patches {
                    use std::fmt::Write;
                    write!(&mut contents, "\n{patch}").else_error("Failed to write to contents")?;
                }

                contents.push_str(
                    "
            _ => (),
        }
    }
}
",
                );

                info!("{contents}");
                file.rewind().else_error("Could not rewind.")?;
                file.write(contents.as_bytes())
                    .else_error("Could not save patches.")?;
            }
            Some(start_of_function) => {
                let mut brackets_open = 1;
                let mut index = start_of_function + 61;
                while brackets_open != 0 {
                    index += 1;
                    let character = *contents
                        .as_bytes()
                        .get(index)
                        .else_error("Closing bracket not found.")?
                        as char;
                    match character {
                        '{' => {
                            brackets_open += 1;
                        }
                        '}' => {
                            brackets_open -= 1;
                        }
                        _ => (),
                    }
                }

                let patch_function = contents
                    .get(start_of_function..index)
                    .else_error("Patch function range is broken.")?
                    .to_owned();

                info!("{patch_function}");

                for (name, patch) in patches {
                    match patch_function.find(&format!("\"{name}\"")) {
                        None => {
                            let last_match = patch_function
                                .find("_ =>")
                                .else_error("Could not find last match.")?;
                            contents.insert_str(
                                last_match + start_of_function - 12,
                                &format!("{patch}\n"),
                            );
                        }
                        Some(start_of_arm) => {
                            let mut brackets_open = 1;
                            let mut index = start_of_function + start_of_arm + name.len() + 7;
                            while brackets_open != 0 {
                                index += 1;
                                let character = *contents
                                    .as_bytes()
                                    .get(index)
                                    .else_error("Closing bracket not found.")?
                                    as char;
                                match character {
                                    '{' => {
                                        brackets_open += 1;
                                    }
                                    '}' => {
                                        brackets_open -= 1;
                                    }
                                    _ => (),
                                }
                            }

                            let arm = contents
                                .get((start_of_function + start_of_arm)..=index)
                                .else_error("Patch function range is broken.")?
                                .to_owned();

                            info!("{arm}");

                            contents.replace_range(
                                (start_of_function + start_of_arm - 12)..=index,
                                &patch,
                            );
                        }
                    }
                }

                //info!("{contents}");
                file.rewind().else_error("Could not rewind.")?;
                file.write(contents.as_bytes())
                    .else_error("Could not save patches.")?;
            }
        }
    }
}

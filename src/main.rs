use bevy::prelude::*;

pub mod events;
pub mod game;
pub mod main_menu;
mod systems;

use crate::game::*;
use crate::main_menu::*;
use crate::systems::*;

fn main() {
    if 1 == 1 && 0 == 0 {
        App::new()
            .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
            //.insert_resource(ClearColor(Color::rgb(9.0/255.0, 13.0/255.0, 30.0/255.0)))
            .add_state::<AppState>()
            .add_startup_system(spawn_camera)
            .add_startup_system(spawn_background)
            .add_plugins(
                DefaultPlugins
                    .set(ImagePlugin::default_nearest())
                    .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Skeletal Defender".into(),
                            resolution: (960.0f32, 540.0f32).into(),
                            // wasm \/
                            fit_canvas_to_parent: true,
                            prevent_default_event_handling: false,
                            ..default()
                        }),
                        ..default()
                    }),
            )
            .add_plugin(MainMenuPlugin)
            .add_plugin(GamePlugin)
            .run();
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
}

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

mod slime;
mod environment;
mod game_state;
mod camera;

use slime::SlimePlugin;
use environment::EnvironmentPlugin;
use game_state::GameStatePlugin;
use camera::CameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Slime Game 3D".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins((
            SlimePlugin,
            EnvironmentPlugin,
            GameStatePlugin,
            CameraPlugin,
        ))
        .add_systems(Startup, setup_physics)
        .add_systems(Update, handle_input)
        .run();
}

fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec3::new(0.0, -9.81, 0.0);
}

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if keyboard.pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

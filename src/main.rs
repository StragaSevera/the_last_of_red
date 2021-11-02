mod platform;
mod player;

use bevy::prelude::*;

use crate::{
    platform::PlatformBundle,
    player::{player_input_system, player_move_system, PlayerBundle},
};

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(DefaultPlugins)
        .add_plugin(MainPlugin)
        .run();
}

pub struct MainPlugin;

impl Plugin for MainPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(player_input_system.system())
            .add_system(player_move_system.system());
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(PlayerBundle::new(&mut materials));
    commands.spawn_bundle(PlatformBundle::new(
        Transform::from_xyz(30.0, -80.0, 0.0),
        &mut materials,
    ));
}

mod player;

use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};

use bevy_ecs_ldtk::prelude::*;

use crate::player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(LevelSelection::index(0))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, (setup_camera, spawn_map))
        .add_plugins(PlayerPlugin)
        .add_plugins(LdtkPlugin)
        .run();
}

fn setup_camera(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scale: 0.5,
                ..OrthographicProjection::default_2d()
            }),
            Transform::from_xyz(1280.0 / 4.0, 720.0 / 4.0, 0.0),
    ));
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("firstlevel.ldtk").into(),
        ..Default::default()
    });
}

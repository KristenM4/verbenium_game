use bevy::prelude::*;
use crate::player::PlayerPlugin;

mod player;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_systems(Startup, setup_camera)
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
            Camera2d,
            Projection::Orthographic(OrthographicProjection {
                scale: 0.25,
                ..OrthographicProjection::default_2d()
            }),
    ));
}


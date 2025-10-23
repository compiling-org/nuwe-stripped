use bevy::prelude::*;
use nuwe_rust::ImmersiveVjPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Immersive VJ System - Node-based Audio-Visual Performance".into(),
                    resolution: (1920, 1080).into(),
                    present_mode: bevy::window::PresentMode::AutoVsync,
                    ..default()
                }),
                ..default()
            }),
            ImmersiveVjPlugin,
        ))
        .run();
}

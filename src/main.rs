use bevy::prelude::*;

mod game;

fn main() {
    App::new()
        .add_plugins(
            (DefaultPlugins
                .set(WindowPlugin {
                        primary_window: Some(Window {
                            title: "Spaceglider".to_string(),
                            ..Default::default()
                        }),
                    ..Default::default()
                }),
                game::Spaceglider
            )
        )
        .run();
}

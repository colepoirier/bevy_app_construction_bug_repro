use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Test Title".to_string(),
            width: 1920.0,
            height: 1080.0,
            ..Default::default()
        })
        .run();
}

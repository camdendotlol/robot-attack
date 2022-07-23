use bevy::prelude::*;
use robot_attack::GamePlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Robot Attack".to_string(),
            width: 840.0,
            height: 630.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(GamePlugin)
        .run();
}

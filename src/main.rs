mod hello_plugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(hello_plugin::HelloPlugin)
        .run();
}


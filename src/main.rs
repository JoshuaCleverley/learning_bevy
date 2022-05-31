use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(hello_world)
        .run();
}

// First system!
fn hello_world() {
    println!("Hello, world!");
}
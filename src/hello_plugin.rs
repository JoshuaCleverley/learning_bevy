use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(add_people)
            .add_system(hello_world)
            .add_system(greet_people);
    }
}

fn hello_world() {
    println!("Hello, world!");
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}

fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello, {}", name.0);
    }
}
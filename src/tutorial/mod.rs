use bevy::prelude::*;

//systems
fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Bruce Banner".to_string())));
    commands.spawn((Person, Name("Tony Stark".to_string())));
    commands.spawn((Person, Name("Steven Strange".to_string())));
}

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!", name.0);
        }
    }
}

fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Steven Strange" {
            name.0 = "Dr. Steven Strange".to_string();
            break; // We don’t need to change any other names
        }
    }
}

//Resource
#[derive(Resource)]
struct GreetTimer(Timer);

// Components
#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Plugins
pub struct HelloPlugin;
impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_systems(Startup, add_people)
            .add_systems(Update, (update_people, greet_people).chain());
    }
}
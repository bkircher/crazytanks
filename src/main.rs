use bevy::prelude::*;

struct Tank;

struct Name(String);

fn add_tanks(commands: &mut Commands) {
    commands
        .spawn((Tank, Name("Elaina Eradicator".to_string())))
        .spawn((Tank, Name("Teddy Terminator".to_string())))
        .spawn((Tank, Name("Slinghot Sammy".to_string())));
}

struct GreetTimer(Timer);

fn greet_tanks(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Tank>>) {
    if !timer.0.tick(time.delta_seconds()).just_finished() {
        return;
    }

    for name in query.iter() {
        println!("Hello {}!", name.0)
    }
}

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(add_tanks.system())
            .add_system(greet_tanks.system());
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TankPlugin)
        .run();
}

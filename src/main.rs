use bevy::prelude::*;

#[derive(Bundle)]
struct Tank {}

struct Materials {
    body: Handle<ColorMaterial>,
}

fn spawn_tank(commands: &mut Commands, materials: Res<Materials>) {
    commands
        .spawn(SpriteBundle {
            material: materials.body.clone(),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Tank {});
}

fn tank_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut tank_positions: Query<&mut Transform, With<Tank>>,
) {
    for mut transform in tank_positions.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            transform.translation.x -= 3.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            transform.translation.x += 3.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            transform.translation.y += 3.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            transform.translation.y -= 3.0;
        }
    }
}

fn setup(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn(Camera2dBundle::default())
        .insert_resource(Materials {
            body: materials.add(Color::rgb(1.0, 0.0, 0.52).into()),
        });
}

fn main() {
    App::build()
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup", SystemStage::single(spawn_tank.system()))
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_system(tank_movement.system())
        .add_plugins(DefaultPlugins)
        .run();
}

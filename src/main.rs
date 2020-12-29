use bevy::prelude::*;

struct Tank {
    speed: f32,
}

struct Name(String);

fn add_tanks(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        // .spawn(SpriteBundle {
        //     material: materials.add(Color::rgb(0.5, 0.3, 1.0).into()),
        //     transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
        //     sprite: Sprite::new(Vec2::new(40.0, 30.0)),
        //     ..Default::default()
        // })
        // .with((Tank { speed: 10.0 }, Name("Elaina Eradicator".to_string())))
        // .with(Collider::Tank)
        // .spawn(SpriteBundle {
        //     material: materials.add(Color::rgb(0.6, 0.2, 1.0).into()),
        //     transform: Transform::from_translation(Vec3::new(0.0, -185.0, 0.0)),
        //     sprite: Sprite::new(Vec2::new(40.0, 30.0)),
        //     ..Default::default()
        // })
        // .with((Tank { speed: 0.0 }, Name("Teddy Terminator".to_string())))
        // .with(Collider::Tank)
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.3, 0.2, 5.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -245.0, 0.0)),
            sprite: Sprite::new(Vec2::new(40.0, 30.0)),
            ..Default::default()
        })
        .with((Tank { speed: 500.0 }, Name("Slinghot Sammy".to_string())))
        .with(Collider::Tank);
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

enum Collider {
    Solid,
    Tank,
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Add the game's entities to the world
    commands
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default());

    // Add some walls
    let wall_material = materials.add(Color::rgb(0.8, 0.8, 0.8).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(900.0, 600.0);
    commands
        // Left
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(-bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // Right
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(bounds.x / 2.0, 0.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness, bounds.y + wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // Top
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness + bounds.x, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid)
        // Bottom
        .spawn(SpriteBundle {
            material: wall_material.clone(),
            transform: Transform::from_translation(Vec3::new(0.0, -bounds.y / 2.0, 0.0)),
            sprite: Sprite::new(Vec2::new(wall_thickness + bounds.x, wall_thickness)),
            ..Default::default()
        })
        .with(Collider::Solid);
}

fn tank_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Tank, &mut Transform)>,
) {
    for (tank, mut transform) in query.iter_mut() {
        let mut direction = Vec2::new(0.0, 0.0);
        if keyboard_input.pressed(KeyCode::Left) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            direction.x += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            direction.y -= 1.0;
        }
        let translation = &mut transform.translation;
        // Move the tank horizontally
        translation.x += time.delta_seconds() * direction.x * tank.speed;
        // Move the tank vertically
        translation.y += time.delta_seconds() * direction.y * tank.speed;

        // TODO: bound within walls, not on other tank
        println!("tank_movement")
    }
}

pub struct TankPlugin;

impl Plugin for TankPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(GreetTimer(Timer::from_seconds(2.0, true)))
            .add_startup_system(setup.system())
            .add_startup_system(add_tanks.system())
            .add_system(greet_tanks.system())
            //.add_system(tank_movement.system())
            ;
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(TankPlugin)
        .run();
}

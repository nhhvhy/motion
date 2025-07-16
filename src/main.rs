use bevy::prelude::*;
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

#[derive(Component)]
struct Icon;

#[derive(Component)]
struct JumpTimer {
    timer: Timer,
    dir: f32,
    steps_remaining: i32
}


fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default()                         
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_wireframe)
        .add_systems(Update, (icon_on_click, cube_jump))
        .run();
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>
) {
    if keyboard.just_pressed(KeyCode::Space) {
        println!("Wireframe Toggled");
        wireframe_config.global = !wireframe_config.global;
    }
}

fn icon_on_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,
    query: Query<Entity, With<Icon>>
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("LMB clicked");

        if let Ok(icon_entity) = query.single() {
            commands.entity(icon_entity).insert(
                JumpTimer {
                timer: Timer::from_seconds(0.01, TimerMode::Repeating),
                dir: 1.0,
                steps_remaining: 12
            });
        }
    }
}

fn cube_jump(
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &mut JumpTimer), With<Icon>>,
    mut commands: Commands
) {
    for (entity, mut transform, mut jumptimer) in &mut query {
        jumptimer.timer.tick(time.delta());
        if jumptimer.timer.just_finished() {
            transform.translation.y += 10.0 * jumptimer.dir;
            jumptimer.steps_remaining -= 1;

            if jumptimer.steps_remaining == 0 {
                jumptimer.dir = jumptimer.dir * -1.0;
                jumptimer.steps_remaining = 12;
                if jumptimer.dir == 1.0 {
                    commands.entity(entity).remove::<JumpTimer>();
                }
            }
        }
    } 
}


fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

    let cube = meshes.add(Rectangle::new(50.0, 50.0));
    commands.spawn((
        Mesh2d(cube),
        MeshMaterial2d(materials.add(Color::hsv(0., 1., 1.))),
        Transform::from_xyz(-400.0, -100.0, 0.0),
        Icon
    ));

    let ground = meshes.add(Rectangle::new(10000.0, 500.0));
    commands.spawn((
        Mesh2d(ground),
        MeshMaterial2d(materials.add(Color::hsv(0.3, 0.4, 0.4))),
        Transform::from_xyz(200.0, -375.0, 0.0)
    ));
}
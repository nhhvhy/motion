#[warn(unused_imports)]
use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume}
};

use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

const CUBE_ICON_WIDTH: f32 = 50.0;
const CUBE_ICON_HEIGHT: f32 = 50.0;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Icon;

#[derive(Component)]
struct JumpTimer {
    timer: Timer,
    dir: f32,
    steps_remaining: i32
}

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Collider)]
struct Ground;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default()                         
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_wireframe)
        .add_event::<CollisionEvent>()
        .add_systems(Update, (icon_on_click, cube_jump, collision_check))
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
    query: Query<(Entity, Option<&JumpTimer>), With<Icon>>
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("LMB clicked");

        if let Ok((icon_entity, jump_timer )) = query.single() {
            if jump_timer.is_some() {
                return
            } else {
                commands.entity(icon_entity).insert(
                    JumpTimer {
                    timer: Timer::from_seconds(0.01, TimerMode::Repeating),
                    dir: 1.0,
                    steps_remaining: 12
                });
            }
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

fn collision_check (
    icon_query: Single<&Transform, With<Icon>>,
    collider_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>
) {
    let icon_transform = icon_query.into_inner();

    for (_collider_entity, collider_transform) in &collider_query {
        let collision = cube_collision(
            Aabb2d::new(
                icon_transform.translation.truncate(),
                icon_transform.scale.truncate() / 2.
            ),
            Aabb2d::new(
                collider_transform.translation.truncate(),
                collider_transform.scale.truncate() / 2.
            )
        );

        if collision {
             collision_events.write_default();
        }
    }
}

fn cube_collision (
    cube: Aabb2d,
    bounding_box: Aabb2d
) -> bool {

    if !cube.intersects(&bounding_box) {
        return false;
    } else {
        return true;
    }

    
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    commands.spawn(Camera2d);

    let cube = meshes.add(Rectangle::new(CUBE_ICON_WIDTH, CUBE_ICON_HEIGHT));
    commands.spawn((
        Mesh2d(cube),
        MeshMaterial2d(materials.add(Color::hsv(0., 1., 1.))),
        Transform::from_xyz(-400.0, -100.0, 0.0),
        Velocity(Vec2::new(1.0, 0.0)),
        Icon
    ));

    let ground = meshes.add(Rectangle::new(10000.0, 500.0));
    commands.spawn((
        Mesh2d(ground),
        MeshMaterial2d(materials.add(Color::hsv(0.3, 0.4, 0.4))),
        Transform::from_xyz(200.0, -375.0, 0.0),
        Ground
    ));
}
#[warn(unused_imports)]
use bevy::{
    prelude::*,
    math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume}
};

use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

const CUBE_ICON_WIDTH: f32 = 50.0;
const CUBE_ICON_HEIGHT: f32 = 50.0;

const MIN_ICON_Y_VELOCITY: f32 = -100.0;
const MAX_ICON_Y_VELOCITY: f32 = 100.0;
const GRAVITY_ACCEL_RATE: f32 = -10.0;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component)]
struct Icon;

#[derive(Event, Default)]
struct CollisionEvent;

#[derive(Event, Default)]
struct ClickEvent;

#[derive(Component, Default)]
struct Collider;

#[derive(Component)]
#[require(Collider)]
struct Ground;

#[derive(Component)]
struct Gravity;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Wireframe2dPlugin::default()                         
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, toggle_wireframe)
        .add_event::<ClickEvent>()
        .add_event::<CollisionEvent>()
        .add_systems(Update, (icon_on_click, cube_jump, collision_check))
        .add_systems(Update, (apply_gravity, apply_velocity))
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
    mut click_events: EventWriter<ClickEvent>
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        println!("LMB clicked");
        click_events.write_default();
    }
}

fn apply_velocity (
    mut query: Query<(&mut Transform, &mut Velocity, Option<&Icon>)>,
    time: Res<Time>
) {
    for (mut transform, mut velocity, icon) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();

        if icon.is_some() {
            velocity.y = velocity.y.clamp(MIN_ICON_Y_VELOCITY, MAX_ICON_Y_VELOCITY);
        }
    }
}

fn apply_gravity (
    mut query: Query<(&mut Velocity, &Gravity)>,
    time: Res<Time>
) {
    for (mut velocity, _) in &mut query {
        velocity.y += GRAVITY_ACCEL_RATE * time.delta_secs();
    }
}

fn cube_jump(
    query: Single<(Entity, &mut Velocity), With<Icon>>,
    mut click_events: EventReader<ClickEvent>,
    collision_events: EventReader<CollisionEvent>,
) {
    let (_, mut icon_velocity) = query.into_inner();
    
    if !click_events.is_empty() {
        click_events.clear();

        if !collision_events.is_empty() {
            icon_velocity.y = 5.0;
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
        Icon,
        Gravity
    ));

    let ground = meshes.add(Rectangle::new(10000.0, 500.0));
    commands.spawn((
        Mesh2d(ground),
        MeshMaterial2d(materials.add(Color::hsv(0.3, 0.4, 0.4))),
        Transform::from_xyz(200.0, -375.0, 0.0),
        Ground
    ));
}
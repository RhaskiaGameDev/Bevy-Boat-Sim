use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::render_resource::Texture;

pub const BG_COLOR: Color = Color::rgb(0.3, 0.65, 1.);
pub const MAX_SPEED: f32 = 200.;
pub const ACCEL: f32 = 30.;
pub const SLOW: f32 = 5.;

fn main()
{
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(BG_COLOR))

        .add_system(boat_movement)
        .add_startup_system(setup)
        .run();
}

#[derive(Component)]
struct Boat
{
    velocity: f32,
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("boat.png"),
            transform: Transform::from_xyz(100., 0., 0.)
                .with_scale(Vec3{ x: 2., y: 2., z: 2. }),
            ..default()
        },
    Boat {velocity: 0.}));
}


fn boat_movement(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut sprite_position: Query<(&mut Transform, &mut Boat)>)
{
    let mut boat = sprite_position.single_mut();

    if keys.pressed(KeyCode::A)
    {
        if boat.1.velocity > -MAX_SPEED
        { boat.1.velocity -= 10.; }
    }
    if keys.pressed(KeyCode::D)
    {
        if boat.1.velocity < MAX_SPEED
        { boat.1.velocity += 10.; }
    }

    if !keys.pressed(KeyCode::D) && !keys.pressed(KeyCode::A)
    {
        if boat.1.velocity > 0.
        {
            boat.1.velocity -= SLOW;
            if boat.1.velocity < 0. { boat.1.velocity = 0.; }
        }
        if boat.1.velocity < 0.
        {
            boat.1.velocity += SLOW;
            if boat.1.velocity > 0. { boat.1.velocity = 0.; }
        }
    }

    boat.0.translation.x += time.delta_seconds() * boat.1.velocity;
    if boat.1.velocity != 0.
    {
        boat.0.rotation = match boat.1.velocity < 0.
        {
            true => Quat::from_rotation_y(std::f32::consts::PI),
            false => Quat::default(),
        };
    }
}
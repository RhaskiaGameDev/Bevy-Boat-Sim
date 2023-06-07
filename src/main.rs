use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::render::render_resource::{Extent3d, Texture, TextureDimension, TextureFormat, TextureUsages};
use bevy::window::PrimaryWindow;

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
        .add_system(camera_zoom)
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
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>)
{
    let boat_text = asset_server.load("boat.png");
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            texture: boat_text.clone(),
            transform: Transform::from_xyz(100., 0., 0.)
                .with_scale(Vec3{ x: 1., y: 1., z: 1. }),
            ..default()
        },
    Boat {velocity: 0.}));

    let size = Extent3d {
        width: 480,
        height: 128,
        ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: bevy::render::render_resource::TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8Unorm,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    // fill image.data with zeroes
    image.resize(size);
    println!("{}", image.data.len());
    for x in 0..size.width
    {
        for y in 0.. size.height
        {
            let index = ((y * size.width + x) * 4) as usize;

            let waves = (x as f32 * 6.).sin() * 2. + 10.
            + (x as f32 * 12.).sin() * 2.;

            let above_waves = y as f32 > waves;

            image.data[index] = 0;
            image.data[index + 1] = 41;
            image.data[index + 2] = 255;
            image.data[index + 3] = above_waves as u8 * 255;
        }
    }

    let image_handle = images.add(image);

    commands.spawn(
        SpriteBundle {
            texture: image_handle.clone(),
            transform: Transform::from_xyz(0., -60., 0.)
                .with_scale(Vec3{ x: 1., y: 1., z: 1. }),
            ..default()
        });
}

fn camera_zoom(
    mut camera: Query<&mut OrthographicProjection>,
    window_query: Query<&Window, With<PrimaryWindow>>)
{
    let window = window_query.get_single().unwrap();

    camera.single_mut().scale = 480. / window.width();
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
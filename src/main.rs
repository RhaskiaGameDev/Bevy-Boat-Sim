// use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
// use bevy::render::render_resource::{Extent3d, Texture, TextureDimension, TextureFormat};
//
// pub const BG_COLOR: Color = Color::rgb(0.3, 0.65, 1.);
// pub const MAX_SPEED: f32 = 200.;
// pub const ACCEL: f32 = 30.;
// pub const SLOW: f32 = 5.;
//
// fn main()
// {
//     App::new()
//         .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
//         .insert_resource(ClearColor(BG_COLOR))
//
//         .add_system(boat_movement)
//         .add_startup_system(setup)
//         .run();
// }
//
// #[derive(Component)]
// struct Boat
// {
//     velocity: f32,
// }
//
// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>)
// {
//     let boat_text = asset_server.load("boat.png");
//     commands.spawn(Camera2dBundle::default());
//     commands.spawn((
//         SpriteBundle {
//             texture: boat_text.clone(),
//             transform: Transform::from_xyz(100., 0., 0.)
//                 .with_scale(Vec3{ x: 2., y: 2., z: 2. }),
//             ..default()
//         },
//     Boat {velocity: 0.}));
//
//     commands.spawn((
//         SpriteBundle {
//             texture: boat_text.clone(),
//             transform: Transform::from_xyz(100., 0., 0.)
//                 .with_scale(Vec3{ x: 2., y: 2., z: 2. }),
//             ..default()
//         },
//         Boat {velocity: 0.}));
//
//     let (width, height) = (256_u32, 256);
//     let mut bytes = Vec::with_capacity((width * height * 4) as usize );
//     for _y in 0..height {
//         for _x in 0..width {
//             bytes.push(0xff);
//             bytes.push(0x00);
//             bytes.push(0x00);
//             bytes.push(0xff);
//         }
//     }
//
//
//
//     let texture = Image::new(
//         Extent3d { width , height , depth_or_array_layers: 0 },
//         TextureDimension::D1,
//         vec![],
//         TextureFormat::R8Unorm);
//
//     commands.spawn(
//         SpriteBundle {
//             texture: texture,
//             transform: Transform::from_xyz(100., 0., 0.)
//                 .with_scale(Vec3{ x: 2., y: 2., z: 2. }),
//             ..default()
//         });
// }
//
//
// fn boat_movement(
//     time: Res<Time>,
//     keys: Res<Input<KeyCode>>,
//     mut sprite_position: Query<(&mut Transform, &mut Boat)>)
// {
//     let mut boat = sprite_position.single_mut();
//
//     if keys.pressed(KeyCode::A)
//     {
//         if boat.1.velocity > -MAX_SPEED
//         { boat.1.velocity -= 10.; }
//     }
//     if keys.pressed(KeyCode::D)
//     {
//         if boat.1.velocity < MAX_SPEED
//         { boat.1.velocity += 10.; }
//     }
//
//     if !keys.pressed(KeyCode::D) && !keys.pressed(KeyCode::A)
//     {
//         if boat.1.velocity > 0.
//         {
//             boat.1.velocity -= SLOW;
//             if boat.1.velocity < 0. { boat.1.velocity = 0.; }
//         }
//         if boat.1.velocity < 0.
//         {
//             boat.1.velocity += SLOW;
//             if boat.1.velocity > 0. { boat.1.velocity = 0.; }
//         }
//     }
//
//     boat.0.translation.x += time.delta_seconds() * boat.1.velocity;
//     if boat.1.velocity != 0.
//     {
//         boat.0.rotation = match boat.1.velocity < 0.
//         {
//             true => Quat::from_rotation_y(std::f32::consts::PI),
//             false => Quat::default(),
//         };
//     }
// }

//! A shader that uses the GLSL shading language.

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::TypeUuid,
    render::{
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
        },
    },
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_startup_system(setup)
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // cube
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        material: materials.add(CustomMaterial {
            color: Color::BLUE,
            color_texture: Some(asset_server.load("player.png")),
            alpha_mode: AlphaMode::Blend,
        }),
        ..default()
    });


    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 0., 5.0),
        ..default()
    });
}

// This is the struct that will be passed to your shader
#[derive(AsBindGroup, Clone, TypeUuid)]
#[uuid = "4ee9c363-1124-4113-890e-199d81b00281"]
pub struct CustomMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
    alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
/// When using the GLSL shading language for your shader, the specialize method must be overriden.
impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        "custom_material.vert".into()
    }

    fn fragment_shader() -> ShaderRef {
        "custom_material.frag".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}
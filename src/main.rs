use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

const GRID_SIZE: u32 = 32;
const LINE_WIDTH: u32 = 3;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
) {
    let bg_color = "#dcd8c0";
    let fg_color = "#454138";

    commands.spawn(Camera2dBundle::default());

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        transform: Transform::default().with_scale(Vec3::splat(1024.)),
        material: materials.add(LineMaterial {
            grid_color: Color::hex(fg_color).unwrap(),
            tick_color: Color::hex(bg_color).unwrap(),
            grid_size: GRID_SIZE as i32,
            line_width: LINE_WIDTH as i32,
            offset: Vec2::new(-70., 120.),
        }),
        ..default()
    });
}

fn main() {
    let bg_color = "#dcd8c0";

    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<LineMaterial>::default()))
        .insert_resource(ClearColor(Color::hex(bg_color).unwrap()))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct LineMaterial {
    #[uniform(0)]
    grid_color: Color,
    #[uniform(1)]
    tick_color: Color,
    #[uniform(2)]
    grid_size: i32,
    #[uniform(3)]
    line_width: i32,
    #[uniform(4)]
    offset: Vec2,
}

impl Material2d for LineMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        "line_material.wgsl".into()
    }
}

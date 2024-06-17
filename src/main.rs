use bevy::{
    prelude::*,
    render::render_resource::AsBindGroup,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
    window::PrimaryWindow,
};

const GRID_SIZE: u32 = 16;
const LINE_WIDTH: u32 = 2;

#[derive(Component)]
struct Grid;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<LineMaterial>>,
    mut color_mat: ResMut<Assets<ColorMaterial>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
) {
    let bg_color = "#dcd8c0";
    let fg_color = "#454138";

    let Ok(window) = primary_window.get_single() else {
        return;
    };

    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(Rectangle::new(window.width(), window.height()))
                .into(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            material: materials.add(LineMaterial {
                grid_color: Color::hex(fg_color).unwrap(),
                tick_color: Color::hex(bg_color).unwrap(),
                grid_size: GRID_SIZE as i32,
                line_width: LINE_WIDTH as i32,
                offset: Vec2::new(0., 0.),
            }),
            ..default()
        },
        Grid,
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::new(100., 50.)).into(),
        transform: Transform::from_xyz(0., 0., 2.0),
        material: color_mat.add(Color::RED),
        ..default()
    });
}

fn camera2d_pan(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut line_material: ResMut<Assets<LineMaterial>>,
    mut grid_query: Query<(&Grid, &Handle<LineMaterial>, &mut Transform), Without<Camera2d>>,
    mut cam_query: Query<(&Camera, &GlobalTransform, &Camera2d, &mut Transform)>,
    mut last_pos: Local<Option<Vec2>>,
) {
    const MOUSE_BUTTONS: [MouseButton; 2] = [MouseButton::Middle, MouseButton::Right];

    let Ok(mut window) = primary_window.get_single_mut() else {
        return;
    };

    let current_pos = match window.cursor_position() {
        Some(c) => Vec2::new(c.x, -c.y),
        None => return,
    };
    let last_pos_safe = last_pos.unwrap_or(current_pos);

    let delta_device_pixels = current_pos - last_pos_safe;

    let Ok((cam, global_transform, _cam2d, mut cam_transform)) = cam_query.get_single_mut() else {
        return;
    };
    let Ok((_grid, line_material_handle, mut grid_transform)) = grid_query.get_single_mut() else {
        return;
    };
    let Some(line_material) = line_material.get_mut(line_material_handle.id()) else {
        return;
    };

    if !MOUSE_BUTTONS
        .iter()
        .any(|btn| mouse_buttons.pressed(*btn) && !mouse_buttons.just_pressed(*btn))
    {
        window.cursor.icon = CursorIcon::Default;
        *last_pos = Some(current_pos);
        return;
    }

    window.cursor.icon = CursorIcon::Grabbing;

    let current_pos_world = cam
        .viewport_to_world_2d(global_transform, current_pos)
        .unwrap_or(Vec2::default())
        .extend(0.);
    let last_pos_world = cam
        .viewport_to_world_2d(global_transform, last_pos_safe)
        .unwrap_or(Vec2::default())
        .extend(0.);

    let delta_device_world = current_pos_world - last_pos_world;

    cam_transform.translation += Vec3::new(
        -delta_device_world.x,
        delta_device_world.y,
        delta_device_world.z,
    );
    grid_transform.translation += Vec3::new(
        -delta_device_world.x,
        delta_device_world.y,
        delta_device_world.z,
    );

    line_material.offset += Vec2::new(-delta_device_pixels.x, delta_device_pixels.y);

    *last_pos = Some(current_pos);
}

fn main() {
    let bg_color = "#dcd8c0";

    App::new()
        .add_plugins((DefaultPlugins, Material2dPlugin::<LineMaterial>::default()))
        .insert_resource(ClearColor(Color::hex(bg_color).unwrap()))
        .add_systems(Startup, setup)
        .add_systems(Update, camera2d_pan)
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

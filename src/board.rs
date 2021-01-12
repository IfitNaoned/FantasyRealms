use bevy::prelude::*;

fn create_board(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let wood_texture_handle = asset_server.load("models/wood.png");
    let wood_material_handle = materials.add(StandardMaterial {
        albedo_texture: Some(wood_texture_handle.clone()),
        shaded: false,
        ..Default::default()
    });

    let metal_black_texture_handle = asset_server.load("models/metal_black.png");
    let metal_black_material_handle = materials.add(StandardMaterial {
        albedo_texture: Some(metal_black_texture_handle.clone()),
        shaded: false,
        ..Default::default()
    });

    let green_carpet_texture_handle = asset_server.load("models/green_carpet.png");
    let green_carpet_material_handle = materials.add(StandardMaterial {
        albedo_texture: Some(green_carpet_texture_handle.clone()),
        shaded: false,
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 36.0 })),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            material: green_carpet_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-18., -9.8, -18.));
                transform.apply_non_uniform_scale(Vec3::new(1., 10., 1.));
                transform
            },
            material: wood_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-18., -9.8, 18.));
                transform.apply_non_uniform_scale(Vec3::new(1., 10., 1.));
                transform
            },
            material: wood_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(18., -9.8, -18.));
                transform.apply_non_uniform_scale(Vec3::new(1., 10., 1.));
                transform
            },
            material: wood_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(18., -9.8, 18.));
                transform.apply_non_uniform_scale(Vec3::new(1., 10., 1.));
                transform
            },
            material: wood_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(-18., -0.4, 0.));
                transform.apply_non_uniform_scale(Vec3::new(1., 1., 36.));
                transform
            },
            material: metal_black_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(18., -0.4, 0.));
                transform.apply_non_uniform_scale(Vec3::new(1., 1., 36.));
                transform
            },
            material: metal_black_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0., -0.4, 18.));
                transform.apply_non_uniform_scale(Vec3::new(36., 1., 1.));
                transform
            },
            material: metal_black_material_handle.clone(),
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0., -0.4, -18.));
                transform.apply_non_uniform_scale(Vec3::new(36., 1., 1.));
                transform
            },
            material: metal_black_material_handle.clone(),
            ..Default::default()
        });
}

fn create_ia_face(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let _scenes: Vec<HandleUntyped> = asset_server.load_folder("models/").unwrap();
    let monkey_handle = asset_server.get_handle("models/Monkey.gltf#Mesh0/Primitive0");

    let material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.8, 0.7, 0.6),
        ..Default::default()
    });

    commands
        .spawn(PbrBundle {
            mesh: monkey_handle.clone(),
            material: material_handle.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(20.0, 5., 0.));
                transform.rotate(Quat::from_rotation_y(-std::f32::consts::PI / 2.));
                transform.apply_non_uniform_scale(Vec3::new(3., 3., 3.));
                transform
            },
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: monkey_handle.clone(),
            material: material_handle.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0., 5., 20.));
                transform.rotate(Quat::from_rotation_y(-(std::f32::consts::PI)));
                transform.apply_non_uniform_scale(Vec3::new(3., 3., 3.));
                transform
            },
            ..Default::default()
        })
        .spawn(PbrBundle {
            mesh: monkey_handle.clone(),
            material: material_handle.clone(),
            transform: {
                let mut transform = Transform::from_translation(Vec3::new(0., 5., -20.));
                transform.rotate(Quat::from_rotation_y(2. * std::f32::consts::PI));
                transform.apply_non_uniform_scale(Vec3::new(3., 3., 3.));
                transform
            },
            ..Default::default()
        });
}

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_board.system())
            .add_startup_system(create_ia_face.system());
    }
}

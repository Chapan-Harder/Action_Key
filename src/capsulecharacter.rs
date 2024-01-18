use bevy::prelude::*;
use bevy_third_person_camera::*;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, characterspawn)
        .add_systems(Update, movement);
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Movement(f32);

fn movement(mut player_query: Query<(&mut Transform, &Movement), With<Player>>, camera_query: Query<&Transform, (With<Camera3d>, Without<Player>)>, keys: Res<Input<KeyCode>>, time: Res<Time>){
    for (mut player_tarnsform, player_speed) in player_query.iter_mut(){
        let cam = match camera_query.get_single(){
            Ok(c) => c,
            Err(e) => Err(format!("Error camera: {}", e)).unwrap()
        };

        // let cam = Transform::from_xyz(0.0, 0.0, 0.0);

        let mut direction = Vec3::ZERO;

        // Forward
        if keys.pressed(KeyCode::W){
            direction += cam.forward();
        }

        // Backward
        if keys.pressed(KeyCode::S){
            direction += cam.back();
        }

        // Left
        if keys.pressed(KeyCode::A){
            direction += cam.left();
        }

        // Right
        if keys.pressed(KeyCode::D){
            direction += cam.right();
        }

        direction.y = 0.0;
        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_tarnsform.translation += movement;

        // Rotate player
        if direction.length_squared() > 0.0{
            player_tarnsform.look_to(direction, Vec3::Y);
        }
    }
}

fn characterspawn(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>, asset_server: Res<AssetServer>){
    let capsule_character: Handle<Mesh> = meshes.add(shape::Capsule{radius: 0.3, rings: 32, depth: 0.5, latitudes: 10, longitudes: 20, uv_profile: shape::CapsuleUvProfile::Fixed}.into());
    let capsule_hat: Handle<Mesh> = meshes.add(shape::Box::new(0.4, 0.1, 0.4).into());
    let transform_character: Vec3 = Vec3{x: 0.0, y: 0.55, z: 0.0};
    let transform_hat: Vec3 = Vec3{x: 0.0, y: 0.35, z: -0.3};

    // Capsule Character

    let player: (MaterialMeshBundle<StandardMaterial>, _, _, _, _) = ((
        PbrBundle{

        mesh: capsule_character.clone(),
        material: materials.add(StandardMaterial{
            base_color_texture: Some(asset_server.load("textures/Yellow ground test.png")),
            // base_color: Color::rgb_u8(150, 150, 0).into(),
            perceptual_roughness: 0.5,
            ..Default::default()
        }),
        transform: Transform::from_translation(transform_character),
        ..Default::default()
    }),
    Movement(3.0),
    Player,
    ThirdPersonCameraTarget,
    Name::new("Cool_Character")
    );

    commands.spawn(player).with_children(|parent|{
        parent.spawn(PbrBundle{
            mesh: capsule_hat.clone(),
            material: materials.add(StandardMaterial{
                base_color_texture: Some(asset_server.load("textures/Red ground test.png")),
                perceptual_roughness: 0.5,
                ..Default::default()
            }),
            transform: Transform::from_translation(transform_hat),
            ..Default::default()
        });
    });


    /*commands.spawn(PbrBundle{
        mesh: capsule_character.clone(),
        material: materials.add(StandardMaterial{
            base_color_texture: Some(asset_server.load("textures/Yellow ground test.png")),
            // base_color: Color::rgb_u8(150, 150, 0).into(),
            perceptual_roughness: 0.5,
            ..Default::default()
        }),
        transform: Transform::from_translation(transform_character),
        ..Default::default()
    }).with_children(|parent|{
        parent.spawn(PbrBundle{
            mesh: capsule_hat.clone(),
            material: materials.add(StandardMaterial{
                base_color_texture: Some(asset_server.load("textures/Red ground test.png")),
                perceptual_roughness: 0.5,
                ..Default::default()
            }),
            transform: Transform::from_translation(transform_hat),
            ..Default::default()
        });
    })
    .insert(Movement{walk_forward: 3.0, walk_backward: 1.5, walk_rl: 2.0},
    ThirdPersonCameraTarget,);*/
}
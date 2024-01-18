use bevy::{pbr::CascadeShadowConfigBuilder, prelude::*};

pub struct SceneLoaderPlugins;

impl Plugin for SceneLoaderPlugins{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scenesystem);
    }
}

fn scenesystem(mut commands: Commands, asset_server: Res<AssetServer>){
    // Ground
    let ground_gltf: Handle<Scene> = asset_server.load("ground.glb#Scene0");
    commands.spawn((SceneBundle{
        scene: ground_gltf,
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..Default::default()
    },
    Name::new("Ground")));

    // Directional light or (SUN Light)
    commands.spawn((DirectionalLightBundle{
        directional_light: DirectionalLight{
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform{
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(23.0),
            ..Default::default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }.into(),
        ..Default::default()
    },
    Name::new("Power Of the Sun")));
}
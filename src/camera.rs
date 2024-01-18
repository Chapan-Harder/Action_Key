use bevy::prelude::*;

use bevy_third_person_camera::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, look_at_the_character);
    }
}

fn look_at_the_character(mut commands: Commands){
    // Camera
    let camera: (Camera3dBundle, _, _) = (Camera3dBundle{
        transform: Transform::from_xyz(-3.5, 2.5, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    },
    ThirdPersonCamera{
        zoom: Zoom::new(1.5, 5.0),
        cursor_lock_key: KeyCode::Escape,
        ..Default::default()
    },
    Name::new("Camera"));
    
    commands.spawn(camera);
}
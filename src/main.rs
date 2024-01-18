mod sceneloader;
mod capsulecharacter;
mod camera;

use bevy::prelude::*;
use bevy_third_person_camera::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use sceneloader::SceneLoaderPlugins;
use capsulecharacter::CharacterPlugin;
use camera::CameraPlugin;

fn main(){
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(SceneLoaderPlugins)
    .add_plugins(CameraPlugin)
    .add_plugins(CharacterPlugin)
    .add_plugins(ThirdPersonCameraPlugin)
    .add_plugins(WorldInspectorPlugin::new())
    .run()
}
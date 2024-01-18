use bevy::prelude::*;

#[derive(Component)]
pub struct Movement{
    pub walk_forward: f32,
    pub walk_backward: f32,
    pub walk_rl: f32
}

pub struct MovePlugin;

impl Plugin for MovePlugin{
    fn build(&self, app: &mut App){
        app.add_systems(Update, movement);
    }
}

fn movement(mut query: Query<(&mut Transform, &Movement)>, input: Res<Input<KeyCode>>, time: Res<Time>){
    for (mut transform, movement) in query.iter_mut(){
        let mut velocity: f32 = 0.0;
        if input.pressed(KeyCode::S){
            velocity -= movement.walk_backward;
        }
        if input.pressed(KeyCode::W){
            velocity += movement.walk_forward;
        }
        transform.translation.x += velocity * time.delta_seconds();
        print!(" location X: {:?}", transform.translation.x);

        let mut velocity: f32 = 0.0;
        if input.pressed(KeyCode::A){
            velocity -= movement.walk_rl;
        }
        if input.pressed(KeyCode::D){
            velocity += movement.walk_rl;
        }
        transform.translation.z += velocity * time.delta_seconds();
        println!("\tlocation Z: {:?}", transform.translation.z);
    }
}

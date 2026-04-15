use bevy::prelude::*;

#[derive(Component)]
struct CircleCollider {
    radius: f32
}
#[derive(Component)]
struct RectangleCollider {
    width: f32,
    height: f32
}

#[derive(Event)]
struct CollisionStay(CircleCollider);

fn detect_collision(
    circles: Query<(&Transform, &CircleCollider)>
) {
    let physics_object_list = circles.iter().collect()[0];
}
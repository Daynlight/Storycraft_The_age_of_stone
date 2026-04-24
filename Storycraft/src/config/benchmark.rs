use bevy::math::Vec2;



pub const COLLISION_BOX_BENCHMARK_AMOUNT: u32 = 1000;
pub const COLLISION_BOX_STATIC_BENCHMARK_AMOUNT: u32 = 500;

pub const COLLISION_BOX_MAX_VELOCITY: f32 = 300.0;
pub const ENERGY_LOSS: f32 = 0.95;

pub const COLLISION_BOX_EDGE: Vec2 = Vec2::new(400.0, 400.0);
pub const COLLISION_BOX_MARGIN: Vec2 = Vec2::new(50.0, 50.0);
pub const COLLISION_BOX_EDGE_SIZE: f32 = 50.0;

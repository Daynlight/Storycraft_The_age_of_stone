use bevy::prelude::*;



pub const CAMERA_ZOOM: f32 = 0.2;

pub const PLAYER_VELOCITY: f32 = 150.0;
pub const PLAYER_ACCELERATION: f32 = 70.0;
pub const PLAYER_COLLISION_ENERGY_LOSS: f32 = 0.9;

pub const BUCKETS_SIZE: Vec2 = Vec2::new(20.0, 20.0);
pub const BUCKET_BUFFER_SIZE: usize = 16;
pub const FIXED_UPDATE_DELTA_TIME: f32 = 1./64.;

pub const BUFFER_SIZE: usize = 4096;
pub const SEED: u32 = 1543433234;



pub const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
pub const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
pub const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
pub const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];



pub const COLLISION_BOX_BENCHMARK_AMOUNT: u32 = 1000;
pub const COLLISION_BOX_STATIC_BENCHMARK_AMOUNT: u32 = 500;

pub const COLLISION_BOX_MAX_VELOCITY: f32 = 100.0;
pub const ENERGY_LOSS: f32 = 0.95;

pub const COLLISION_BOX_EDGE: Vec2 = Vec2::new(600.0, 320.0);
pub const COLLISION_BOX_MARGIN: Vec2 = Vec2::new(100.0, 100.0);
pub const COLLISION_BOX_EDGE_SIZE: f32 = 50.0;

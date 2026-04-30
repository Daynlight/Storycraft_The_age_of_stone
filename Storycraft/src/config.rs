use bevy::prelude::*;



pub const CAMERA_ZOOM: f32 = 0.2;

pub const PLAYER_VELOCITY: f32 = 150.0;
pub const PLAYER_ACCELERATION: f32 = 70.0;
pub const PLAYER_COLLISION_ENERGY_LOSS: f32 = 0.9;

pub const BUCKETS_SIZE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub const BUCKET_BUFFER_SIZE: usize = 16;
pub const FIXED_UPDATE_DELTA_TIME: f32 = 1./64.;

pub const SPRITE_SIZE: Vec2 = Vec2::new(32.0, 32.0);
pub const SPRITE_SIZE_LONG: Vec2 = Vec2::new(32.0, 64.0);

pub const BUFFER_SIZE: usize = 4096;
pub const SEED: u32 = 1543433234;



pub const UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
pub const DOWN: [KeyCode; 2] = [KeyCode::KeyS, KeyCode::ArrowDown];
pub const LEFT: [KeyCode; 2] = [KeyCode::KeyA, KeyCode::ArrowLeft];
pub const RIGHT: [KeyCode; 2] = [KeyCode::KeyD, KeyCode::ArrowRight];

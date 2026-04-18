use bevy::math::Vec2;




pub const CAMERA_ZOOM: f32 = 0.5;

pub const PLAYER_VELOCITY: f32 = 200.0;
pub const PLAYER_ACCELERATION: f32 = 1000.0;
pub const PLAYER_COLLISION_ENERGY_LOSS: f32 = 0.8;

pub const BUCKETS_SIZE: Vec2 = Vec2::new(20.0, 20.0);
pub const BUCKET_BUFFER_SIZE: usize = 1024;

pub const FIXED_UPDATE_DELTA_TIME: f32 = 1./64.;

pub const BUFFER_SIZE: usize = 4096;
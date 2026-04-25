use bevy::prelude::*;



pub const WORLD_SIZE: IVec3 = IVec3::new(32, 16, 32);

const WORLD_SIZE_MAT: Mat3 = Mat3::from_cols(
  Vec3::new(WORLD_SIZE.x as f32 / 2.0, 0.0,                       0.0),
  Vec3::new(0.0,                       WORLD_SIZE.y as f32 / 2.0, 0.0),
  Vec3::new(0.0,                       0.0,                       WORLD_SIZE.z as f32 / 2.0),
);

const INVERSE_WORLD_SIZE_MAT: Mat3 = Mat3::from_cols(
  Vec3::new(1.0 / (WORLD_SIZE.x as f32 / 2.0), 0.0,                               0.0),
  Vec3::new(0.0,                               1.0 / (WORLD_SIZE.y as f32 / 2.0), 0.0),
  Vec3::new(0.0,                               0.0,                               1.0 / (WORLD_SIZE.z as f32 / 2.0)),
);

const ISO_MAT: Mat3 = Mat3::from_cols(
  Vec3::new(1.0, 1.0, -1.0),
  Vec3::new(-1.0, 1.0, -1.0),
  Vec3::new(0.0, 2.0, 1.0),
);

const INVERSE_ISO_MAT: Mat3 = Mat3::from_cols(
  Vec3::new( 1.0 / 2.0, -1.0 / 2.0, 0.0),
  Vec3::new( 1.0 / 6.0,  1.0 / 6.0, 1.0 / 3.0),
  Vec3::new(-1.0 / 3.0, -1.0 / 3.0, 1.0 / 3.0),
);

const NORMALIZE_MOVEMENT_MAT: Mat3 = Mat3::from_cols(
  Vec3::new(1.0, 0.0, 0.0),
  Vec3::new(0.0, 2.0, 0.0),
  Vec3::new(0.0, 0.0, 1.0),
);



pub fn world_to_view(
  grid_position: Vec3,
) -> Vec3{
  return WORLD_SIZE_MAT * ISO_MAT * grid_position;
}


// pub fn view_to_world(
//   world_position: Vec3,
// ) -> Vec3{ 
//   return INVERSE_ISO_MAT * INVERSE_WORLD_SIZE_MAT * world_position;
// }


pub fn world_to_view_movement(
  world_position: Vec3,
) -> Vec3{
  return  INVERSE_ISO_MAT * NORMALIZE_MOVEMENT_MAT * INVERSE_WORLD_SIZE_MAT * world_position.normalize_or_zero();
}


pub fn hash2d(x: i32, y: i32, seed: u32) -> u32 {
  let mut h = seed;
  h ^= (x as u32).wrapping_mul(374761393);
  h ^= (y as u32).wrapping_mul(668265263);

  h = h.wrapping_mul(1274126177);

  return h ^ (h >> 16);
}
use bevy::prelude::*;



const ISO_MAT: Mat3 = Mat3::from_cols(
  Vec3::new(-16.0, 8.0, -16.0),
  Vec3::new(16.0, 8.0, -16.0),
  Vec3::new(0.0, 15.0, 16.0),
);

const INVERSE_ISO_MAT: Mat3 = Mat3::from_cols(
  Vec3::new( -1.0 / 32.0, 1.0 / 32.0, 0.0),
  Vec3::new( 1.0 / 46.0,  1.0 / 46.0, 1.0 / 23.0),
  Vec3::new(-15.0 / 736.0, -15.0 / 736.0, 1.0 / 46.0),
);

const NORMALIZE_MOVEMENT_MAT: Mat3 = Mat3::from_cols(
  Vec3::new(1.0, 0.0, 0.0),
  Vec3::new(0.0, 2.0, 0.0),
  Vec3::new(0.0, 0.0, 1.0),
);


// const ISO_MAT: Mat3 = Mat3::from_cols(
//   Vec3::new(19.0, 0.0, 0.0),
//   Vec3::new(0.0, 19.0, 0.0),
//   Vec3::new(0.0, 0.0, 19.0),
// );

// const INVERSE_ISO_MAT: Mat3 = Mat3::from_cols(
//   Vec3::new(1.0/19.0, 0.0, 0.0),
//   Vec3::new(0.0, 1.0/19.0, 0.0),
//   Vec3::new(0.0, 0.0, 1.0/19.0),
// );

// const NORMALIZE_MOVEMENT_MAT: Mat3 = Mat3::from_cols(
//   Vec3::new(1.0, 0.0, 0.0),
//   Vec3::new(0.0, 1.0, 0.0),
//   Vec3::new(0.0, 0.0, 1.0),
// );


pub fn world_to_view(
  grid_position: Vec3,
) -> Vec3{
  return ISO_MAT * grid_position;
}


// pub fn view_to_world(
//   world_position: Vec3,
// ) -> Vec3{ 
//   return INVERSE_ISO_MAT * world_position;
// }


pub fn world_to_view_movement(
  world_position: Vec3,
) -> Vec3{
  return  INVERSE_ISO_MAT * NORMALIZE_MOVEMENT_MAT * world_position.normalize_or_zero();
}


pub fn hash3d(x: i32, y: i32, z: i32, seed: u32) -> u32 {
  let mut h = seed;
  h ^= (x as u32).wrapping_mul(374761393);
  h ^= (y as u32).wrapping_mul(668265263);
  h ^= (z as u32).wrapping_mul(2147483647);

  h = h.wrapping_mul(1274126177);

  return h ^ (h >> 16);
}
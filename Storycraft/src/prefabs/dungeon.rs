use bevy::prelude::*;

use crate::utils::{tags, utils};
use crate::config;
use crate::mechanics::collisions;
use crate::mechanics::movement;



#[derive(Component)]
pub struct DungeonTile;

impl DungeonTile{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
  ){
    let txid = utils::hash2d(position.x.floor() as i32, position.y.floor() as i32, config::SEED) % 3;
    let texture = match txid {
      0 => asset_server.load("Dungeon/Tiles/Tile_1.png"),
      1 => asset_server.load("Dungeon/Tiles/Tile_2.png"),
      _ => asset_server.load("Dungeon/Tiles/Tile_3.png")
    };

    commands.spawn((
      Sprite{
        image: texture,
        custom_size: Some(Vec2::new(32.0, 32.0)),
        ..default()
      }, 
      tags::GameEntity,
      movement::WorldPos(position),
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      }
    ));
  }
}



#[derive(Component)]
pub struct DungeonWall;

impl DungeonWall{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
  ){
    let texture = asset_server.load("Dungeon/Walls/Wall.png");
    commands.spawn((
      Sprite{
        image: texture,
        custom_size: Some(Vec2::new(32.0, 32.0)),
        ..default()
      },
      collisions::CollisionBox::new(Vec2::ZERO, Vec2::new(16.0, 16.0)),
      tags::GameEntity,
      movement::WorldPos(position),
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      }
    ));
  }
}



#[derive(Component)]
pub struct DungeonStandingLamp;

impl DungeonStandingLamp{
  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
  ){
    let texture = asset_server.load("Dungeon/Lamp/Lamp.png");
    commands.spawn((
      Sprite{
        image: texture,
        custom_size: Some(Vec2::new(32.0, 64.0)),
        ..default()
      },
      collisions::CollisionBox::new(Vec2::new(0.0, -28.0), Vec2::new(16.0, 16.0)),
      tags::GameEntity,
      movement::WorldPos(position),
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      }
    ));
  }
}

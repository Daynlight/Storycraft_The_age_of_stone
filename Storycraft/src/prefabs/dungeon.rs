use bevy::prelude::*;

use crate::config;
use crate::utils::{tags, utils};
use crate::mechanics::{collisions, movement};
use crate::prefabs;



#[derive(Component)]
pub struct DungeonTile;

impl DungeonTile{
  pub fn from_range(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    start: IVec3,
    end: IVec3,
  ){
    let size = (start - end).abs();

    for x in 0..=size.x{
      for y in 0..=size.y{
        for z in 0..=size.z{
          let position = Vec3::new((x + start.x) as f32, (y + start.y) as f32, (z + start.z) as f32);
          DungeonTile::spawn(commands, asset_server, position);
        }
      }
    }
  }

  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
  ){
    let txid = utils::hash3d(position.x.floor() as i32, position.y.floor() as i32, position.z.floor() as i32, config::SEED) % 3;
    let texture = match txid {
      0 => asset_server.load("Dungeon/Tiles/Tile1.png"),
      1 => asset_server.load("Dungeon/Tiles/Tile2.png"),
      _ => asset_server.load("Dungeon/Tiles/Tile3.png")
    };

    commands.spawn((
      DungeonTile,
      Sprite{
        image: texture,
        custom_size: Some(config::SPRITE_SIZE),
        ..default()
      }, 
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      },
      movement::WorldPos(position),
      tags::GameEntity,
    ));
  }
}



#[derive(Component)]
pub struct DungeonWall;

impl DungeonWall{
  pub fn from_range(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    start: IVec3,
    end: IVec3,
  ){
    prefabs::collider::Collider::from_range(commands, start, end);
    let size = (start - end).abs();

    for x in 0..=size.x{
      for y in 0..=size.y{
        for z in 0..=size.z{
          let position = Vec3::new((x + start.x) as f32, (y + start.y) as f32, (z + start.z) as f32);
          DungeonWall::spawn(commands, asset_server, position);
        }
      }
    }
  }

  pub fn spawn(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Vec3,
  ){
    let txid = utils::hash3d(position.x.floor() as i32, position.y.floor() as i32, position.z.floor() as i32, config::SEED) % 3;
    let texture = match txid {
      0 => asset_server.load("Dungeon/Walls/Wall1.png"),
      1 => asset_server.load("Dungeon/Walls/Wall2.png"),
      _ => asset_server.load("Dungeon/Walls/Wall3.png")
    };

    commands.spawn((
      DungeonWall,
      Sprite{
        image: texture,
        custom_size: Some(config::SPRITE_SIZE),
        ..default()
      },
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      },
      // collisions::CollisionBox::new(Vec3::ZERO, Vec3::new(1.0, 1.0, 1.0)),
      movement::WorldPos(position),
      tags::GameEntity,
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
    let position = position + Vec3::new(0.0, 0.0, 1.3);

    commands.spawn((
      DungeonStandingLamp,
      Sprite{
        image: texture,
        custom_size: Some(config::SPRITE_SIZE_LONG),
        ..default()
      },
      Transform{
        translation: utils::world_to_view(position),
        ..default()
      },
      collisions::CollisionBox::new(Vec3::new(0.0, 0.0, -1.8), Vec3::new(0.5, 0.5, 0.2)),
      movement::WorldPos(position),
      tags::GameEntity,
    ));
  }
}

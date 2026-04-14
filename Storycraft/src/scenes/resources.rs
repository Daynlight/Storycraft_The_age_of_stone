use bevy::prelude::*;

use crate::scenes::register;



#[derive(Resource, Deref, Default)]
pub struct ActiveScene(pub register::RegisteredScenes);


#[derive(Resource, Deref, Default)]
pub struct LastScene(pub register::RegisteredScenes);

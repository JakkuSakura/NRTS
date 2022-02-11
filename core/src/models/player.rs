use crate::models::geometry::PhyPos;
use crate::models::UniqueEntityId;
use bevy::prelude::*;

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct PlayerId(pub u64);

#[derive(Bundle)]
struct Player {
    pub unique_id: UniqueEntityId,
    pub id: PlayerId,
    pub pos: PhyPos,
}

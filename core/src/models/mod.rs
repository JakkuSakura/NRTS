use bevy::prelude::*;
use bevy::utils::Uuid;

pub mod geometry;
pub mod message;
pub mod player;

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash, Component, Reflect)]
#[reflect(Component)]
pub struct UniqueEntityId(pub u128);
impl UniqueEntityId {
    pub fn new() -> Self {
        UniqueEntityId(Uuid::new_v4().to_u128_le())
    }
}

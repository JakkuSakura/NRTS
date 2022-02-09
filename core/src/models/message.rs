use bevy::prelude::*;
use crate::models::geometry::TargetPos;
use super::geometry::{PhyPos, Speed};


pub enum MessageOrigin {
    Known(PhyPos),
    Unknown,
}

pub enum MessageType {
    UnitReady,
    UnitDestroyed,
    UnitMoved

}

#[derive(Component)]
pub struct Message {
    pub origin_pos: PhyPos,
    pub origin: MessageOrigin,
    pub msg_type: MessageType,
}

#[derive(Bundle)]
pub struct Messenger {
    pub position: PhyPos,
    pub msg: Message,
    pub target: TargetPos,
    pub speed: Speed,
}



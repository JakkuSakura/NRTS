use bevy::prelude::*;
use crate::models::geometry::{PhyPos, Speed, TargetPos};

pub fn system_movement(mut query: Query<(&mut PhyPos, &Speed, &TargetPos)>) {
    for (mut pos, speed, target) in query.iter_mut() {
        let direction = (target.0 - pos.0).normalize();
        pos.0 += speed.0 * direction;
    }

}
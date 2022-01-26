use std::mem::transmute;
use bevy::math::Vec2;

#[derive(Debug, Copy, Clone)]
pub struct WorldShape {
    pub size: Vec2,
    // /// regular box
    // Rectangular {
    //     width: f32,
    //     height: f32
    // }
}

pub struct PhyPos(pub Vec2);

impl PhyPos {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }
    pub fn x(&self) -> f32 {
        self.0.x
    }
    pub fn y(&self) -> f32 {
        self.0.y
    }
}


use bevy::math::Vec2;
use bevy::prelude::*;

#[derive(Debug, Default, Copy, Clone, Reflect, Component)]
#[reflect(Component)]
pub struct WorldShape {
    pub size: Vec2,
}

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
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

#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct TargetPos(pub Vec2);

// speed per tick: 1/20
#[derive(Default, Component, Reflect)]
#[reflect(Component)]
pub struct Speed(pub f32);

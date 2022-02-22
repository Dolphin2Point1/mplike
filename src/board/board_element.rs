use bevy::prelude::*;

#[derive(Component)]
pub struct BoardElement {
    pub next_space: Entity,
    pub last_space: Entity
}

#[derive(Component)]
pub struct PassEventBoardElement {
    pub pass_event: &'static (dyn Fn(PassElementEvent) + Sync)
}

#[derive(Component)]
pub struct LandEventBoardElement {
    pub land_event: &'static (dyn Fn(LandElementEvent) + Sync)
}

pub struct LandElementEvent {
    pub player: Entity,
    pub space: Entity
}

pub struct PassElementEvent {
    pub player: Entity,
    pub space: Entity
}
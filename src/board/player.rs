use bevy::prelude::*;

#[derive(Component)]
pub struct BoardPlayer {
    pub current_space: Entity,
    pub spaces_to_move: u32
}

#[derive(Component)]
pub struct CoinCarrier {
    pub coins: u32
}

#[derive(Component)]
pub struct StarCarrier {
    pub stars: u32
}

// TODO handle coin increase and decrease events for visuals
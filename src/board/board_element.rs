use bevy::prelude::*;
use thiserror::Error;
use crate::board::player::{BoardPlayer, CoinCarrier, StarCarrier};

#[derive(Component)]
pub struct BoardElement {
    pub next_spaces: Vec<Entity>,
    pub last_spaces: Vec<Entity>
}

#[derive(Component)]
pub struct PassEventBoardElement {
    pub pass_event: &'static (dyn Fn(ElementEvent) -> Result<(), InvalidBoardError> + Sync)
}

#[derive(Component)]
pub struct LandEventBoardElement {
    pub land_event: &'static (dyn Fn(ElementEvent) -> Result<(), InvalidBoardError> + Sync)
}

pub struct ElementEvent<'world, 'state, 'a> {
    pub active_player: Entity,
    pub space: Entity,
    pub player_query: &'a mut Query<'world, 'state, (Entity, &'a mut BoardPlayer, &'a mut StarCarrier, &'a mut CoinCarrier)>,
    pub board_element_query: &'a mut Query<'world, 'state, (Entity, &'a mut BoardElement)>
}

#[derive(Error, Debug)]
pub enum InvalidBoardError {
    #[error("Only a junction can have multiple next spaces!")]
    MultipleNextSpaces
}
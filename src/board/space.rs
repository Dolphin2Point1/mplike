use bevy::ecs::system::{lifetimeless::SRes, SystemParamItem};
use bevy::pbr::{MaterialPipeline, SpecializedMaterial};
use bevy::prelude::*;
use crate::board::{
    board_element::{BoardElement, ElementEvent, InvalidBoardError},
    player::{BoardPlayer, CoinCarrier}
};
use crate::board::board_element::{LandEventBoardElement, PassEventBoardElement};
use crate::board::space_shader::CircleMaterial;

pub fn empty_function(event: ElementEvent) -> Result<(), InvalidBoardError> {
    // IT DOES NOTHING!!!!
    Ok(())
}

pub fn pass_function(pass_event: ElementEvent) -> Result<(), InvalidBoardError> {
    if let Ok(mut player) = pass_event.player_query
            .get_component_mut::<BoardPlayer>(pass_event.active_player) {
        if player.spaces_to_move != 0 {
            player.spaces_to_move -= 1;
            if let Ok(element) = pass_event.board_element_query
                    .get_component_mut::<BoardElement>(player.current_space) {
                if element.next_spaces.len() == 1 {
                    player.current_space = element.next_spaces[0];
                } else {
                    return Err(InvalidBoardError::MultipleNextSpaces);
                }
            }
        }
    }
    Ok(())
}

// blue space function
fn blue_space(land_event: ElementEvent) -> Result<(), InvalidBoardError> {
    if let Ok(mut coin_carrier) = land_event.player_query.get_component_mut::<CoinCarrier>(land_event.active_player) {
        // TODO increase by 5 if its the last 5 turns
        coin_carrier.coins += 3;
    }
    Ok(())
}

#[derive(Bundle)]
pub struct BlueSpaceBundle {
    element: BoardElement,
    pass_event: PassEventBoardElement,
    land_event: LandEventBoardElement,
    #[bundle]
    material_bundle: MaterialMeshBundle<CircleMaterial>
}

// red space function
fn red_space(land_event: ElementEvent) -> Result<(), InvalidBoardError> {
    if let Ok(mut coin_carrier) = land_event.player_query.get_component_mut::<CoinCarrier>(land_event.active_player) {
        // TODO decrease by 5 if its the last 5 turns
        if coin_carrier.coins < 3 {
            coin_carrier.coins = 0;
        } else {
            coin_carrier.coins -= 3;
        }
    }
    Ok(())
}
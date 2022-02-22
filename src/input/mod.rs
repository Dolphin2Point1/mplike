pub mod intent;
pub mod gamepad;

use bevy::prelude::*;
use intent::Intent;

#[derive(Component)]
pub struct Input {
    intents: Vec<Intent>
}
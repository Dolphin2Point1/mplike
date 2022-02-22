use bevy::prelude::*;
use crate::input::intent::Intent;
use crate::input::intent::Intent::*;

#[derive(Component)]
pub struct GamepadInput {
    id: usize
}

pub fn update_controller_intents() {

}

fn get_current_intents(gamepad: Gamepad,
                           button_inputs: Res<Input<GamepadButton>>,
                           button_axes: Res<Axis<GamepadButton>>,
                           axes: Res<Axis<GamepadAxis>>) -> Vec<Intent> {
    let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
    let axis_ly = GamepadAxis(gamepad, GamepadAxisType::LeftStickY);

    let axis_rx = GamepadAxis(gamepad, GamepadAxisType::RightStickX);
    let axis_ry = GamepadAxis(gamepad, GamepadAxisType::RightStickY);


    let mut intents = Vec::new();

    // Control Stick
    if let (Some(x), Some(y)) = (axes.get(axis_lx), axes.get(axis_ly)) {
        intents.push(MainJoystick(x, y));
    }

    // Camera Stick
    if let (Some(x), Some(y)) = (axes.get(axis_rx), axes.get(axis_ry)) {
        intents.push(CStick(x, y));
    }

    let axis_zl = GamepadAxis(gamepad, GamepadAxisType::LeftZ);
    let axis_zr = GamepadAxis(gamepad, GamepadAxisType::RightZ);

    let digital_zl = GamepadButton(gamepad, GamepadButtonType::LeftTrigger2);
    let digital_zr = GamepadButton(gamepad, GamepadButtonType::RightTrigger2);

    // ZL button
    if let Some(distance) = axes.get(axis_zl) {
        intents.push(AnalogZL(distance));
    } else if button_inputs.pressed(digital_zl) {
        intents.push(AnalogZL(1f32));
    } else {
        intents.push(AnalogZL(0f32));
    }

    // ZR button
    if let Some(distance) = axes.get(axis_zr) {
        intents.push(AnalogZR(distance));
    } else if button_inputs.pressed(digital_zr) {
        intents.push(AnalogZR(1f32));
    } else {
        intents.push(AnalogZR(0f32));
    }

    let l = GamepadButton(gamepad, GamepadButtonType::LeftTrigger);
    let r = GamepadButton(gamepad, GamepadButtonType::RightTrigger);

    // L button
    intents.push(ShoulderL(button_inputs.pressed(l)));
    // R button
    intents.push(ShoulderR(button_inputs.pressed(r)));

    let a = GamepadButton(gamepad, GamepadButtonType::East);
    let b = GamepadButton(gamepad, GamepadButtonType::South);
    let x = GamepadButton(gamepad, GamepadButtonType::North);
    let y = GamepadButton(gamepad, GamepadButtonType::West);

    // A button = confirm
    intents.push(Confirm(button_inputs.pressed(a)));
    // B button = deny
    intents.push(Deny(button_inputs.pressed(b)));
    // X button = Action 1
    intents.push(Action1(button_inputs.pressed(x)));
    // Y button = Action 2
    intents.push(Action2(button_inputs.pressed(y)));

    let up = GamepadButton(gamepad, GamepadButtonType::DPadUp);
    let down = GamepadButton(gamepad, GamepadButtonType::DPadDown);
    let left = GamepadButton(gamepad, GamepadButtonType::DPadLeft);
    let right = GamepadButton(gamepad, GamepadButtonType::DPadRight);

    // DPad Up = Up
    intents.push(Up(button_inputs.pressed(up)));
    // DPad Down = Down
    intents.push(Down(button_inputs.pressed(down)));
    // DPad Left = Left
    intents.push(Left(button_inputs.pressed(left)));
    // DPad Right = Right
    intents.push(Right(button_inputs.pressed(right)));

    let start = GamepadButton(gamepad, GamepadButtonType::Start);

    // Start = Pause
    intents.push(Pause(button_inputs.pressed(start)));

    intents
}
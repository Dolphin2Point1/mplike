use bevy::prelude::*;

#[derive(Component)]
pub enum Intent {
    MainJoystick(f32, f32), // maps to the main joystick of the controller
    CStick(f32, f32), // maps to the c stick of the controller
    AnalogZL(f32), // maps to left analog shoulder
    AnalogZR(f32), // maps to right analog shoulder
    ShoulderL(bool), // maps to left shoulder
    ShoulderR(bool), // maps to right shoulder
    Confirm(bool), // maps to a confirm action (A)
    Deny(bool), // maps to a deny action  (B)
    Action1(bool), // maps to an other action (X)
    Action2(bool), // maps to another other action (Y)
    Up(bool), // maps to up dpad
    Down(bool), // maps to down dpad
    Left(bool), // maps to left dpad
    Right(bool), // maps to right dpad
    Pause(bool), // maps to the pause button (Start)
}
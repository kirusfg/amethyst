use serde::{Deserialize, Serialize};

use crate::event::InputEvent;

/// Controller axes matching SDL controller model
#[derive(Eq, PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ControllerAxis {
    /// The X axis on the left stick
    LeftX,
    /// The Y axis on the left stick
    LeftY,
    /// The X axis on the right stick
    RightX,
    /// The Y axis on the right stick
    RightY,
    /// The analog left trigger, not to be confused with the left bumper.
    LeftTrigger,
    /// The analog right trigger, not to be confused with the right bumper.
    RightTrigger,
}

/// Controller buttons matching SDL controller model
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash, Serialize, Deserialize)]
pub enum ControllerButton {
    /// The A button, typically the lower button in the "diamond" of buttons on the right side
    /// of the controller.
    A,
    /// The B button, typically the right button in the "diamond" of buttons on the right side
    /// of the controller.
    B,
    /// The X button, typically the left button in the "diamond" of buttons on the right side
    /// of the controller.
    X,
    /// The Y button, typically the top button in the "diamond" of buttons on the right side
    /// of the controller.
    Y,
    /// The dpad button pointed towards the player
    DPadDown,
    /// The dpad button pointed to the player's left
    DPadLeft,
    /// The dpad button pointed to the player's right
    DPadRight,
    /// The dpad button pointed away from the player.
    DPadUp,
    /// The digital left shoulder bumper. Usually located above the left trigger.
    LeftShoulder,
    /// The digital right shoulder bumper. Usually located above the right trigger.
    RightShoulder,
    /// If your press the left analog stick into the controller this button is pressed.
    LeftStick,
    /// If your press the right analog stick into the controller this button is pressed.
    RightStick,
    /// The back button is typically a button slightly left of center with a leftward arrow on it.
    Back,
    /// The start button is typically a button slightly right of center with a rightward arrow on it.
    Start,
    /// The centermost button on the controller. Large and green on an Xbox controller.
    Guide,
}

/// Controller events generated by the SDL events system.
#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ControllerEvent {
    /// Movement event on a controller axis.
    ///
    /// Corresponds to [`SDL_CONTROLLERAXISMOTION`].
    ///
    /// [`SDL_CONTROLLERAXISMOTION`]: https://wiki.libsdl.org/SDL_ControllerAxisEvent
    ControllerAxisMoved {
        /// The joystick instance id.
        which: u32,
        /// The controller axis.
        axis: ControllerAxis,
        /// The axis value (range: -32768 to 32767).
        value: f32,
    },
    /// Button press event on a controller.
    ///
    /// Corresponds to [`SDL_CONTROLLERBUTTONDOWN`].
    ///
    /// [`SDL_CONTROLLERBUTTONDOWN`]: https://wiki.libsdl.org/SDL_ControllerButtonEvent
    ControllerButtonPressed {
        /// The joystick instance id.
        which: u32,
        /// The controller button.
        button: ControllerButton,
    },
    /// Button press event on a controller.
    ///
    /// Corresponds to [`SDL_CONTROLLERBUTTONUP`].
    ///
    /// [`SDL_CONTROLLERBUTTONUP`]: https://wiki.libsdl.org/SDL_ControllerButtonEvent
    ControllerButtonReleased {
        /// The joystick instance id.
        which: u32,
        /// The controller button.
        button: ControllerButton,
    },
    /// Controller disconnect event.
    ///
    /// Corresponds to [`SDL_CONTROLLERDEVICEREMOVED`].
    ///
    /// [`SDL_CONTROLLERDEVICEREMOVED`]: https://wiki.libsdl.org/SDL_ControllerDeviceEvent
    ControllerDisconnected {
        /// The joystick device index for the `SDL_CONTROLLERDEVICEADDED` event or instance id for
        /// the `SDL_CONTROLLERDEVICEREMOVED` or `SDL_CONTROLLERDEVICEREMAPPED` event
        which: u32,
    },
    /// Controller connected event.
    ///
    /// Corresponds to [`SDL_CONTROLLERDEVICEADDED`].
    ///
    /// [`SDL_CONTROLLERDEVICEADDED`]: https://wiki.libsdl.org/SDL_ControllerDeviceEvent
    ControllerConnected {
        /// The joystick device index for the `SDL_CONTROLLERDEVICEADDED` event or instance id for
        /// the `SDL_CONTROLLERDEVICEREMOVED` or `SDL_CONTROLLERDEVICEREMAPPED` event
        which: u32,
    },
}

impl From<&ControllerEvent> for InputEvent {
    fn from(c: &ControllerEvent) -> Self {
        use self::ControllerEvent::{
            ControllerAxisMoved, ControllerButtonPressed, ControllerButtonReleased,
            ControllerConnected, ControllerDisconnected,
        };
        match *c {
            ControllerAxisMoved { which, axis, value } => {
                InputEvent::ControllerAxisMoved { which, axis, value }
            }
            ControllerButtonPressed { which, button } => {
                InputEvent::ControllerButtonPressed { which, button }
            }
            ControllerButtonReleased { which, button } => {
                InputEvent::ControllerButtonReleased { which, button }
            }
            ControllerConnected { which } => InputEvent::ControllerConnected { which },
            ControllerDisconnected { which } => InputEvent::ControllerDisconnected { which },
        }
    }
}

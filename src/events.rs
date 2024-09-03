//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use winit::dpi::{PhysicalPosition, PhysicalSize};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: MousseButtonPressedState
//{{{ enum: MouseButtonPressedState
#[derive(Debug, PartialEq)]
pub enum MouseButtonPressedState {
    NotPressed,
    LeftPressed,
    MiddlePressed,
    RightPressed,
}
//}}}
//{{{ impl: Default for MouseButtonPressedState
impl Default for MouseButtonPressedState {
    fn default() -> Self {
        MouseButtonPressedState::NotPressed
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ collection: KeyModifierState
//{{{ enum: KeyModifierState
#[derive(Debug, PartialEq)]
pub enum KeyModifierState {
    NotModified,
    Shift,
    Ctrl,
    Alt,
    Logo,
}
//}}}
//{{{ impl: Default for KeyModifierState
impl Default for KeyModifierState {
    fn default() -> Self {
        KeyModifierState::NotModified
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ collection: ResizedState
//{{{ enum: ResizedState
#[derive(Debug, PartialEq)]
pub enum ResizedState {
    NotResized,
    Resized((f32, f32)),
}
//}}}
//{{{ impl: Default for ResizedState
impl Default for ResizedState {
    fn default() -> Self {
        ResizedState::NotResized
    }
}
//}}}
//}}}
//{{{ collection: KeyStrokeState
//{{{ enum: KeyStrokeState
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyStrokeState {
    None,
    Left,
    Right,
    Up,
    Down,
    Space,
}
//}}}
//{{{ impl: Default for KeyStrokeState
impl Default for KeyStrokeState {
    fn default() -> Self {
        KeyStrokeState::None
    }
}
//}}}
//}}}
//{{{ collection: EventController
//{{{ struct: EventController
/// Represents the state of the event controller, which manages various input events such as mouse,
/// keyboard, and window resizing.
///
/// The `EventController` struct contains the following fields:
///
/// - `mouse_button_pressed_state`: Tracks the current state of mouse button presses.
/// - `mouse_position`: Stores the current mouse position.
/// - `mouse_position_delta`: Stores the change in mouse position since the last update.
/// - `mouse_wheel_delta`: Stores the change in mouse wheel scroll since the last update.
/// - `key_modifier_state`: Tracks the current state of keyboard modifiers (e.g., Shift, Ctrl, Alt).
/// - `resized_state`: Tracks whether the window has been resized.
/// - `key_stroke_state`: Tracks the current state of keyboard input (e.g., arrow keys, space).
#[derive(Default, Debug)]
pub struct EventController {
    pub mouse_button_pressed_state: MouseButtonPressedState,
    pub mouse_position: [f32; 2],
    pub mouse_position_delta: [f32; 2],
    pub mouse_wheel_delta: Option<f32>,
    pub key_modifier_state: winit::keyboard::ModifiersState,
    pub resized_state: ResizedState,
    pub key_stroke_state: KeyStrokeState,
}
//}}}
//{{{ impl: EventController
impl EventController {
    //{{{ fun: mouse_wheel_update
    /// Updates the mouse wheel delta value based on the provided mouse scroll delta.
    ///
    /// This method is used to track the change in mouse wheel scroll since the last update.
    /// The `mouse_wheel_delta` field in the `EventController` struct is updated with the
    /// vertical component of the mouse scroll delta.
    ///
    /// # Parameters
    /// - `delta`: The mouse scroll delta, which can be either a line delta or a pixel delta.
    pub fn mouse_wheel_update(&mut self, delta: winit::event::MouseScrollDelta) {
        self.mouse_wheel_delta = match delta {
            winit::event::MouseScrollDelta::LineDelta(_, y) => Some(y),
            winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => {
                Some(y as f32)
            }
        }
    }
    //}}}
    //{{{ fun: mouse_input_update
    /// Updates the state of mouse button presses.
    ///
    /// This method is used to track the current state of mouse button presses (left, middle, right).
    /// The `mouse_button_pressed_state` field in the `EventController` struct is updated based on
    /// the provided mouse button state and button.
    ///
    /// # Parameters
    /// - `state`: The state of the mouse button (pressed or released).
    /// - `button`: The mouse button that was pressed or released.
    pub fn mouse_input_update(
        &mut self,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
    ) {
        self.mouse_button_pressed_state = match state {
            winit::event::ElementState::Pressed => match button {
                winit::event::MouseButton::Left => MouseButtonPressedState::LeftPressed,
                winit::event::MouseButton::Middle => MouseButtonPressedState::MiddlePressed,
                winit::event::MouseButton::Right => MouseButtonPressedState::RightPressed,
                _ => MouseButtonPressedState::NotPressed,
            },
            winit::event::ElementState::Released => MouseButtonPressedState::NotPressed,
        }
    }
    //}}}
    //{{{ fun: cursor_moved_update
    /// Updates the mouse cursor position and delta.
    ///
    /// This method is used to track the current position of the mouse cursor and the delta (change) in
    /// the cursor position since the last update. The `mouse_position` and `mouse_position_delta`
    /// fields in the `EventController` struct are updated based on the provided cursor position.
    ///
    /// # Parameters
    /// - `pos`: The current physical position of the mouse cursor.
    pub fn cursor_moved_update(&mut self, pos: PhysicalPosition<f64>) {
        let new_x = pos.x as f32;

        let del_x = new_x - self.mouse_position[0];

        let new_y = pos.y as f32;

        let del_y = new_y - self.mouse_position[1];

        self.mouse_position[0] = new_x;

        self.mouse_position[1] = new_y;

        self.mouse_position_delta[0] = del_x;

        self.mouse_position_delta[1] = del_y;
    }
    //}}}
    //{{{ fun: key_input_update
    /// Updates the key stroke state based on the provided key input.
    ///
    /// This method is used to track the current state of the keyboard input, specifically the
    /// arrow keys and the /// space key. The `key_stroke_state` field in the `EventController`
    /// struct is updated based on the provided key state and key.
    ///
    /// # Parameters
    /// - `state`: The current state of the key (pressed or released).
    /// - `key`: The specific key that was pressed or released.
    pub fn key_input_update(
        &mut self,
        state: winit::event::ElementState,
        key: winit::keyboard::NamedKey,
    ) {
        match state {
            winit::event::ElementState::Pressed => match key {
                winit::keyboard::NamedKey::ArrowLeft => {
                    self.key_stroke_state = KeyStrokeState::Left;
                }
                winit::keyboard::NamedKey::ArrowRight => {
                    self.key_stroke_state = KeyStrokeState::Right;
                }
                winit::keyboard::NamedKey::ArrowUp => {
                    self.key_stroke_state = KeyStrokeState::Up;
                }
                winit::keyboard::NamedKey::ArrowDown => {
                    self.key_stroke_state = KeyStrokeState::Down;
                }
                winit::keyboard::NamedKey::Space => {
                    self.key_stroke_state = KeyStrokeState::Space;
                }
                _ => {
                    self.key_stroke_state = KeyStrokeState::None;
                }
            },
            winit::event::ElementState::Released => {
                self.key_stroke_state = KeyStrokeState::None;
            }
        }
    }
    //}}}
    //{{{ fun: key_modifiers_update
    /// This method is used to update the key modifier state, such as whether the Shift, Ctrl, or
    /// Alt keys are currently pressed. The `key_modifier_state` field in the `EventController`
    /// struct is updated based on the provided modifier state.
    ///
    /// # Parameters
    /// - `modifiers`: The current state of the keyboard modifiers.
    pub fn key_modifiers_update(&mut self, modifiers: winit::keyboard::ModifiersState) {
        self.key_modifier_state = modifiers;
    }
    //}}}
    //{{{ fun: resize_update
    /// This method is used to update the state of the application when the window is resized.
    /// The `resized_state` field in the `EventController` struct is updated with the new
    /// physical size of the window.
    ///
    /// # Parameters
    /// - `size`: The new physical size of the window.
    pub fn resize_update(&mut self, size: PhysicalSize<u32>) {
        self.resized_state = ResizedState::Resized((size.width as f32, size.height as f32));
    }
    //}}}
}
//}}}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {

    use super::*;

    //{{{ test: test_mouse_wheel_update
    #[test]
    fn test_mouse_wheel_update() {
        let mut event_controller = EventController::default();

        // Test mouse wheel scroll
        let s1 = winit::event::MouseScrollDelta::LineDelta(1.0, 1.0);
        event_controller.mouse_wheel_update(s1);
        assert_eq!(event_controller.mouse_wheel_delta, Some(1.0));

        // Test subsequent mouse wheel scroll
        let s2 = winit::event::MouseScrollDelta::PixelDelta(winit::dpi::PhysicalPosition {
            x: 2.0,
            y: 2.0,
        });
        event_controller.mouse_wheel_update(s2);
        assert_eq!(event_controller.mouse_wheel_delta, Some(2.0));
    }
    //}}}
    //{{{ test: test_mouse_input_update
    #[test]
    fn test_mouse_input_update() {
        let mut event_controller = EventController::default();

        // Test mouse button press
        event_controller.mouse_input_update(
            winit::event::ElementState::Pressed,
            winit::event::MouseButton::Left,
        );
        assert_eq!(
            event_controller.mouse_button_pressed_state,
            MouseButtonPressedState::LeftPressed
        );

        // Test mouse button release
        event_controller.mouse_input_update(
            winit::event::ElementState::Released,
            winit::event::MouseButton::Left,
        );
        assert_eq!(
            event_controller.mouse_button_pressed_state,
            MouseButtonPressedState::NotPressed
        );

        // Test other mouse button press
        event_controller.mouse_input_update(
            winit::event::ElementState::Pressed,
            winit::event::MouseButton::Right,
        );
        assert_eq!(
            event_controller.mouse_button_pressed_state,
            MouseButtonPressedState::RightPressed
        );
    }
    //}}}
    //{{{ test: test_cursor_moved_update
    #[test]
    fn test_cursor_moved_update() {
        let mut event_controller = EventController::default();

        // Test mouse position update
        event_controller.cursor_moved_update(PhysicalPosition { x: 100.0, y: 200.0 });
        assert_eq!(event_controller.mouse_position[0], 100.0);
        assert_eq!(event_controller.mouse_position[1], 200.0);
        assert_eq!(event_controller.mouse_position_delta[0], 100.0);
        assert_eq!(event_controller.mouse_position_delta[1], 200.0);

        // Test subsequent mouse position update
        event_controller.cursor_moved_update(PhysicalPosition { x: 150.0, y: 250.0 });
        assert_eq!(event_controller.mouse_position[0], 150.0);
        assert_eq!(event_controller.mouse_position[1], 250.0);
        assert_eq!(event_controller.mouse_position_delta[0], 50.0);
        assert_eq!(event_controller.mouse_position_delta[1], 50.0);
    }
    //}}}
    //{{{ test: test_key_input_update
    #[test]
    fn test_key_input_update() {
        let mut event_controller = EventController::default();

        // Test arrow key press
        event_controller.key_input_update(
            winit::event::ElementState::Pressed,
            winit::keyboard::NamedKey::ArrowLeft,
        );
        assert_eq!(event_controller.key_stroke_state, KeyStrokeState::Left);

        // Test arrow key release
        event_controller.key_input_update(
            winit::event::ElementState::Released,
            winit::keyboard::NamedKey::ArrowLeft,
        );
        assert_eq!(event_controller.key_stroke_state, KeyStrokeState::None);

        // Test space key press
        event_controller.key_input_update(
            winit::event::ElementState::Pressed,
            winit::keyboard::NamedKey::Space,
        );
        assert_eq!(event_controller.key_stroke_state, KeyStrokeState::Space);

        // Test space key release 
        event_controller.key_input_update(
            winit::event::ElementState::Released,
            winit::keyboard::NamedKey::Space,
        );
        assert_eq!(event_controller.key_stroke_state, KeyStrokeState::None);

        // Test anything else
        event_controller.key_input_update(
            winit::event::ElementState::Pressed,
            winit::keyboard::NamedKey::CapsLock,
        );
        assert_eq!(event_controller.key_stroke_state, KeyStrokeState::None);
    }
    //}}}
    //{{{ test: test_key_modifiers_update
    #[test]
    fn test_key_modifiers_update() {
        let mut event_controller = EventController::default();

        // Test modifier key press
        event_controller.key_modifiers_update(winit::keyboard::ModifiersState::SHIFT);
        assert_eq!(
            event_controller.key_modifier_state,
            winit::keyboard::ModifiersState::SHIFT
        );

        // Test no modifier keys press
        event_controller.key_modifiers_update(winit::keyboard::ModifiersState::empty());
        assert_eq!(
            event_controller.key_modifier_state,
            winit::keyboard::ModifiersState::empty()
        );
    }
    //}}}
}
//}}}

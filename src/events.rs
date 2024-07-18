

use winit::dpi::{PhysicalPosition, PhysicalSize};


#[derive(Debug)]

pub enum MouseButtonPressedState
{
    NotPressed,
    LeftPressed,
    MiddlePressed,
    RightPressed,
}

impl Default for MouseButtonPressedState
{
    fn default() -> Self
    {
        MouseButtonPressedState::NotPressed
    }
}

//..................................................................................................

pub enum KeyModifierState
{
    NotModified,
    Shift,
    Ctrl,
    Alt,
    Logo,
}

impl Default for KeyModifierState
{
    fn default() -> Self
    {
        KeyModifierState::NotModified
    }
}

//..................................................................................................

#[derive(Debug)]

pub enum ResizedState
{
    NotResized,
    Resized((f32, f32)),
}

impl Default for ResizedState
{
    fn default() -> Self
    {
        ResizedState::NotResized
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]

pub enum KeyStrokeState
{
    None,
    Left,
    Right,
    Up,
    Down,
    Space,
}

impl Default for KeyStrokeState
{
    fn default() -> Self
    {
        KeyStrokeState::None
    }
}



#[derive(Default, Debug)]
pub struct EventController
{
    pub mouse_button_pressed_state: MouseButtonPressedState,
    pub mouse_position: [f32; 2],
    pub mouse_position_delta: [f32; 2],
    pub mouse_wheel_delta: Option<f32>,
    pub key_modifier_state: winit::event::Modifiers,
    pub resized_state: ResizedState,
    pub key_stroke_state: KeyStrokeState,
}

impl EventController
{
    pub fn mouse_wheel_update(
        &mut self,
        delta: winit::event::MouseScrollDelta,
    )
    {
        self.mouse_wheel_delta = match delta
        {
            winit::event::MouseScrollDelta::LineDelta(_, y) => Some(y),
            winit::event::MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => Some(y as f32),
        }
    }

    pub fn mouse_input_update(
        &mut self,
        state: winit::event::ElementState,
        button: winit::event::MouseButton,
    )
    {
        self.mouse_button_pressed_state = match state
        {
            winit::event::ElementState::Pressed => match button
            {
                winit::event::MouseButton::Left => MouseButtonPressedState::LeftPressed,
                winit::event::MouseButton::Middle => MouseButtonPressedState::MiddlePressed,
                winit::event::MouseButton::Right => MouseButtonPressedState::RightPressed,
                _ => MouseButtonPressedState::NotPressed,
            },
            winit::event::ElementState::Released => MouseButtonPressedState::NotPressed,
        }
    }

    pub fn cursor_moved_update(
        &mut self,
        pos: PhysicalPosition<f64>,
    )
    {
        let new_x = pos.x as f32;

        let del_x = new_x - self.mouse_position[0];

        let new_y = pos.y as f32;

        let del_y = new_y - self.mouse_position[1];

        self.mouse_position[0] = new_x;

        self.mouse_position[1] = new_y;

        self.mouse_position_delta[0] = del_x;

        self.mouse_position_delta[1] = del_y;
    }

    pub fn key_input_update(
        &mut self,
        state: winit::event::ElementState,
        key: winit::keyboard::NamedKey
    )
    {
        match state
        {
            winit::event::ElementState::Pressed => match key
            {
                winit::keyboard::NamedKey::ArrowLeft => 
                {
                    self.key_stroke_state = KeyStrokeState::Left;
                }
                winit::keyboard::NamedKey::ArrowRight => 
                {
                    self.key_stroke_state = KeyStrokeState::Right;
                }
                winit::keyboard::NamedKey::ArrowUp => 
                {
                    self.key_stroke_state = KeyStrokeState::Up;
                }
                winit::keyboard::NamedKey::ArrowDown => 
                {
                    self.key_stroke_state = KeyStrokeState::Down;
                }
                winit::keyboard::NamedKey::Space => 
                {
                    self.key_stroke_state = KeyStrokeState::Space;
                }
                _ =>
                {
                    self.key_stroke_state = KeyStrokeState::None;
                }
            },
            winit::event::ElementState::Released =>
            {
                self.key_stroke_state = KeyStrokeState::None;
            }
        }
    }

    pub fn key_modifiers_update(
        &mut self,
        modifiers: winit::event::Modifiers,
    )
    {
        self.key_modifier_state = modifiers;
    }

    pub fn resize_update(
        &mut self,
        size: PhysicalSize<u32>,
    )
    {
        self.resized_state = ResizedState::Resized((size.width as f32, size.height as f32));
    }
}
//! This module defines the 3D camera and the associated state which is used by wgpu to perform 
//! transforms on the vertices of the model.
//!
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use crate::events::*;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use bytemuck::{Pod, Zeroable};
use embed_doc_image::embed_doc_image;
use winit::keyboard::ModifiersKeyState;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ col: constants
/// The global up vector.
const GLOBAL_UP: Vec3 = Vec3::new(0.0, 0.0, 1.0);
/// The maximum angle the camera can pitch up or down.
const PITCH_SAFE: f32 = 1.0e-2;
/// The initial distance of the camer
const INV_SQRT_3: f32 = 0.5773502691896258;
/// Positions of the 8 octants of the sphere.
const DIAGONAL_POSITIONS: [Vec3; 8] = [
    vector!(INV_SQRT_3, INV_SQRT_3, INV_SQRT_3),
    vector!(-INV_SQRT_3, INV_SQRT_3, INV_SQRT_3),
    vector!(-INV_SQRT_3, -INV_SQRT_3, INV_SQRT_3),
    vector!(INV_SQRT_3, -INV_SQRT_3, INV_SQRT_3),
    vector!(INV_SQRT_3, INV_SQRT_3, -INV_SQRT_3),
    vector!(-INV_SQRT_3, INV_SQRT_3, -INV_SQRT_3),
    vector!(-INV_SQRT_3, -INV_SQRT_3, -INV_SQRT_3),
    vector!(INV_SQRT_3, -INV_SQRT_3, -INV_SQRT_3),
];
//}}}
//{{{ col: Camera
//{{{ struct: Camera
/// Represents a camera in 3D space. This camera has a position, a focus, a pitch and yaw.
/// The pitch and yaw angles, denoted $\phi$ and $\theta$ respectively,  are illustrated below.
///
/// ![][pitch-yaw]
#[embed_doc_image("pitch-yaw", "docs/images/pitch-yaw.png")]
#[derive(Debug)]
pub struct Camera
{
    /// position of the camera, denoted $\mbf{p}$
    position: Vec3,
    /// location of focus of the camera, denoted $\mbf{f}$, this also determines its direction:
    /// $$
    /// \mbf{d} = \frac{\mbf{f} - \mbf{p}}{||\mbf{f} - \mbf{p}||}
    /// $$
    focus: Vec3,
    /// pitch angle of the camera direction vector $\mbf{d}$, defined as the angle betweeen the
    /// positive z-axis and the camera direction vector $\mbf{d}$. It is defined on the range
    /// $[\epsilon, \pi - \epsilon]$ where $\epsilon$ is a small angle. This is done to avoid
    /// gimbal lock.
    pitch: f32,
    /// yaw angle of the camera direction vector $\mbf{d}$, defined as the angle betweeen the
    /// positive x-axis and the projection of the $\mbf{d}$ vector on the xy-plane. It is defined
    /// on the range $[0, 2\pi]$
    yaw: f32,
    /// Current quadrant the camera resides in. Note this is only the exact octant of the camera
    /// when the user presses <SHIFT + UP/DOWN/LEFT/RIGHT>
    octant: i8,
}
//..................................................................................................
//}}}
//{{{ impl: Camera
impl Camera
{
    pub fn calc_matrix(&self) -> Mat4
    {
        let position: Poi3 = self.position.into();

        let target: Poi3 = self.focus.into();

        Mat4::look_at_rh(&position, &target, &GLOBAL_UP)
    }

    pub fn set_octant(
        &mut self,
        new_oct: i8,
    )
    {
        assert!(new_oct < 8);

        self.octant = new_oct;

        let cur_dist = self.dist();

        let new_dir = DIAGONAL_POSITIONS[self.octant as usize];

        let new_position = self.focus + cur_dist * new_dir;

        self.position = new_position;

        let (new_pitch, new_yaw) = pitch_and_yaw(&new_dir);

        self.pitch = new_pitch;

        self.yaw = new_yaw;
    }

    pub fn zoom(
        &mut self,
        delta: f32,
    )
    {
        let direction = (self.focus - self.position).normalize();
        self.position += direction * delta;
    }

    fn orbit(
        &mut self,
        delta_pitch: f32,
        delta_yaw: f32,
    )
    {
        let d = self.dist();
        self.pitch =
            (self.pitch - delta_pitch).clamp(PITCH_SAFE, std::f32::consts::PI - PITCH_SAFE);
        self.yaw = mod_angle(self.yaw + delta_yaw);
        let dir = direction(self.pitch, self.yaw);
        self.position = self.focus + dir * d;
    }

    fn pan(
        &mut self,
        delta_x: f32,
        delta_y: f32,
    )
    {
        let forward = (self.focus - self.position).normalize();
        let up = GLOBAL_UP;
        let right = forward.cross(&GLOBAL_UP).normalize();
        let displacement = up * delta_y + right * delta_x;
        self.position += displacement;
        self.focus += displacement;
    }

    fn dist(&self) -> f32
    {
        (self.position - self.focus).norm()
    }

    fn direction(&self) -> Vec3
    {
        (self.focus - self.position).normalize()
    }
}
//}}}
//{{{ impl: Default for Camera
impl Default for Camera
{
    fn default() -> Self
    {
        Self {
            position: DIAGONAL_POSITIONS[0],
            focus: Vec3::zeros(),
            pitch: 2.186276,
            yaw: -2.3561945,
            octant: 0,
        }
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ col: Projection
//{{{ struct: Projection
#[derive(Debug)]
pub struct Projection
{
    fov: f32,
    near: f32,
    far: f32,
    aspect: f32,
}
//}}}
//{{{ impl: Projection
impl Projection
{
    pub fn calc_matrix(&self) -> Mat4
    { 
        Mat4::new_perspective(self.aspect, self.fov, self.near, self.far)
        // OPENGL_TO_WGPU_MATRIX * Mat4::new_perspective(self.aspect, self.fov, self.near, self.far)
    }
}
//}}}
//{{{ impl: Default for Projection
impl Default for Projection
{
    fn default() -> Self
    {
        Self {
            fov: rad(45.0),
            near: 0.1,
            far: 100.0,
            aspect: 2.0,
        }
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ col: ViewOptions
//{{{ struct: ViewOptions
/// This struct contains options for how the internal state of View changes in response to
/// various events.
#[derive(Debug)]
pub struct ViewOptions
{
    /// This scales distance the camera will move along its lateral direction vectors when
    /// the arrow keys are pressed with the alt key held down.
    /// The formula for the scaling is:
    ///
    pub key_pan_delta: f32,
    /// This is the radian delta in each either the pitch or yaw angles the camera will move
    /// in reponse to arrow key presses with no modifiers
    pub key_orbit_delta: f32,
    /// This is the sensitivety of the mouse wheel when moving forward and backward
    pub zoom_speed: f32,
}
//}}}
//{{{ impl: Default for ViewOptions
impl Default for ViewOptions
{
    fn default() -> Self
    {
        Self {
            key_pan_delta: 0.1,
            key_orbit_delta: rad(2.5),
            zoom_speed: 0.001,
        }
    }
}
//}}}
//..................................................................................................
//}}}
//{{{ col: View
//{{{ struct: View
/// The `View` struct represents a 3D camera view, including the camera, projection, and uniform data.
/// 
/// The `options` field contains configuration options for how the camera view responds to user input.
/// The `camera` field represents the position, orientation, and other properties of the camera.
/// The `projection` field represents the projection parameters for the camera view.
/// The `uniform` field contains data that is passed to the graphics shader as a uniform.
#[derive(Default, Debug)]
pub struct View
{
    pub options: ViewOptions,
    camera: Camera,
    projection: Projection,
    pub uniform: ViewUniform,
}
//}}}
//{{{ impl: View
impl View
{
    fn calc_matrix(&self) -> Mat4
    {
        self.projection.calc_matrix() * self.camera.calc_matrix()
    }

    pub fn update_uniform(&mut self)
    {
        self.uniform.view_position[0..3].copy_from_slice(self.camera.position.as_slice());
        self.uniform.view_position[3] = 1.0;

        let view_dir: Vec3 = self.camera.direction().into();
        self.uniform.view_direction[0..3].copy_from_slice(view_dir.as_slice());
        self.uniform.view_direction[3] = 0.0;

        let view_proj: Mat4 = self.calc_matrix();
        self.uniform.view_proj = view_proj.into();
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ col: ViewUniform
//{{{ struct: ViewUniform   
#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct ViewUniform
{
    view_position: [f32; 4],
    view_direction: [f32; 4],
    view_proj: [[f32; 4]; 4],
}
//}}}
//{{{ impl: Default for ViewUniform
impl Default for ViewUniform
{
    fn default() -> Self
    {
        Self {
            view_position: [0.0; 4],
            view_direction: [0.0; 4],
            view_proj: Mat4::identity().into(),
        }
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ impl: EventntController
impl EventController
{
    //{{{ fun: update_view_3d
    pub fn update_view_3d(
        &mut self,
        view: &mut View,
    )
    {
        // handle resizing
        if let ResizedState::Resized(new_size) = self.resized_state
        {
            view.projection.aspect = new_size.0 as f32 / new_size.1 as f32;

            self.resized_state = ResizedState::NotResized;
        }

        // handle key stroke to change octant of camera
        if self.key_stroke_state != KeyStrokeState::None
        {
            match (self.key_modifier_state)
            {
                winit::keyboard::ModifiersState::SHIFT => {
                    let old_octant = view.camera.octant;
                    let new_octant = octant_change(old_octant, self.key_stroke_state);
                    if new_octant != old_octant
                    {
                        view.camera.set_octant(new_octant);
                    }
                },
                winit::keyboard::ModifiersState::ALT => {
                    let delta_dir = view.options.key_pan_delta;
                    let (del_x, del_y) = match self.key_stroke_state
                    {
                        KeyStrokeState::Left => (-delta_dir, 0.0),
                        KeyStrokeState::Right => (delta_dir, 0.0),
                        KeyStrokeState::Up => (0.0, delta_dir),
                        KeyStrokeState::Down => (0.0, -delta_dir),
                        _ => (0.0, 0.0),
                    };
                    view.camera.pan(del_x, del_y);
                }
                _ => {
                    let delta_angle = view.options.key_orbit_delta;
                    let (pitch_delta, yaw_delta) = match self.key_stroke_state
                    {
                        KeyStrokeState::Left => (0.0, -delta_angle),
                        KeyStrokeState::Right => (0.0, delta_angle),
                        KeyStrokeState::Up => (delta_angle, 0.0),
                        KeyStrokeState::Down => (-delta_angle, 0.0),
                        _ => (0.0, 0.0),
                    };
                    view.camera.orbit(pitch_delta, yaw_delta);
                }

            }
            self.key_stroke_state = KeyStrokeState::None;
        }

        // handle mouse wheel
        if let Some(mwd) = self.mouse_wheel_delta
        {
            view.camera.zoom(mwd * view.options.zoom_speed);
            self.mouse_wheel_delta = None;
        }

        view.update_uniform();
    }
    //}}}
}
//}}}
//{{{ fun: octant_change
fn octant_change(
    octant: i8,
    key_stroke_state: KeyStrokeState,
) -> i8
{
    let octant_out = match key_stroke_state
    {
        KeyStrokeState::Left =>
        {
            if octant < 4
            {
                wrap(octant - 1, 4)
            }
            else
            {
                wrap(octant - 5, 4) + 4
            }
        }
        KeyStrokeState::Right =>
        {
            if octant < 4
            {
                wrap(octant + 1, 4)
            }
            else
            {
                wrap(octant + 3, 4)
            }
        }
        KeyStrokeState::Up =>
        {
            if octant < 4
            {
                octant + 4
            }
            else
            {
                octant
            }
        }
        KeyStrokeState::Down =>
        {
            if octant >= 4
            {
                octant - 4
            }
            else
            {
                octant
            }
        }
        _ => octant,
    };

    octant_out
}
//..................................................................................................
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
    use nalgebra::vector;
    use winit::event::ElementState;

    use super::*;

    fn build_view() -> View
    {
        let pos = vector![3.0, 3.0, 3.0];

        let focus = vector![0.0, 0.0, 0.0];

        let dir = (focus - pos).normalize();

        let (pitch, yaw) = pitch_and_yaw(&dir);

        let fov = rad(90.0);

        let near = 0.1;

        let far = 100.0;

        let aspect = 2.0;

        let mut view = View {
            options: ViewOptions::default(),
            camera: Camera {
                position: pos,
                focus: focus,
                pitch: pitch,
                yaw: yaw,
                octant: octant(&(pos - focus)),
            },
            projection: Projection {
                fov,
                near,
                far,
                aspect,
            },
            uniform: ViewUniform::default(),
        };

        view.update_uniform();

        view
    }

    #[test]

    fn camera_calc_matrix_test()
    {
        let view = build_view();
        let mat = view.camera.calc_matrix();
    }

    #[test]

    fn projection_calc_matrix_test()
    {
        let proj = Projection {
            fov: rad(45.0),
            near: 0.1,
            far: 100.0,
            aspect: 2.0,
        };

        let mat = proj.calc_matrix();

    }

    #[test]

    fn view_calc_matrix_test()
    {
        let view = build_view();

        let view_mat = view.calc_matrix();

        {
            let p1 = vector![0.0, 0.0, 0.0, 1.0];
            let p2 = view_mat * p1;
        }
        {
            let p1 = vector![1.0, 0.0, 0.0, 1.0];
            let p2 = view_mat * p1;
        }
        {
            let p1 = vector![0.0, 1.0, 0.0, 1.0];
            let p2 = view_mat * p1;
        }
    }

    //{{{ test: left_key_stroke_test
    #[test]
    fn left_key_stroke_test()
    {
        let state = ElementState::Pressed;

        let key = winit::keyboard::NamedKey::ArrowLeft;

        let mut view = build_view();

        let mut view_controller = EventController::default();

        view_controller.key_modifiers_update(winit::keyboard::ModifiersState::SHIFT);

        assert_eq!(view.camera.octant, 0);
        view_controller.key_input_update(state, key);
        view_controller.update_view_3d(&mut view);
        assert_eq!(view.camera.octant, 3);
        view_controller.key_input_update(state, key);
        view_controller.update_view_3d(&mut view);
        assert_eq!(view.camera.octant, 2);
        view_controller.key_input_update(state, key);
        view_controller.update_view_3d(&mut view);
        assert_eq!(view.camera.octant, 1);
        view_controller.key_input_update(state, key);
        view_controller.update_view_3d(&mut view);
        assert_eq!(view.camera.octant, 0);
        view_controller.key_input_update(state, key);
        view_controller.update_view_3d(&mut view);
        assert_eq!(view.camera.octant, 3);
        view_controller.key_input_update(state, key);
        view_controller.update_view_3d(&mut view);
        assert_eq!(view.camera.octant, 2);
        view_controller.key_input_update(state, key);
        view_controller.update_view_3d(&mut view);
        assert_eq!(view.camera.octant, 1);
    }
    //}}}
}
//}}}
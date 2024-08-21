//! Implements the Camera functionality for a 2D scene.
//!
//! Includes functions for updating the camera's position, rotation, and zoom level.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use crate::events::*;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use bytemuck::{Pod, Zeroable};
use::topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

const ZOOM_MIN: f32 = 0.001;
const ZOOM_MAX: f32 = 10.0;

//{{{ collection: Camera
//{{{ struct: Camera
/// Represents a cameral in a 2D scene. 
/// 
/// This camera has a 2D position, rotation, and zoom level.
#[derive(Debug)]
pub struct Camera
{
    /// Position of camera in the global 2D coordinates of the scene
    pub position: Vec2,
    /// Rotation of the camera in radians, measured counter-clockwise from the positive x-axis
    pub rotation: f32,
    /// Zoom level of the camera, where 1.0 is the default zoom
    pub zoom: f32,

    x_axis: Vec2,
    y_axis: Vec2,
}
//..................................................................................................
//}}}
//{{{ impl: Camera
impl Camera 
{
    //{{{ fun: zoom
    pub fn zoom(
        &mut self,
        delta: f32,
    )
    {
        //{{{ trace
        debug!("delta by {}", delta);
        debug!("old zoom: {}", self.zoom);
        //}}}
        self.zoom += delta;
        self.zoom = self.zoom.clamp(ZOOM_MIN, ZOOM_MAX);
        //{{{ trace
        debug!("new zoom: {}", self.zoom);
        //}}}
    }
    //}}}
    //{{{ fun: rotate
    pub fn rotate(&mut self, delta: f32)
    {
        //{{{ trace
        debug!("delta by {}", delta);
        //}}}
        self.rotation += delta;
        let s = self.rotation.sin();
        let c = self.rotation.cos();
        self.x_axis[0] = c;
        self.x_axis[1] = s;
        self.y_axis[0] = -s;
        self.y_axis[1] = c;
    }
    //}}}
    //{{{ fun: pan
    pub fn pan(&mut self, delta_x: f32, delta_y: f32)   
    {
        //{{{ trace
        debug!("delta_x: {}, delta_y: {}", delta_x, delta_y);
        //}}}
        self.position[0] += delta_x * self.x_axis[0] + delta_y * self.y_axis[0];
        self.position[1] += delta_x * self.x_axis[1] + delta_y * self.y_axis[1];
    }
    //}}}
}
//..................................................................................................
//}}}
//{{{ impl Default for Camera
impl Default for Camera
{
    //{{{ fun: default
    fn default() -> Self {
        Self {
            position: Vec2::zeros(),
            rotation: 0.0,
            zoom: 1.0,
            x_axis: Vec2::new(1.0, 0.0),
            y_axis: Vec2::new(0.0, 1.0),
        }
    }
    //}}}
}
//..................................................................................................
//}}}
//}}}
//{{{ collection: ViewOptions
//{{{ struct: ViewOptions
/// Short Description
///
/// Longer Description
#[derive(Debug)]
pub struct ViewOptions
{
    /// This scales distance the camera will move along its lateral direction vectors when
    /// the arrow keys are pressed with the alt key held down.
    /// The formula for the scaling is:
    ///
    pub key_pan_delta: f32,
    /// The angle of rotation for every press of the SHIFT+LEFT (counter-clockwise) or 
    /// SHIFT+RIGHT (clockwise) arrow keys 
    pub rotate_delta: f32,
    /// This is the sensitivety of the mouse wheel when moving forward and backward
    pub zoom_speed: f32,
   
}
//}}}
//{{{ impl Default for ViewOptions
impl Default for ViewOptions
{
    fn default() -> Self
    {
        Self {
            key_pan_delta: 0.25,
            rotate_delta: rad(2.5),
            zoom_speed: 0.001,
        }
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ collection: View
//{{{ struct: View
/// Short Description
///
/// Longer Description
#[derive(Debug)]
pub struct View
{
    pub options: ViewOptions,
    pub uniform: ViewUniform,   
    camera: Camera, 
}
//}}}
//{{{ impl: View
impl View 
{
    pub fn update_uniform(&mut self)
    {
        //{{{ trace
        debug!("updating uniform with camera:");
        debug!("{:?}", self.camera);
        //}}}
        let theta = self.camera.rotation;
        let s = theta.sin();
        let c = theta.cos();
        let rot_matrix = Mat4::new(  c,  -s, 0.0, 0.0, 
                                     s,   c, 0.0, 0.0,  
                                   0.0, 0.0, 1.0, 0.0, 
                                   0.0, 0.0, 0.0, 0.0 );
        let rho = self.camera.zoom;
        let dx = self.camera.position[0];
        let dy = self.camera.position[1];   
        //{{{ trace
        debug!("theta: {}, rho: {}, dx: {}, dy: {}", theta, rho, dx, dy);
        //}}}
        let zoom_pan_matrix = Mat4::new(  rho, 0.0, dx, 0.0,
                                          0.0, rho, dy, 0.0,              
                                          0.0, 0.0, 1.0, 0.0,
                                          0.0, 0.0, 0.0, 0.0 );


        let view_matrix = rot_matrix * zoom_pan_matrix; 
        //{{{ trace
        trace!(target: "update_uniform", "rot_matrix: {}", rot_matrix);
        trace!(target: "update_uniform", "zoom_pan_matrix: {}", zoom_pan_matrix);
        trace!(target: "update_uniform", "view_matrix: {}", view_matrix);
        //}}}
        self.uniform.view_matrix = view_matrix.into();
    }
}
//}}}
//{{{ impl Default for View
impl Default for View
{
    fn default() -> Self {
        Self {
            options: ViewOptions::default(),
            uniform: ViewUniform::default(),
            camera: Camera::default(),
        }
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ struct: ViewUniform
/// Short Description
///
/// Longer Description
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct  ViewUniform
{
    view_matrix: [[f32; 4]; 4],
}

impl Default for ViewUniform
{
    fn default() -> Self {
        Self {
            view_matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 0.0],
            ], 
        }
    }
}
//..................................................................................................
//}}}
//{{{ impl: EventController
impl EventController
{
    pub fn update_view_2d(&mut self, view: &mut View)
    {

        // handle resizing
        if let ResizedState::Resized(_) = self.resized_state
        {
            self.resized_state = ResizedState::NotResized;
        }

        // handle key stroke to change octant of camera
        if self.key_stroke_state != KeyStrokeState::None
        {
            if self.key_modifier_state == winit::keyboard::ModifiersState::SHIFT
            {
                let delta_angle = match self.key_stroke_state
                {
                    KeyStrokeState::Left => -view.options.rotate_delta,
                    KeyStrokeState::Right => view.options.rotate_delta,
                    _ => 0.0
                };
                view.camera.rotate(delta_angle);
            }
            else
            {
                let pan_dist = view.options.key_pan_delta * (1.0 / (view.camera.zoom)).sqrt();

                let displ = match self.key_stroke_state
                {
                    KeyStrokeState::Left => pan_dist * view.camera.x_axis,
                    KeyStrokeState::Right => -pan_dist * view.camera.x_axis,
                    KeyStrokeState::Up =>  -pan_dist * view.camera.y_axis,
                    KeyStrokeState::Down =>  pan_dist * view.camera.y_axis,
                    _ => Vec2::zeros(),
                };
                view.camera.pan(displ[0], displ[1]);
            }
            self.key_stroke_state = KeyStrokeState::None;
        }

        // handle mouse wheel
        if let Some(mwd) = self.mouse_wheel_delta
        {
            let zoom_delta = mwd * view.options.zoom_speed * view.camera.zoom.sqrt();
            view.camera.zoom(zoom_delta);
            self.mouse_wheel_delta = None;
        }

        view.update_uniform();   
    }
}
//..................................................................................................
//}}}


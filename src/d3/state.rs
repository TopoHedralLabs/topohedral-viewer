//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::{
    camera::View,
    mesh::{LineDescriptor, TriangleDescriptor, PlaneDescriptor, CuboidDescriptor, CylinderDescriptor, SphereDescriptor, 
    AxesDescriptor, Mesh, Mesh3D},
    vertex::Vertex
};
use crate::core::{StateCore, ViewStateCore};
use crate::events::EventController; 
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


#[derive(Default, Debug)]
pub struct ViewState
{
    view: View,
    view_controller: EventController,
}
//..................................................................................................

impl ViewState
{
    pub fn new() -> Self
    {
        Self {
            view: View::default(),
            view_controller: EventController::default(),
        }
    }
}
impl ViewStateCore for ViewState
{
    fn update(&mut self)
    {
        self.view_controller.update_view_3d(&mut self.view)
    }

    fn view_controller(&mut self) -> &mut EventController
    {
        &mut self.view_controller
    }

    fn view_uniform_buffer(&self) -> &[u8]
    {
        bytemuck::bytes_of(&self.view.uniform)
    }
}
//..................................................................................................

pub type State<'a> = StateCore<'a, Vertex, ViewState>;

pub trait State3D<'a>
{
    fn add_line(
        &mut self,
        line: &LineDescriptor,
    ) -> usize;
    fn add_triangle(
        &mut self,
        triangle: &TriangleDescriptor,
    ) -> usize;
    fn add_plane(
        &mut self,
        plane: &PlaneDescriptor,
    ) -> usize;
    fn add_cuboid(
        &mut self,
        cuboid: &CuboidDescriptor,
    ) -> usize;
    fn add_cylinder(
        &mut self,
        cylinder: &CylinderDescriptor,
    ) -> usize;
    fn add_sphere(
        &mut self,
        sphere: &SphereDescriptor,
    ) -> usize; 
    fn add_axes(
        &mut self,
        axes_desc: &AxesDescriptor,
    ) -> usize; 
}

impl<'a> State3D<'a> for State<'a>
{

    fn add_line(
        &mut self,
        line_desc: &LineDescriptor,
    ) -> usize
    {
        let line_mesh = Mesh::create_line(line_desc);
        self.add_mesh(line_mesh)
    }

    fn add_triangle(
            &mut self,
            triangle: &TriangleDescriptor,
        ) -> usize {
        let triangle_mesh = Mesh::create_triangle(triangle);
        self.add_mesh(triangle_mesh)
    }

    fn add_plane(
        &mut self,
        plane_desc: &PlaneDescriptor,
    ) -> usize
    {
        let plane_mesh = Mesh::create_plane(plane_desc);
        self.add_mesh(plane_mesh)
    }

    fn add_cuboid(
        &mut self,
        cuboid_desc: &CuboidDescriptor,
    ) -> usize
    {
        let cuboid_mesh = Mesh::create_cuboid(cuboid_desc);
        self.add_mesh(cuboid_mesh)
    }

    fn add_cylinder(
        &mut self,
        cyl_desc: &CylinderDescriptor,
    ) -> usize
    {
        let cyl_mesh = Mesh::create_cylinder(cyl_desc);
        self.add_mesh(cyl_mesh)
    }

    fn add_sphere(
        &mut self,
        sphere_desc: &SphereDescriptor,
    ) -> usize
    {
        let sphere_mesh = Mesh::create_sphere(sphere_desc);
        self.add_mesh(sphere_mesh)
    }

    fn add_axes(
        &mut self,
        axes_desc: &AxesDescriptor,
    ) -> usize
    {
        let axes_mesh = Mesh::create_axes(axes_desc);
        self.add_mesh(axes_mesh)
    }
}
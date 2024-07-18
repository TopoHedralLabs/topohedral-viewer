//.................................. std
//.................................. 3rd party
//.................................. crate
use super::{
    camera::View,
    mesh::{AxesDescriptor, CircleDescriptor, Mesh, Mesh2D, SquareDescriptor},
    vertex::Vertex,
};
use crate::core::{StateCore, ViewStateCore};
use crate::events::EventController;
//..................................................................................................

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
        self.view_controller.update_view_2d(&mut self.view)
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

pub trait State2D<'a>
{
    fn add_axes(
        &mut self,
        axes_desc: &AxesDescriptor,
    ) -> usize;

    fn add_square(
        &mut self,
        square_desc: &SquareDescriptor,
    ) -> usize;

    fn add_circle(
        &mut self,
        circle_desc: &CircleDescriptor,
    ) -> usize; 
}

impl<'a> State2D<'a> for State<'a>
{
    fn add_axes(
        &mut self,
        axes_desc: &AxesDescriptor,
    ) -> usize
    {
        let axes_mesh = Mesh::create_axes(axes_desc);
        self.add_mesh(axes_mesh)
    }

    fn add_square(
        &mut self,
        square_desc: &SquareDescriptor,
    ) -> usize
    {
        let square_mesh = Mesh::create_square(square_desc);
        self.add_mesh(square_mesh)
    }

    fn add_circle(
        &mut self,
        circle_desc: &CircleDescriptor,
    ) -> usize
    {
        let circle_mesh = Mesh::create_circle(circle_desc);
        self.add_mesh(circle_mesh)
    }
}

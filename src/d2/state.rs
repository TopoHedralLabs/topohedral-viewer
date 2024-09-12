//! The 2D state module for the TopoViewer application.
//!
//! Contains the 2D-specific state and logic for the TopoViewer application.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::{
    camera::View,
    mesh::{AxesDescriptor, CircleDescriptor, Mesh, Mesh2D, SquareDescriptor, LineDescriptor},
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

//{{{ collection: ViewState
//{{{ struct: VeiwState
#[derive(Default, Debug)]
pub struct ViewState
{
    view: View,
    view_controller: EventController,
}
//..................................................................................................
//}}}
//{{{ impl: ViewState
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
//}}}
//{{{ impl: ViewStateCore for ViewState
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
//}}}
//}}}
//{{{ type: State
pub type State<'a> = StateCore<'a, Vertex, ViewState>;
//}}}
//{{{ trait: State2D
pub trait State2D<'a>
{
    fn add_axes(
        &mut self,
        axes_desc: &AxesDescriptor,
    ) -> usize;

    fn add_line(
        &mut self, 
        line_desc: &LineDescriptor,
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
//}}}
//{{{ impl: State2D for State
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

    fn add_line(
        &mut self, 
        line_desc: &LineDescriptor,
    ) -> usize
    {
        let line_mesh = Mesh::create_line(line_desc);
        self.add_mesh(line_mesh)
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
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
}
//}}}
//! This module contains the implementation of the `Mesh` struct for the 2D viewer.
//!
//! This module defines the set of items which may be  drawn in the 2D viewer and provides an 
//! API for each type of item. The general item will simply take a mesh and draw it. 
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::*;
use crate::core::MeshCore;
use super::vertex::{Vertex, VertexDescriptor};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use serde::{Deserialize, Serialize};    
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ type: Mesh
pub type Mesh<'a> = MeshCore<'a, Vertex>;
//}}}
//{{{ struct: AxesDescriptor
#[derive(Deserialize, Serialize)]
pub struct AxesDescriptor
{
    pub origin: Vec2,
    pub x_axis: Vec2,
    pub y_axis: Vec2,
    pub neg_len: f32,
    pub pos_len: f32,
}
//..................................................................................................
//}}}
//{{{ struct: SquareDescriptor
#[derive(Deserialize, Serialize)]
pub struct SquareDescriptor
{
    pub origin: Vec2,
    pub x_axis: Vec2,
    pub y_axis: Vec2,
    pub lenx: f32, 
    pub leny: f32,
    pub line_color: Color,
    pub tri_color: Color,
    pub cell_type: CellType,
}
//..................................................................................................
//}}}
//{{{ struct: CircleDescriptor
#[derive(Deserialize, Serialize)]
pub struct CircleDescriptor
{
    pub center: Vec2,
    pub radius: f32,
    pub line_color: Color,
    pub tri_color: Color,
    pub cell_type: CellType,
}
//..................................................................................................
//}}}
//{{{ trait: Mesh2D
/// Defines a trait for creating and manipulating 2D meshes.
///
/// The `Mesh2D` trait provides a set of methods for creating and modifying 2D meshes, including:
///
/// - `create_axes`: Creates a mesh representing a set of coordinate axes.
/// - `create_square`: Creates a mesh representing a 2D square.
/// - `create_circle`: Creates a mesh representing a 2D circle.
/// - `add_line`: Adds a line segment to the mesh.
/// - `add_triangle`: Adds a triangle to the mesh.
///
/// Implementations of this trait can be used to generate and manipulate 2D meshes for various purposes, such as rendering or visualization.
pub trait Mesh2D<'a>
{
    fn create_axes(
        axes: &AxesDescriptor,
    ) -> Self;

    fn create_square(
        square: &SquareDescriptor,
    ) -> Self;

    fn create_circle(
        circle: &CircleDescriptor,
    ) -> Self;

    fn add_line(
        &mut self,
        v1: &Vec2,
        v2: &Vec2,
        line_color: &Color,
        tri_color: &Color,
    );

    fn add_triangle(
        &mut self,
        v1: &Vec2,
        v2: &Vec2,
        v2: &Vec2,
        line_color: &Color,
        tri_color: &Color,
    );
}
//..................................................................................................
//}}}
//{{{ impl: Mesh2D for Mesh
impl<'a> Mesh2D<'a> for Mesh<'a>
{
    //{{{ fun: create_axes
    fn create_axes(
        axes: &AxesDescriptor,
    ) -> Self
    {
        let mut mesh = Mesh::from_num_lines(2);
        let v1 = axes.origin - axes.x_axis * axes.neg_len;
        let v2 = axes.origin + axes.x_axis * axes.pos_len;
        mesh.add_line(&v1, &v2, &Color::Red, &Color::default());
        let v3 = axes.origin - axes.y_axis * axes.neg_len;
        let v4 = axes.origin + axes.y_axis * axes.pos_len;
        mesh.add_line(&v3, &v4, &Color::Green, &Color::default());
        mesh

    }
    //}}}
    //{{{ fun: create_square
    /// Creates a 2D square mesh based on the provided `SquareDescriptor`.
    ///
    /// The `create_square` function takes a `SquareDescriptor` as input and returns a new `Mesh2D` instance. The function handles two cases:
    ///
    /// 1. `CellType::Line`: Creates a mesh with 4 lines representing the square.
    /// 2. `CellType::Triangle`: Creates a mesh with 2 triangles representing the square.
    ///
    /// The function calculates the vertices of the square based on the `origin`, `x_axis`, `y_axis`, `lenx`, and `leny` fields of the `SquareDescriptor`. It then adds the lines or triangles to the mesh using the `add_line` and `add_triangle` methods.
    ///
    /// If the `cell_type` field of the `SquareDescriptor` is not `CellType::Line` or `CellType::Triangle`, the function will panic with the message "Unknown cell type".
    fn create_square(
        square_disc: &SquareDescriptor,
    ) -> Self
    {
        match(square_disc.cell_type) {
            //{{{ case: CellType::Line
            CellType::Line => {
                let mut mesh = Mesh::from_num_lines(4);

                let o = square_disc.origin;
                let dx = square_disc.x_axis * square_disc.lenx;
                let dy =  square_disc.y_axis * square_disc.leny;

                let v0 = o;
                let v1 = o + dx;
                let v2 = o + dx + dy;
                let v3 = o + dy;
                mesh.add_line(&v0, &v1, &square_disc.line_color, &square_disc.tri_color);
                mesh.add_line(&v1, &v2, &square_disc.line_color, &square_disc.tri_color);
                mesh.add_line(&v2, &v3, &square_disc.line_color, &square_disc.tri_color);
                mesh.add_line(&v3, &v0, &square_disc.line_color, &square_disc.tri_color);
                mesh
            },
            //}}}
            //{{{ case: CellType::Triangle
            CellType::Triangle => {
                let mut mesh = Mesh::from_num_triangles(2);

                let o = square_disc.origin;
                let dx = square_disc.x_axis * square_disc.lenx;
                let dy =  square_disc.y_axis * square_disc.leny;

                let v0 = o;
                let v1 = o + dx;
                let v2 = o + dx + dy;
                let v3 = o + dy;
                mesh.add_triangle(&v0, &v1, &v2,&square_disc.line_color, &square_disc.tri_color);
                mesh.add_triangle(&v0, &v2, &v3,&square_disc.line_color, &square_disc.tri_color); 
                mesh
            },
            //}}}
            //{{{ default
            _ => {
                panic!("Unknown cell type");
            }
            //}}}
        }


    }
    //}}}
    //{{{ fun: create_circle
    fn create_circle(
        circle: &CircleDescriptor,
    ) -> Self
    {
        todo!()
    }
    //}}}
    //{{{ fun: add_line
    fn add_line(
        &mut self,
        v1: &Vec2,
        v2: &Vec2,
        line_color: &Color,
        tri_color: &Color,
    )
    {
        assert!(self.is_line());

        let nv = self.num_vertices() as u32;
        let indices = [nv, nv + 1];
        self.append_indices(&indices);

        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v1,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v2,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
    }
    //}}}
    //{{{ fun: add_triangle
    fn add_triangle(
        &mut self,
        v1: &Vec2,
        v2: &Vec2,
        v3: &Vec2,
        line_color: &Color,
        tri_color: &Color,
    )
    {
        assert!(self.is_triangle());
        let nv = self.num_vertices() as u32;
        let indices = [nv, nv + 1, nv + 2];
        self.append_indices(&indices);

        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v1,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v2,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v3,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
    }
    //}}}
}
//..................................................................................................
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
    use super::*;

    #[test]
    fn create_axes_test()
    {
        let axes_disc = AxesDescriptor{
            origin: Vec2::new(0.0, 0.0),
            x_axis: Vec2::new(1.0, 0.0),
            y_axis: Vec2::new(0.0, 1.0),
            neg_len: 10.0,
            pos_len: 1.0,
        };
        let mesh = Mesh::create_axes(&axes_disc);
    }

    #[test]
    fn create_square_test()
    {
        let square_disc = SquareDescriptor{
            origin: Vec2::new(0.0, 0.0),
            x_axis: Vec2::new(1.0, 0.0),
            y_axis: Vec2::new(0.0, 1.0),
            lenx: 10.0,
            leny: 1.0,
            line_color: Color::Red,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        };
        let mesh = Mesh::create_square(&square_disc);
    }


}
//}}}
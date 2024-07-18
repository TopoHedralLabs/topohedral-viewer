//.................................. std
//.................................. 3rd party
use serde::{Deserialize, Serialize};    
//.................................. crate
use super::vertex::{Vertex, VertexDescriptor};
use crate::common::*;
use crate::core::MeshCore;

pub type Mesh<'a> = MeshCore<'a, Vertex>;

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

#[derive(Deserialize, Serialize)]
pub struct CircleDescriptor
{
    pub center: Vec2,
    pub radius: f32,
    pub line_color: Color,
    pub tri_color: Color,
}
//..................................................................................................

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

impl<'a> Mesh2D<'a> for Mesh<'a>
{
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

    fn create_square(
        square: &SquareDescriptor,
    ) -> Self
    {
        let mut mesh = Mesh::from_num_triangles(2);

        let o = square.origin;
        let dx = square.x_axis * square.lenx;
        let dy =  square.y_axis * square.leny;

        let v0 = o;
        let v1 = o + dx;
        let v2 = o + dx + dy;
        let v3 = o + dy;
        mesh.add_triangle(&v0, &v1, &v2,&square.line_color, &square.tri_color);
        mesh.add_triangle(&v0, &v2, &v3,&square.line_color, &square.tri_color); 
        mesh

    }

    fn create_circle(
        circle: &CircleDescriptor,
    ) -> Self
    {
        todo!()
    }

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
}
//..................................................................................................



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


use crate::{common::*, core::VertexCore, core::VertexViewCore};
use bytemuck;
use serde::{Deserialize, Serialize};


pub struct VertexDescriptor {
    pub position: Vec3, 
    pub normal: Vec3,   
    pub line_color: Color,
    pub triangle_color: Color,
}
//..................................................................................................

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Serialize, Deserialize)]
pub struct Vertex {

    /// The internal data has the layout: ``[position normal color]`` where:
    /// - position: [x, y, z]
    /// - normal: [nx, ny, nz]
    /// - line_color : [r, g, b]
    /// - triangle_color : [r, g, b]
    data: [f32; 12],
}

impl Vertex {

    pub fn new(vert_disc: &VertexDescriptor) -> Vertex
    {
        let mut data = [0.0; 12];
        data[0..3].copy_from_slice(vert_disc.position.as_slice());
        data[3..6].copy_from_slice(vert_disc.normal.as_slice());
        data[6..9].copy_from_slice(&vert_disc.line_color.to_rgb());
        data[9..12].copy_from_slice(&vert_disc.triangle_color.to_rgb());
        Vertex { data }
    }

    fn normal_offset() -> usize {
        3
    }

}

impl VertexCore for Vertex {

    type Vec = Vec3;    

    fn to_slice(&self) -> &[f32] {
        &self.data
    }

    fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 6]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }

    fn position_offset() -> usize 
    {
        0
    }


    fn line_color_offset() -> usize {
        6
    }

    fn triangle_color_offset() -> usize {   
        9
    }

    fn len() -> usize {
       12 
    }

    fn dim() -> usize {
        3
    }
}
//..................................................................................................

pub struct VertexView<'a>
{
    vertex_data: &'a mut [f32],
}

impl<'a> VertexView<'a> {
    fn set_normal(&mut self, norm: &Vec3) {

        let start = Vertex::normal_offset();
        let end  = Vertex::normal_offset() + 3;
        self.vertex_data[start..end].copy_from_slice(norm.as_slice());
    }

    fn get_normal(&self) -> Vec3 {
        let start = Vertex::normal_offset();
        let end = Vertex::normal_offset() + 3;
        Vec3::from_column_slice(&self.vertex_data[start..end])
    }   
}


impl<'a> VertexViewCore<'a> for VertexView<'a> {  

    type Vec = Vec3;

    fn new(vert_slice: &'a mut [f32]) -> Self {
        VertexView { vertex_data: vert_slice }
    }

    fn set_position(&mut self, pos: &Vec3) {
        let start = Vertex::position_offset();
        let end  = Vertex::position_offset() + 3;
        self.vertex_data[start..end].copy_from_slice(pos.as_slice());
    }

    fn get_position(&self, ) -> Vec3 {
        let start = Vertex::position_offset();
        let end = Vertex::position_offset() + 3;
        Vec3::from_column_slice(&self.vertex_data[start..end])
    }


    fn set_line_color(&mut self, color:  &[f32; 3]) {
        let start = Vertex::line_color_offset();
        let end = Vertex::line_color_offset() + 3;
        self.vertex_data[start..end].copy_from_slice(color);
    }   

    fn get_line_color(&self) -> [f32; 3] {
        let start = Vertex::line_color_offset();
        let end = Vertex::line_color_offset() + 3;
        let mut color = [0.0; 3];
        color.copy_from_slice(&self.vertex_data[start..end]);
        color
    }

    fn set_triangle_color(&mut self, color: &[f32; 3]) {
        let start = Vertex::triangle_color_offset();
        let end = Vertex::triangle_color_offset() + 3;
        self.vertex_data[start..end].copy_from_slice(color);
    }

    fn get_triangle_color(&self) -> [f32; 3] {
        let start = Vertex::triangle_color_offset();
        let end = Vertex::triangle_color_offset() + 3;
        let mut color = [0.0; 3];
        color.copy_from_slice(&self.vertex_data[start..end]);
        color
    }
}
//..................................................................................................
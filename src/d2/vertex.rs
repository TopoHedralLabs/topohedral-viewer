use crate::common::*;
use crate::core::{VertexCore, VertexViewCore};

use serde::{Deserialize, Serialize};
use bytemuck;

pub struct VertexDescriptor
{
    pub position: Vec2,
    pub line_color: Color,
    pub triangle_color: Color,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable, Serialize, Deserialize)]
pub struct Vertex
{
    /// The internal data has the layout:
    /// - position: [x, y]
    /// - line_color : [r, g, b]
    /// - triangle_color : [r, g, b]
    data: [f32; 8],
}

impl Vertex
{
    pub fn new(vert_desc: &VertexDescriptor) -> Vertex
    {
        let mut data = [0.0; 8];
        data[0..2].copy_from_slice(vert_desc.position.as_slice());
        data[2..5].copy_from_slice(&vert_desc.line_color.to_rgb());
        data[5..8].copy_from_slice(&vert_desc.triangle_color.to_rgb());
        Vertex { data }
    }
}

impl VertexCore for Vertex
{

    type Vec = Vec2;

    fn to_slice(&self) -> &[f32]
    {
        &self.data
    }

    fn desc() -> wgpu::VertexBufferLayout<'static>
    {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ],
        }
    }

    fn position_offset() -> usize
    {
        0
    }

    fn line_color_offset() -> usize
    {
        2
    }

    fn triangle_color_offset() -> usize
    {
        5
    }

    fn len() -> usize
    {
        8
    }

    fn dim() -> usize
    {
        2
    }
}
//..................................................................................................

pub struct VertexView<'a>
{
    vertex_data: &'a mut [f32],
}
//..................................................................................................

impl<'a> VertexViewCore<'a> for VertexView<'a>
{
    type Vec = Vec2;

    fn new(vert_slice: &'a mut [f32]) -> Self
    {
        VertexView {
            vertex_data: vert_slice,
        }
    }

    fn set_position(
        &mut self,
        pos: &Vec2,
    )
    {
        let start = Vertex::position_offset();
        let end = Vertex::position_offset() + 2;
        self.vertex_data[start..end].copy_from_slice(pos.as_slice());
    }

    fn get_position(&self) -> Vec2
    {
        let start = Vertex::position_offset();
        let end = Vertex::position_offset() + 3;
        Vec2::from_column_slice(&self.vertex_data[start..end])
    }

    fn set_line_color(
        &mut self,
        color: &[f32; 3],
    )
    {
        let start = Vertex::line_color_offset();
        let end = Vertex::line_color_offset() + 3;
        self.vertex_data[start..end].copy_from_slice(color);
    }

    fn get_line_color(&self) -> [f32; 3]
    {
        let start = Vertex::line_color_offset();
        let end = Vertex::line_color_offset() + 3;
        let mut color = [0.0; 3];
        color.copy_from_slice(&self.vertex_data[start..end]);
        color
    }

    fn set_triangle_color(
        &mut self,
        color: &[f32; 3],
    )
    {
        let start = Vertex::triangle_color_offset();
        let end = Vertex::triangle_color_offset() + 3;
        self.vertex_data[start..end].copy_from_slice(color);
    }

    fn get_triangle_color(&self) -> [f32; 3]
    {
        let start = Vertex::triangle_color_offset();
        let end = Vertex::triangle_color_offset() + 3;
        let mut color = [0.0; 3];
        color.copy_from_slice(&self.vertex_data[start..end]);
        color
    }
}
//..................................................................................................

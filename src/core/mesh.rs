//! This module defines the core mesh funcitonality shared between the 2D and 3D viewers.
//!
//! Because the topology of the mesh is the same in both viewers, that being lines and triangles, 
//! much of the functionality is shared between the two. The main difference is the vertex type, 
//! which for 2D is 8 f32s, (x,y) and a triplet of rgbs for lines and triangles. For 3D is 9 f32s, 
//! ((x,y,z) and a triplet of rgbs for lines and triangles.  
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{CellType, Color};
use crate::Colormap;
//}}}
//{{{ std imports 
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
//}}}
//{{{ dep imports 
use serde::{Deserialize, Serialize};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ trait: VertexViewCore
/// A trait that defines the core functionality for a vertex view in a mesh.
///
/// This trait provides methods for interacting with the vertex data of a vertiex already in a mesh, 
/// such as setting and getting the position, line color, and triangle color.
pub trait VertexViewCore<'a>
{
    type Vec;
    fn new(vert_slice: &'a mut [f32]) -> Self;
    fn set_position(
        &mut self,
        pos: &Self::Vec,
    );
    fn get_position(&self) -> Self::Vec;
    fn set_line_color(
        &mut self,
        color: &[f32; 3],
    );
    fn get_line_color(&self) -> [f32; 3];
    fn set_triangle_color(
        &mut self,
        color: &[f32; 3],
    );
    fn get_triangle_color(&self) -> [f32; 3];
}
//..................................................................................................
//}}}
//{{{ trait: VertexCore
/// A trait that defines the core functionality for a vertex in a mesh.
///
/// This trait provided the key query methods needed to define the memory layout of a vertex 
/// for use in wgpu
pub trait VertexCore
{
    type Vec;
    /// Returns a slice of the vertex data.
    fn to_slice(&self) -> &[f32];

    /// Returns the vertex buffer layout for the mesh vertex type.
    /// This layout is used to define how vertex data is structured in a wgpu vertex buffer.
    fn desc() -> wgpu::VertexBufferLayout<'static>;

    /// Returns the offset in the vertex data slice where the position data is stored.
    fn position_offset() -> usize;

    /// Returns the offset in the vertex data slice where the line color data is stored.
    fn line_color_offset() -> usize;

    /// Returns the offset in the vertex data slice where the triangle color data is stored.
    fn triangle_color_offset() -> usize;

    /// Returns the number of elements in the vertex data slice.
    fn len() -> usize;

    /// Returns the dimension of the uncerlying vector type of the vertex.
    fn dim() -> usize;
}
//..................................................................................................
//}}}
//{{{ collection: MeshCore
//{{{ struct: MeshCore
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MeshCore<'a, V>
where
    V: VertexCore + Deserialize<'a> + Serialize,
{
    pub(crate) vertices: Vec<f32>,
    pub(crate) indices: Vec<u32>,
    pub(crate) cell_type: CellType,
    #[serde(skip)]
    pub uid: usize,
    #[serde(skip)]
    pub(crate) phant: PhantomData<&'a V>,
}
//}}}
//{{{ impl: MeshCore
impl<'a, V> MeshCore<'a, V>
where
    V: VertexCore + Deserialize<'a> + Serialize,
{
    pub fn from_num_lines(num_lines: usize) -> Self
    {
        Self {
            vertices: Vec::<f32>::with_capacity(2 * V::len() * num_lines),
            indices: Vec::<u32>::with_capacity(2 * num_lines),
            cell_type: CellType::Line,
            uid: 0,
            phant: PhantomData,
        }
    }

    pub fn from_num_triangles(num_triangles: usize) -> Self
    {
        Self {
            vertices: Vec::<f32>::with_capacity(3 * V::len() * num_triangles),
            indices: Vec::<u32>::with_capacity(3 * num_triangles),
            cell_type: CellType::Triangle,
            uid: 0,
            phant: PhantomData,
        }
    }

    pub fn is_line(&self) -> bool
    {
        self.cell_type == CellType::Line
    }

    pub fn is_triangle(&self) -> bool
    {
        self.cell_type == CellType::Triangle
    }

    pub fn num_vertices(&self) -> usize
    {
        self.vertices.len() / V::len()
    }

    pub fn num_indices(&self) -> usize
    {
        self.indices.len()
    }

    pub fn num_triangles(&self) -> usize
    {
        self.indices.len() / 3
    }

    pub fn vertex_slice(&self) -> &[f32]
    {
        self.vertices.as_slice()
    }

    pub fn index_slice(&self) -> &[u32]
    {
        self.indices.as_slice()
    }

    pub fn shift_indices(
        &mut self,
        shift: usize,
    )
    {
        self.indices.iter_mut().for_each(|x| *x += shift as u32);
    }

    pub fn append_vertex(
        &mut self,
        vertex: &V,
    )
    {
        self.vertices.extend_from_slice(vertex.to_slice());
    }

    pub fn append_indices(
        &mut self,
        indices: &[u32],
    )
    {
        self.indices.extend_from_slice(indices);
    }

    pub fn merge(
        &mut self,
        mut other: MeshCore<'a, V>,
    )
    {
        assert!(self.cell_type == other.cell_type);
        let nv = self.num_vertices();
        other.shift_indices(nv);

        self.vertices.extend_from_slice(&other.vertices);
        self.indices.extend_from_slice(&other.indices);
    }

    pub fn check(&self) -> bool
    {
        let nv = self.num_vertices();

        let min_idx = self.indices.iter().min_by(|a, b| a.cmp(b)).unwrap();

        let max_idx = self.indices.iter().max_by(|a, b| a.cmp(b)).unwrap();

        min_idx >= &0 && max_idx < &(nv as u32 / 3u32)
    }

    pub fn set_line_colors(
        &mut self,
        color: Color,
    )
    {
        let nv = self.num_vertices();
        for i in 0..nv
        {
            let offset = i * V::len() + V::line_color_offset();
            self.vertices[offset..offset + 3].copy_from_slice(&color.to_rgb());
        }
    }

    pub fn set_triangle_colors(
        &mut self,
        color: Color,
    )
    {
        let nv = self.num_vertices();
        for i in 0..nv
        {
            let offset = i * V::len() + V::triangle_color_offset();
            self.vertices[offset..offset + 3].copy_from_slice(&color.to_rgb());
        }
    }

    pub fn set_line_colors_from_colormap(
        &mut self,
        fvals: &[f32],
        colormap: &str,
    )
    {
        assert_eq!(fvals.len(), self.num_vertices());
        let cmap = Colormap::new(colormap.to_string()).unwrap();

        let nv = self.num_vertices();
        for i in 0..nv
        {
            let offset = i * V::len() + V::line_color_offset();
            let color = cmap.get_color(fvals[i]);
            self.vertices[offset..offset + 3].copy_from_slice(&color);
        }
    }

    pub fn set_triangle_colors_from_colormap(
        &mut self,
        fvals: &[f32],
        colormap: &str,
    )
    {
        assert_eq!(fvals.len(), self.num_vertices());
        let cmap = Colormap::new(colormap.to_string()).unwrap();

        let nv = self.num_vertices();
        for i in 0..nv
        {
            let offset = i * V::len() + V::triangle_color_offset();
            let color = cmap.get_color(fvals[i]);
            self.vertices[offset..offset + 3].copy_from_slice(&color);
        }
    }
}
//}}}
//{{{ impl: Hash for MeshCore
impl<'a, V> Hash for MeshCore<'a, V>
where
    V: VertexCore + Deserialize<'a> + Serialize,
{
    fn hash<H: Hasher>(
        &self,
        state: &mut H,
    )
    {
        self.uid.hash(state)
    }
}
//}}}
//{{{ impl:  PartialEq for MeshCore
impl<'a, V> PartialEq for MeshCore<'a, V>
where
    V: VertexCore + Deserialize<'a> + Serialize,
{
    fn eq(
        &self,
        other: &Self,
    ) -> bool
    {
        self.uid == other.uid
    }
}
//..................................................................................................
//}}}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
}
//}}}
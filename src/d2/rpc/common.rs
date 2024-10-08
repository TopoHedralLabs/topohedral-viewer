//! Module provides conversion functions for converting between the RPC and the local types.
//!
//! 
//--------------------------------------------------------------------------------------------------


//{{{ crate imports 
use crate::common::{Vec2, Color, CellType, Validated};
use super::d2rpc;
use super::super::mesh::{AxesDescriptor, LineDescriptor, SquareDescriptor, CircleDescriptor, Mesh};
//}}}
//{{{ std imports 
use std::marker::PhantomData;
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: From<d2rpc::Vec2> for Vec2
impl From<d2rpc::Vec2> for Vec2
{
    fn from(v: d2rpc::Vec2) -> Self
    {
        Vec2::new(v.x, v.y)
    }
}
//}}}
//{{{ impl: From<Vec2> for d2rpc::Vec2
impl From<Vec2> for d2rpc::Vec2 
{
    fn from(v: Vec2) -> Self
    {
        d2rpc::Vec2 { x: v.x, y: v.y }
    }
}
//}}}

//{{{ impl: From<d2rpc::Color> for Color
impl From<d2rpc::Color> for Color
{
    fn from(c: d2rpc::Color) -> Self
    {
        Color::Other((c.r, c.g, c.b,))
    }
}
//}}}
//{{{ impl: From<Color> for d2rpc::Color
impl From<Color> for d2rpc::Color 
{
    fn from(c: Color) -> Self
    {
        let rgp = c.to_rgb();
        Self {
            r: rgp[0],
            g: rgp[1],
            b: rgp[2],
        }
    }
}
//}}}

//{{{ impl: Validated for d2rpc::AddAxesRequest
impl Validated for d2rpc::AddAxesRequest
{
    fn is_valid(&self) -> bool
    {
        let mut is_val = true;
        match self.axes_descriptor
        {
            Some(ref axes_descriptor) =>
            {
                is_val &= axes_descriptor.origin.is_some();
                is_val &= axes_descriptor.x_axis.is_some();
                is_val &= axes_descriptor.y_axis.is_some();
                is_val &= axes_descriptor.pos_len > 0.0 && axes_descriptor.neg_len > 0.0;
            }
            None =>
            {
                is_val = false;
            }
        }
        is_val
    }
}
//}}}
//{{{ impl: From<d2rpc::AxesDescriptor> for AxesDescriptor
impl From<d2rpc::AxesDescriptor> for AxesDescriptor
{
    fn from(axes_desc: d2rpc::AxesDescriptor) -> Self
    {
        let axes_desc_out = AxesDescriptor {
            origin: axes_desc.origin.unwrap().into(),
            x_axis: axes_desc.x_axis.unwrap().into(),
            y_axis: axes_desc.y_axis.unwrap().into(),
            pos_len: axes_desc.pos_len,
            neg_len: axes_desc.neg_len,
        };
        axes_desc_out
    }
}
//}}}
//{{{ impl: From<AxesDescriptor> for d2rpc::AxesDescriptor
impl From<AxesDescriptor> for d2rpc::AxesDescriptor
{
    fn from (axes_desc: AxesDescriptor) -> Self
    {
        let axes_desc_out = d2rpc::AxesDescriptor {
            origin: Some(axes_desc.origin.into()),
            x_axis: Some(axes_desc.x_axis.into()),
            y_axis: Some(axes_desc.y_axis.into()),
            pos_len: axes_desc.pos_len,
            neg_len: axes_desc.neg_len,
        };
        axes_desc_out
    }
}
//}}}

//{{{ impl: Validated for d2rpc::AddLineRequest   
impl Validated for d2rpc::AddLineRequest 
{
    fn is_valid(&self) -> bool
    {
        let mut is_val = true;
        match self.line_descriptor
        {
            Some(ref line_descriptor) =>
            {
                is_val &= line_descriptor.v1.is_some();
                is_val &= line_descriptor.v2.is_some();
            }
            None =>
            {
                is_val = false;
            }
        }
        is_val
    }
}
//}}}
//{{{  impl: From<d2rpc::LineDescriptor> for LineDescriptor
impl From<d2rpc::LineDescriptor> for LineDescriptor
{
    fn from(line_desc: d2rpc::LineDescriptor) -> Self
    {
        let line_desc_out = LineDescriptor {
            v1: line_desc.v1.unwrap().into(),
            v2: line_desc.v2.unwrap().into(),
            color: line_desc.color.unwrap().into(),
        };
        line_desc_out
    }
}
//}}}
//{{{ impl: From<LineDescriptor> for d2rpc::LineDescriptor
impl From<LineDescriptor> for d2rpc::LineDescriptor 
{
    fn from(line_desc: LineDescriptor) -> Self
    {
        let line_desc_out = d2rpc::LineDescriptor {
            v1: Some(line_desc.v1.into()),
            v2: Some(line_desc.v2.into()),
            color: Some(line_desc.color.into()),
        };
        line_desc_out
    }
}
//}}}

//{{{ impl: Validated for d2rpc::AddSquareRequest
impl Validated for d2rpc::AddSquareRequest
{
    fn is_valid(&self) -> bool
    {
        let mut is_val = true;
        match self.square_descriptor
        {
            Some(ref square_descriptor) =>
            {
                is_val &= square_descriptor.origin.is_some();
                is_val &= square_descriptor.x_axis.is_some();
                is_val &= square_descriptor.y_axis.is_some();
                is_val &= square_descriptor.lenx > 0.0 && square_descriptor.leny > 0.0;
            }
            None =>
            {
                is_val = false;
            }
        }
        is_val
    }
}
//}}}
//{{{ impl: From<d2rpc::SquareDescriptor> for SquareDescriptor
impl From<d2rpc::SquareDescriptor> for SquareDescriptor
{
    fn from(square_desc: d2rpc::SquareDescriptor) -> Self
    {
        let square_desc_out = SquareDescriptor {
            origin: square_desc.origin.unwrap().into(),
            x_axis: square_desc.x_axis.unwrap().into(),
            y_axis: square_desc.y_axis.unwrap().into(),
            lenx: square_desc.lenx,
            leny: square_desc.leny,
            line_color: square_desc.line_color.unwrap().into(),
            tri_color: square_desc.tri_color.unwrap().into(),
            cell_type: (square_desc.cell_type as i32).into()
        };
        square_desc_out
    }
}
//}}}
//{{{ impl: From<SquareDescriptor> for d2rpc::SquareDescriptor
impl From<SquareDescriptor> for d2rpc::SquareDescriptor
{
    fn from (square_desc: SquareDescriptor) -> Self
    {
        let square_desc_out = d2rpc::SquareDescriptor {
            origin: Some(square_desc.origin.into()),
            x_axis: Some(square_desc.x_axis.into()),
            y_axis: Some(square_desc.y_axis.into()),
            lenx: square_desc.lenx,
            leny: square_desc.leny,
            line_color: Some(square_desc.line_color.into()),
            tri_color: Some(square_desc.tri_color.into()),
            cell_type: (square_desc.cell_type as i32).into()
        };
        square_desc_out
    }
}
//}}}

//{{{ impl: Validated for d2rpc::AddCircleRequest
impl Validated for d2rpc::AddCircleRequest
{
    fn is_valid(&self) -> bool {
        let mut is_val = true;
        match self.circle_descriptor
        {
            Some(ref circle_descriptor) =>
            {
                is_val &= circle_descriptor.center.is_some();
                is_val &= circle_descriptor.radius > 0.0;
                is_val &= circle_descriptor.num_sides > 0;
            }
            None =>
            {
                is_val = false;
            }
        }
        is_val
    }
}

//}}}
//{{{ impl: From<d2rpc::CircleDescriptor> for CircleDescriptor
impl From<d2rpc::CircleDescriptor> for CircleDescriptor
{
    fn from(circle_desc: d2rpc::CircleDescriptor) -> Self
    {
        let circle_desc_out = CircleDescriptor {
            center: circle_desc.center.unwrap().into(),
            radius: circle_desc.radius,
            num_sides: circle_desc.num_sides,
            line_color: circle_desc.line_color.unwrap().into(),
            tri_color: circle_desc.tri_color.unwrap().into(),
            cell_type: (circle_desc.cell_type as i32).into()
        };
        circle_desc_out
    }
}
//}}}
//{{{ impl: From<CircleDescriptor> for d2rpc::CircleDescriptor
impl From<CircleDescriptor> for d2rpc::CircleDescriptor
{
    fn from (circle_desc: CircleDescriptor) -> Self
    {
        let circle_desc_out = d2rpc::CircleDescriptor {
            center: Some(circle_desc.center.into()),
            radius: circle_desc.radius,
            num_sides: circle_desc.num_sides,
            line_color: Some(circle_desc.line_color.into()),
            tri_color: Some(circle_desc.tri_color.into()),
            cell_type: (circle_desc.cell_type as i32).into()
        };
        circle_desc_out
    }
}
//}}}

//{{{ impl: Validated for d2rpc::AddMeshRequest
impl Validated for d2rpc::AddMeshRequest
{
    fn is_valid(&self) -> bool
    {
        let mut is_val = true;
        match self.mesh_descriptor
        {
            Some(ref mesh_descriptor) =>
            {
                is_val &= mesh_descriptor.vertices.len() > 0;
                is_val &= mesh_descriptor.indices.len() > 0;
            }
            None =>
            {
                is_val = false;
            }
        }
        is_val
    }
}
//}}}
//{{{ impl From<d2rpc::MeshDescriptor> for Mesh
impl<'a> From<d2rpc::MeshDescriptor> for Mesh<'a>
{
    fn from(mesh_desc: d2rpc::MeshDescriptor) -> Self
    {
        let mesh= Mesh {
            vertices: mesh_desc.vertices,
            indices: mesh_desc.indices,
            cell_type: (mesh_desc.cell_type as i32).into(),
            uid: 0,
            phant: PhantomData,
        };
        mesh
    }
}
//}}}
//{{{ impl From<Mesh> for d2rpc::MeshDescriptor
impl<'a> From<Mesh<'a>> for d2rpc::MeshDescriptor
{
    fn from(mesh: Mesh<'a>) -> Self
    {
        let mesh_desc = d2rpc::MeshDescriptor {
            vertices: mesh.vertices,
            indices: mesh.indices,
            cell_type: (mesh.cell_type as i32).into(),
        };
        mesh_desc
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
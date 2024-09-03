//! Module provides conversion functions for converting between the RPC and the local types.
//!
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use super::d3rpc;
use crate::common::{CellType, Color, Validated, Vec3};
use crate::d3::mesh::*;
//}}}
//{{{ std imports
use std::marker::PhantomData;
use std::sync::Arc;
//}}}
//{{{ dep imports
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl From<d3rpc::Vec3> for Vec3
impl From<d3rpc::Vec3> for Vec3 {
    fn from(v: d3rpc::Vec3) -> Self {
        Vec3::new(v.x, v.y, v.z)
    }
}
//}}}
//{{{ impl From<Vec3> for d3rpc::Vec3
impl From<Vec3> for d3rpc::Vec3 {
    fn from(v: Vec3) -> Self {
        d3rpc::Vec3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
//}}}

//{{{ impl From<d3rpc::Color> for Color
impl From<d3rpc::Color> for Color {
    fn from(c: d3rpc::Color) -> Self {
        Color::Other((c.r, c.g, c.b))
    }
}
//}}}
//{{{ impl From<Color> for d3rpc::Color
impl From<Color> for d3rpc::Color {
    fn from(c: Color) -> Self {
        let rgp = c.to_rgb();
        Self {
            r: rgp[0],
            g: rgp[1],
            b: rgp[2],
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddLineRequest
impl Validated for d3rpc::AddLineRequest {
    fn is_valid(&self) -> bool {
        let is_val = match self.line_descriptor {
            None => false,
            Some(ref ld) => ld.v1.is_some() && ld.v2.is_some() && ld.color.is_some(),
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::LineDescriptor> for LineDescriptor
impl From<d3rpc::LineDescriptor> for LineDescriptor {
    fn from(ld: d3rpc::LineDescriptor) -> Self {
        LineDescriptor {
            v1: ld.v1.unwrap().into(),
            v2: ld.v2.unwrap().into(),
            color: ld.color.unwrap().into(),
        }
    }
}
//}}}
//{{{ impl From<LineDescriptor> for d3rpc::LineDescriptor
impl From<LineDescriptor> for d3rpc::LineDescriptor {
    fn from(ld: LineDescriptor) -> Self {
        d3rpc::LineDescriptor {
            v1: Some(ld.v1.into()),
            v2: Some(ld.v2.into()),
            color: Some(ld.color.into()),
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddTriangleRequest
impl Validated for d3rpc::AddTriangleRequest {
    fn is_valid(&self) -> bool {
        let is_val = match self.triangle_descriptor {
            None => false,
            Some(ref td) => {
                td.v1.is_some()
                    && td.v2.is_some()
                    && td.v3.is_some()
                    && td.line_color.is_some()
                    && td.tri_color.is_some()
            }
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::TriangleDescriptor> for TriangleDescriptor 
impl From<d3rpc::TriangleDescriptor> for TriangleDescriptor 
{
    fn from(td: d3rpc::TriangleDescriptor) -> Self {
        TriangleDescriptor {
            v1: td.v1.unwrap().into(),
            v2: td.v2.unwrap().into(),
            v3: td.v3.unwrap().into(),
            line_color: td.line_color.unwrap().into(),
            tri_color: td.tri_color.unwrap().into(),
            cell_type: td.cell_type.into(),
        }
    }
}
//}}}
//{{{ impl From<TriangleDescriptor> for d3rpc::TriangleDescriptor
impl From<TriangleDescriptor> for d3rpc::TriangleDescriptor {
    fn from(td: TriangleDescriptor) -> Self {
        d3rpc::TriangleDescriptor {
            v1: Some(td.v1.into()),
            v2: Some(td.v2.into()),
            v3: Some(td.v3.into()),
            line_color: Some(td.line_color.into()),
            tri_color: Some(td.tri_color.into()),
            cell_type: td.cell_type.into(),
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddPlaneRequest
impl Validated for d3rpc::AddPlaneRequest {
    fn is_valid(&self) -> bool {
        let is_val = match self.plane_descriptor {
            None => false,
            Some(ref pd) => {
                pd.origin.is_some() && 
                pd.x_axis.is_some() && 
                pd.y_axis.is_some() &&
                pd.x_min < pd.x_max &&
                pd.y_min < pd.y_max &&
                pd.line_color.is_some() &&
                pd.tri_color.is_some()
            }
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::PlaneDescriptor> for PlaneDescriptor
impl From<d3rpc::PlaneDescriptor> for PlaneDescriptor {
    fn from(pd: d3rpc::PlaneDescriptor) -> Self {
        PlaneDescriptor {
            origin: pd.origin.unwrap().into(),
            x_axis: pd.x_axis.unwrap().into(),
            y_axis: pd.y_axis.unwrap().into(),
            x_min: pd.x_min,
            x_max: pd.x_max,
            y_min: pd.y_min,
            y_max: pd.y_max,
            line_color: pd.line_color.unwrap().into(),
            tri_color: pd.tri_color.unwrap().into(),
            cell_type: pd.cell_type.into(),
        }
    }
}
//}}}
//{{{ impl From<PlaneDescriptor> for d3rpc::PlaneDescriptor
impl From<PlaneDescriptor> for d3rpc::PlaneDescriptor {
    fn from(pd: PlaneDescriptor) -> Self {
        d3rpc::PlaneDescriptor {
            origin: Some(pd.origin.into()),
            x_axis: Some(pd.x_axis.into()),
            y_axis: Some(pd.y_axis.into()),
            x_min: pd.x_min,
            x_max: pd.x_max,
            y_min: pd.y_min,
            y_max: pd.y_max,
            line_color: Some(pd.line_color.into()),
            tri_color: Some(pd.tri_color.into()),
            cell_type: pd.cell_type.into(),
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddCuboidRequest
impl Validated for d3rpc::AddCuboidRequest
{
    fn is_valid(&self) -> bool {
        let is_val = match self.cuboid_descriptor {
            None => false,
            Some(ref cd) => {
                cd.origin.is_some() &&
                cd.x_axis.is_some() &&
                cd.y_axis.is_some() &&
                cd.z_axis.is_some() &&
                cd.lenx > 0.0 &&
                cd.leny > 0.0 &&
                cd.lenz > 0.0 &&
                cd.line_color.is_some() &&
                cd.tri_color.is_some()
            }
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::CuboidDescriptor> for CuboidDescriptor
impl From<d3rpc::CuboidDescriptor> for CuboidDescriptor
{
    fn from(cd: d3rpc::CuboidDescriptor) -> Self {
        CuboidDescriptor {
            origin: cd.origin.unwrap().into(),
            x_axis: cd.x_axis.unwrap().into(),
            y_axis: cd.y_axis.unwrap().into(),
            z_axis: cd.z_axis.unwrap().into(),
            lenx: cd.lenx,
            leny: cd.leny,
            lenz: cd.lenz,
            line_color: cd.line_color.unwrap().into(),
            tri_color: cd.tri_color.unwrap().into(),
            cell_type: cd.cell_type.into(),
        }
    }
}
//}}}
//{{{ impl From<CuboidDescriptor> for d3rpc::CuboidDescriptor
impl From<CuboidDescriptor> for d3rpc::CuboidDescriptor
{
    fn from(cd: CuboidDescriptor) -> Self {
        d3rpc::CuboidDescriptor {
            origin: Some(cd.origin.into()),
            x_axis: Some(cd.x_axis.into()),
            y_axis: Some(cd.y_axis.into()),
            z_axis: Some(cd.z_axis.into()),
            lenx: cd.lenx,
            leny: cd.leny,
            lenz: cd.lenz,
            line_color: Some(cd.line_color.into()),
            tri_color: Some(cd.tri_color.into()),
            cell_type: cd.cell_type.into(),
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddCylinderRequest
impl Validated for d3rpc::AddCylinderRequest
{
    fn is_valid(&self) -> bool {
        let is_val = match self.cylinder_descriptor {
            None => false,
            Some(ref cd) => {
                cd.origin.is_some() &&
                cd.axis.is_some() &&
                cd.radius > 0.0 &&
                cd.height > 0.0 &&
                cd.num_sides > 0 &&
                cd.line_color.is_some() &&
                cd.tri_color.is_some()
            }
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::CylinderDescriptor> for CylinderDescriptor
impl From<d3rpc::CylinderDescriptor> for CylinderDescriptor
{
    fn from(cd: d3rpc::CylinderDescriptor) -> Self {
        CylinderDescriptor {
            origin: cd.origin.unwrap().into(),
            axis: cd.axis.unwrap().into(),
            radius: cd.radius,
            height: cd.height,
            num_sides: cd.num_sides as usize,
            line_color: cd.line_color.unwrap().into(),
            tri_color: cd.tri_color.unwrap().into(),
            open: cd.open,
            cell_type: cd.cell_type.into(),
        }
    }
}
//}}}
//{{{ impl From<CylinderDescriptor> for d3rpc::CylinderDescriptor
impl From<CylinderDescriptor> for d3rpc::CylinderDescriptor
{
    fn from(cd: CylinderDescriptor) -> Self {
        d3rpc::CylinderDescriptor {
            origin: Some(cd.origin.into()),
            axis: Some(cd.axis.into()),
            radius: cd.radius,
            height: cd.height,
            num_sides: cd.num_sides as u32,
            line_color: Some(cd.line_color.into()),
            tri_color: Some(cd.tri_color.into()),
            open: cd.open,
            cell_type: cd.cell_type.into(),
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddSphereRequest
impl Validated for d3rpc::AddSphereRequest
{
    fn is_valid(&self) -> bool {
        let is_val = match self.sphere_descriptor {
            None => false,
            Some(ref cd) => {
                cd.origin.is_some() &&
                cd.axis.is_some() &&    
                cd.radius > 0.0 &&
                cd.n_lat > 0 &&
                cd.n_long > 0 &&
                cd.line_color.is_some() &&
                cd.tri_color.is_some()
            }
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::SphereDescriptor> for SphereDescriptor
impl From<d3rpc::SphereDescriptor> for SphereDescriptor
{
    fn from(cd: d3rpc::SphereDescriptor) -> Self {
        SphereDescriptor {
            origin: cd.origin.unwrap().into(),
            axis: cd.axis.unwrap().into(),
            radius: cd.radius,
            n_lat: cd.n_lat as usize,
            n_long: cd.n_long as usize,
            line_color: cd.line_color.unwrap().into(),
            tri_color: cd.tri_color.unwrap().into(),
            cell_type: cd.cell_type.into(),
        }
    }
}
//}}}
//{{{ impl From<SphereDescriptor> for d3rpc::SphereDescriptor
impl From<SphereDescriptor> for d3rpc::SphereDescriptor
{
    fn from(cd: SphereDescriptor) -> Self {
        d3rpc::SphereDescriptor {
            origin: Some(cd.origin.into()),
            axis: Some(cd.axis.into()),
            radius: cd.radius,
            n_lat: cd.n_lat as u32,
            n_long: cd.n_long as u32,
            line_color: Some(cd.line_color.into()),
            tri_color: Some(cd.tri_color.into()),
            cell_type: cd.cell_type.into(),
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddAxesRequest
impl Validated for d3rpc::AddAxesRequest 
{
    fn is_valid(&self) -> bool {
        let is_val = match self.axes_descriptor {
            None => false,
            Some(ref ad) => {
                ad.origin.is_some() &&
                ad.x_axis.is_some() &&
                ad.y_axis.is_some() &&
                ad.z_axis.is_some() &&
                ad.neg_len > 0.0 && 
                ad.pos_len > 0.0
            }
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::AxesDescriptor> for AxesDescriptor
impl From<d3rpc::AxesDescriptor> for AxesDescriptor
{
    fn from(ad: d3rpc::AxesDescriptor) -> Self {
        AxesDescriptor {
            origin: ad.origin.unwrap().into(),
            x_axis: ad.x_axis.unwrap().into(),
            y_axis: ad.y_axis.unwrap().into(),
            z_axis: ad.z_axis.unwrap().into(),
            neg_len: ad.neg_len,
            pos_len: ad.pos_len,
        }
    }
}
//}}}
//{{{ impl From<AxesDescriptor> for d3rpc::AxesDescriptor
impl From<AxesDescriptor> for d3rpc::AxesDescriptor
{
    fn from(ad: AxesDescriptor) -> Self {
        d3rpc::AxesDescriptor {
            origin: Some(ad.origin.into()),
            x_axis: Some(ad.x_axis.into()),
            y_axis: Some(ad.y_axis.into()),
            z_axis: Some(ad.z_axis.into()),
            neg_len: ad.neg_len,
            pos_len: ad.pos_len,
        }
    }
}
//}}}

//{{{ impl Validated for d3rpc::AddMeshRequest
impl Validated for d3rpc::AddMeshRequest
{
    fn is_valid(&self) -> bool {
        let is_val = match self.mesh_descriptor {
            None => false,
            Some(ref md) => {
                md.vertices.len() > 0 &&
                md.indices.len() > 0
            }
        };
        is_val
    }
}
//}}}
//{{{ impl From<d3rpc::MeshDescriptor> for Mesh<'a>
impl<'a> From<d3rpc::MeshDescriptor> for Mesh<'a>
{
    fn from(md: d3rpc::MeshDescriptor) -> Self {
        Mesh {
            vertices: md.vertices, 
            indices: md.indices,
            cell_type: (md.cell_type as i32).into(),
            uid: 0,
            phant: PhantomData,
        }
    }
}
//}}}
//{{{ impl From<Mesh<'a>> for d3rpc::MeshDescriptor
impl<'a> From<Mesh<'a>> for d3rpc::MeshDescriptor
{
    fn from(md: Mesh<'a>) -> Self {
        d3rpc::MeshDescriptor {
            vertices: md.vertices,
            indices: md.indices,
            cell_type: (md.cell_type as i32).into(),
        }
    }
}
//}}}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {}
//}}}



























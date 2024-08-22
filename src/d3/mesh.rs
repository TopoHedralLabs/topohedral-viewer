//! This module contains the implementation of the `Mesh` struct for the 3D viewer.
//!
//! This module defines the set of items which may be  drawn in the 3D viewer and provides an 
//! API for each type of item. The general item will simply take a mesh and draw it. 
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::common::{self, Color, Vec3, CellType};
use crate::d3::vertex::{Vertex, VertexDescriptor};
use crate::core::MeshCore;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: constants and type
pub type Mesh<'a> = MeshCore<'a, Vertex>;
//}}}
//{{{ struct: LineDescriptor
/// This struct encapuslates the geometric information needed to fully specify a line.
pub struct LineDescriptor
{
    /// First endpoint of line
    pub v1: Vec3,
    /// Second endpoint of line
    pub v2: Vec3,
    /// Color of line
    pub color: Color,
}
//}}}
//{{{ struct: TriangleDescriptor
pub struct TriangleDescriptor
{
    /// First vertex of triangle
    pub v1: Vec3,
    /// Second vertex of triangle
    pub v2: Vec3,
    /// Third vertex of triangle
    pub v3: Vec3,
    /// Color of line
    pub line_color: Color,
    /// Color of triangle
    pub tri_color: Color,
    /// Type of 
    pub cell_type: CellType,
}
//}}}
//{{{ struct: PlaneDescriptor
pub struct PlaneDescriptor
{    
    /// Origin of local coordinate system
    pub origin: Vec3,
    /// X-axis of local coordinate system
    pub x_axis: Vec3,
    /// Y-axis of local coordinate system
    pub y_axis: Vec3,
    /// Min distance along X
    pub x_min: f32,
    /// Max distance along X
    pub x_max: f32,
    /// Min distance along Y
    pub y_min: f32,
    /// Max distance along Y
    pub y_max: f32,
    /// Color of lines in render
    pub line_color: Color,
    /// Color of triangles in render
    pub tri_color: Color,
    /// Type of cell
    pub cell_type: CellType,    
}
//}}}
//{{{ struct: CuboidDescriptor
/// This struct encapsulates the geometric information needed to fully specify a cuboid.
pub struct CuboidDescriptor
{
    /// The bottom left corner of the cuboid
    pub origin: Vec3,
    /// X-vector for the cuboid
    pub x_axis: Vec3,
    /// Y-vector for the cuboid
    pub y_axis: Vec3,
    /// Z-vector for the cuboid
    pub z_axis: Vec3,
    /// length along x
    pub lenx: f32,
    /// length along y
    pub leny: f32,
    /// length along z
    pub lenz: f32,
    /// Color of lines in render
    pub line_color: Color,
    /// Color of triangles in render
    pub tri_color: Color,
    /// Type of cell
    pub cell_type: CellType,
}
//}}}
//{{{ struct: CylinderDescriptor
/// This struct encapsulates the geometric information needed to fully specify a cylinder.
pub struct CylinderDescriptor
{
    /// This is the center of the circle which is the bottom face of the cylinder
    pub origin: Vec3,
    /// This is the axis of the cylinder, both top and bottom faces are normal to this vector
    pub axis: Vec3,
    /// Radius of the cylinder, as measured from ``axis``.
    pub radius: f32,
    /// Length from the bottom face to the top face of the cylinder
    pub height: f32,
    /// Number of sides (triangles) to use when approximating the cylinder
    pub num_sides: usize,
    /// Color of lines in render
    pub line_color: Color,
    /// Color of triangles in render
    pub tri_color: Color,
    /// Flag indicating whether to make a closed cylinder or open cylinder
    pub open: bool,
}
//}}}
//{{{ struct: SphereDescriptor
/// This struct encapuslates the geometric information needed to fully specify a sphere
pub struct SphereDescriptor
{
    /// Center of the sphere
    pub origin: Vec3,
    /// The polar axis of the sphere
    pub axis: Vec3,
    /// Radius of the sphere, as measured from ``origin``.
    pub radius: f32,
    /// Number of latitudinal subdivisions of the sphere (number of divisions from pole to pole)
    pub n_lat: usize,
    /// Number of longitudinal subdivisions of the sphere (number of divisions around the equator)
    pub n_long: usize,
    /// Color of lines in render
    pub line_color: Color,
    /// Color of triangles in render
    pub tri_color: Color,
}
//}}}
//{{{ struct: AxesDescriptor
pub struct AxesDescriptor
{
    pub origin: Vec3,
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
    pub neg_len: f32,
    pub pos_len: f32,
}
//}}}
//{{{ trait: Mesh3D
pub trait Mesh3D<'a>
{
    fn create_line(line_disc: &LineDescriptor) -> Self;
    fn create_triangle(triangle_disc: &TriangleDescriptor) -> Self; 
    fn create_plane(plane_disc: &PlaneDescriptor) -> Self;
    fn create_cuboid(cuboid: &CuboidDescriptor) -> Self;
    fn create_cylinder(cylinder: &CylinderDescriptor) -> Self;  
    fn create_sphere(sphere: &SphereDescriptor) -> Self;
    fn create_axes(axes: &AxesDescriptor) -> Self;
    fn add_line(
        &mut self,
        v1: &Vec3,
        v2: &Vec3,
        line_color: &Color,
        tri_color: &Color,
    );

    fn add_triangle(
        &mut self,
        v1: &Vec3,
        v2: &Vec3,
        v2: &Vec3,
        line_color: &Color,
        tri_color: &Color,
    );
}
//}}}
//{{{ impl: Mesh3D for Mesh
impl<'a> Mesh3D<'a> for Mesh<'a>
{
    //{{{ fn: create_line
    fn create_line(line_disc: &LineDescriptor) -> Self
    {
        let mut out = Mesh::from_num_lines(1);
        let v1 = Vertex::new(&VertexDescriptor {
            position: line_disc.v1,
            normal: Vec3::zeros(),
            line_color: line_disc.color,
            triangle_color: line_disc.color,
        });
        let v2 = Vertex::new(&VertexDescriptor {
            position: line_disc.v2,
            normal: Vec3::zeros(),
            line_color: line_disc.color,
            triangle_color: line_disc.color,
        });
        out.append_vertex(&v1);
        out.append_vertex(&v2);
        out.append_indices(&[0, 1]);
        out
    }
    //}}}
    //{{{ fn: create_triangle
    fn create_triangle(triangle_disc: &TriangleDescriptor) -> Self {
        let mut out = Mesh::from_num_triangles(1);
        let v1 = Vertex::new(&VertexDescriptor {
            position: triangle_disc.v1,
            normal: Vec3::zeros(),
            line_color: triangle_disc.line_color,
            triangle_color: triangle_disc.tri_color,
        });
        let v2 = Vertex::new(&VertexDescriptor {
            position: triangle_disc.v2,
            normal: Vec3::zeros(),
            line_color: triangle_disc.line_color,
            triangle_color: triangle_disc.tri_color,
        });
        let v3 = Vertex::new(&VertexDescriptor {
            position: triangle_disc.v3,
            normal: Vec3::zeros(),
            line_color: triangle_disc.line_color,
            triangle_color: triangle_disc.tri_color,
        });
        out.append_vertex(&v1);
        out.append_vertex(&v2);
        out.append_vertex(&v3);
        out.append_indices(&[0, 1, 2]);
        out
    }
    //}}}
    //{{{ fn: create_plane
    fn create_plane(plane_disc: &PlaneDescriptor) -> Self 
    {

        let mut out = Self::from_num_triangles(2);

        let xmin = plane_disc.x_min;
        let xmax = plane_disc.x_max;
        let ymin = plane_disc.y_min;
        let ymax = plane_disc.y_max;
        let xax = plane_disc.x_axis;
        let yax = plane_disc.y_axis;

        let v0 = plane_disc.origin + (xmin * xax) + (ymin * yax);
        let v1 = plane_disc.origin + (xmax * xax) + (ymin * yax);
        let v2 = plane_disc.origin + (xmax * xax) + (ymax * yax);
        let v3 = plane_disc.origin + (xmin * xax) + (ymax * yax);

        let line_color = plane_disc.line_color; 
        let tri_color = plane_disc.tri_color;
        out.add_triangle(&v0, &v1, &v2, &line_color, &tri_color);
        out.add_triangle(&v0, &v2, &v3, &line_color, &tri_color);

        out

    }
    //}}}
    //{{{ fn: create_cuboid
    fn create_cuboid(cuboid_disc: &CuboidDescriptor) -> Self
    {
        let mut out = Self::from_num_triangles(12);

        let lc = cuboid_disc.line_color;
        let tc = cuboid_disc.tri_color;

        let o = cuboid_disc.origin;
        let dx = cuboid_disc.x_axis * cuboid_disc.lenx;
        let dy = cuboid_disc.y_axis * cuboid_disc.leny;
        let dz = cuboid_disc.z_axis * cuboid_disc.lenz;

        let v0 = o;
        let v1 = o + dx;
        let v2 = o + dx + dy;
        let v3 = o + dy;
        let v4 = o + dz;
        let v5 = o + dx + dz;
        let v6 = o + dx + dy + dz;
        let v7 = o + dy + dz;

        // -ve x
        out.add_triangle(&v0, &v7, &v3, &lc, &tc);
        out.add_triangle(&v0, &v4, &v7, &lc, &tc);
        // +ve x
        out.add_triangle(&v1, &v2, &v6, &lc, &tc);
        out.add_triangle(&v1, &v6, &v5, &lc, &tc);
        // -ve y
        out.add_triangle(&v0, &v1, &v5, &lc, &tc);
        out.add_triangle(&v0, &v5, &v4, &lc, &tc);
        // +ve y
        out.add_triangle(&v2, &v3, &v7, &lc, &tc);
        out.add_triangle(&v2, &v7, &v6, &lc, &tc);
        // -ve z
        out.add_triangle(&v0, &v2, &v1, &lc, &tc);
        out.add_triangle(&v0, &v3, &v2, &lc, &tc);
        // +ve z
        out.add_triangle(&v4, &v5, &v6, &lc, &tc);
        out.add_triangle(&v4, &v6, &v7, &lc, &tc);

        out
    }
    //}}}
    //{{{ fn: create_cylinder
    fn create_cylinder(cyl_disc: &CylinderDescriptor) -> Self
    {
        let n = cyl_disc.num_sides;
        let origin = cyl_disc.origin;
        let axis = cyl_disc.axis;
        let radius = cyl_disc.radius;
        let height = cyl_disc.height;
        let line_color = cyl_disc.line_color;
        let tri_color = cyl_disc.tri_color;

        let num_trinangles = n * 2;

        let mut out = Self::from_num_triangles(num_trinangles);

        let x_axis = common::orthogonal_vector(&axis);

        let y_axis = x_axis.cross(&axis);

        let d_theta = 2.0 * std::f32::consts::PI / (n - 1) as f32;

        let bottom = origin;

        let top = origin + height * axis;

        for i in 0..n
        {
            let theta = i as f32 * d_theta;

            let (sin_theta, cos_theta) = theta.sin_cos();

            let cyl_pnt = bottom + (radius * cos_theta * x_axis) + (radius * sin_theta * y_axis);

            let cyl_normal = (cyl_pnt - bottom).normalize();

            let v = Vertex::new(&VertexDescriptor {
                position: cyl_pnt,
                normal: cyl_normal,
                line_color: line_color,
                triangle_color: tri_color,
            });

            out.append_vertex(&v);
        }

        for i in 0..n
        {
            let theta = i as f32 * d_theta;

            let (sin_theta, cos_theta) = theta.sin_cos();

            let cyl_pnt = top + (radius * cos_theta * x_axis) + (radius * sin_theta * y_axis);

            let cyl_normal = (cyl_pnt - top).normalize();

            let v = Vertex::new(&VertexDescriptor {
                position: cyl_pnt,
                normal: cyl_normal,
                line_color: line_color,
                triangle_color: tri_color,
            });

            out.append_vertex(&v);
        }

        // first to penultimate iteration
        for i in 0..n - 1
        {
            let t1 = [(i + n + 1) as u32, (i + n) as u32, i as u32];

            let t2 = [(i + 1) as u32, (i + n + 1) as u32, i as u32];

            out.append_indices(&t1);

            out.append_indices(&t2);
        }

        // last iterations
        {
            let i = n - 1;

            let t1 = [(n + 1) as u32, n as u32, i as u32];

            let t2 = [(i + 1) as u32, (n + 1) as u32, i as u32];

            out.append_indices(&t1);

            out.append_indices(&t2);
        }

        out
    }
    //}}}
    //{{{ fn: create_sphere
    fn create_sphere(sphere_disc: &SphereDescriptor) -> Self
    {
        let nt = 2 * sphere_disc.n_lat * sphere_disc.n_long;
        let out = Mesh::from_num_triangles(nt);

        out
    }
    //}}}
    //{{{ fn: create_axes
    fn create_axes(axes_disc: &AxesDescriptor) -> Self
    {
        let r = 0.01;
        let np = 20;

        let mut out = Self::from_num_triangles(np * 2);
        let glob_origin = axes_disc.origin;
        {
            let axis = axes_disc.x_axis;
            let x_origin = glob_origin - axes_disc.neg_len * axis;
            let height = axes_disc.pos_len + axes_disc.neg_len;
            let x_cyl = Self::create_cylinder(&CylinderDescriptor {
                origin: x_origin,
                axis: axis,
                radius: r,
                height: height,
                num_sides: np,
                line_color: Color::Red,
                tri_color: Color::Red,
                open: false,
            });
            out.merge(x_cyl);
        }
        {
            let axis = axes_disc.y_axis;
            let y_origin = glob_origin - axes_disc.neg_len * axis;
            let height = axes_disc.pos_len + axes_disc.neg_len;
            let x_cyl = Self::create_cylinder(&CylinderDescriptor {
                origin: y_origin,
                axis: axis,
                radius: r,
                height: height,
                num_sides: np,
                line_color: Color::Blue,
                tri_color: Color::Blue,
                open: false,
            });
            out.merge(x_cyl);
        }
        {
            let axis = axes_disc.z_axis;
            let z_origin = glob_origin - axes_disc.neg_len * axis;
            let height = axes_disc.pos_len + axes_disc.neg_len;
            let x_cyl = Self::create_cylinder(&CylinderDescriptor {
                origin: z_origin,
                axis: axis,
                radius: r,
                height: height,
                num_sides: np,
                line_color: Color::Green,
                tri_color: Color::Green,
                open: false,
            });
            out.merge(x_cyl);
        }

        out
    }
    //}}}
    //{{{ fn: add_line
    fn add_line(
        &mut self,
        v1: &Vec3,
        v2: &Vec3,
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
            normal: Vec3::zeros(),
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v2,
            normal: Vec3::zeros(),
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
    }
    //}}}
    //{{{ fn: add_triangle
    fn add_triangle(
        &mut self,
        v1: &Vec3,
        v2: &Vec3,
        v3: &Vec3,
        line_color: &Color,
        tri_color: &Color,
    )
    {
        assert!(self.is_triangle());
        let nv = self.num_vertices() as u32;
        let indices = [nv, nv + 1, nv + 2];
        self.append_indices(&indices);

        let d1 = v2 - v1;
        let d2 = v3 - v1;
        let normal = d1.cross(&d2).normalize();

        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v1,
            normal: normal,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v2,
            normal: normal,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor{
            position: *v3,
            normal: normal,
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
    fn create_cuboid_test()
    {
        let cube = Mesh::create_cuboid(&CuboidDescriptor {
            origin: Vec3::new(0.1, 0.1, 0.0),
            x_axis: Vec3::x(),
            y_axis: Vec3::y(),
            z_axis: Vec3::z(),
            lenx: 1.0,
            leny: 1.0,
            lenz: 1.0,
            line_color: Color::White,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        });
    }

    #[test]
    fn from_axes_test()
    {
        let mut axes_mesh = Mesh::create_axes(&AxesDescriptor {
            origin: Vec3::new(0.0, 0.0, 0.0),
            neg_len: 1.0,
            pos_len: 2.0,
            x_axis: Vec3::new(1.0, 0.0, 0.0),
            y_axis: Vec3::new(0.0, 1.0, 0.0),
            z_axis: Vec3::new(0.0, 0.0, 1.0),
        });

        axes_mesh.uid = 1;
    }

    #[test]
    fn vertex_view_test()
    {
        let cube = Mesh::create_cuboid(&CuboidDescriptor {
            origin: Vec3::new(0.1, 0.1, 0.0),
            x_axis: Vec3::x(),
            y_axis: Vec3::y(),
            z_axis: Vec3::z(),
            lenx: 1.0,
            leny: 1.0,
            lenz: 1.0,
            line_color: Color::White,
            tri_color: Color::Green,
            cell_type: CellType::Triangle,
        });

        // let mut vertex_view = cube.vertex_view_mut(0);
    }
}
//}}}
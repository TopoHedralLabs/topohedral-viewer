//! This module contains the implementation of the `Mesh` struct for the 3D viewer.
//!
//! This module defines the set of items which may be  drawn in the 3D viewer and provides an
//! API for each type of item. The general item will simply take a mesh and draw it.
//--------------------------------------------------------------------------------------------------


//{{{ crate imports
use crate::common::{self, CellType, Color, Vec3};
use crate::core::MeshCore;
use crate::d3::vertex::{Vertex, VertexDescriptor};
//}}}
//{{{ std imports
use core::panic;
//}}}
//{{{ dep imports
use thiserror::Error;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: constants and type
pub type Mesh<'a> = MeshCore<'a, Vertex>;
//}}}
//{{{ enum: Error
#[derive(Debug, Error)]
pub enum Error 
{
    #[error("Indices out of bounds")]
    IndexOutOfBounds,
}
//}}}
//{{{ struct: LineDescriptor
/// This struct encapuslates the geometric information needed to fully specify a line.
pub struct LineDescriptor {
    /// First endpoint of line
    pub v1: Vec3,
    /// Second endpoint of line
    pub v2: Vec3,
    /// Color of line
    pub color: Color,
}
//}}}
//{{{ struct: TriangleDescriptor
pub struct TriangleDescriptor {
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
pub struct PlaneDescriptor {
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
pub struct CuboidDescriptor {
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
pub struct CylinderDescriptor {
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
    /// Type of cell
    pub cell_type: CellType,
}
//}}}
//{{{ struct: DiscDescriptor
/// This struct encapsulates the geometric information needed to fully specify a disc.
/// The disc is defined by a center point, a normal vector, and a radius. The disc can
/// be used to represent a flat circular surface.
pub struct DiscDescriptor {
    /// This is the center of the circle which is the bottom face of the cylinder
    pub origin: Vec3,
    /// This is the normal vector of the cylinder
    pub axis: Vec3,
    /// Radius of the cylinder, as measured from ``axis``.
    pub radius: f32,
    /// Number of sides (triangles) to use when approximating the disc
    pub num_sides: usize,
    /// Color of lines in render
    pub line_color: Color,
    /// Color of triangles in render
    pub tri_color: Color,
    /// Type of cell
    pub cell_type: CellType,
}
//}}}
//{{{ struct: SphereDescriptor
/// This struct encapuslates the geometric information needed to fully specify a sphere
pub struct SphereDescriptor {
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
    /// Type of cell
    pub cell_type: CellType,
}
//}}}
//{{{ struct: AxesDescriptor
pub struct AxesDescriptor {
    pub origin: Vec3,
    pub x_axis: Vec3,
    pub y_axis: Vec3,
    pub z_axis: Vec3,
    pub neg_len: f32,
    pub pos_len: f32,
}
//}}}
//{{{ trait: Mesh3D
pub trait Mesh3D<'a> {
    fn create_line(line_disc: &LineDescriptor) -> Self;
    fn create_triangle(triangle_disc: &TriangleDescriptor) -> Self;
    fn create_plane(plane_disc: &PlaneDescriptor) -> Self;
    fn create_cuboid(cuboid: &CuboidDescriptor) -> Self;
    fn create_cylinder(cylinder: &CylinderDescriptor) -> Self;
    fn create_disc(disc: &DiscDescriptor) -> Self;
    fn create_sphere(sphere: &SphereDescriptor) -> Self;
    fn create_axes(axes: &AxesDescriptor) -> Self;
    fn add_vertex(&mut self, v: &Vec3, normal: &Vec3, line_color: &Color, tri_color: &Color);
    fn add_triangle_indices(&mut self, i1: u32, i2: u32, i3: u32) -> Result<(), Error> ;
    fn add_line(&mut self, v1: &Vec3, v2: &Vec3, line_color: &Color, tri_color: &Color);
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
impl<'a> Mesh3D<'a> for Mesh<'a> {
    //{{{ fun: create_line
    fn create_line(line_disc: &LineDescriptor) -> Self {
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
    //{{{ fun: create_triangle
    fn create_triangle(triangle_disc: &TriangleDescriptor) -> Self {
        match triangle_disc.cell_type {
            CellType::Triangle => {
                let mut out = Mesh::from_num_triangles(1);
                out.add_triangle(
                    &triangle_disc.v1,
                    &triangle_disc.v2,
                    &triangle_disc.v3,
                    &triangle_disc.line_color,
                    &triangle_disc.tri_color,
                );
                out
            }
            CellType::Line => {
                let mut out = Mesh::from_num_lines(3);
                out.add_line(
                    &triangle_disc.v1,
                    &triangle_disc.v2,
                    &triangle_disc.line_color,
                    &triangle_disc.tri_color,
                );
                out.add_line(
                    &triangle_disc.v2,
                    &triangle_disc.v3,
                    &triangle_disc.line_color,
                    &triangle_disc.tri_color,
                );
                out.add_line(
                    &triangle_disc.v3,
                    &triangle_disc.v1,
                    &triangle_disc.line_color,
                    &triangle_disc.tri_color,
                );
                out
            }
            _ => {
                panic!("Invalid cell type");
            }
        }
    }
    //}}}
    //{{{ fun: create_plane
    fn create_plane(plane_disc: &PlaneDescriptor) -> Self {
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
        match plane_disc.cell_type {
            CellType::Triangle => {
                let mut out = Self::from_num_triangles(2);
                out.add_triangle(&v0, &v1, &v2, &line_color, &tri_color);
                out.add_triangle(&v0, &v2, &v3, &line_color, &tri_color);
                out
            }
            CellType::Line => {
                let mut out = Self::from_num_lines(4);
                out.add_line(&v0, &v1, &line_color, &tri_color);
                out.add_line(&v1, &v2, &line_color, &tri_color);
                out.add_line(&v2, &v3, &line_color, &tri_color);
                out.add_line(&v3, &v0, &line_color, &tri_color);
                out
            }
            _ => {
                panic!("Invalid cell type");
            }
        }
    }
    //}}}
    //{{{ fun: create_cuboid
    fn create_cuboid(cuboid_disc: &CuboidDescriptor) -> Self {
        //{{{ locals
        let lc = cuboid_disc.line_color;
        let tc = cuboid_disc.tri_color;
        //}}}
        //{{{ com: compute the vertices of the cuboid
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
        //}}}
        //{{{ com: append the cells
        match cuboid_disc.cell_type {
            //{{{ case: triangle
            CellType::Triangle => {
                let mut out = Self::from_num_triangles(12);
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
            //{{{ case line
            CellType::Line => {
                let mut out = Self::from_num_lines(12);
                // bottom 4 edges
                out.add_line(&v0, &v1, &lc, &tc);
                out.add_line(&v1, &v2, &lc, &tc);
                out.add_line(&v2, &v3, &lc, &tc);
                out.add_line(&v3, &v0, &lc, &tc);
                // middle 4 edges
                out.add_line(&v0, &v4, &lc, &tc);
                out.add_line(&v1, &v5, &lc, &tc);
                out.add_line(&v2, &v6, &lc, &tc);
                out.add_line(&v3, &v7, &lc, &tc);
                // top 4 edges
                out.add_line(&v4, &v5, &lc, &tc);
                out.add_line(&v5, &v6, &lc, &tc);
                out.add_line(&v6, &v7, &lc, &tc);
                out.add_line(&v7, &v4, &lc, &tc);

                out
            }
            //}}}
            //{{{ default
            _ => {
                panic!("Invalid cell type");
            } //}}}
        }
        //}}}
    }
    //}}}
    //{{{ fun: create_cylinder
    fn create_cylinder(cyl_disc: &CylinderDescriptor) -> Self {
        //{{{ locals
        let n = cyl_disc.num_sides;
        let origin = cyl_disc.origin;
        let axis = cyl_disc.axis;
        let radius = cyl_disc.radius;
        let height = cyl_disc.height;
        let line_color = cyl_disc.line_color;
        let tri_color = cyl_disc.tri_color;
        let open = cyl_disc.open;
        let cell_type = cyl_disc.cell_type;
        let x_axis = common::orthogonal_vector(&axis);
        let y_axis = x_axis.cross(&axis);
        let d_theta = 2.0 * std::f32::consts::PI / (n - 1) as f32;
        let bottom = origin;
        let top = origin + height * axis;
        //}}}
        //{{{ com: initialize mesh
        let mut out = match cell_type {
            CellType::Triangle => {
                let num_triangles = if open { n * 2 } else { n * 4 };
                Self::from_num_triangles(num_triangles)
            }
            CellType::Line => {
                let num_lines = if open { n * 3 + 1 } else { n * 5 + 1 };
                Self::from_num_lines(num_lines)
            }
            _ => {
                panic!("Invalid cell type");
            }
        };
        //}}}
        //{{{ com: add vertices of bottom circle
        for i in 0..n {
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
        //}}}
        //{{{ com: add vertices of top circle
        for i in 0..n {
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
        //}}}
        //{{{ com: add the cells
        match cell_type {
            //{{{ case: triangle
            CellType::Triangle => {
                // first to penultimate iteration
                for i in 0..n - 1 {
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
            }
            //}}}
            //{{{ case: line
            CellType::Line => {
                for i in 0..n-1 {
                    let bottom_line = [i as u32, (i+1) as u32];
                    let top_line = [(i+n) as u32, (i+n+1) as u32];
                    let middle_line = [i as u32, (i+n) as u32];
                    out.append_indices(&bottom_line);
                    out.append_indices(&top_line);
                    out.append_indices(&middle_line);
                }
            }
            //}}}
            //{{{ default 
            _ => {
                panic!("Invalid cell type");
            }
            //}}}
        }
        //}}}
        //{{{ com: add caps
        if !open {
            let bottom_disc_desc = DiscDescriptor {
                origin: origin,
                axis: -axis,
                radius: radius,
                num_sides: n,
                line_color: line_color,
                tri_color: tri_color,   
                cell_type: cell_type,
            };
            let bottom_disc = Mesh::create_disc(&bottom_disc_desc);
            out.merge(bottom_disc);

            let top_disc_desc = DiscDescriptor {
                origin: origin + height * axis,
                axis: axis,
                radius: radius,
                num_sides: n,
                line_color: line_color,
                tri_color: tri_color,   
                cell_type: cell_type,
            };
            let top_disc = Mesh::create_disc(&top_disc_desc);
            out.merge(top_disc);
        }
        //}}}
        out
    }
    //}}}
    //{{{ fun: create_disc
    fn create_disc(disc: &DiscDescriptor) -> Self {
        //{{{ locals
        let n = disc.num_sides;
        let origin = disc.origin;
        let axis = disc.axis;
        let x_axis = common::orthogonal_vector(&axis);
        let y_axis = x_axis.cross(&axis);
        let radius = disc.radius;
        let d_theta = 2.0 * std::f32::consts::PI / (n - 1) as f32;
        let lc = disc.line_color;
        let tc = disc.tri_color;
        let cell_type = disc.cell_type;
        //}}}
        //{{{ com: initialize mesh
        let mut out = match cell_type {
            CellType::Triangle => Self::from_num_triangles(n),
            CellType::Line => Self::from_num_lines(2 * n),
            _ => {
                panic!("Invalid cell type");
            }
        };
        //}}}
        //{{{ com: append central vertex
        {
            let v = Vertex::new(&VertexDescriptor {
                position: origin,
                normal: axis,
                line_color: lc,
                triangle_color: tc,
            });
            out.append_vertex(&v);
        }
        //}}}
        //{{{ com: append vertices of circle
        for i in 0..n {
            let theta = i as f32 * d_theta;
            let (sin_theta, cos_theta) = theta.sin_cos();
            let cyl_pnt = origin + (radius * cos_theta * x_axis) + (radius * sin_theta * y_axis);
            let v = Vertex::new(&VertexDescriptor {
                position: cyl_pnt,
                normal: axis,
                line_color: lc,
                triangle_color: tc,
            });
            out.append_vertex(&v)
        }
        //}}}
        //{{{ com: append the cells
        match cell_type {
            //{{{ case: triangle
            CellType::Triangle => {
                for i in 0..n {
                    let tri = [0 as u32, i as u32, (i + 1) as u32];
                    out.append_indices(&tri);
                }
            }
            //}}}
            //{{{ case: line
            CellType::Line => {
                for i in 0..n {
                    let line = [0 as u32, i as u32];
                    out.append_indices(&line);
                    let line = [i as u32, (i + 1) as u32];
                    out.append_indices(&line);
                }
            }
            //}}}
            //{{{ default
            _ => {
                panic!("Invalid cell type");
            } //}}}
        }
        //}}}
        out
    }
    //..............................................................................
    //}}}
    //{{{ fun: create_sphere
    fn create_sphere(sphere_disc: &SphereDescriptor) -> Self {
        //{{{ locals
        let origin = sphere_disc.origin;
        let axis = sphere_disc.axis;
        let radius = sphere_disc.radius;
        let n_lat = sphere_disc.n_lat;
        let n_long = sphere_disc.n_long;
        let lc = sphere_disc.line_color;
        let tc = sphere_disc.tri_color;
        let cell_type = sphere_disc.cell_type;
        let x_axis = common::orthogonal_vector(&axis);
        let y_axis = x_axis.cross(&axis);
        //}}}
        //{{{ com:  initialize mesh 
        let mut out =  match cell_type {
            CellType::Triangle => Self::from_num_triangles(2 * n_lat * n_long),
            CellType::Line => Self::from_num_lines(2 * n_lat * n_long),
            _ => {
                panic!("Invalid cell type");
            }
        };
        //}}}
        //{{{ com: append vertices 
        let pi = std::f32::consts::PI;
        let d_lat = pi / ((n_lat - 1) as f32);
        let d_long = (2.0 * pi) / ((n_long - 1) as f32);

        for i in 0..n_lat {
            let phi = i as f32 * d_lat;
            let (sin_phi, cos_phi) = phi.sin_cos();
            for j in 0..n_long {
                let theta = j as f32 * d_long;
                let (sin_theta, cos_theta) = theta.sin_cos();
                let pnt = origin + (radius * sin_phi * cos_theta * x_axis) + 
                                   (radius * sin_phi * sin_theta * y_axis) + 
                                   (radius * cos_phi * axis);
                let normal = (pnt - origin).normalize();
                let v = Vertex::new(&VertexDescriptor {
                    position: pnt,
                    normal: normal,
                    line_color: lc,
                    triangle_color: tc,
                });
                out.append_vertex(&v);
            }
        }
        //}}}

        let cart_map = |i, j| i * n_long + j;

        match cell_type {
            CellType::Triangle => {

                for i in 0..n_lat-1 {
                    for j in 0..n_long-1 {
                        let i1 = cart_map(i, j) as u32;
                        let i2 = cart_map(i+1, j) as u32;  
                        let i3 = cart_map(i+1, j+1) as u32;
                        let i4 = cart_map(i, j+1) as u32;
                        let tri1 = [i1, i2, i3];
                        let tri2 = [i1, i3, i4];
                        out.append_indices(&tri1);
                        out.append_indices(&tri2);
                    }
                }
            }, 
            CellType::Line => {

                for i in 1..n_lat-1 {
                    for j in 0..n_long-1 {
                        let i1 = cart_map(i, j) as u32;
                        let i2 = cart_map(i, j+1) as u32;  
                        let li = [i1, i2];
                        out.append_indices(&li);
                    }
                }

                for j in 0..n_long {
                    for i in 0..n_lat-1 {
                        let i1 = cart_map(i, j) as u32;
                        let i2 = cart_map(i+1, j) as u32;
                        let li = [i1, i2];
                        out.append_indices(&li);
                    }
                }
            },
            _ => {
                panic!("Invalid cell type");
            },
        }
        out
    }
    //}}}
    //{{{ fun: create_axes
    fn create_axes(axes_disc: &AxesDescriptor) -> Self {
        let mut out = Self::from_num_lines(3);
        let glob_origin = axes_disc.origin;
        //{{{ com: x axis
        {
            let axis = axes_disc.x_axis;
            let p1 = glob_origin - axes_disc.neg_len * axis;
            let p2 = glob_origin + axes_disc.pos_len * axis;
            out.add_line(&p1, &p2, &Color::Green, &Color::Green);
        }
        //}}}
        //{{{ com: y axis
        {
            let axis = axes_disc.y_axis;
            let p1 = glob_origin - axes_disc.neg_len * axis;
            let p2 = glob_origin + axes_disc.pos_len * axis;
            out.add_line(&p1, &p2, &Color::Red, &Color::Red);
        }
        //}}}
        //{{{ com: z axis
        {
            let axis = axes_disc.z_axis;
            let p1 = glob_origin - axes_disc.neg_len * axis;
            let p2 = glob_origin + axes_disc.pos_len * axis;
            out.add_line(&p1, &p2, &Color::Blue, &Color::Blue);
        }
        //}}}

        out
    }
    //}}}
    //{{{ fun: add_vertex
    fn add_vertex(&mut self, v: &Vec3, normal: &Vec3, line_color: &Color, tri_color: &Color) 
    {
        self.append_vertex(&Vertex::new(&VertexDescriptor {
            position: *v,
            normal: *normal,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
    }
    //}}}
    //{{{ fun: add_line_indices
    fn add_triangle_indices(&mut self, i1: u32, i2: u32, i3: u32) -> Result<(), Error>  {
        if i1 <= self.num_vertices() as u32 && i2 <= self.num_vertices() as u32 && i3 <= self.num_vertices() as u32 {
            self.append_indices(&[i1, i2, i3]);
            Ok(())
        }
        else {
            Err(Error::IndexOutOfBounds)
        }
    }
    //}}}
    //{{{ fun: add_line
    fn add_line(&mut self, v1: &Vec3, v2: &Vec3, line_color: &Color, tri_color: &Color) {
        assert!(self.is_line());

        let nv = self.num_vertices() as u32;
        let indices = [nv, nv + 1];
        self.append_indices(&indices);

        self.append_vertex(&Vertex::new(&VertexDescriptor {
            position: *v1,
            normal: Vec3::zeros(),
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor {
            position: *v2,
            normal: Vec3::zeros(),
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
    }
    //}}}
    //{{{ fun: add_triangle
    fn add_triangle(
        &mut self,
        v1: &Vec3,
        v2: &Vec3,
        v3: &Vec3,
        line_color: &Color,
        tri_color: &Color,
    ) {
        assert!(self.is_triangle());
        let nv = self.num_vertices() as u32;
        let indices = [nv, nv + 1, nv + 2];
        self.append_indices(&indices);

        let d1 = v2 - v1;
        let d2 = v3 - v1;
        let normal = d1.cross(&d2).normalize();

        self.append_vertex(&Vertex::new(&VertexDescriptor {
            position: *v1,
            normal: normal,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor {
            position: *v2,
            normal: normal,
            line_color: *line_color,
            triangle_color: *tri_color,
        }));
        self.append_vertex(&Vertex::new(&VertexDescriptor {
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
mod tests {

    use super::*;

    #[test]
    fn create_cuboid_test() {
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
    fn from_axes_test() {
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
    fn vertex_view_test() {
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

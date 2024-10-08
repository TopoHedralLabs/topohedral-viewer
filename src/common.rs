//! This module contains types, functions, constants, enums, traits and other utilities that are
//! used throughout the project.
//!
//! These include:
//! - Aliases for nalgebra types
//! - Utility functions for degree and radian conversions
//!
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
//}}}
//{{{ std imports
use std::f32::consts::PI;
//}}}
//{{{ dep imports
use na::Const;
use nalgebra as na;
use serde::{Deserialize, Serialize};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: nalgebra types
pub type Vec2 = na::Vector2<f32>;
pub type Vec3 = na::Vector3<f32>;
pub type Vec4 = na::Vector4<f32>;
pub type VecD<const D: usize> = na::Vector<f32, Const<D>, na::ArrayStorage<f32, D, 1>>;
pub type Mat2 = na::Matrix2<f32>;
pub type Mat3 = na::Matrix3<f32>;
pub type Mat4 = na::Matrix4<f32>;
pub type Poi2 = na::geometry::Point2<f32>;
pub type Poi3 = na::geometry::Point3<f32>;
pub type Poi4 = na::geometry::Point4<f32>;
pub type PoiD<const D: usize> = na::geometry::Point<f32, D>;
pub use na::vector;
//}}}
//{{{ fun: rad
/// Converts an angle in degrees to radians.
///
/// This function takes an angle in degrees as input and returns the equivalent angle in radians.
///
/// # Arguments
///
/// * `deg` - The angle in degrees to be converted.
///
/// # Returns
///
/// The angle in radians.
pub fn rad(deg: f32) -> f32 {
    deg * PI / 180.0
}
//}}}
//{{{ fun: deg
/// Converts an angle in radians to degrees.
///
/// This function takes an angle in radians as input and returns the equivalent angle in degrees.
///
/// # Arguments
///
/// * `rad` - The angle in radians to be converted.
///
/// # Returns
///
/// The angle in degrees.
pub fn deg(rad: f32) -> f32 {
    rad * 180.0 / PI
}
//}}}
//{{{ fun: mod_angle
/// Normalizes an angle to be within the range of 0 to 2π.
///
/// This function takes an angle in radians as input and returns the equivalent angle within the range of 0 to 2π.
///
/// # Arguments
///
/// * `angle` - The angle in radians to be normalized.
///
/// # Returns
///
/// The normalized angle in radians, within the range of 0 to 2π.
pub fn mod_angle(angle: f32) -> f32 {
    let two_pi = 2.0 * std::f32::consts::PI;
    let mod_angle = angle % two_pi;
    if mod_angle < 0.0 {
        mod_angle + two_pi
    } else {
        mod_angle
    }
}
//}}}
//{{{ fun: pitch_and_yaw
/// Calculates the pitch and yaw angles from a given forward direction vector.
///
/// This function takes a 3D vector representing the forward direction and calculates the
/// corresponding pitch and yaw angles in radians.
///
/// # Arguments
///
/// * `forward` - A 3D vector representing the forward direction.
///
/// # Returns
///
/// A tuple containing the pitch and yaw angles in radians.
pub fn pitch_and_yaw(forward: &Vec3) -> (f32, f32) {
    let fx = forward.x;

    let fy = forward.y;

    let fz = forward.z;

    let pitch = fz.acos();

    let yaw = fy.atan2(fx);

    (pitch, yaw)
}
//}}}
//{{{ fun: direction
/// Calculates a 3D direction vector from the given pitch and yaw angles.
///
/// This function takes the pitch and yaw angles in radians and returns a 3D vector
/// representing the corresponding direction.
///
/// # Arguments
///
/// * `pitch` - The pitch angle in radians.
/// * `yaw` - The yaw angle in radians.
///
/// # Returns
///
/// A 3D vector representing the direction.
pub fn direction(pitch: f32, yaw: f32) -> Vec3 {
    let fx = pitch.sin() * yaw.cos();

    let fy = pitch.sin() * yaw.sin();

    let fz = pitch.cos();

    let out = Vec3::new(fx, fy, fz);

    out
}
//}}}
//{{{ fun: octant
/// Calculates the octant of a 3D direction vector.
///
/// This function takes a 3D direction vector and returns the octant number (0-7) that the vector falls within.
///
/// # Arguments
///
/// * `forward` - The 3D direction vector.
///
/// # Returns
///
/// The octant number (0-7) that the direction vector falls within.
pub fn octant(forward: &Vec3) -> i8 {
    let direction_vector = forward.normalize();

    let x = direction_vector.x;

    let y = direction_vector.y;

    let z = direction_vector.z;

    let oct = if z > 0.0 {
        if y > 0.0 {
            if x > 0.0 {
                0_i8
            } else {
                1_i8
            }
        } else {
            if x > 0.0 {
                3_i8
            } else {
                2_i8
            }
        }
    } else {
        if y > 0.0 {
            if x > 0.0 {
                4_i8
            } else {
                5_i8
            }
        } else {
            if x > 0.0 {
                7_i8
            } else {
                6_i8
            }
        }
    };

    oct
}
//}}}
//{{{ fun: wrap
/// Wraps a value `n` within the given `range`.
///
/// This function takes a value `n` and a range, and returns the value wrapped within the range.
/// If the value is negative, it is wrapped to the positive side of the range.
///
/// # Examples
///
/// assert_eq!(wrap(10.0, 5.0), 0.0);
/// assert_eq!(wrap(-3.0, 5.0), 2.0);
///
///
/// # Parameters
/// - `n`: The value to be wrapped.
/// - `range`: The range within which the value should be wrapped.
///
/// # Returns
/// The value `n` wrapped within the given `range`.
pub fn wrap<T>(n: T, range: T) -> T
where
    T: std::ops::Rem<Output = T>
        + std::cmp::PartialOrd
        + Copy
        + std::ops::Add<Output = T>
        + Default,
{
    let result = n % range;

    if result < T::default() {
        result + range
    } else {
        result
    }
}
//}}}
//{{{ fun: orthogonal_vector
/// Computes an orthogonal vector to the given 3D vector.
///
/// This function takes a 3D vector as input and returns a new 3D vector that is orthogonal to the input vector.
/// The returned vector is normalized to have the same magnitude as the input vector.
///
/// # Parameters
/// - `vec`: The input 3D vector.
///
/// # Returns
/// A new 3D vector that is orthogonal to the input vector.
pub fn orthogonal_vector(vec: &Vec3) -> Vec3 {
    let (max_idx, _) =
        vec.iter()
            .enumerate()
            .fold((0, f32::MIN), |(max_idx, max_val), (idx, &val)| {
                let abs_val = val.abs();
                if abs_val > max_val {
                    (idx, abs_val)
                } else {
                    (max_idx, max_val)
                }
            });

    let first_idx = match max_idx {
        0 => 1,
        1 => 0,
        2 => 1,
        _ => {
            panic!()
        }
    };

    let mut ortho_vector = Vec3::zeros();
    ortho_vector[max_idx] = -vec[first_idx] / vec[max_idx];
    ortho_vector[first_idx] = 1.0;

    let vec_norm = vec.norm();
    let ortho_vector_norm = ortho_vector.norm();
    ortho_vector = ortho_vector * (vec_norm / ortho_vector_norm);
    ortho_vector
}
//}}}
//{{{ collection: Color
//{{{ struct: Color
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Cyan,
    Magenta,
    Lime,
    Pink,
    Teal,
    Navy,
    Maroon,
    Olive,
    Brown,
    Black,
    Gray,
    White,
    Other((f32, f32, f32)),
}
//}}}
//{{{ impl: Color
impl Color {
    pub fn to_rgb(&self) -> [f32; 3] {
        match *self {
            Color::Red => [1.0, 0.0, 0.0],
            Color::Green => [0.0, 1.0, 0.0],
            Color::Blue => [0.0, 0.0, 1.0],
            Color::Yellow => [1.0, 1.0, 0.0],
            Color::Orange => [1.0, 0.5, 0.0],
            Color::Purple => [0.5, 0.0, 1.0],
            Color::Cyan => [0.0, 1.0, 1.0],
            Color::Magenta => [1.0, 0.0, 1.0],
            Color::Lime => [0.5, 1.0, 0.0],
            Color::Pink => [1.0, 0.75, 0.79],
            Color::Teal => [0.0, 0.5, 0.5],
            Color::Navy => [0.0, 0.0, 0.5],
            Color::Maroon => [0.5, 0.0, 0.0],
            Color::Olive => [0.5, 0.5, 0.0],
            Color::Brown => [0.6, 0.4, 0.2],
            Color::Black => [0.0, 0.0, 0.0],
            Color::Gray => [0.5, 0.5, 0.5],
            Color::White => [1.0, 1.0, 1.0],
            Color::Other(rgb) => [rgb.0, rgb.1, rgb.2],
        }
    }
}
//}}}
//{{{ impl: Default for Color
impl Default for Color {
    fn default() -> Self {
        Color::Gray
    }
}
//}}}
//}}}
//{{{ collection: CellType
//{{{ struct: CellType
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Copy)]
pub enum CellType {
    None = 0,
    Line = 1,
    Triangle = 2,
}
//}}}
//{{{ impl: Default for CellType
impl Default for CellType {
    fn default() -> Self {
        CellType::None
    }
}
//..................................................................................................
//}}}

//{{{ impl: From<i32> for CellType
impl  From<i32> for CellType
{
    fn from(c: i32) -> Self
    {
        match c
        {
            2 => CellType::Triangle,
            1 =>  CellType::Line,
            _ => CellType::None,
        }
    }
}
//}}}
//{{{ impl: From<CellType> for i32
impl From<CellType> for i32
{
    fn from(c: CellType) -> Self
    {
        match c
        {
            CellType::Triangle => 2,
            CellType::Line => 1,
            CellType::None => 0,
        }
    }
}
//}}}
//}}}
//{{{ trait: Validated
pub trait Validated {
    fn is_valid(&self) -> bool;
}
//}}}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {

    use super::*;

    #[test]

    fn pitch_and_yaw_test() {
        let v1 = Vec3::new(1.0, 1.0, 1.0).normalize();

        let v2 = Vec3::new(-1.0, 1.0, 1.0).normalize();

        let v3 = Vec3::new(-1.0, -1.0, 1.0).normalize();

        let v4 = Vec3::new(1.0, -1.0, 1.0).normalize();

        let v5 = Vec3::new(1.0, 1.0, -1.0).normalize();

        let v6 = Vec3::new(-1.0, 1.0, -1.0).normalize();

        let v7 = Vec3::new(-1.0, -1.0, -1.0).normalize();

        let v8 = Vec3::new(1.0, -1.0, -1.0).normalize();

        let v9 = Vec3::new(1.0, 0.0, 0.0);

        let v10 = Vec3::new(-1.0, 0.0, 0.0);

        let v11 = Vec3::new(1.0, 1.0, 0.0);

        let v12 = Vec3::new(0.0, -1.0, 0.0);

        let v13 = Vec3::new(0.0, 0.0, 1.0);

        let v14 = Vec3::new(0.0, 0.0, -1.0);

        let dirs = vec![v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14];

        for dir in dirs {
            let (pitch, yaw) = pitch_and_yaw(&dir);
            let dir2 = direction(pitch, yaw);
        }
    }

    // generate test cases for octant function for each of the 8 octants, put them all in the same test function
    #[test]

    fn octant_test() {
        // Octant 0: +x, +y, +z
        let v0 = Vec3::new(1.0, 1.0, 1.0);

        assert_eq!(octant(&v0), 0_i8);

        // Octant 1: -x, +y, +z
        let v1 = Vec3::new(-1.0, 1.0, 1.0);

        assert_eq!(octant(&v1), 1_i8);

        // Octant 2: -x, -y, +z
        let v2 = Vec3::new(-1.0, -1.0, 1.0);

        assert_eq!(octant(&v2), 2_i8);

        // Octant 3: +x, -y, +z
        let v3 = Vec3::new(1.0, -1.0, 1.0);

        assert_eq!(octant(&v3), 3_i8);

        // Octant 4: +x, +y, -z
        let v4 = Vec3::new(1.0, 1.0, -1.0);

        assert_eq!(octant(&v4), 4_i8);

        // Octant 5: -x, +y, -z
        let v5 = Vec3::new(-1.0, 1.0, -1.0);

        assert_eq!(octant(&v5), 5_i8);

        // Octant 6: -x, -y, -z
        let v6 = Vec3::new(-1.0, -1.0, -1.0);

        assert_eq!(octant(&v6), 6_i8);

        // Octant 7: +x, -y, -z
        let v7 = Vec3::new(1.0, -1.0, -1.0);

        assert_eq!(octant(&v7), 7_i8);
    }

    #[test]

    fn wrap_test() {
        assert_eq!(wrap(-3, 3), 0);

        assert_eq!(wrap(-2, 3), 1);

        assert_eq!(wrap(-1, 3), 2);

        assert_eq!(wrap(0, 3), 0);

        assert_eq!(wrap(1, 3), 1);

        assert_eq!(wrap(2, 3), 2);

        assert_eq!(wrap(3, 3), 0);
    }

    #[test]
    fn orthogonal_vector_test() {
        {
            let v1 = Vec3::new(1.0, 1.0, 1.0);
            let v2 = orthogonal_vector(&v1);
            assert!(v1.dot(&v2) < 1e-8)
        }
        {
            let v1 = Vec3::new(1.0, 0.0, 0.0);
            let v2 = orthogonal_vector(&v1);
            assert!(v1.dot(&v2) < 1e-8)
        }
        {
            let v1 = Vec3::new(0.0, 1.0, 0.0);
            let v2 = orthogonal_vector(&v1);
            assert!(v1.dot(&v2) < 1e-8)
        }
        {
            let v1 = Vec3::new(0.0, 0.0, 1.0);
            let v2 = orthogonal_vector(&v1);
            assert!(v1.dot(&v2) < 1e-8)
        }
    }
}
//}}}

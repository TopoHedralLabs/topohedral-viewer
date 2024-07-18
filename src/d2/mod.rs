//! This module contains all the functionality for rendering 2D scenes.
//!
//--------------------------------------------------------------------------------------------------

mod camera;
mod vertex;
mod mesh;
mod state;
mod rpc;

pub use vertex::{Vertex, VertexDescriptor};
pub use mesh::{AxesDescriptor, Mesh, SquareDescriptor, CircleDescriptor, Mesh2D};
pub use state::{State, State2D};
pub use rpc::{run_server};
//! This module contains all the functionality for rendering 2D scenes.
//!
//--------------------------------------------------------------------------------------------------

mod camera;
mod vertex;
mod mesh;
mod state;
mod rpc;

pub(crate) use state::State;
pub(crate) use rpc::run_server;

pub use mesh::{AxesDescriptor,SquareDescriptor, CircleDescriptor, Mesh, Mesh2D};
pub use rpc::Client2D;
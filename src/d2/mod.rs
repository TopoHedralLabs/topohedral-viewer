//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------


mod camera;
mod vertex;
mod mesh;
mod state;
mod rpc;

pub(crate) use state::State;
pub(crate) use rpc::run_server;

pub use mesh::{AxesDescriptor,LineDescriptor, SquareDescriptor, CircleDescriptor, Mesh, Mesh2D};
pub use rpc::Client2D;
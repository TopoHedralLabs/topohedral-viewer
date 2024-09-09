//! This module defines an RPC client for the 2D viewer.
//!
//! This client also launches and kills the RPC server rather than connecting to a pre-existing
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::super::mesh::{AxesDescriptor, SquareDescriptor, CircleDescriptor, Mesh};
use super::d2rpc::state_service_client::StateServiceClient;
use std::result::Result;
use super::d2rpc;
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use tonic::Request;
use topohedral_tracing::*;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::runtime;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: types
/// The RpcClient type is an alias for the StateServiceClient from the tonic crate, which provides 
/// a client for the 2D viewer's state service RPC API.
type RpcClient = StateServiceClient<tonic::transport::Channel>;
//}}}
//{{{ enum: Error
/// This enum defines the possible error types that can occur when using the RPC client.
/// 
/// - `IoError`: Represents an I/O error that occurred during the RPC call.
/// - `TonicError`: Represents a transport-level error that occurred during the RPC call.
/// - `StatusError`: Represents an error returned by the RPC server in the form of a Tonic status.
#[derive(Debug, Error)]
pub enum Error
{
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Tonic error: {0}")]    
    TonicError(#[from] tonic::transport::Error),
    #[error("Status error: {0}")]
    StatusError(#[from] tonic::Status), 
}
//..............................................................................
//}}}
//{{{ struct: ClientD2
/// The `Client2D` struct represents a client for the 2D viewer's RPC API.
/// 
/// It contains the following fields:
/// - `client_name`: A `String` representing the name of the client.
/// - `stub`: An `RpcClient` instance, which is an alias for the `StateServiceClient` from 
///    the `tonic` crate, providing the client for the 2D viewer's state service RPC API.
/// - `tokio_runtime`: A `Runtime` instance from the `tokio` crate, which is used to run the 
///    RPC calls asynchronously.
pub struct Client2D
{
    client_name: String,
    stub: RpcClient,
    tokio_runtime: Runtime,
}
//..............................................................................
//}}}
//{{{ impl Client2D
impl Client2D 
{
    //{{{ fun: new
    pub fn new(port: usize) -> Result<Self, Error>
    {
        //{{{ trace
        info!("Starting 2D client");
        //}}}
        let client_name = "client2d";
        let tokio_runtime = runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

        let stub =  tokio_runtime.block_on(RpcClient::connect(format!("http://127.0.0.1:{}", port)))?;
        Ok(
            Self{
                client_name: client_name.to_string(), 
                stub: stub,
                tokio_runtime: tokio_runtime,
            }
        )
    }
    //}}}
    //{{{ fun: add_axes
    pub fn add_axes(&mut self, axes_desc: AxesDescriptor) -> Result<usize, Error> {

        let axes_desc_rpc: d2rpc::AxesDescriptor = axes_desc.into();
        let request = Request::new(
            d2rpc::AddAxesRequest{
                client_name: self.client_name.clone(), 
                axes_descriptor: Some(axes_desc_rpc)
            }
        );

        let response = self.tokio_runtime.block_on(self.stub.add_axes(request))?;
        Ok(response.into_inner().id as usize)
    }
    //}}}
    //{{{ fun: add_square
    pub fn add_square(&mut self, square_desc: SquareDescriptor) -> Result<usize, Error> {

        let square_desc_rpc: d2rpc::SquareDescriptor = square_desc.into();
        let request = Request::new(
            d2rpc::AddSquareRequest{
                client_name: self.client_name.clone(),
                square_descriptor: Some(square_desc_rpc)
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_square(request))?;
        Ok(response.into_inner().id as usize)
    }
    //}}}
    //{{{ fun: add_circle
    pub fn add_circle(&mut self, circle_desc: CircleDescriptor) -> Result<usize, Error> {
        let circle_desc_rpc: d2rpc::CircleDescriptor = circle_desc.into();
        let request = Request::new(
            d2rpc::AddCircleRequest{
                client_name: self.client_name.clone(),
                circle_descriptor: Some(circle_desc_rpc)
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_circle(request))?;
        Ok(response.into_inner().id as usize)
    }
    //}}}
    //{{{ fun: add_mesh
    pub fn add_mesh<'a>(&mut self, mesh: Mesh<'a>) -> Result<usize, Error>
    {
        let mesh_desc_rpc: d2rpc::MeshDescriptor = mesh.clone().into();
        let request = Request::new(
            d2rpc::AddMeshRequest{
                client_name: self.client_name.clone(),
                mesh_descriptor: Some(mesh_desc_rpc)
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_mesh(request))?;
        Ok(response.into_inner().id as usize)
    }
    //}}}
    //{{{ fun: clear
    pub fn clear(&mut self) -> Result<(), Error>  
    {
        let request = Request::new(
            d2rpc::ClearRequest {
                client_name: self.client_name.clone(),
            }
        );
        let _ = self.tokio_runtime.block_on(self.stub.clear(request))?;
        Ok(())
    }
    //..............................................................................
    //}}}
    //{{{ fun: kill_server
    pub fn kill_server(&mut self) -> Result<(), Error>
    {
        let request = Request::new(
            d2rpc::KillServerRequest {
                client_name: self.client_name.clone(),
            }
        );
        let _ = self.tokio_runtime.block_on(self.stub.kill_server(request))?;
        Ok(())
    }
    //..............................................................................
    //}}}
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
}
//}}}
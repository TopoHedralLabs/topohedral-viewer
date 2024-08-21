//! Implements the server-side computation for the TopoHedralViewer RPC server running in 2D mode.
//!
//! The general process for handling requests to add geometry is we:
//! - Receive the request 
//! - Validate the inputs
//! - If valid we offload to the corresponding operation on the state object (i.e. adding meshes, 
//!   removing meshes etc)
//! 
//! 
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::d2rpc;
use super::common::*;
use super::super::mesh::{AxesDescriptor, SquareDescriptor, CircleDescriptor};
use super::super::state::{State, State2D};
use crate::common::Validated;
//}}}
//{{{ std imports 
use core::net::SocketAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
//}}}
//{{{ dep imports 
use log::{error, info};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: StateServer
pub struct StateServer
{
    state: Arc<Mutex<State<'static>>>,
    shutdown_sender: mpsc::Sender<()>,
}
//}}}
//{{{ trait StateService for StateServer
#[tonic::async_trait]
impl d2rpc::state_service_server::StateService for StateServer
{
    //{{{ fun: add_axes
    async fn add_axes(
        &self,
        request: Request<d2rpc::AddAxesRequest>,
    ) -> Result<Response<d2rpc::AddItemResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_axes request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let out:  Result<Response<d2rpc::AddItemResponse>, Status> = if msg.is_valid() 
        {
            let axes_desc = msg.axes_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_axes(&axes_desc);
            let add_axes_result = d2rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_axes_result))
        }
        else {
            Err(Status::invalid_argument("Invalid axes descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_square
    async fn add_square(
        &self,
        request: Request<d2rpc::AddSquareRequest>,
    ) -> Result<Response<d2rpc::AddItemResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();

        //{{{ trace
        info!(
            "Received add_axes request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        
        let out: Result<Response<d2rpc::AddItemResponse>, Status> = if msg.is_valid() 
        {
            let square_desc = msg.square_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_square(&square_desc);
            let add_square_result = d2rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_square_result))
        }
        else 
        {
            Err(Status::invalid_argument("Invalid square descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_circle
    async fn add_circle(
        &self,
        request: Request<d2rpc::AddCircleRequest>,
    ) -> Result<Response<d2rpc::AddItemResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_axes request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}

        let out = if msg.is_valid()
        {
            let circle_desc = msg.circle_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_circle(&circle_desc);
            let add_circle_result = d2rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_circle_result))
        } 
        else 
        {
            Err(Status::invalid_argument("Invalid circle descriptor"))
        };
        out
    }   
    //}}}
    //{{{ fun: kill_server
    async fn kill_server(
        &self,
        request: Request<d2rpc::KillServerRequest>,
    ) -> Result<Response<d2rpc::KillServerResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();

        info!(
            "Received kill_server request from {} on port {:?}",
            msg.client_name, addr
        );

        self.shutdown_sender.send(()).await.map_err(|e| {
            Status::internal(format!(
                "Failed to send shutdown signal due to error: {}",
                e
            ))
        })?;

        Ok(Response::new(d2rpc::KillServerResponse {}))
    }
    //}}}
}
//}}}
//{{{ fun: run_server
pub async fn run_server(
    state: Arc<Mutex<State<'static>>>,
    rpc_address: SocketAddr,
    shutdown_sender: mpsc::Sender<()>,
    mut shutdown_receiver: mpsc::Receiver<()>,
)
{
    info!("Starting RPC server on port {}", rpc_address);


    let state_server = StateServer {
        state: state,
        shutdown_sender: shutdown_sender,
    };

    let server = Server::builder()
        .add_service(d2rpc::state_service_server::StateServiceServer::new(
            state_server,
        ))
        .serve_with_shutdown(rpc_address, async {
            // wait for shutdown signal
            shutdown_receiver.recv().await.unwrap();
            info!("Received shutdown signal, now re-sending");
        });

    if let Err(e) = server.await
    {
        error!("Server error: {}", e);
    }
}
//}}}

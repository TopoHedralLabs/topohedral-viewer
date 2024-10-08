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
use super::super::mesh::{AxesDescriptor, SquareDescriptor, CircleDescriptor, Mesh};
use super::super::state::{State, State2D};
use crate::common::Validated;
use crate::app::TopoHedralEvent;
//}}}
//{{{ std imports 
use core::net::SocketAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
//}}}
//{{{ dep imports 
use topohedral_tracing::{error, info, topo_log};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use winit::event_loop::EventLoopProxy;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: StateServer
/// The `StateServer` struct is responsible for managing the state of the 2D rendering system.
/// It holds a reference to the shared state object (`State<'static>`), which is protected by a mutex,
/// and a channel for sending shutdown signals.
/// This struct is likely used as part of the RPC server implementation to handle incoming requests
/// and update the rendering state accordingly.
pub struct StateServer
{
    state: Arc<Mutex<State<'static>>>,
    shutdown_sender: mpsc::Sender<()>,
}
//}}}
//{{{ impl d2rpc::state_service_server::StateService for StateServer
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
    async fn add_line(
        &self, 
        request: Request<d2rpc::AddLineRequest>,
    ) -> Result<Response<d2rpc::AddItemResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!("Received add_line request from {} on port {:?}", msg.client_name, addr);
        //}}}
        let out: Result<Response<d2rpc::AddItemResponse>, Status> = if msg.is_valid()   
        {
            let line_desc = msg.line_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_line(&line_desc);
            let add_line_result = d2rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_line_result))
        }
        else {
            Err(Status::invalid_argument("Invalid line descriptor"))
        };
        out
    }
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
    //{{{ fun: add_mesh
    async fn add_mesh(
        &self,
        request: Request<d2rpc::AddMeshRequest>,
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
            let mesh: Mesh = msg.mesh_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_mesh(mesh);
            let add_mesh_result = d2rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_mesh_result))
        }
        else 
        {
            Err(Status::invalid_argument("Invalid mesh descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: clear
    async fn clear(
        &self,
        request: Request<d2rpc::ClearRequest>,
    ) -> Result<Response<d2rpc::ClearResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received clear_all request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let mut state = self.state.lock().unwrap();
        state.clear();
        Ok(Response::new(d2rpc::ClearResponse {}))
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
        //{{{  trace
        info!(
            "Received kill_server request from {} on port {:?}",
            msg.client_name, addr
        );
        info!("Sending Shutdown signal");
        //}}}
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
    event_loop_proxy: EventLoopProxy<TopoHedralEvent>,
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
            info!("Received shutdown signal, shutting down RPC server");
        });

    if let Err(e) = server.await
    {
        error!("Server error: {}", e);
    }

    //{{{ trace
    info!("Sending Shutdown Event");
    //}}}
    event_loop_proxy.send_event(TopoHedralEvent::RcpShutdown).unwrap();
}
//}}}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {}
//}}}
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
use super::super::mesh::{AxesDescriptor, SquareDescriptor, CircleDescriptor};
use super::super::state::{State, State2D};
use crate::common::Vec2;
//}}}
//{{{ std imports 
use core::net::SocketAddr;
use std::sync::{Arc, Mutex};
//}}}
//{{{ dep imports 
use log::{error, info};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ impl: AddAxesRequest
impl d2rpc::AddAxesRequest
{
    pub fn is_valid(&self) -> bool
    {
        let mut is_val = true;
        match self.axes_descriptor
        {
            Some(ref axes_descriptor) =>
            {
                is_val &= axes_descriptor.origin.is_some();
                is_val &= axes_descriptor.x_axis.is_some();
                is_val &= axes_descriptor.y_axis.is_some();
                is_val &= axes_descriptor.pos_len > 0.0 && axes_descriptor.neg_len > 0.0;
            }
            None =>
            {
                is_val = false;
            }
        }
        is_val
    }
}
//}}}
//{{{ impl: From<d2rpc::Vec2> for Vec2
impl From<d2rpc::Vec2> for Vec2
{
    fn from(v: d2rpc::Vec2) -> Self
    {
        Vec2::new(v.x, v.y)
    }
}
//}}}
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
    async fn add_axes(
        &self,
        request: Request<d2rpc::AddAxesRequest>,
    ) -> Result<Response<d2rpc::AddAxesResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();

        info!(
            "Received add_axes request from {} on port {:?}",
            msg.client_name, addr
        );

        let mut out: Result<Response<d2rpc::AddAxesResponse>, Status> =
            Err(Status::invalid_argument("Invalid axes descriptor"));

        if let Some(axes_desc_rpc) = msg.axes_descriptor
        {
            let axes_desc = AxesDescriptor {
                origin: axes_desc_rpc.origin.unwrap().into(),
                x_axis: axes_desc_rpc.x_axis.unwrap().into(),
                y_axis: axes_desc_rpc.y_axis.unwrap().into(),
                pos_len: axes_desc_rpc.pos_len,
                neg_len: axes_desc_rpc.neg_len,
            };

            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_axes(&axes_desc);
            let add_axes_result = d2rpc::AddAxesResponse {
                axes_id: mesh_uid as u64,
            };
            out = Ok(Response::new(add_axes_result));
        }
        out
    }

    async fn add_square(
        &self,
        request: Request<d2rpc::AddSquareRequest>,
    ) -> Result<Response<d2rpc::AddSquareResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();
    }

    async fn add_circle(
        &self,
        request: Request<d2rpc::AddCircleRequest>,
    ) -> Result<Response<d2rpc::AddCircleResponse>, Status>
    {
        let addr = request.remote_addr();
        let msg = request.into_inner();
    }   

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

//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports
use crate::common::Validated;
use super::common::*;
use super::d3rpc;
use crate::d3;
use crate::d3::mesh::{
    AxesDescriptor, CuboidDescriptor, CylinderDescriptor, LineDescriptor, PlaneDescriptor,
    SphereDescriptor, TriangleDescriptor, Mesh
};
use crate::d3::state::{State, State3D};
use crate::app::TopoHedralEvent;
//}}}
//{{{ std imports
use core::net::SocketAddr;
use std::result::Result;
use std::sync::{Arc, Mutex};
//}}}
//{{{ dep imports
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
use topohedral_tracing::{error, info, topo_log};
use winit::event_loop::EventLoopProxy;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ struct: StateServer
struct StateServer {
    state: Arc<Mutex<State<'static>>>,
    shutdown_sender: mpsc::Sender<()>,
}
//..............................................................................
//}}}
//{{{ impl d3rpc::state_service_server::StateService for StateServer
#[tonic::async_trait]
impl d3rpc::state_service_server::StateService for StateServer {
    //{{{ fun: add_line
    async fn add_line(
        &self,
        request: tonic::Request<d3rpc::AddLineRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_line request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let out = if msg.is_valid() {
            let line_desc = msg.line_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_line(&line_desc);
            let add_line_result = d3rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_line_result))
        }
        else 
        {
            Err(Status::invalid_argument("Invalid line descriptor"))    
        };
        out
    }
    //}}}
    //{{{ fun: add_triangle
    async fn add_triangle(
        &self,
        request: tonic::Request<d3rpc::AddTriangleRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_triangle request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let out = if msg.is_valid() {
            let triangle_desc = msg.triangle_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_triangle(&triangle_desc);
            let add_triangle_result = d3rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_triangle_result))  
        }
        else 
        {
            Err(Status::invalid_argument("Invalid triangle descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_plane
    async fn add_plane(
        &self,
        request: tonic::Request<d3rpc::AddPlaneRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {

        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_plane request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let out = if msg.is_valid() {
            let plane_desc = msg.plane_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_plane(&plane_desc);
            let add_triangle_result = d3rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_triangle_result))  
        }
        else 
        {
            Err(Status::invalid_argument("Invalid triangle descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_cuboid
    async fn add_cuboid(
        &self,
        request: tonic::Request<d3rpc::AddCuboidRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {
        let addr = request.remote_addr();
        let msg = request.into_inner(); 
        //{{{ trace
        info!("Received add_cuboid request from {} on port {:?}", msg.client_name, addr);
        //}}}
        let out = if msg.is_valid() {
            let cuboid_desc = msg.cuboid_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_cuboid(&cuboid_desc);
            let add_cuboid_result = d3rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_cuboid_result))
        }
        else
        {
            Err(Status::invalid_argument("Invalid cuboid descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_cylinder
    async fn add_cylinder(
        &self,
        request: tonic::Request<d3rpc::AddCylinderRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_cylinder request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let out = if msg.is_valid() {
            let cylinder_desc = msg.cylinder_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_cylinder(&cylinder_desc);
            let add_cylinder_result = d3rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_cylinder_result))
        }
        else
        {
            Err(Status::invalid_argument("Invalid cylinder descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_sphere
    async fn add_sphere(
        &self,
        request: tonic::Request<d3rpc::AddSphereRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_sphere request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let out = if msg.is_valid() {
            let sphere_desc = msg.sphere_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_sphere(&sphere_desc);
            let add_sphere_result = d3rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_sphere_result))
        }
        else
        {
            Err(Status::invalid_argument("Invalid sphere descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_axes
    async fn add_axes(
        &self,
        request: tonic::Request<d3rpc::AddAxesRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!( "Received add_axes request from {} on port {:?}", msg.client_name , addr);
        //}}}
        let out = if msg.is_valid() {
            let axes_desc = msg.axes_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_axes(&axes_desc);
            let add_axes_result = d3rpc::AddItemResponse {
                id: mesh_uid as u64,
            };
            Ok(Response::new(add_axes_result))
        }
        else
        {
            Err(Status::invalid_argument("Invalid axes descriptor"))
        };
        out
    }
    //}}}
    //{{{ fun: add_mesh
    async fn add_mesh(
        &self,
        request: tonic::Request<d3rpc::AddMeshRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::AddItemResponse>, tonic::Status> {
        let addr = request.remote_addr();
        let msg = request.into_inner();
        //{{{ trace
        info!(
            "Received add_mesh request from {} on port {:?}",
            msg.client_name, addr
        );
        //}}}
        let out = if msg.is_valid() {
            let mesh: Mesh = msg.mesh_descriptor.unwrap().into();
            let mut state = self.state.lock().unwrap();
            let mesh_uid = state.add_mesh(mesh);
            let add_mesh_result = d3rpc::AddItemResponse {
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
        request: tonic::Request<d3rpc::ClearRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::ClearResponse>, tonic::Status> {
        todo!();
    }
    //}}}
    //{{{ fun: kill_server
    async fn kill_server(
        &self,
        request: tonic::Request<d3rpc::KillServerRequest>,
    ) -> std::result::Result<tonic::Response<d3rpc::KillServerResponse>, tonic::Status> {

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

        Ok(Response::new(d3rpc::KillServerResponse {}))
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
    event_loop_proxy: EventLoopProxy<TopoHedralEvent>
)
{
    info!("Starting RPC server on port {}", rpc_address);


    let state_server = StateServer {
        state: state,
        shutdown_sender: shutdown_sender,
    };

    let server = Server::builder()
        .add_service(d3rpc::state_service_server::StateServiceServer::new(
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

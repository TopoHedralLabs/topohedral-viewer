//! This module defines an RPC client for the 2D viewer.
//!
//! This client also launches and kills the RPC server rather than connecting to a pre-existing
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use super::super::mesh::{AxesDescriptor, SquareDescriptor, CircleDescriptor};
use super::super::state::{State, State2D};
use crate::common::Vec2;
use super::d2rpc::state_service_client::StateServiceClient;
use std::path::PathBuf;
use std::result::Result;
use super::d2rpc;
//}}}
//{{{ std imports 
use std::sync::{Arc, Mutex};
use core::net::SocketAddr;
use std::process::{Command, Child};
use clap::error;
//}}}
//{{{ dep imports 
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
use topohedral_tracing::*;
use thiserror::Error;

use tokio::runtime::{Handle, Runtime};
use tokio::task;
use tokio::runtime;
//}}}
//--------------------------------------------------------------------------------------------------


type RpcClient = StateServiceClient<tonic::transport::Channel>;

//{{{ enum:Client2DError
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
pub struct Client2D
{
    client_name: String,
    stub: RpcClient,
    tokio_runtime: Runtime,
}
//..............................................................................
//}}}
impl Client2D 
{

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

}
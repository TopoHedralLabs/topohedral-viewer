
use std::borrow::Borrow;
//.................................. std
use std::sync::{Arc, Mutex};
use core::net::SocketAddr;
//.................................. 3rd party
use log::{error, info};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};
//.................................. crate

use super::d2rpc::state_service_client::StateServiceClient;

use super::super::mesh::AxesDescriptor;
use super::super::state::{State, State2D};
use crate::common::Vec2;


pub type Client = StateServiceClient<tonic::transport::Channel>;
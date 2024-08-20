//! This module defines the TopoHedralViewer application.
//!
//! This module defines the options which the application accepts, and the entry point for the
//! application itself in ``run_topoviewer``.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::d2;
use crate::d3;
//}}}
//{{{ std imports 
use core::net::SocketAddr;
use std::fmt::Display;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::{Arc, Mutex};
//}}}
//{{{ dep imports 
use winit::{
    self,
    application::ApplicationHandler,
    event_loop::{ControlFlow, EventLoop},
};
use winit::event::WindowEvent;

use tokio::runtime::{Handle, Runtime};
use tokio::task;
use tokio::{runtime, sync::mpsc};
use clap::{Parser, Subcommand, ValueEnum};

use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ col: StateHandles
/// A shared, thread-safe handle to the 2D viewer state.
pub type State2Handle<'a> = Arc<Mutex<d2::State<'a>>>;
/// A shared, thread-safe handle to the 3D viewer state.
pub type State3Handle<'a> = Arc<Mutex<d3::State<'a>>>;
//..................................................................................................
//}}}
//{{{ enum: RPCOption
/// The RPCOption enum contains the options for whether to start the RPC server and if so on
/// what port
#[derive(Clone, Copy, Debug, Subcommand)]
pub enum RPCOption
{
    /// The NoRPC option indicates that the RPC server should not be started.
    None,
    /// The WithPort option indicates that the RPC server should be started on the specified
    WithPort
    {
        port: u16
    },
}
//..................................................................................................
//}}}
//{{{ enum: Mode
/// The Mode enum contains the options for whether to start the 2D or 3D viewer.
#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Mode
{
    /// The D2 option indicates that the 2D viewer should be started.
    D2,
    /// The D3 option indicates that the 3D viewer should be started.
    D3,
}
//..................................................................................................
//}}}
//{{{ struct: TopoViewerOptions
/// The TopoViewerOptions struct contains the options that can be passed to the TopoViewer
/// constructor.
#[derive(Debug, Clone, Copy, Parser)]
#[command(
    name = "TopoViewer",
    about = "A 2D and 3D viewer for topological data",
    version = "0.1.0",
    author = "John Alexander Ferguson, JAFerguson952@gmail.com"
)]
pub struct TopoViewerOptions
{
    /// The mode option indicates whether to start the 2D or 3D viewer.
    #[arg(value_enum)]
    pub mode: Mode,
    /// The with_rpc option indicates whether to start the RPC server.
    #[command(subcommand)]
    pub with_rpc: RPCOption,
}

impl Display for TopoViewerOptions
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result
    {
        write!(f, "Mode: ")?;
        match self.mode
        {
            Mode::D2 => write!(f, "2D")?,
            Mode::D3 => write!(f, "3D")?,
        }
        write!(f, ", RPC: ")?;
        match self.with_rpc
        {
            RPCOption::None => write!(f, "None")?,
            RPCOption::WithPort { port } => write!(f, "RPC server on port {}", port)?,
        }
        std::fmt::Result::Ok(())
    }
}
//..................................................................................................
//}}}
//{{{ enum: RcpStatus
/// ENcodes the status of the RPC server. This can be non-existent if running topoviewer in non-rcp 
/// mode, running if the rcp exists and is running or stopped if the rcp exists but is not running.
enum RcpStatus
{
    /// Denotes non-existent RPC server.    
    None, 
    /// Denotes that the RPC server exists but is running.
    Running, 
    /// Denotes that the RPC server exists but is not running.
    Stopped,
}
//..................................................................................................
//}}}
//{{{ col: TopoViewer
//{{{ struct: TopoViewer
/// The TopoViewer class is the main entry point for the TopoViewer application.
pub struct TopoViewer<'a>
{
    mode: Mode,
    rpc_port: Option<u16>,
    state_2d: Option<State2Handle<'a>>,
    rpc_handle_2d: Option<task::JoinHandle<()>>,
    state_3d: Option<State3Handle<'a>>,
    rpc_handle_3d: Option<task::JoinHandle<()>>,
    tokio_runtime: Runtime,
    shutdown_sender: Option<mpsc::Sender<()>>,
}
//}}}
//{{{ impl TopoViewer
impl<'a> TopoViewer<'a>
{
    //{{{ fun: new
    /// Initialises a TopoViewer instance.
    ///
    /// This initialises the underlying state objects for either the 2D or 3D viewer and
    /// creates the Tokio runtime. If the RCP option is set then the tokio runtime is mult-threaded
    /// to handle the RPC requests on asynchronous tasks. If the RCP option is not set then the
    /// tokio runtime is single-threaded and is really just there to block async code to allow it
    /// co-exist with sync code.
    ///
    /// # Arguments
    /// * `topoviewer_options` - The options for the TopoViewer instance.
    ///
    /// # Returns
    /// The TopoViewer instance.
    pub fn new(topoviewer_options: &TopoViewerOptions) -> Self
    {
        //{{{ trace: enter
        info!("Initialising TopoViewer with options {}", topoviewer_options);
        //}}}

        let state_2d = match topoviewer_options.mode
        {
            Mode::D2 => {
                //{{{ trace
                info!("Creating  2D state");
                //}}}
                Some(d2::State::new_arc_mutex())
            },
            Mode::D3 => None,
        };

        let state_3d = match topoviewer_options.mode
        {
            Mode::D2 => None,
            Mode::D3 => {
                //{{{ trace
                info!("Creating 3D state");
                //}}}
                Some(d3::State::new_arc_mutex())
            },
        };

        let tokio_runtime = match topoviewer_options.with_rpc
        {
            RPCOption::None =>  {
                //{{{ trace
                info!("Building single-threaded tokio runtime");
                //}}}
                runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
            },
            RPCOption::WithPort { port: _ } =>  {
                //{{{ trace
                info!("Building multi-threaded tokio runtime");
                //}}}
                runtime::Builder::new_multi_thread()
                    .worker_threads(2)
                    .enable_all()
                    .build()
                    .unwrap()
            },
        };

        TopoViewer {
            mode: topoviewer_options.mode,
            rpc_port: match topoviewer_options.with_rpc
            {
                RPCOption::None => None,
                RPCOption::WithPort { port } => Some(port),
            },
            state_2d: state_2d,
            rpc_handle_2d: None,
            state_3d: state_3d,
            rpc_handle_3d: None,
            tokio_runtime: tokio_runtime,
            shutdown_sender: None,
        }
    }
    //}}}
    //{{{ fun: runtime_handle§
    pub fn runtime_handle(&self) -> Handle
    {
        self.tokio_runtime.handle().clone()
    }
    //}}}
    //{{{ fun: get_state_2d_mut
    /// Returns a mutable reference to the 2D state handle, if the current mode is 2D.
    ///
    /// If the current mode is 3D, this method returns `None`.
    pub fn get_state_2d_mut(&mut self) -> Option<State2Handle<'a>>
    {
        match self.mode
        {
            Mode::D2 =>
            {
                if let Some(state) = self.state_2d.as_ref()
                {
                    Some(state.clone())
                }
                else
                {
                    None
                }
            }
            Mode::D3 => None,
        }
    }
    //}}}
    //{{{ fun: get_state_3d_mut
    /// Returns a mutable reference to the 3D state handle, if the current mode is 3D.
    ///
    /// If the current mode is 2D, this method returns `None`.
    pub fn get_state_3d_mut(&mut self) -> Option<State3Handle<'a>>
    {
        match self.mode
        {
            Mode::D2 => None,
            Mode::D3 =>
            {
                if let Some(state) = self.state_3d.as_ref()
                {
                    Some(state.clone())
                }
                else
                {
                    None
                }
            }
        }
    }
    //}}}
    //{{{ fun: server_status 
    /// Returns whether the RPC server is currently running.
    ///
    /// This method checks the status of the RPC server handle for the current mode (2D or 3D) and
    /// returns `true` if the handle is still running, indicating that the RPC server is active.
    fn server_status(&self) -> RcpStatus
    {
        let mut status = RcpStatus::None;

        match self.mode
        {
            Mode::D2 =>
            {
                if let Some(handle) = self.rpc_handle_2d.as_ref()
                {
                    if handle.is_finished()
                    {
                        status = RcpStatus::Stopped;
                    }
                    else 
                    {
                        status = RcpStatus::Running;
                    }
                }
            }
            Mode::D3 =>
            {
                if let Some(handle) = self.rpc_handle_3d.as_ref()
                {
                    if handle.is_finished()
                    {
                        status = RcpStatus::Stopped;
                    }
                    else 
                    {
                        status = RcpStatus::Running;
                    }
                }
            }
        }

        status 
    }
    //}}}
}
//}}}
//{{{ impl: ApplicationHandler for TopoViewer
impl ApplicationHandler<TopoHedralEvent> for TopoViewer<'static>
{
    //{{{ fun: resumed
    fn resumed(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
    )
    {
        //{{{ trace
        info!("Resumed application");
        //}}}
        match self.mode
        {
            //{{{ case: 2D
            Mode::D2 =>
            {
                if let Some(state) = self.state_2d.clone()
                {
                    if let Some(port) = self.rpc_port
                    {
                        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);

                        let state_clone = state.clone();

                        let (shutdown_sender, shutdown_receiver) = mpsc::channel::<()>(4);
                        self.shutdown_sender = Some(shutdown_sender.clone());

                        let shutdown_sender_clone = shutdown_sender.clone();

                        self.tokio_runtime.spawn(async move {
                            tokio::signal::ctrl_c().await.unwrap();
                            //{{{ trace
                            info!("Received a Ctrl-C signal");
                            info!("Sending shutdown signal");
                            //}}}
                            shutdown_sender_clone.send(()).await.unwrap();
                        });
                        //{{{ trace
                        info!("Launching RPC server with socket: {}", socket);
                        //}}}

                        let handle = self.tokio_runtime.spawn(async move {
                            //{{{ trace
                            info!("Launching RPC server");
                            //}}}
                            d2::run_server(state_clone, socket, shutdown_sender, shutdown_receiver)
                                .await
                        });
                        self.rpc_handle_2d = Some(handle);
                    }
                    info!("Launching 2D window");
                    self.tokio_runtime
                        .block_on(state.lock().unwrap().launch_window(event_loop));
                }
            }
            //}}}
            //{{{ case: 3D
            Mode::D3 =>
            {}
            //}}}
        }
    }
    //}}}
    //{{{ fun: window_event
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    )
    {
        match event
        {
            //{{{ case: CloseRequested
            WindowEvent::CloseRequested =>
            {
                //{{{ trace
                info!("Close requested");
                //}}}
                if let Some(shutdown_sender) = self.shutdown_sender.as_ref()
                {
                    //{{{ trace
                    info!("Shutting down RPC server");
                    //}}}
                    self.tokio_runtime.block_on(shutdown_sender.send(())).unwrap();
                }
                //{{{ trace
                info!("Exiting Application");
                //}}}
                event_loop.exit();
            }
            //}}}
            //{{{ default
            _ => match self.mode
            {
                Mode::D2 =>
                {
                    if let Some(state) = self.state_2d.as_ref()
                    {
                        state.lock().unwrap().handle_event(&window_id, &event);
                    }
                }
                Mode::D3 =>
                {
                    if let Some(state) = self.state_3d.as_ref()
                    {
                        state.lock().unwrap().handle_event(&window_id, &event);
                    }
                }
            },
            //}}}
        }

        match self.server_status()
        {
            //{{{ case: Stopped
            RcpStatus::Stopped =>
            {
                //{{{ trace
                info!("RPC server is stopped, closing 2D window");
                //}}}
                event_loop.exit();
            }
            //}}}
            //{{{ default
            _ => {},
            //}}}
        }
    }
    //}}}
    //{{{ fun: user_event
    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: TopoHedralEvent) {
        match event
        {
            TopoHedralEvent::RcpShutdown =>
            {
                //{{{ trace
                info!("Received RPC shutdown event, exiting application");
                //}}}
                event_loop.exit();
            }
        }
    }
    //}}}
}
//}}}
//..................................................................................................
//}}}
//{{{ enum:  TopoHedralEvent
enum TopoHedralEvent
{
    RcpShutdown,
}
//}}}
//{{{ fun: run_topoviewer
pub fn run_topoviewer(topoviewer_options: &TopoViewerOptions)
{
    //{{{ trace
    info!("Initializing winit event loop");
    //}}}
    // initialize the winit event loop
    let event_loop = EventLoop::<TopoHedralEvent>::with_user_event().build().expect("Failed to create winit event loop");
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = TopoViewer::new(topoviewer_options);
    event_loop.run_app(&mut app).unwrap();
}
//}}}

//.................................. std
//.................................. 3rd party
use env_logger;
use log::info;

use clap::Parser;
//.................................. crate
use topohedral_viewer::app::{run_topoviewer, TopoViewerOptions, Mode, RPCOption};



fn main()
{
    env_logger::init();
    let opts = TopoViewerOptions::parse();
    info!("Starting TopoViewer with options {}", opts);
    run_topoviewer(&opts);
}
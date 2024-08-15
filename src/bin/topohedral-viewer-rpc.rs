//! This executable is the topohedral-viewer-rpc. It is a command line tool that
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use topohedral_viewer::app::{run_topoviewer, TopoViewerOptions};
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
use clap::Parser;
use topohedral_tracing::{topo_log, init, info};
//}}}
//--------------------------------------------------------------------------------------------------


fn main()
{
    init().unwrap();
    let opts = TopoViewerOptions::parse();
    //{{{ trace
    info!("Starting TopoViewer with options {}", opts);
    //}}}
    run_topoviewer(&opts);
}
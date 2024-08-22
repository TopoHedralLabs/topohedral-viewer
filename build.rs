//! The purpose of this build script is to generate the Rust code for the RPC services defined 
//! in the `protos` directory.
//!
//! The protos are compiled using the `tonic` crate, which generates Rust code from the proto files.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
//}}}
//{{{ std imports 
//}}}
//{{{ dep imports 
//}}}
//--------------------------------------------------------------------------------------------------

fn main() -> Result<(), Box<dyn std::error::Error>> {   

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/d2/rpc")
        .compile(&["protos/d2.proto"], &["protos"])?;

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/d3/rpc")
        .compile(&["protos/d3.proto"], &["protos"])?;

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {   

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("src/d2/rpc")
        .compile(&["protos/d2.proto"], &["protos"])?;

    Ok(())
}
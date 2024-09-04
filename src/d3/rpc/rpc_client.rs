//! Short Description of module
//!
//! Longer description of module
//--------------------------------------------------------------------------------------------------


//{{{ crate imports
use super::super::mesh::*;
use super::d3rpc;
//}}}
//{{{ std imports
//}}}
//{{{ dep imports
use tonic::Request;
use topohedral_tracing::*;
use thiserror::Error;
use tokio::runtime::Runtime;
use tokio::runtime;
//}}}
//--------------------------------------------------------------------------------------------------

type RpcClient = d3rpc::state_service_client::StateServiceClient<tonic::transport::Channel>;

//{{{ enum: Error
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Tonic error: {0}")]
    TonicError(#[from] tonic::transport::Error),
    #[error("Status error: {0}")]
    StatusError(#[from] tonic::Status),
}
//..............................................................................
//}}}

//{{{ struct: Client3D
pub struct Client3D {
    client_name: String,
    stub: RpcClient,
    tokio_runtime: Runtime,
}
//..............................................................................
//}}}

impl Client3D {

    //{{{ fun: new 
    pub fn new(port: usize) -> Result<Self, Error> {
        //{{{ trace
        info!("Starting 3D client");
        //}}}
        let client_name = "client3d";
        let tokio_runtime = runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        let stub =
            tokio_runtime.block_on(RpcClient::connect(format!("http://127.0.0.1:{}", port)))?;
        Ok(Self {
            client_name: client_name.to_string(),
            stub: stub,
            tokio_runtime: tokio_runtime,
        })
    }
    //}}}
    //{{{ fun: add_line
    pub fn add_line(&mut self, line_desc: LineDescriptor)  -> Result<usize, Error> 
    {
        let line_desc_rpc: d3rpc::LineDescriptor = line_desc.into();
        let request = Request::new(
            d3rpc::AddLineRequest {
                client_name: self.client_name.clone(),
                line_descriptor: Some(line_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_line(request))?;
        Ok(response.into_inner().id as usize)    
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_triangle
    pub fn add_triangle(&mut self, triangle_desc: TriangleDescriptor)  -> Result<usize, Error>
    {
        let triangle_desc_rpc: d3rpc::TriangleDescriptor = triangle_desc.into();
        let request = Request::new(
            d3rpc::AddTriangleRequest {
                client_name: self.client_name.clone(),
                triangle_descriptor: Some(triangle_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_triangle(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_plane
    pub fn add_plane(&mut self, plane_desc: PlaneDescriptor) -> Result<usize, Error>
    {
        let plane_desc_rpc: d3rpc::PlaneDescriptor = plane_desc.into(); 
        let request = Request::new(
            d3rpc::AddPlaneRequest {
                client_name: self.client_name.clone(),
                plane_descriptor: Some(plane_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_plane(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_cuboid
    pub fn add_cuboid(&mut self, cuboid_desc: CuboidDescriptor) -> Result<usize, Error>
    {
        let cuboid_desc_rpc: d3rpc::CuboidDescriptor = cuboid_desc.into();
        let request = Request::new(
            d3rpc::AddCuboidRequest {
                client_name: self.client_name.clone(),
                cuboid_descriptor: Some(cuboid_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_cuboid(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_cylinder
    pub fn add_cylinder(&mut self, cylinder_desc: CylinderDescriptor) -> Result<usize, Error>
    {
        let cylinder_desc_rpc: d3rpc::CylinderDescriptor = cylinder_desc.into();
        let request = Request::new(
            d3rpc::AddCylinderRequest {
                client_name: self.client_name.clone(),
                cylinder_descriptor: Some(cylinder_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_cylinder(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_disc
    pub fn add_disc(&mut self, disc_desc: DiscDescriptor) -> Result<usize, Error>
    {
        let disc_desc_rpc: d3rpc::DiscDescriptor = disc_desc.into();
        let request = Request::new(
            d3rpc::AddDiscRequest {
                client_name: self.client_name.clone(),
                disc_descriptor: Some(disc_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_disc(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_sphere
    pub fn add_sphere(&mut self, sphere_desc: SphereDescriptor) -> Result<usize, Error>
    {
        let sphere_desc_rpc: d3rpc::SphereDescriptor = sphere_desc.into();
        let request = Request::new(
            d3rpc::AddSphereRequest{
                client_name: self.client_name.clone(),
                sphere_descriptor: Some(sphere_desc_rpc)
            });
        let response = self.tokio_runtime.block_on(self.stub.add_sphere(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_axes
    pub fn add_axes(&mut self, axes_desc: AxesDescriptor) -> Result<usize, Error>
    {
        let axes_desc_rpc: d3rpc::AxesDescriptor = axes_desc.into();
        let request = Request::new(
            d3rpc::AddAxesRequest {
                client_name: self.client_name.clone(),
                axes_descriptor: Some(axes_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on( self.stub.add_axes(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: add_mesh
    pub fn add_mesh<'a>(&mut self, mesh: Mesh<'a>) -> Result<usize, Error>
    {
        let mesh_desc_rpc: d3rpc::MeshDescriptor = mesh.into();
        let request = Request::new(
            d3rpc::AddMeshRequest {
                client_name: self.client_name.clone(),
                mesh_descriptor: Some(mesh_desc_rpc),
            }
        );
        let response = self.tokio_runtime.block_on(self.stub.add_mesh(request))?;
        Ok(response.into_inner().id as usize)
    }
    //..............................................................................
    //}}}
    //{{{ fun: clear
    pub fn clear(&mut self) -> Result<(), Error>  
    {
        let request = Request::new(
            d3rpc::ClearRequest {
                client_name: self.client_name.clone(),
            }
        );
        let _ = self.tokio_runtime.block_on(self.stub.clear(request))?;
        Ok(())
    }
    //..............................................................................
    //}}}
    //{{{ fun: kill_server
    pub fn kill_server(&mut self) -> Result<(), Error>
    {
        let request = Request::new(
            d3rpc::KillServerRequest {
                client_name: self.client_name.clone(),
            }
        );
        let _ = self.tokio_runtime.block_on(self.stub.kill_server(request))?;
        Ok(())
    }
    //..............................................................................
    //}}}



}

//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests {}
//}}}

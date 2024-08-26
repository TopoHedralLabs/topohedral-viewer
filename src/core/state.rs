//! This module contains the state of the application which is shared between the 2D and 3D viewers.
//!
//! This includes handling of events, rendering, and other shared functionality.
//--------------------------------------------------------------------------------------------------

//{{{ crate imports 
use crate::core::{MeshCore, VertexCore};
use crate::depth_texture as dt;
use crate::events::EventController;
//}}}
//{{{ std imports 
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
//}}}
//{{{ dep imports 
use serde::{Deserialize, Serialize};
use wgpu::{self, util::DeviceExt, Device, Features};
use winit::window::Window;
use winit::{self, event::WindowEvent, event_loop::ActiveEventLoop};
use topohedral_tracing::*;
//}}}
//--------------------------------------------------------------------------------------------------

//{{{ collection: constants
const SHADER_2D: &str = include_str!("../d2/shader2d.wgsl");
const SHADER_3D: &str = include_str!("../d3/shader3d.wgsl");
//}}}
//{{{ fun: shader_module_desc
/// This function creates a WGPU shader module descriptor based on the provided dimension value.
///
/// # Arguments
///
/// * `d` - The dimension value, either 2 or 3.
///
/// # Returns
///
/// A `wgpu::ShaderModuleDescriptor` with the appropriate shader source based on the dimension value.
///
/// # Panics
///
/// This function will panic if an invalid dimension value is provided.
fn shader_module_desc(d: usize) -> wgpu::ShaderModuleDescriptor<'static>
{
    if d == 2
    //{{{ case: 2D
    {
        wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module 2D"),
            source: wgpu::ShaderSource::Wgsl(SHADER_2D.into()),
        }
    }
    //}}}
    else if d == 3
    //{{{ case: 3D
    {
        wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module 3D"),
            source: wgpu::ShaderSource::Wgsl(SHADER_3D.into()),
        }
    }
    //}}}
    else
    //{{{ case: invalid
    {
        panic!("Invalid dimension")
    }
    //}}}
}
//}}}
//{{{ fun: create_render_pipelines
/// Creates the render pipelines for the application.
///
/// This function creates the necessary render pipelines for rendering lines, triangle edges, and triangle faces.
/// It takes in the device, surface configuration, depth texture, vertex buffer layout, and a descriptor index.
/// The function returns the created render pipelines, which can be used for rendering the corresponding geometry.
fn create_render_pipelines(
    device: &Device,
    config: &wgpu::SurfaceConfiguration,
    depth_texture: &dt::DepthTexture,
    vert_buf_layout: &[wgpu::VertexBufferLayout],
    d: usize,
) -> (
    wgpu::RenderPipeline,
    Option<wgpu::RenderPipeline>,
    wgpu::RenderPipeline,
)
{
    let shader = device.create_shader_module(shader_module_desc(d));

    let camera_bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("camera_bind_group_layout"),
        });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout 1"),
        bind_group_layouts: &[&camera_bind_group_layout],
        push_constant_ranges: &[],
    });

    let line_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Line Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vert_buf_layout,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main_line",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::LineList,
            ..Default::default()
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: dt::DepthTexture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let tri_edge_render_pipeline = if device
        .features()
        .contains(wgpu::Features::POLYGON_MODE_LINE)
    {
        Some(
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Triangle Edge Render Pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: "vs_main",
                    buffers: vert_buf_layout,
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: "fs_main_line",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: config.format,
                        blend: Some(wgpu::BlendState {
                            color: wgpu::BlendComponent::REPLACE,
                            alpha: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    strip_index_format: None,
                    polygon_mode: wgpu::PolygonMode::Line,
                    unclipped_depth: false,
                    conservative: false,
                },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: dt::DepthTexture::DEPTH_FORMAT,
                    depth_write_enabled: true,
                    depth_compare: wgpu::CompareFunction::Less,
                    stencil: wgpu::StencilState::default(),
                    bias: wgpu::DepthBiasState {
                        constant: -2,
                        slope_scale: -2.0,
                        clamp: 0.0,
                    },
                }),
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview: None,
            }),
        )
    }
    else
    {
        None
    };

    let tri_face_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Triangle Face Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vert_buf_layout,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main_triangle",
            targets: &[Some(wgpu::ColorTargetState {
                format: config.format,
                blend: Some(wgpu::BlendState {
                    color: wgpu::BlendComponent::REPLACE,
                    alpha: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: dt::DepthTexture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    (
        line_render_pipeline,
        tri_edge_render_pipeline,
        tri_face_render_pipeline,
    )
}
//..................................................................................................
//}}}
//{{{ collection: WgpuState
//{{{ struct: WgpuState
/// The `WgpuState` struct represents the state of the WGPU (WebGPU) rendering system. It contains
/// the necessary WGPU infrastructure, such as the surface, device, queue, configuration, depth
/// texture, and render pipelines. It also manages the camera buffer, bind group, and mesh buffers.
/// This struct is used to manage the rendering state and provide access to the WGPU resources
/// required for rendering.
struct WgpuState<'a>
{
    //................................. wgpu infrastructure
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    depth_texture: dt::DepthTexture,
    line_render_pipeline: wgpu::RenderPipeline,
    tri_edge_render_pipeline: Option<wgpu::RenderPipeline>,
    tri_face_render_pipeline: wgpu::RenderPipeline,
    // ............................... uniforms
    camera_buffer: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
    // ............................... mesh buffers
    wgpu_line_buffers: HashMap<usize, (u32, wgpu::Buffer, wgpu::Buffer)>,
    wgpu_tri_buffers: HashMap<usize, (u32, wgpu::Buffer, wgpu::Buffer)>,
    // ............................... Winit data
    window: Arc<Window>,
}
//}}}
//{{{ impl: WgpuState
impl<'a> WgpuState<'a>
{
    //{{{ fun: new
    /// Creates a new `WgpuState` instance with the given event loop, uniform buffer, vertex buffer 
    /// layout, and depth value.
    ///
    /// This function sets up the necessary WGPU infrastructure, including the surface, device, 
    /// queue, configuration, depth texture, and render pipelines. It also creates the camera 
    /// buffer and bind group.
    pub async fn new(
        event_loop: &ActiveEventLoop,
        uniform_buffer: &[u8],
        vert_buf_layout: &[wgpu::VertexBufferLayout<'static>],
        d: usize,
    ) -> Self
    {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let features = Features::POLYGON_MODE_LINE;

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Device for Topoviewer"),
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let surface_capbilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capbilities
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_capbilities.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capbilities.present_modes[0],
            alpha_mode: surface_capbilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let depth_texture =
            dt::DepthTexture::create_depth_texture(&device, &config, "Depth Texture");

        let (lrp, terp, tfrp) =
            create_render_pipelines(&device, &config, &depth_texture, vert_buf_layout, d);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: uniform_buffer,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Camera Bind Group Layout"),
            });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Bind Group"),
        });

        Self {
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            depth_texture: depth_texture,
            line_render_pipeline: lrp,
            tri_edge_render_pipeline: terp,
            tri_face_render_pipeline: tfrp,
            camera_bind_group: camera_bind_group,
            camera_buffer: camera_buffer,
            wgpu_line_buffers: HashMap::new(),
            wgpu_tri_buffers: HashMap::new(),
            window: window,
        }
    }
    //}}}
    //{{{ fun: update
    /// Takes an updated mesh state and updates the buffers used by the wgpu state.
    ///
    /// Meshes are static, therefore we only need to add or delete buffers as and
    /// when meshes they are created or deleted. We will not need to edit existing
    /// buffers as their corresponding meshes cannot be edited.
    pub fn update<'b, V>(
        &mut self,
        mesh_state: &mut MeshState<'b, V>,
        uniform_buffer: &[u8],
    ) where
        V: VertexCore + Deserialize<'b> + Serialize,
    {
        // delete line buffers corresponding to deleted meshes
        {
            // first, find the beffers which no longer exist
            let deleted_mesh_set: Vec<usize> = self
                .wgpu_line_buffers
                .keys()
                .filter(|mesh_uid| !mesh_state.meshes.contains_key(mesh_uid))
                .cloned()
                .collect();

            // delete the buffers
            for mesh_uid in deleted_mesh_set
            {
                self.wgpu_line_buffers.remove(&mesh_uid);
            }
        }
        // delete triangle buffers corresponding to deleted meshes
        {
            // first, find the beffers which no longer exist
            let deleted_mesh_set: Vec<usize> = self
                .wgpu_tri_buffers
                .keys()
                .filter(|mesh_uid| !mesh_state.meshes.contains_key(mesh_uid))
                .cloned()
                .collect();

            // delete the buffers
            for mesh_uid in deleted_mesh_set
            {
                self.wgpu_tri_buffers.remove(&mesh_uid);
            }
        }

        // now, find the buffers which have been added
        for (mesh_uid, mesh) in mesh_state.meshes.iter()
        {
            if mesh.is_line() && !self.wgpu_line_buffers.contains_key(mesh_uid)
            {
                let vertex_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some(format!("Vertex Buffer {}", mesh_uid).as_str()),
                            contents: bytemuck::cast_slice(mesh.vertex_slice()),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                let index_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some(format!("Index Buffer {}", mesh_uid).as_str()),
                            contents: bytemuck::cast_slice(mesh.index_slice()),
                            usage: wgpu::BufferUsages::INDEX,
                        });

                self.wgpu_line_buffers.insert(
                    *mesh_uid,
                    (mesh.num_indices() as u32, vertex_buffer, index_buffer),
                );
            }

            if mesh.is_triangle() && !self.wgpu_tri_buffers.contains_key(mesh_uid)
            {
                let vertex_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some(format!("Vertex Buffer {}", mesh_uid).as_str()),
                            contents: bytemuck::cast_slice(mesh.vertex_slice()),
                            usage: wgpu::BufferUsages::VERTEX,
                        });

                let index_buffer =
                    self.device
                        .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some(format!("Index Buffer {}", mesh_uid).as_str()),
                            contents: bytemuck::cast_slice(mesh.index_slice()),
                            usage: wgpu::BufferUsages::INDEX,
                        });

                self.wgpu_tri_buffers.insert(
                    *mesh_uid,
                    (mesh.num_indices() as u32, vertex_buffer, index_buffer),
                );
            }
        }

        // next update the uniforms
        self.queue
            .write_buffer(&self.camera_buffer, 0, uniform_buffer);
    }
    //}}}
    //{{{ fun: render
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError>
    {
        //{{{ init: local variables
        let output = self.surface.get_current_texture()?;

        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        //}}}
        //{{{ com: perform render passes
        {
            //{{{ com: initialize render pass
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Line Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.5,
                            g: 0.5,
                            b: 0.5,
                            a: 0.5,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                occlusion_query_set: None,
                timestamp_writes: None,
            });
            //}}}
            //{{{ com: line render pass
            {
                render_pass.set_pipeline(&self.line_render_pipeline);

                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

                for (uid, (num_indices, vertex_buffer, index_buffer)) in &self.wgpu_line_buffers
                {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                    render_pass.draw_indexed(0..*num_indices, 0, 0..1)
                }
            }
            //}}}
            //{{{ com: face render pass
            {
                render_pass.set_pipeline(&self.tri_face_render_pipeline);
                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

                for (uid, (num_indices, vertex_buffer, index_buffer)) in &self.wgpu_tri_buffers
                {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                    render_pass.draw_indexed(0..*num_indices, 0, 0..1)
                }
            }
            //}}}
            //{{{ com: edge render pass
            if let Some(tri_edge_render_pipeline) = &self.tri_edge_render_pipeline
            {
                render_pass.set_pipeline(tri_edge_render_pipeline);

                render_pass.set_bind_group(0, &self.camera_bind_group, &[]);

                for (uid, (num_indices, vertex_buffer, index_buffer)) in &self.wgpu_tri_buffers
                {
                    render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

                    render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint32);

                    render_pass.draw_indexed(0..*num_indices, 0, 0..1)
                }
            }
            //}}}
        }
        //}}}
        //{{{ com: submit the render pass
        self.queue.submit(Some(encoder.finish()));
        output.present();
        Ok(())
        //}}}
    }
    //}}}
    //{{{ fun: resize
    pub fn resize(
        &mut self,
        width: u32,
        height: u32,
    )
    {
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.depth_texture =
            dt::DepthTexture::create_depth_texture(&self.device, &self.config, "Depth Texture");
    }
    //}}}
    //{{{ fun: update_camera
    pub fn window_request_redraw(&mut self)
    {
        self.window.request_redraw();
    }
    //}}}
}
//..................................................................................................
//}}}
//}}}
//{{{ collection: MeshState
//{{{ struct: MeshState
/// Represents the state of a mesh in the application.
///
/// This struct contains the next unique identifier (UID) to be assigned to a new mesh,
/// as well as a HashMap that stores all the existing meshes, indexed by their UIDs.
pub 
struct MeshState<'a, V>
where
    V: VertexCore + Deserialize<'a> + Serialize,
{
    pub next_uid: usize,
    pub meshes: HashMap<usize, MeshCore<'a, V>>,
}
//}}}
//{{{ impl: MeshState   
impl<'a, V> MeshState<'a, V>
where
    V: VertexCore + Deserialize<'a> + Serialize,
{
    const START_UID: usize = 5;

    pub fn new() -> Self
    {
        Self {
            next_uid: Self::START_UID,
            meshes: HashMap::new(),
        }
    }

    pub fn add_mesh(
        &mut self,
        mut mesh: MeshCore<'a, V>,
    ) -> usize
    {
        let uid = self.next_uid();

        mesh.uid = uid;

        self.meshes.insert(uid, mesh);

        uid
    }

    fn next_uid(&mut self) -> usize
    {
        let out = self.next_uid;

        self.next_uid += 1;

        out
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ collection: StateError
//{{{ enum: StateError
#[derive(Debug)]
pub enum StateError
{
    CommandError(String),
}
//}}}
//{{{ impl: Display for StateError
impl std::fmt::Display for StateError
{
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result
    {
        match self
        {
            StateError::CommandError(msg) => write!(f, "Command Error: {}", msg),
        }
    }
}
//..................................................................................................
//}}}
//}}}
//{{{ trait: ViewStateCore
pub trait ViewStateCore
{
    fn update(&mut self);
    fn view_controller(&mut self) -> &mut EventController;
    fn view_uniform_buffer(&self) -> &[u8];
}
//..................................................................................................
//}}}
//{{{ collection: StateCore
//{{{ struct: StateCore
pub struct StateCore<'a, V, ViewState>
where
    V: VertexCore + Deserialize<'a> + Serialize,
    ViewState: ViewStateCore + Default,
{
    view_state: ViewState,
    wgpu_state: Option<WgpuState<'a>>,
    mesh_state: MeshState<'a, V>,
}
//}}}
//{{{ impl: StateCore
impl<'a, V, ViewState> StateCore<'a, V, ViewState>
where
    V: VertexCore + Deserialize<'a> + Serialize,
    ViewState: ViewStateCore + Default,
{
    //{{{ fun: new
    pub fn new() -> Self
    {
        //{{{ trace
        info!("Creating new StateCore");
        //}}}

        let view_state = ViewState::default();
        let mesh_state = MeshState::new();
        let out = StateCore {
            view_state: view_state,
            wgpu_state: None,
            mesh_state: mesh_state,
        };
        out
    }
    //}}}
    //{{{ fun: new_arc_mutex
    pub fn new_arc_mutex() -> Arc<Mutex<Self>>
    {
        Arc::new(Mutex::new(Self::new()))
    }
    //}}}
    //{{{ fun: launch_window
    pub async fn launch_window(
        &mut self,
        event_loop: &ActiveEventLoop,
    )
    {
        //{{{ trace
        info!("Launching window");
        //}}}
        let wgpu_state = WgpuState::new(
            event_loop,
            self.view_state.view_uniform_buffer(),
            &[V::desc()],
            V::dim(),
        )
        .await;
        self.wgpu_state = Some(wgpu_state);
    }
    //}}}
    //{{{ fun: handle_event
    pub fn handle_event(
        &mut self,
        window_id: &winit::window::WindowId, 
        event: &winit::event::WindowEvent,
    )
    {
        match event
        {
            //{{{ case: MouseWheel
            WindowEvent::MouseWheel { delta, .. } if self.has_window(window_id) =>
            {
                //{{{ trace
                debug!("Mouse wheel: {:?}", delta);
                //}}}
                self.view_state.view_controller().mouse_wheel_update(*delta);

                self.window_request_redraw();
            }
            //}}}
            //{{{ case: MouseInput
            WindowEvent::MouseInput { state, button, .. }if self.has_window(window_id) =>
            {
                //{{{ trace
                debug!("Mouse input: {:?} {:?}", state, button);
                //}}}
                self.view_state
                    .view_controller()
                    .mouse_input_update(*state, *button);

                self.window_request_redraw();
            }
            //}}}
            //{{{ case: CursorMoved
            WindowEvent::CursorMoved { position, .. } if self.has_window(window_id) =>
            {
                //{{{ trace
                trace!("Cursor moved: {:?}", position);
                //}}}
                self.view_state
                    .view_controller()
                    .cursor_moved_update(*position);
            }
            //}}}
            //{{{ case: KeyboardInput
            WindowEvent::KeyboardInput { event, .. } if self.has_window(window_id) =>
            {
                //{{{ trace
                debug!("Keyboard input: {:?}", event);
                //}}}
                match event.logical_key
                {
                    winit::keyboard::Key::Named(key) =>
                    {
                        self.view_state
                            .view_controller()
                            .key_input_update(event.state, key);

                        self.window_request_redraw();
                    }
                    _ =>
                    {}
                }
            }
            //}}}
            //{{{ case: ModifiersChanged
            WindowEvent::ModifiersChanged(ev) if self.has_window(window_id) =>
            {
                //{{{ trace
                debug!("Modifiers changed: {:?}", ev);
                //}}}
                self.view_state.view_controller().key_modifiers_update(ev.state());
                self.window_request_redraw();
            }
            //}}}
            //{{{ case: Resized
            WindowEvent::Resized(size) if self.has_window(window_id) =>
            {
                //{{{ trace
                debug!("Window resized: {:?}", size);
                //}}}
                self.view_state.view_controller().resize_update(*size);
                self.wgpu_state
                    .as_mut()
                    .unwrap()
                    .resize(size.width, size.height);
                self.window_request_redraw();
            }
            //}}}
            //{{{ case: RedrawRequested
            WindowEvent::RedrawRequested if self.has_window(window_id) =>
            {
                //{{{ trace
                debug!("Redraw requested");
                //}}}
                self.view_state.update();
                self.wgpu_state
                    .as_mut()
                    .unwrap()
                    .update(&mut self.mesh_state, self.view_state.view_uniform_buffer());

                match self.wgpu_state.as_mut().unwrap().render()
                {
                    Ok(()) =>
                    {
                        //{{{ trace
                        info!("Render successful");
                        //}}}
                    }
                    Err(e) =>
                    {
                        //{{{ trace
                        error!("WGPU error: {}", e);    
                        //}}}
                    }
                }
            },
            //}}}
            //{{{ default
            _ => (),
            //}}}
        }
    }
    //..............................................................
    //}}}
    //{{{ fun: has_window
    pub fn has_window(
        &mut self,
        window_id: &winit::window::WindowId,
    ) -> bool
    {
        self.wgpu_state.as_mut().unwrap().window.id() == *window_id
    }
    //..............................................................
    //}}}
    //{{{ fun: window_request_redraw
    pub fn window_request_redraw(&mut self)
    {
        self.wgpu_state.as_mut().unwrap().window_request_redraw();
    }
    //..............................................................
    //}}}
    //{{{ fun: add_mesh
    pub fn add_mesh(
        &mut self,
        mesh: MeshCore<'a, V>,
    ) -> usize
    {
        let uid = self.mesh_state.add_mesh(mesh);
        uid
    }
    //..............................................................
    //}}}
    //{{{ fun: get_mesh
    pub fn get_mesh(
        &self,
        uid: usize,
    ) -> Option<&MeshCore<'a, V>>
    {
        self.mesh_state.meshes.get(&uid)
    }
    //..............................................................
    //}}}
    //{{{ fun: get_mesh_mut
    pub fn get_mesh_mut(
        &mut self,
        uid: usize,
    ) -> Option<&mut MeshCore<'a, V>>
    {
        self.mesh_state.meshes.get_mut(&uid)
    }
    //..............................................................
    //}}}
    //{{{ fun: clear_meshes
    pub fn clear_meshes(&mut self)
    {
        self.mesh_state.meshes.clear();
    }
    //}}}
}
//}}}
//..................................................................................................
//}}}


//-------------------------------------------------------------------------------------------------
//{{{ mod: tests
#[cfg(test)]
mod tests
{
  
}
//}}}
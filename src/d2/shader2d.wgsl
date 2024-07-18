struct ViewUniform {
    view_matrix: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> view: ViewUniform;


struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) line_color: vec3<f32>,
    @location(2) tri_color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) line_color: vec3<f32>,
    @location(1) tri_color: vec3<f32>,
}


@vertex
fn vs_main( model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let pos =  view.view_matrix * vec4<f32>(model.position, 1.0, 0.0);
    out.position = vec4<f32>(pos.xy, 0.0, 1.0);
    // out.position = vec4<f32>(model.position, 0.0, 1.0);
    out.line_color = model.line_color;
    out.tri_color = model.tri_color;
    return out;
}

@fragment 
fn fs_main_line(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.line_color, 1.0);
}

@fragment 
fn fs_main_triangle(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.tri_color, 1.0);  
}
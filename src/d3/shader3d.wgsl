// This shader is a standard triangle shader, with a lighting model t

struct ViewUniform {
    view_pos: vec4<f32>, 
    view_dir: vec4<f32>,
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0)
var<uniform> view: ViewUniform;


struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) line_color: vec3<f32>,
    @location(3) tri_color: vec3<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) normal: vec3<f32>,
    @location(1) line_color: vec3<f32>,
    @location(2) tri_color: vec3<f32>,
}


@vertex
fn vs_main( model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.position = view.view_proj * vec4<f32>(model.position, 1.0);
    out.normal = model.normal;
    out.line_color = model.line_color;
    out.tri_color = model.tri_color;
    return out;
}

// let the color of the light be white
const light_color: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);
const ambient_strength: f32 = 0.5;

@fragment 
fn fs_main_line(in: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(in.line_color, 1.0);
}

// Here we use a very simple diffuse-reflection light model.
@fragment 
fn fs_main_triangle(in: VertexOutput) -> @location(0) vec4<f32> {

    let light_dir: vec3<f32> =  normalize(vec3<f32>(1.0, 1.0, 1.0));

    let ambient_color: vec3<f32> = light_color * ambient_strength;

    let diffuse_strength: f32 = max(dot(in.normal, light_dir), 0.0);
    let diffuse_color: vec3<f32> = light_color * diffuse_strength;

    let result = (ambient_color + diffuse_color)  * in.tri_color;
    return vec4<f32>(result, 1.0);  

}
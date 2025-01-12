// Vertex shader

// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0) // 1.
var<uniform> camera: CameraUniform;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
};

struct VertexOutput {
   @builtin(position) clip_position: vec4<f32>,
   @location(0) tex_coords: vec2<f32>,
   @location(1) world_normal: vec3<f32>,
   @location(2) world_position: vec3<f32>,
};

struct ModelMat{
   @location(5) model_matrix_0: vec4<f32>,
   @location(6) model_matrix_1: vec4<f32>,
   @location(7) model_matrix_2: vec4<f32>,
   @location(8) model_matrix_3: vec4<f32>,
}

@vertex
fn vs_main(
    model: VertexInput,
    model_mat: ModelMat,
) -> VertexOutput {
    var out: VertexOutput;
    let model_matrix = mat4x4<f32>(
        model_mat.model_matrix_0,
        model_mat.model_matrix_1,
        model_mat.model_matrix_2,
        model_mat.model_matrix_3,
    );

    let world_position: vec4<f32> = model_matrix * vec4<f32>(model.position, 1.0);
    out.clip_position = camera.view_proj * world_position;
    out.tex_coords = model.tex_coords;
    out.world_normal = model.normal;
    out.world_position = world_position.xyz;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let lightPos = vec3<f32>(4.0, 5.0, -3.0);
    let lightColor = vec3<f32>(0.5, 0.5, 0.5);
    let lightDir = normalize(lightPos - in.world_position);
    let lightDirectionalStrength: f32 = max(dot(in.world_normal, lightDir), 0.0);
    let lightStrength = ( 0.3 + 7 * lightDirectionalStrength);

    return textureSample(t_diffuse, s_diffuse, in.tex_coords) * vec4<f32>(lightStrength *lightColor, 1.0);
}




// Vertex shader

// Vertex shader
struct CameraUniform {
    view_proj: mat4x4<f32>,
};
@group(1) @binding(0)
var<uniform> camera: CameraUniform;

@group(3) @binding(0)
var<storage, read> joints: array<mat2x4<f32>>;

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) tex_coords: vec2<f32>,
    @location(3) affected_joints: vec4<u32>,
    @location(4) joint_weights: vec4<f32>,
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

fn getJointTransform(affected_joints: vec4<u32>, weights: vec4<f32>) -> mat2x4<f32> {
    let dq0:mat2x4<f32> = joints[affected_joints.x];
    let dq1:mat2x4<f32> = joints[affected_joints.y];
    let dq2:mat2x4<f32> = joints[affected_joints.z];
    let dq3:mat2x4<f32> = joints[affected_joints.w];

    let wx = weights.x;
    let wy = weights.y * sign(dot(dq0[0], dq1[0]));
    let wz = weights.z * sign(dot(dq0[0], dq2[0]));
    let ww = weights.w * sign(dot(dq0[0], dq3[0]));

    let result: mat2x4<f32> = wx * dq0 + wy * dq1 + wz * dq2 + ww * dq3;

    let norm = length(result[0]);
    return result * (1 / norm);
}

fn getskinMat(model: VertexInput) -> mat4x4<f32> {
    let bone:mat2x4<f32> = getJointTransform(model.affected_joints, model.joint_weights);
    let r = bone[0];
    let t = bone[1];

    return mat4x4<f32>(
        1.0 - (2.0 * r.y * r.y) - (2.0 * r.z * r.z),
            (2.0 * r.x * r.y) + (2.0 * r.w * r.z),
            (2.0 * r.x * r.z) - (2.0 * r.w * r.y),
        0.0,

            (2.0 * r.x * r.y) - (2.0 * r.w * r.z),
        1.0 - (2.0 * r.x * r.x) - (2.0 * r.z * r.z),
            (2.0 * r.y * r.z) + (2.0 * r.w * r.x),
        0.0,

            (2.0 * r.x * r.z) + (2.0 * r.w * r.y),
            (2.0 * r.y * r.z) - (2.0 * r.w * r.x),
        1.0 - (2.0 * r.x * r.x) - (2.0 * r.y * r.y),
        0.0,

        2.0 * (-t.w * r.x + t.x * r.w - t.y * r.z + t.z * r.y),
        2.0 * (-t.w * r.y + t.x * r.z + t.y * r.w - t.z * r.x),
        2.0 * (-t.w * r.z - t.x * r.y + t.y * r.x + t.z * r.w),
        1);
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

    let normal_matrix = mat3x3<f32>(
        model_mat.model_matrix_0.xyz,
        model_mat.model_matrix_1.xyz,
        model_mat.model_matrix_2.xyz,
    );
    
    let skinMat = getskinMat(model);

    let world_position: vec4<f32> = model_matrix * skinMat  * vec4<f32>(model.position, 1.0);
    out.clip_position = camera.view_proj * world_position;
    out.tex_coords = model.tex_coords;
    out.world_normal = normalize(normal_matrix * model.normal);
    out.world_position = world_position.xyz;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

struct LightUniform {
    pos: vec3<f32>,
    _padding: f32,
    color: vec3<f32>,
    _padding2: f32,
};

@group(2) @binding(0)
var<uniform> light: LightUniform;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let lightDir = normalize(light.pos - in.world_position);
    let lightDirectionalStrength: f32 = max(dot(in.world_normal, lightDir), 0.0);
    let lightStrength = ( 0.1 + 0.9 * lightDirectionalStrength);

    return textureSample(t_diffuse, s_diffuse, in.tex_coords) * vec4<f32>(lightStrength * light.color, 1.0);
}




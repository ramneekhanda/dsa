
struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) coord: vec2<f32>,
};

struct Uniforms {
    mouse: vec2<f32>,
    time: f32,
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let r: f32 = dot(in.coord, in.coord + 0.2);

    if (r > .1) {
        discard;
    }

    let normalized = (in.coord + vec2<f32>(3., 0.5)) / 2.;
    return vec4<f32>(normalized.rg, 0.5, 1.0);
}

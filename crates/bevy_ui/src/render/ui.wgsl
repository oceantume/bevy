struct View {
    view_proj: mat4x4<f32>;
    world_position: vec3<f32>;
};
[[group(0), binding(0)]]
var<uniform> view: View;

struct VertexOutput {
    [[location(0)]] uv: vec2<f32>;
    [[location(1)]] color: vec4<f32>;
    [[location(2)]] border_radius: f32;
    [[location(3)]] center: vec2<f32>;
    [[location(4)]] size: vec2<f32>;
    [[builtin(position)]] position: vec4<f32>;
};

[[stage(vertex)]]
fn vertex(
    [[location(0)]] vertex_position: vec3<f32>,
    [[location(1)]] vertex_uv: vec2<f32>,
    [[location(2)]] vertex_color: u32,
    [[location(3)]] center: vec2<f32>,
    [[location(4)]] border_radius: f32
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex_uv;
    out.position = view.view_proj * vec4<f32>(vertex_position, 1.0);
    out.color = vec4<f32>((vec4<u32>(vertex_color) >> vec4<u32>(0u, 8u, 16u, 24u)) & vec4<u32>(255u)) / 255.0;
    out.border_radius = border_radius;
    out.center = center;
    out.size = abs(vertex_position.xy - center) * 2.0;
    return out;
}

[[group(1), binding(0)]]
var sprite_texture: texture_2d<f32>;
[[group(1), binding(1)]]
var sprite_sampler: sampler;

// Calculate the distance from the fragment to the border of the rounded rectangle,
// return negative value when the fragment is inside the rounded rectangle.
fn distance_round_border(point: vec2<f32>, size: vec2<f32>, radius: f32) -> f32 {
    let dr = abs(point) - (size - radius);
    let d = length(max(dr, vec2<f32>(0.0))) - radius;
    let t = min(dr, vec2<f32>(0.0));
    let d_extra = max(t.x, t.y);

    return d + d_extra;
}

[[stage(fragment)]]
fn fragment(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    var color = textureSample(sprite_texture, sprite_sampler, in.uv);
    color = in.color * color;

    if (in.border_radius > 0.0) {
        var d = distance_round_border(in.position.xy - in.center, in.size * 0.5, in.border_radius);
        color.a = color.a * -d;
    }

    return color;
}

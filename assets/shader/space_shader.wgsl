struct CircleMaterial {
    circle_color: vec4<f32>;
    outline_color: vec4<f32>;
    outer_color: vec4<f32>;
};

[[group(1), binding(0)]]
var<uniform> material: CircleMaterial;

struct FragmentInput {
    [[builtin(front_facing)]] is_front: bool;
    [[builtin(position)]] frag_coord: vec4<f32>;
    [[location(0)]] world_position: vec4<f32>;
    [[location(1)]] world_normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
#ifdef VERTEX_TANGENTS
    [[location(3)]] world_tangent: vec4<f32>;
#endif
};

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    let uvx: f32 = (in.uv[0] * 2.0) - 1.0;
    let uvy: f32 = (in.uv[1] * 2.0) - 1.0;
    let squared_distance: f32 = (uvx * uvx) + (uvy * uvy);

    if(squared_distance < 0.8) {
        return material.circle_color;
    } else if(squared_distance < 1.0) {
        return material.outline_color;
    } else {
        if(material.outer_color.w > 0.0) {
            return material.outer_color;
        } else {
            discard;
        }
    }
}

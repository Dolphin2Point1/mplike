#import bevy_pbr::pbr

struct CircleMaterial {
    inner_color: vec4<f32>;
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
}

[[stage(fragment)]]
fn main(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    var squared_distance = in.uv[0] ^ 2 + in.uv[1] ^ 2;

    var output_color = mix(material.inner_color, material.outer_color, squared_distance > 1);

    // accumulate color
    var light_accum: vec3<f32> = vec3<f32>(0.0);

    let view_z = dot(vec4<f32>(
                view.inverse_view[0].z,
                view.inverse_view[1].z,
                view.inverse_view[2].z,
                view.inverse_view[3].z
            ), in.world_position);

    return output_color;
}

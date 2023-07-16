#import bevy_pbr::mesh_vertex_output MeshVertexOutput
#import bevy_pbr::mesh_view_bindings    view
#import bevy_pbr::pbr_types             STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT
#import bevy_core_pipeline::tonemapping tone_mapping
#import bevy_pbr::pbr_functions as fns

struct CustomMaterial {
    threshold: f32,
    shadow_multiplier: f32,
};


@group(1) @binding(0)
var<uniform> material: CustomMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
    @builtin(front_facing) is_front: bool,
    mesh: MeshVertexOutput,
) -> @location(0) vec4<f32> {
    let layer = i32(mesh.world_position.x) & 0x3;

    var pbr_input: fns::PbrInput = fns::pbr_input_new();
    pbr_input.material.base_color = vec4<f32>(1., 1., 1., 1.);

    pbr_input.frag_coord = mesh.position;
    pbr_input.world_position = mesh.world_position;
    pbr_input.world_normal = fns::prepare_world_normal(
        mesh.world_normal,
        (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
        is_front,
    );

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = fns::apply_normal_mapping(
        pbr_input.material.flags,
        mesh.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        mesh.world_tangent,
#endif
#endif
        mesh.uv,
        view.mip_bias,
    );
    pbr_input.V = fns::calculate_view(mesh.world_position, pbr_input.is_orthographic);


    let result = fns::pbr(pbr_input).x;

    let color =  textureSample(base_color_texture, base_color_sampler, mesh.uv);

    if (result > material.threshold) {
        return color;
    }
    return color * material.shadow_multiplier;
}

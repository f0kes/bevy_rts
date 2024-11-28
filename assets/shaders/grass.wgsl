#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}
#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif

struct GrassMaterialConfig {
    plane_size_x: f32,
    plane_size_z: f32,
    tile_size: f32,
    normal_tiles_x: u32,
} 

@group(2) @binding(100)
var<uniform> grass_material_config: GrassMaterialConfig;

fn hash2(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(0.1031, 0.1030, 0.0973));
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise2d(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    
    // Four corners
    let a = hash2(i);
    let b = hash2(i + vec2<f32>(1.0, 0.0));
    let c = hash2(i + vec2<f32>(0.0, 1.0));
    let d = hash2(i + vec2<f32>(1.0, 1.0));
    
    // Smooth interpolation
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(a, b, u.x),
        mix(c, d, u.x),
        u.y
    );
}

// Fractal Brownian Motion (fbm) for more natural looking noise
fn fbm(p: vec2<f32>) -> f32 {
    let num_octaves = 3;
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 3.0;

    for (var i = 0; i < num_octaves; i++) {
        value += amplitude * noise2d(p * frequency);
        amplitude *= 0.5;
        frequency *= 2.0;
    }

    return value;
}

@fragment
fn fragment(
    mesh_vertex_output: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Create a mutable copy of the input
    var in = mesh_vertex_output;
    
    // Create modified UV coordinates for normal map scaling/repeating
    var scaled_uv = in.uv;
    scaled_uv.x *= grass_material_config.plane_size_x / grass_material_config.tile_size ;
    scaled_uv.y *= grass_material_config.plane_size_z / grass_material_config.tile_size ;
    let tile_coord = floor(scaled_uv);
    let checker = (tile_coord.x + tile_coord.y) % 2.0 == 0.0;
    let checker_value = select(0.8, 1.0, checker); // 0.8 for dark squares, 1.0 for light squares

    scaled_uv = fract(scaled_uv);

    let random_tint = hash2(tile_coord);

    // Store original UV
    let original_uv = in.uv;
    // Replace UV with scaled version
    in.uv = scaled_uv / f32(grass_material_config.normal_tiles_x); //multiplying is good

    let noise_scale = 100.0;
    let grass_noise = fbm(scaled_uv * noise_scale);
    let noise_intensity =0.6;
    let noise_factor = 1.0 - noise_intensity + noise_intensity * grass_noise;
    // generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    let checker_tint = mix(0.0, 1.0, checker_value);
    let combined_tint = mix(checker_tint, random_tint, 0.17);
    pbr_input.material.base_color *= combined_tint * noise_factor;

    //pbr_input.material.base_color = mix(pbr_input.material.base_color, pbr_input.material.base_color * random_tint, 0.2);
    // Restore original UV for other texture samples if needed
    in.uv = original_uv;
    
    // alpha discard
    pbr_input.material.base_color = alpha_discard(pbr_input.material, pbr_input.material.base_color);

#ifdef PREPASS_PIPELINE
    let out = deferred_output(in, pbr_input);
#else
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    out.color = out.color * 2.0;
#endif
    return out;
}
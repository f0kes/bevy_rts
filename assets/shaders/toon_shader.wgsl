struct ToonShaderMaterial {
    color: vec4<f32>,
    cliff_color: vec4<f32>,
    sun_dir: vec3<f32>,
    sun_color: vec4<f32>,
    camera_pos: vec3<f32>,
    ambient_color: vec4<f32>,
    bands: f32,
};

@group(2) @binding(0)
var<uniform> material: ToonShaderMaterial;
@group(2) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(2) @binding(2)
var base_color_sampler: sampler;

#import bevy_pbr::forward_io::VertexOutput

fn grad(p: vec2<i32>, d: vec2<f32>) -> f32 {
    let hash = (p.x * 383) ^ (p.y * 1493);
    let h = hash & 7;
    
    var gradient = vec2<f32>(0.0);
    
    switch h {
        case 0: { gradient = vec2<f32>(-1.0, -1.0); }
        case 1: { gradient = vec2<f32>(1.0, -1.0); }
        case 2: { gradient = vec2<f32>(-1.0, 1.0); }
        case 3: { gradient = vec2<f32>(1.0, 1.0); }
        case 4: { gradient = vec2<f32>(-1.0, 0.0); }
        case 5: { gradient = vec2<f32>(1.0, 0.0); }
        case 6: { gradient = vec2<f32>(0.0, -1.0); }
        default: { gradient = vec2<f32>(0.0, 1.0); }
    }
    
    return dot(gradient, d);
}

fn noise2d(pos: vec2<f32>) -> f32 {
    let i = vec2<i32>(floor(pos));
    let f = fract(pos);
    
    // Cubic Hermine Curve
    let u = f * f * (3.0 - 2.0 * f);
    
    let n00 = grad(i + vec2<i32>(0, 0), f - vec2<f32>(0.0, 0.0));
    let n10 = grad(i + vec2<i32>(1, 0), f - vec2<f32>(1.0, 0.0));
    let n01 = grad(i + vec2<i32>(0, 1), f - vec2<f32>(0.0, 1.0));
    let n11 = grad(i + vec2<i32>(1, 1), f - vec2<f32>(1.0, 1.0));
    
    let nx0 = mix(n00, n10, u.x);
    let nx1 = mix(n01, n11, u.x);
    let nxy = mix(nx0, nx1, u.y);
    
    return nxy * 0.5 + 0.5;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // if model doesn't have uvs, this lets it still render.
    #ifdef VERTEX_UVS_A
    let uv = in.uv;
    #else
    let uv = vec2(1.0, 1.0);
    #endif

    let base_color = material.color * textureSample(base_color_texture, base_color_sampler, uv);
    let cliff_color = material.cliff_color * textureSample(base_color_texture, base_color_sampler, uv);
    let normal = normalize(in.world_normal);
    let n_dot_l = dot(material.sun_dir, normal);
    var light_intensity = 0.0;

    if n_dot_l > 0.0 {
        let bands = material.bands;
        var x = n_dot_l * bands;

        x = round(x);

        light_intensity = x / bands;
    } else {
        light_intensity = 0.0;
    }

    let light = light_intensity * material.sun_color;

    let view_dir: vec3<f32> = normalize(material.camera_pos - in.world_position.xyz);

    let half_vector = normalize(material.sun_dir + view_dir);
    let n_dot_h = dot(normal, half_vector);
    let glossiness = 32.0;
    let specular_intensity = pow(n_dot_h, glossiness * glossiness);

    let specular_intensity_smooth = smoothstep(0.005, 0.01, specular_intensity); //0.005, 0.01, specular_intensity);
    let specular = specular_intensity_smooth * vec4<f32>(0.9, 0.9, 0.9, 1.0); //0.9, 0.9, 0.9, 1.0);

    //let slope = dot(normal, vec3<f32>(0.0, 1.0, 0.0));
    let bands = 4.0; // Adjust number of bands as needed

    //let cliff_angle = smoothstep(cos(radians(18.0)), cos(radians(0.0)), slope);
   // let cliff_blend = round(cliff_angle * bands) / bands;
    let perlin_blend = noise2d(in.world_position.xz * 0.04) ;
    let end_color = mix(cliff_color, base_color, perlin_blend);
    return end_color * (light + material.ambient_color + specular);
}
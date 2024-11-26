#import bevy_pbr::mesh_functions::{get_world_from_local, mesh_position_local_to_clip}
#import bevy_pbr::mesh_view_bindings::globals

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) i_pos_scale: vec4<f32>,
    @location(4) i_color: vec4<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) world_normal: vec3<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    
    // Wind animation - varies based on position and time
    let wind_strength = 0.1;
    let wind_speed = 1.0;
    let time = globals.time;
    
    // Wind is stronger at the top of the grass blade
    let wind = sin(time * wind_speed + vertex.i_pos_scale.x + vertex.i_pos_scale.z) 
             * wind_strength 
             * vertex.position.y; // More movement at the top
    
    // Apply wind offset to x position
    var position = vertex.position;
    position.x += wind;
    
    // Scale and position the grass blade
    position = position * vertex.i_pos_scale.w + vertex.i_pos_scale.xyz;

    out.clip_position = mesh_position_local_to_clip(
        get_world_from_local(0u),
        vec4<f32>(position, 1.0)
    );
    
    // Basic lighting calculation
    let light_direction = normalize(vec3<f32>(0.5, 0.8, 0.3));
    let normal = normalize(vertex.normal);
    let diffuse = max(dot(normal, light_direction), 0.0);
    let ambient = 0.3;
    let lighting = ambient + diffuse * 0.7;
    
    // Apply lighting to color
    out.color = vertex.i_color * lighting;
    out.world_normal = normal;
    
    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
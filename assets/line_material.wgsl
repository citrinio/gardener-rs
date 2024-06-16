#import bevy_pbr::forward_io::VertexOutput

@group(2) @binding(0) var<uniform> grid_color: vec4f;
@group(2) @binding(1) var<uniform> tick_color: vec4f;
@group(2) @binding(2) var<uniform> grid_size: i32;
@group(2) @binding(3) var<uniform> line_width: i32;
@group(2) @binding(4) var<uniform> offset: vec2f;

@fragment
fn fragment(@builtin(position) position: vec4f) -> @location(0) vec4f {

    if ((i32(position.x + offset.x) % grid_size) < (grid_size - line_width)) || ((i32(position.y + offset.y) % grid_size) < (grid_size - line_width)) {
      return tick_color;
    } else {
      return grid_color;
    }
}


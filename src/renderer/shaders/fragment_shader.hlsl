#version 300 es
// fragment shaders don't have a default precision so we need
// to pick one. mediump is a good default
precision highp float;

#define PI 3.1415926535897932384626433832795
#define TAU 6.283185307179586476925286766559

//layout(origin_upper_left) in vec4 gl_FragCoord;

uniform vec2 u_res;
uniform mat3 transform; // general transform matrix
uniform uint u_brush_type;
// solid color brush
uniform vec4 u_color;
// gradients
uniform vec2 brush_start;
uniform vec2 brush_end;
uniform vec4 colors[64];
uniform float gradient_stops[64];
uniform int gradient_stops_count;
// textures
uniform mat3 texture_transform;
uniform sampler2D u_texture;




out vec4 out_color;

vec2 get_coord();
vec2 get_coord() {
    vec3 coord = vec3(gl_FragCoord.x, u_res.y - gl_FragCoord.y, 0);
    coord = coord - vec3(transform[0].z, transform[1].z, 0);
    coord = coord * inverse(transform);
    // vec2(gl_FragCoord.x,u_res.y - gl_FragCoord.y) * transform;
    // coord += transform[0];
    return coord.xy;
}


vec4 compute_gradient_color(in float t);
vec4 compute_gradient_color(in float t) {
    
    vec4 result_color = mix(colors[0], colors[1], smoothstep( gradient_stops[0], gradient_stops[1], t ));

    for (int i=2; i<gradient_stops_count; i++ ) {
        result_color = mix(result_color, colors[i], smoothstep( gradient_stops[i - 1], gradient_stops[i], t ));
    }
    return result_color;
}

vec2 transform_point(in vec2 point_vec, in mat3 transform_matrix);
vec2 transform_point(in vec2 point_vec, in mat3 transform_matrix) {
    return point_vec;//(vec3(point_vec, 0) * transform_matrix).xy;
}

void main() {
    // gl_FragColor is a special variable a fragment shader
    // is responsible for setting
    if (u_brush_type == uint(1)) { // solid color
        out_color = u_color;
    } else if (u_brush_type == uint(2)) { // linear gradient

        vec2 gradient_start = transform_point(brush_start, transform);
        vec2 gradient_end = transform_point(brush_end, transform);

        float angle = atan(gradient_start.y - gradient_end.y, gradient_end.x - gradient_start.y);
        //vec2 dir = vec2(cos(angle), sin(angle));
        float start = gradient_start.x * cos(angle) - gradient_start.y * sin(angle);
        float dis = (gradient_end.x * cos(angle) - gradient_end.y * sin(angle)) - start;
        
        vec2 coord = get_coord();

        float pos = coord.x * cos(angle) - coord.y * sin(angle);
        float t = (pos - start) / dis;

        out_color = compute_gradient_color(t);
    } else if (u_brush_type == uint(3)) { // radial_gradient 
        vec2 coord = get_coord();

        coord = (coord - brush_start) / (brush_end - brush_start);

        float t = sqrt((coord.x * coord.x) + (coord.y * coord.y)) / 1.0;

        out_color = compute_gradient_color(t);
    } else if (u_brush_type == uint(4)) { // conic_gradient 
        vec2 coord = get_coord();

        vec2 gradient_start = transform_point(brush_start, transform);

        float t = mod(atan(coord.y - gradient_start.y, coord.x - gradient_start.x), TAU) / TAU;

        out_color = compute_gradient_color(t);
    } else if (u_brush_type == uint(5)) { // texture_brush
        vec2 coord = get_coord();
        vec2 texture_end = transform_point(brush_end, transform);
        vec2 tex_pos = vec2(((coord.x) / texture_end.x),  1.0-((coord.y) / texture_end.y));
        // vec2 coord = gl_FragCoord.xy;
        out_color = texture(u_texture, tex_pos);

    } else {
        out_color = vec4(1.0, 0.2, 0.2, 1.0);
    }
}
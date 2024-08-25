#version 300 es
// fragment shaders don't have a default precision so we need
// to pick one. mediump is a good default
precision highp float;


//layout(origin_upper_left) in vec4 gl_FragCoord;

uniform vec2 u_res;
uniform uint u_brush_type;
uniform vec4 u_color;
uniform vec2 gradient_start;
uniform vec2 gradient_end;
uniform vec4 colors[64];
uniform float gradient_stops[64];
uniform int gradient_stops_count;



out vec4 out_color;


vec4 compute_gradient_color(in float t);
vec4 compute_gradient_color(in float t) {
    
    vec4 result_color = mix(colors[0], colors[1], smoothstep( gradient_stops[0], gradient_stops[1], t ));

    for (int i=2; i<gradient_stops_count; i++ ) {
        result_color = mix(result_color, colors[i], smoothstep( gradient_stops[i - 1], gradient_stops[i], t ));
    }
    return result_color;
}

void main() {
    // gl_FragColor is a special variable a fragment shader
    // is responsible for setting
    if (u_brush_type == uint(1)) { // solid color
        out_color = u_color;
    } else if (u_brush_type == uint(2)) { // linear gradient
        float angle = atan(gradient_start.y - gradient_end.y, gradient_end.x - gradient_start.y);
        //vec2 dir = vec2(cos(angle), sin(angle));
        float start = gradient_start.x * cos(angle) - gradient_start.y * sin(angle);
        float dis = (gradient_end.x * cos(angle) - gradient_end.y * sin(angle)) - start;
        
        vec2 coord = vec2(gl_FragCoord.x, u_res.y - gl_FragCoord.y);

        float pos = coord.x * cos(angle) - coord.y * sin(angle);
        float t = (pos - start) / dis;

        out_color = compute_gradient_color(t);
    } else if (u_brush_type == uint(3)) {// radial_gradient 
        vec2 coord = vec2(gl_FragCoord.x,u_res.y - gl_FragCoord.y);

        coord = (coord - gradient_start) / (gradient_end - gradient_start);

        float t = sqrt((coord.x * coord.x) + (coord.y * coord.y)) / 1.0;

        out_color = compute_gradient_color(t);
    } else {
        out_color = vec4(1.0, 0.2, 0.2, 1.0);
    }
}
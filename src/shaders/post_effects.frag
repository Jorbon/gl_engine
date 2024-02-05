#version 150

in vec2 screen_position;
out vec4 color;

uniform vec2 resolution;
uniform int step_num;
uniform float normals_outline_weight;
uniform float depth_outline_weight;
uniform vec4 outline_color;
uniform float z_far;
uniform float z_near;
uniform sampler2D bayer16_texture;
uniform sampler2D main_buffer;
uniform sampler2D normals_buffer;
uniform sampler2D depth_buffer;


float get_actual_depth(float buffer_depth) {
	return z_far * z_near / (z_far - buffer_depth * (z_far - z_near));
}

float get_depth_precision_density(float buffer_depth) {
	float actual_depth = get_actual_depth(buffer_depth);
	return z_far * z_near / ((z_far - z_near) * actual_depth * actual_depth);
}


const vec3 kernel[8] = vec3[](
	vec3(-1.0,  0.0, 2.0),
	vec3( 1.0,  0.0, 2.0),
	vec3( 0.0, -1.0, 2.0),
	vec3( 0.0,  1.0, 2.0),
	vec3(-1.0, -1.0, 1.0),
	vec3( 1.0, -1.0, 1.0),
	vec3(-1.0,  1.0, 1.0),
	vec3( 1.0,  1.0, 1.0)
);

void main() {
	vec4 points[8];
	for (int i = 0; i < 8; i++) {
		vec2 position = screen_position + kernel[i].xy / resolution;
		vec3 normal_value = texture(normals_buffer, position).rgb;
		float depth_value = texture(depth_buffer, position).r;
		points[i] = vec4(normal_value, depth_value) * kernel[i].z;
	}
	
	vec3 ngx = points[0].rgb + points[4].rgb + points[6].rgb - points[1].rgb - points[5].rgb - points[7].rgb;
	vec3 ngy = points[2].rgb + points[4].rgb + points[5].rgb - points[3].rgb - points[6].rgb - points[7].rgb;
	float normal_gradient = ngx.r*ngx.r + ngx.g*ngx.g + ngx.b*ngx.b + ngy.r*ngy.r + ngy.g*ngy.g + ngy.b*ngy.b;
	
	float dgx = points[0].a + points[4].a + points[6].a - points[1].a - points[5].a - points[7].a;
	float dgy = points[2].a + points[4].a + points[5].a - points[3].a - points[6].a - points[7].a;
	
	float buffer_depth = texture(depth_buffer, screen_position).r;
	float buffer_depth_precision_density = get_depth_precision_density(buffer_depth);
	float depth_gradient = (dgx*dgx + dgy*dgy) / (buffer_depth_precision_density * buffer_depth_precision_density);
	
	float total_gradient = clamp(normal_gradient * normals_outline_weight + depth_gradient * depth_outline_weight, 0.0, 1.0);
	
	
	vec3 main_color = texture(main_buffer, screen_position).rgb;
	vec3 outlined_color = mix(main_color, outline_color.rgb, total_gradient * outline_color.a);
	vec3 dither_offset = (texture(bayer16_texture, gl_FragCoord.xy / 16.0).rgb - 0.5);
	vec3 posterized_color = round(outlined_color * step_num + dither_offset) / step_num;
	color = vec4(posterized_color, 1.0);
}




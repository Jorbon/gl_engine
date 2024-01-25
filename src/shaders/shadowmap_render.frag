#version 150

in vec2 screen_position;
out vec4 color;

//uniform vec2 resolution;
//uniform int step_num;
//uniform sampler2D main_buffer;
//uniform sampler2D normals_buffer;
//uniform sampler2D depth_buffer;
uniform sampler2D shadowmap_texture;

vec3 rainbow_gradient(float t) {
	t = mod(t, 1.0) * 6.0;
	if (t <= 1.0) return vec3(1.0, t, 0.0);
	if (t <= 2.0) return vec3(2.0 - t, 1.0, 0.0);
	if (t <= 3.0) return vec3(0.0, 1.0, t - 2.0);
	if (t <= 4.0) return vec3(0.0, 4.0 - t, 1.0);
	if (t <= 5.0) return vec3(t - 4.0, 0.0, 1.0);
	return vec3(1.0, 0.0, 6.0 - t);
}

void main() {
	float t = texture(shadowmap_texture, screen_position).r;
	color = vec4(rainbow_gradient(t), 1.0);
}
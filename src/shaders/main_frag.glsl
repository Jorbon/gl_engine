#version 150

in vec3 pos;
in vec3 cam_pos;
in vec3 norm;
in vec3 cam_norm;
in vec3 sm_pos;
out vec4 main_color;
out vec4 aux_color;

uniform vec3 cam;
uniform vec3 light;
uniform sampler2D sm;
uniform float sm_u;
uniform float sm_tol;

const vec3 diff_color = vec3(1.0, 0.0, 0.0);
const vec3 amb_color = vec3(0.0, 0.0, 0.0);
const vec3 spec_color = vec3(1.0, 1.0, 1.0);


float sample_shade_plain(vec3 n) {
	float bias = max(10.0 * (1.0 - dot(n, light)), 1.0) * sm_tol;
	float shade = 0.0;
	if (texture(sm, sm_pos.xy).x + bias < sm_pos.z) shade += 1.0;
	return shade;
}

float sample_shade(vec3 n) {
	float bias = max(10.0 * (1.0 - dot(n, light)), 1.0) * sm_tol;
	float shade = 0.0;
	for (int x = -1; x <= 1; x++) for (int y = -1; y <= 1; y++) {
		float sm_depth = texture(sm, sm_pos.xy + vec2(x, y)*sm_u).x;
		if (sm_depth + bias < sm_pos.z) {
			shade += 0.0625;
			if (x == 0) shade += 0.0625;
			if (y == 0) shade += 0.0625;
		}
	}
	return shade;
}

float sample_shade2(vec3 n) {
	vec2 tpos = sm_pos.xy - 0.5 * sm_u;
	vec2 w = (tpos / sm_u) % 1.0;
	
	float bias = max(10.0 * (1.0 - dot(n, light)), 1.0) * sm_tol;
	float shade = 0.0;
	if (texture(sm, tpos).x + bias < sm_pos.z) shade += (1.0 - w.x)*(1.0 - w.y);
	if (texture(sm, vec2(tpos.x + sm_u, tpos.y)).x + bias < sm_pos.z) shade += w.x*(1.0 - w.y);
	if (texture(sm, vec2(tpos.x, tpos.y + sm_u)).x + bias < sm_pos.z) shade += (1.0 - w.x)*w.y;
	if (texture(sm, tpos + sm_u).x + bias < sm_pos.z) shade += w.x*w.y;
	return shade;
}

void main() {
	vec3 n = normalize(norm);
	float diff_brightness = max(dot(n, light), 0.0);
	float spec_brightness = pow(max(dot(n, normalize(light + normalize(cam - pos))), 0.0), 32.0) * diff_brightness;
	
	float shade = sample_shade(n);
	spec_brightness *= 1.0 - shade*1.5;
	diff_brightness *= 1.0 - shade;
	
	main_color = vec4(mix(amb_color, n, diff_brightness) + spec_color * spec_brightness, 1.0);
	aux_color = vec4((n + 1.0) * 0.5, 1.0);
}
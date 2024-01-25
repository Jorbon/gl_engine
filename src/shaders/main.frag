#version 150

in vec3 world_position;
in vec3 shadowmap_position;
out vec4 color;
out vec4 normal_color;

uniform vec3 camera_location;
uniform vec3 light_direction;
uniform sampler2DShadow shadowmap_texture;
uniform vec2 shadowmap_resolution;
uniform float shadowmap_tolerance;
uniform float dummy;


const float ambient_level = 0.2;
const vec3 specular_color = vec3(1.0, 1.0, 1.0);


const vec2 sample_pos[16] = vec2[](
	vec2( 0.97484398,  0.75648379),
	vec2(-0.81409955,  0.91437590),
	vec2( 0.94558609, -0.76890725),
	vec2(-0.81544232, -0.87912464),
	vec2( 0.44323325, -0.97511554),
	vec2(-0.94201624, -0.39906216),
	vec2(-0.09418410, -0.92938870),
	vec2( 0.34495938,  0.29387760),
	vec2(-0.91588581,  0.45771432),
	vec2(-0.38277543,  0.27676845),
	vec2( 0.53742981, -0.47373420),
	vec2(-0.26496911, -0.41893023),
	vec2( 0.79197514,  0.19090188),
	vec2(-0.24188840,  0.99706507),
	vec2( 0.19984126,  0.78641367),
	vec2( 0.14383161, -0.14100790)
);

float sample_shade_kernel_hardware(vec3 normal) {
	float bias = max(10.0 - 10.0 * dot(normal, light_direction), 1.0) * shadowmap_tolerance;
	float shade = 0.0;
	for (int i = 0; i < 4; i++) {
		shade += texture(shadowmap_texture, vec3(shadowmap_position.xy + sample_pos[i] / shadowmap_resolution, shadowmap_position.z - bias));
	}
	if (shade >= 3.9) return 1.0;
	for (int i = 4; i < sample_pos.length(); i++) {
		shade += texture(shadowmap_texture, vec3(shadowmap_position.xy + sample_pos[i] / shadowmap_resolution, shadowmap_position.z - bias));
	}
	return shade / sample_pos.length();
}


void main() {
	vec3 normal = normalize(cross(dFdx(world_position), dFdy(world_position)));
	
	float diffuse_brightness = max(dot(normal, light_direction), 0.0);
	float specular_brightness = pow(max(dot(normal, normalize(light_direction + normalize(camera_location - world_position))), 0.0), 16.0) * diffuse_brightness;
	
	float shade = sample_shade_kernel_hardware(normal);
	specular_brightness *= max(1.0 - shade*1.5, 0.0);
	diffuse_brightness *= 1.0 - shade;
	
	color = vec4(mix(ambient_level, 1.0, diffuse_brightness) * abs(normal) + specular_color * specular_brightness, 1.0);
	normal_color = vec4(normal, 1.0);
}








float sample_shade_hardware_smooth(vec3 normal) {
	float bias = max(10.0 * (1.0 - dot(normal, light_direction)), 1.0) * shadowmap_tolerance;
	return texture(shadowmap_texture, vec3(shadowmap_position.xy, shadowmap_position.z - bias));
}


float random(vec4 seed4){
	float dot_product = dot(seed4, vec4(12.9898, 78.233, 45.164, 94.673));
	return fract(sin(dot_product) * 43758.5453);
}

float sample_shade_noise(vec3 normal) {
	float bias = max(10.0 * (1.0 - dot(normal, light_direction)), 1.0) * shadowmap_tolerance;
	int index = int(16.0 * random(gl_FragCoord.xyxy)) % 16;
	return texture(shadowmap_texture, vec3(shadowmap_position.xy + sample_pos[index] * 3.0 / shadowmap_resolution, shadowmap_position.z - bias));
}




/*
float sample_shade_plain(vec3 n) {
	float bias = max(10.0 * (1.0 - dot(n, light)), 1.0) * sm_tol;
	float shade = 0.0;
	if (texture(sm, sm_pos.xy).x + bias < sm_pos.z) shade += 1.0;
	return shade;
}

float sample_shade_kernel(vec3 n) {
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

float sample_shade_smooth1(vec3 n) {
	vec2 tpos = sm_pos.xy - sm_u * 0.5;
	vec2 w = (tpos / sm_u) % 1.0;
	
	float bias = max(10.0 * (1.0 - dot(n, light)), 1.0) * sm_tol;
	float shade = 0.0;
	if (texture(sm, tpos).x + bias < sm_pos.z) shade += (1.0 - w.x)*(1.0 - w.y);
	if (texture(sm, vec2(tpos.x + sm_u, tpos.y)).x + bias < sm_pos.z) shade += w.x*(1.0 - w.y);
	if (texture(sm, vec2(tpos.x, tpos.y + sm_u)).x + bias < sm_pos.z) shade += (1.0 - w.x)*w.y;
	if (texture(sm, tpos + sm_u).x + bias < sm_pos.z) shade += w.x*w.y;
	return shade;
}

float sample_shade_smooth2(vec3 n) {
	vec2 tpos = sm_pos.xy - 0.5 * sm_u;
	vec2 w = (tpos / sm_u) % 1.0;
	
	float bias = max(10.0 * (1.0 - dot(n, light)), 1.0) * sm_tol;
	float shade = 0.0;
	
	vec2 w1 = 0.25 * (1.0 - w);
	vec2 w2 = 0.25 + w1;
	vec2 w4 = 0.25 * w;
	vec2 w3 = 0.25 + w4;
	
	
	if (texture(sm, tpos - sm_u).x + bias < sm_pos.z) shade += w1.x*w1.y;
	if (texture(sm, vec2(tpos.x, tpos.y - sm_u)).x + bias < sm_pos.z) shade += w2.x*w1.y;
	if (texture(sm, vec2(tpos.x + sm_u, tpos.y - sm_u)).x + bias < sm_pos.z) shade += w3.x*w1.y;
	if (texture(sm, vec2(tpos.x + 2.0*sm_u, tpos.y - sm_u)).x + bias < sm_pos.z) shade += w4.x*w1.y;
	
	if (texture(sm, vec2(tpos.x - sm_u, tpos.y)).x + bias < sm_pos.z) shade += w1.x*w2.y;
	if (texture(sm, tpos).x + bias < sm_pos.z) shade += w2.x*w2.y;
	if (texture(sm, vec2(tpos.x + sm_u, tpos.y)).x + bias < sm_pos.z) shade += w3.x*w2.y;
	if (texture(sm, vec2(tpos.x + 2.0*sm_u, tpos.y)).x + bias < sm_pos.z) shade += w4.x*w2.y;
	
	if (texture(sm, vec2(tpos.x - sm_u, tpos.y + sm_u)).x + bias < sm_pos.z) shade += w1.x*w3.y;
	if (texture(sm, vec2(tpos.x, tpos.y + sm_u)).x + bias < sm_pos.z) shade += w2.x*w3.y;
	if (texture(sm, tpos + sm_u).x + bias < sm_pos.z) shade += w3.x*w3.y;
	if (texture(sm, vec2(tpos.x + 2.0*sm_u, tpos.y + sm_u)).x + bias < sm_pos.z) shade += w4.x*w3.y;
	
	if (texture(sm, vec2(tpos.x - sm_u, tpos.y + 2.0*sm_u)).x + bias < sm_pos.z) shade += w1.x*w4.y;
	if (texture(sm, vec2(tpos.x, tpos.y + 2.0*sm_u)).x + bias < sm_pos.z) shade += w2.x*w4.y;
	if (texture(sm, vec2(tpos.x + sm_u, tpos.y + 2.0*sm_u)).x + bias < sm_pos.z) shade += w3.x*w4.y;
	if (texture(sm, tpos + 2.0*sm_u).x + bias < sm_pos.z) shade += w4.x*w4.y;
	
	return shade;
}
*/



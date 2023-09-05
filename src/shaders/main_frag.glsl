#version 150

in vec3 pos;
in vec3 norm;
in vec3 sm_pos;
in vec2 uvf;
//in vec3 cam_pos;
//in vec3 cam_norm;
out vec4 main_color;
out vec4 aux_color;

uniform vec3 cam;
uniform vec3 light;
uniform sampler2D tex;
uniform sampler2D spec_map;

uniform sampler2DShadow sm;
uniform float sm_u;
uniform float sm_tol;
uniform float d;

const vec3 diff_color = vec3(1.0, 0.0, 0.0);
const vec3 amb_color = vec3(0.0, 0.0, 0.0);
const vec3 spec_color = vec3(1.0, 1.0, 1.0);


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

float sample_shade_kernel_hardware(vec3 n) {
	float bias = max(10.0 - 10.0 * dot(n, light), 1.0) * sm_tol;
	float shade = 0.0;
	for (int i = 0; i < 4; i++) {
		shade += texture(sm, vec3(sm_pos.xy + sample_pos[i]*sm_u, sm_pos.z - bias));
	}
	if (shade >= 3.9) return 1.0;
	for (int i = 4; i < sample_pos.length(); i++) {
		shade += texture(sm, vec3(sm_pos.xy + sample_pos[i]*sm_u, sm_pos.z - bias));
	}
	return shade / sample_pos.length();
}


void main() {
	vec3 n = normalize(norm);
	
	float diff_brightness = max(dot(n, light), 0.0);
	float spec_brightness = pow(max(dot(n, normalize(light + normalize(cam - pos))), 0.0), 16.0) * diff_brightness;
	
	float shade = sample_shade_kernel_hardware(n);
	spec_brightness *= (1.0 - shade*1.5) * (1.0 - texture(spec_map, uvf).r) * 15.0;
	diff_brightness *= 1.0 - shade;
	
	main_color = vec4(mix(amb_color, texture(tex, uvf).rgb, diff_brightness) + spec_color * spec_brightness, 1.0);
	aux_color = vec4(n * 0.5 + 0.5, 1.0);
}








float sample_shade_hardware_smooth(vec3 n) {
	float bias = max(10.0 - 10.0 * dot(n, light), 1.0) * sm_tol;
	return texture(sm, vec3(sm_pos.xy, sm_pos.z - bias));
}


float random(vec4 seed4){
	float dot_product = dot(seed4, vec4(12.9898, 78.233, 45.164, 94.673));
	return fract(sin(dot_product) * 43758.5453);
}

float sample_shade_noise(vec3 n) {
	float bias = max(10.0 * (1.0 - dot(n, light)), 1.0) * sm_tol;
	int index = int(16.0 * random(gl_FragCoord.xyxy)) % 16;
	return texture(sm, vec3(sm_pos.xy + sample_pos[index]*sm_u*3.0, sm_pos.z - bias));
}



/*
mat3 cotangent_frame(vec3 normal, vec3 position, vec2 uv) {
    vec3 dpx = dFdx(position);
    vec3 dpy = dFdy(position);
    vec2 duvx = dFdx(uv);
    vec2 duvy = dFdy(uv);

    vec3 dpyperp = cross(dpy, normal);
    vec3 dpxperp = cross(normal, dpx);
    vec3 T = dpyperp * duvx.x + dpxperp * duvy.x;
    vec3 B = dpyperp * duvx.y + dpxperp * duvy.y;

    float invmax = inversesqrt(max(dot(T, T), dot(B, B)));
    return mat3(-T * invmax, -B * invmax, normal);
}
*/

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



#version 150

in vec2 p;
out vec4 color;

uniform vec2 u;
uniform float step_num;
uniform sampler2D main_tex;
uniform sampler2D aux_tex;
uniform sampler2D depth_tex;

const float norm_weight = 1.0;
const float depth_weight = 1.0;

void main() {
	vec2 a = vec2(p.x - u.x, p.y);
	vec2 b = vec2(p.x + u.x, p.y);
	vec2 c = vec2(p.x, p.y - u.y);
	vec2 d = vec2(p.x, p.y + u.y);
	vec2 e = p - u;
	vec2 f = vec2(p.x + u.x, p.y - u.y);
	vec2 g = vec2(p.x - u.x, p.y + u.y);
	vec2 h = p + u;
	
	vec4 an = texture(aux_tex, a) * 2.0;
	vec4 bn = texture(aux_tex, b) * 2.0;
	vec4 cn = texture(aux_tex, c) * 2.0;
	vec4 dn = texture(aux_tex, d) * 2.0;
	vec4 en = texture(aux_tex, e);
	vec4 fn = texture(aux_tex, f);
	vec4 gn = texture(aux_tex, g);
	vec4 hn = texture(aux_tex, h);
	
	//vec4 n = texture(aux_tex, p);
	vec4 ngx = an + en + gn - bn - fn - hn;
	vec4 ngy = cn + en + fn - dn - gn - hn;
	float ng = ngx.x*ngx.x + ngx.y*ngx.y + ngx.z*ngx.z + ngy.x*ngy.x + ngy.y*ngy.y + ngy.z* ngy.z;
	
	
	float ad = texture(depth_tex, a).x * 2.0;
	float bd = texture(depth_tex, b).x * 2.0;
	float cd = texture(depth_tex, c).x * 2.0;
	float dd = texture(depth_tex, d).x * 2.0;
	float ed = texture(depth_tex, e).x;
	float fd = texture(depth_tex, f).x;
	float gd = texture(depth_tex, g).x;
	float hd = texture(depth_tex, h).x;
	
	//float depth = texture(depth_tex, p).x;
	float dgx = ad + ed + gd - bd - fd - hd;
	float dgy = cd + ed + fd - dd - gd - hd;
	float dg = dgx*dgx + dgy*dgy;
	
	float tg = ng * norm_weight + dg * depth_weight;
	
	vec4 mc = texture(main_tex, p);
	
	color = mix(vec4(round(mc.xyz * step_num) / step_num, 1.0), vec4(0.0, 0.0, 0.0, 1.0), tg);
}



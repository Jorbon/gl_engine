use crate::{math_structs::{Mat4, Vec3}, object::Object};

pub fn run(objects: &mut [Object], dt: f32) {
	let mut collision = None;
	let mut dt_remaining = dt;
	
	while dt_remaining > 0.0 {
		
		let new_transforms = (0..objects.len()).map(|i| objects[i].future_transform(dt_remaining)).collect::<Vec<Mat4>>();
		let transformed_vertices = (0..objects.len()).map(|i| objects[i].vertex_buffer.read().unwrap().iter().map(|v| (v.apply_transform(&objects[i].transform), v.apply_transform(&new_transforms[i]))).collect::<Vec<(Vec3, Vec3)>>()).collect::<Vec<Vec<(Vec3, Vec3)>>>();
		
		for i in 0..objects.len() { for j in 0..objects.len() {
			if i == j { continue; }
			for k in 0..transformed_vertices[i].len() {
				let (this_v, next_v) = transformed_vertices[i][k];
				for l in 0..objects[j].indices.len() {
					let (a_index, b_index, c_index) = objects[j].indices[l];
					let (this_a, next_a) = transformed_vertices[j][a_index as usize];
					let (this_b, next_b) = transformed_vertices[j][b_index as usize];
					let (this_c, next_c) = transformed_vertices[j][c_index as usize];
					
					
					let p0 = this_v - this_a;
					let g0 = this_b - this_a;
					let h0 = this_c - this_a;
					let dp = next_v - next_a - p0;
					let dg = next_b - next_a - g0;
					let dh = next_c - next_a - h0;
					
					let cubic_a = dg.cross(dh).dot(dp);
					let cubic_b = dg.cross(dh).dot(p0) + (dg.cross(h0) + g0.cross(dh)).dot(dp);
					let cubic_c = (dg.cross(h0) + g0.cross(dh)).dot(p0) + g0.cross(h0).dot(dp);
					let cubic_d = g0.cross(h0).dot(p0);
					
					let time_to_beat = match collision { Some((t, _, _, _, _)) => t, None => 1.0 };
					
					
					let t = if cubic_a.abs() > 1e-7 {
						let b = cubic_b / cubic_a;
						let c = cubic_c / cubic_a;
						let d = cubic_d / cubic_a;
						
						let q = (3.0*c - b*b) / 9.0;
						let r = b * (9.0*c - 2.0*b*b) / 54.0 - 0.5 * d;
						
						let discriminant = q*q*q + r*r;
						if discriminant >= 0.0 {
							let sqrtd = f32::sqrt(discriminant);
							let t = -b / 3.0 + (r + sqrtd).cbrt() + (r - sqrtd).cbrt();
							match t >= 0.0 && t <= time_to_beat {
								true => Some(t),
								false => None
							}
						} else {
							let dum1 = f32::acos(r / f32::sqrt(-q*q*q));
							let r13 = 2.0 * f32::sqrt(-q);
							let t1 = -b / 3.0 + r13 * f32::cos(dum1 / 3.0);
							let t2 = -b / 3.0 + r13 * f32::cos((dum1 + 2.0*std::f32::consts::PI) / 3.0);
							let t3 = -b / 3.0 + r13 * f32::cos((dum1 + 4.0*std::f32::consts::PI) / 3.0);
							
							match (t1 >= 0.0 && t1 <= time_to_beat, t2 >= 0.0 && t2 <= time_to_beat, t3 >= 0.0 && t3 <= time_to_beat, t1 < t2, t1 < t3, t2 < t3) {
								(true, true, true, true, true, _) => Some(t1),
								(true, true, true, false, _, true) => Some(t2),
								(true, true, true, _, false, false) => Some(t3),
								(true, true, false, true, _, _) => Some(t1),
								(true, true, false, false, _, _) => Some(t2),
								(true, false, true, _, true, _) => Some(t1),
								(true, false, true, _, false, _) => Some(t3),
								(false, true, true, _, _, true) => Some(t2),
								(false, true, true, _, _, false) => Some(t3),
								(true, false, false, _, _, _) => Some(t1),
								(false, true, false, _, _, _) => Some(t1),
								(false, false, true, _, _, _) => Some(t1),
								(false, false, false, _, _, _) => None,
								(true, true, true, false, true, false) | (true, true, true, true, false, true) => unreachable!()
							}
						}
					} else if cubic_b.abs() > 1e-7 {
						let b = cubic_c / cubic_b;
						let c = cubic_d / cubic_b;
						
						let discriminant = b*b - 4.0*c;
						if discriminant >= 0.0 {
							let sqrtd = discriminant.sqrt();
							let t1 = (-b + sqrtd) * 0.5;
							let t2 = (-b - sqrtd) * 0.5;
							
							match (t1 >= 0.0 && t1 <= time_to_beat, t2 >= 0.0 && t2 <= time_to_beat, t1 < t2) {
								(true, true, true) => Some(t1),
								(true, true, false) => Some(t2),
								(true, false, _) => Some(t1),
								(false, true, _) => Some(t2),
								(false, false, _) => None
							}
						} else {
							None
						}
					} else {
						let t = -cubic_d / cubic_c;
						match t >= 0.0 && t <= time_to_beat {
							true => Some(t),
							false => None
						}
					};
					
					
					if let Some(t) = t {
						let p = p0 + dp * t;
						let g = g0 + dg * t;
						let h = h0 + dh * t;
						
						let (larger, smaller) = match f32::max(g.0*g.1, f32::max(g.0*g.2, g.1*g.2)) > f32::max(h.0*h.1, f32::max(h.0*h.2, h.1*h.2)) {
							true => (g, h),
							false => (h, g)
						};
						
						let (la, lb, sa, sb, pa, pb) = match (larger.0 < larger.1, larger.0 < larger.2, larger.1 < larger.2) {
							(true, true, true) => (larger.2, larger.1, smaller.2, smaller.1, p.2, p.1),
							(true, true, false) => (larger.1, larger.2, smaller.1, smaller.2, p.1, p.2),
							(true, false, true) => unreachable!(),
							(true, false, false) => (larger.1, larger.0, smaller.1, smaller.0, p.1, p.0),
							(false, true, true) => (larger.2, larger.0, smaller.2, smaller.0, p.2, p.0),
							(false, true, false) => unreachable!(),
							(false, false, true) => (larger.0, larger.2, smaller.0, smaller.2, p.0, p.2),
							(false, false, false) => (larger.0, larger.1, smaller.0, smaller.1, p.0, p.1)
						};
						
						let s = (pa / la - pb / lb) / (sa / la - sb / lb);
						let r = (pa - sa * s) / la;
						// larger * r + smaller * s = p
						
						if r >= 0.0 && s >= 0.0 && r + s <= 1.0 {
							collision = Some((t, i, j, k, l));
						}
					}
				}
			}
		}}
		
		if let Some((t, i, j, k, l)) = collision {
			let t_step = dt_remaining * f32::max(t - 0.001, t * 0.5);
			for i in 0..objects.len() {
				objects[i].transform = objects[i].future_transform(t_step);
			}
			dt_remaining -= t_step;
			
			
			let position = objects[i].vertex_buffer.read().unwrap().get(k).unwrap().apply_transform(&objects[i].transform);
			
			let (a_index, b_index, c_index) = objects[j].indices[l];
			let a = *objects[j].vertex_buffer.read().unwrap().get(a_index as usize).unwrap();
			let b = *objects[j].vertex_buffer.read().unwrap().get(b_index as usize).unwrap();
			let c = *objects[j].vertex_buffer.read().unwrap().get(c_index as usize).unwrap();
			let normal = (b - a).cross(c - a).apply_transform(&objects[j].transform.set_position(Vec3(0.0, 0.0, 0.0))).normalize();
			
			collide(objects, i, j, position, normal);
			
			collision = None;
			
		} else {
			for i in 0..objects.len() {
				objects[i].transform = new_transforms[i];
			}
			break;
		}
	}
}


fn collide(objects: &mut [Object], i: usize, j: usize, p: Vec3, n: Vec3) {
	
	let v1 = objects[i].velocity + objects[i].angular_velocity.cross(p - objects[i].transform.get_position());
	let v2 = objects[j].velocity + objects[j].angular_velocity.cross(p - objects[j].transform.get_position());
	
	objects[i].velocity = -objects[i].velocity;
	
}








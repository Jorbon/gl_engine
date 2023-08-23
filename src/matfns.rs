

pub struct Matrix {
	pub m: [[f32; 4]; 4]
}

impl Matrix {
	pub fn _mat4_mult(&self, m: [[f32; 4]; 4]) -> Matrix {
		Matrix { m: [
			[
				self.m[0][0]*m[0][0] + self.m[1][0]*m[0][1] + self.m[2][0]*m[0][2] + self.m[3][0]*m[0][3],
				self.m[0][1]*m[0][0] + self.m[1][1]*m[0][1] + self.m[2][1]*m[0][2] + self.m[3][1]*m[0][3],
				self.m[0][2]*m[0][0] + self.m[1][2]*m[0][1] + self.m[2][2]*m[0][2] + self.m[3][2]*m[0][3],
				self.m[0][3]*m[0][0] + self.m[1][3]*m[0][1] + self.m[2][3]*m[0][2] + self.m[3][3]*m[0][3],
			], [
				self.m[0][0]*m[1][0] + self.m[1][0]*m[1][1] + self.m[2][0]*m[1][2] + self.m[3][0]*m[1][3],
				self.m[0][1]*m[1][0] + self.m[1][1]*m[1][1] + self.m[2][1]*m[1][2] + self.m[3][1]*m[1][3],
				self.m[0][2]*m[1][0] + self.m[1][2]*m[1][1] + self.m[2][2]*m[1][2] + self.m[3][2]*m[1][3],
				self.m[0][3]*m[1][0] + self.m[1][3]*m[1][1] + self.m[2][3]*m[1][2] + self.m[3][3]*m[1][3],
			], [
				self.m[0][0]*m[2][0] + self.m[1][0]*m[2][1] + self.m[2][0]*m[2][2] + self.m[3][0]*m[2][3],
				self.m[0][1]*m[2][0] + self.m[1][1]*m[2][1] + self.m[2][1]*m[2][2] + self.m[3][1]*m[2][3],
				self.m[0][2]*m[2][0] + self.m[1][2]*m[2][1] + self.m[2][2]*m[2][2] + self.m[3][2]*m[2][3],
				self.m[0][3]*m[2][0] + self.m[1][3]*m[2][1] + self.m[2][3]*m[2][2] + self.m[3][3]*m[2][3],
			], [
				self.m[0][0]*m[3][0] + self.m[1][0]*m[3][1] + self.m[2][0]*m[3][2] + self.m[3][0]*m[3][3],
				self.m[0][1]*m[3][0] + self.m[1][1]*m[3][1] + self.m[2][1]*m[3][2] + self.m[3][1]*m[3][3],
				self.m[0][2]*m[3][0] + self.m[1][2]*m[3][1] + self.m[2][2]*m[3][2] + self.m[3][2]*m[3][3],
				self.m[0][3]*m[3][0] + self.m[1][3]*m[3][1] + self.m[2][3]*m[3][2] + self.m[3][3]*m[3][3],
			]
		] }
	}
	
	pub fn new() -> Matrix {
		Matrix { m: [
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		] }
	}
	
	pub fn rotate_x(&self, angle: f32) -> Matrix {
		let c = angle.cos();
		let s = angle.sin();
		Matrix { m: [
			[self.m[0][0], self.m[0][1]*c - self.m[0][2]*s, self.m[0][2]*c + self.m[0][1]*s, self.m[0][3]],
			[self.m[1][0], self.m[1][1]*c - self.m[1][2]*s, self.m[1][2]*c + self.m[1][1]*s, self.m[1][3]],
			[self.m[2][0], self.m[2][1]*c - self.m[2][2]*s, self.m[2][2]*c + self.m[2][1]*s, self.m[2][3]],
			[self.m[3][0], self.m[3][1]*c - self.m[3][2]*s, self.m[3][2]*c + self.m[3][1]*s, self.m[3][3]],
		] }
	}
	
	pub fn rotate_y(&self, angle: f32) -> Matrix {
		let c = angle.cos();
		let s = angle.sin();
		Matrix { m: [
			[self.m[0][0]*c + self.m[0][2]*s, self.m[0][1], self.m[0][2]*c - self.m[0][0]*s, self.m[0][3]],
			[self.m[1][0]*c + self.m[1][2]*s, self.m[1][1], self.m[1][2]*c - self.m[1][0]*s, self.m[1][3]],
			[self.m[2][0]*c + self.m[2][2]*s, self.m[2][1], self.m[2][2]*c - self.m[2][0]*s, self.m[2][3]],
			[self.m[3][0]*c + self.m[3][2]*s, self.m[3][1], self.m[3][2]*c - self.m[3][0]*s, self.m[3][3]],
		] }
	}
	
	pub fn _rotate_z(&self, angle: f32) -> Matrix {
		let c = angle.cos();
		let s = angle.sin();
		Matrix { m: [
			[self.m[0][0]*c - self.m[0][1]*s, self.m[0][1]*c + self.m[0][0]*s, self.m[0][2], self.m[0][3]],
			[self.m[1][0]*c - self.m[1][1]*s, self.m[1][1]*c + self.m[1][0]*s, self.m[1][2], self.m[1][3]],
			[self.m[2][0]*c - self.m[2][1]*s, self.m[2][1]*c + self.m[2][0]*s, self.m[2][2], self.m[2][3]],
			[self.m[3][0]*c - self.m[3][1]*s, self.m[3][1]*c + self.m[3][0]*s, self.m[3][2], self.m[3][3]],
		] }
	}
	
	pub fn translate(&self, dx: f32, dy: f32, dz: f32) -> Matrix {
		Matrix { m: [self.m[0], self.m[1], self.m[2], [self.m[3][0] + dx, self.m[3][1] + dy, self.m[3][2] + dz, self.m[3][3]]] }
	}
	
	pub fn scale(&self, s: f32) -> Matrix {
		Matrix { m: [
			[self.m[0][0]*s, self.m[0][1]*s, self.m[0][2]*s, self.m[0][3]],
			[self.m[1][0]*s, self.m[1][1]*s, self.m[1][2]*s, self.m[1][3]],
			[self.m[2][0]*s, self.m[2][1]*s, self.m[2][2]*s, self.m[2][3]],
			[self.m[3][0]*s, self.m[3][1]*s, self.m[3][2]*s, self.m[3][3]],
		] }
	}
	
	pub fn scale_xyz(&self, sx: f32, sy: f32, sz: f32) -> Matrix {
		Matrix { m: [
			[self.m[0][0]*sx, self.m[0][1]*sy, self.m[0][2]*sz, self.m[0][3]],
			[self.m[1][0]*sx, self.m[1][1]*sy, self.m[1][2]*sz, self.m[1][3]],
			[self.m[2][0]*sx, self.m[2][1]*sy, self.m[2][2]*sz, self.m[2][3]],
			[self.m[3][0]*sx, self.m[3][1]*sy, self.m[3][2]*sz, self.m[3][3]],
		] }
	}
}




pub fn normalize_vec3(v: [f32; 3]) -> [f32; 3] {
	let f = 1.0 / (v[0]*v[0] + v[1]*v[1] + v[2]*v[2]).sqrt();
	[v[0]*f, v[1]*f, v[2]*f]
}




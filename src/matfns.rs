#[derive(Copy, Clone, Debug, Default)]
pub struct Vec2(pub f32, pub f32);
#[derive(Copy, Clone, Debug, Default)]
pub struct Vec3(pub f32, pub f32, pub f32);
#[derive(Copy, Clone, Debug)]
pub struct Mat4(pub [[f32; 4]; 4]);

#[allow(dead_code)]
impl Mat4 {
	pub fn mult_mat4(&self, m: [[f32; 4]; 4]) -> Mat4 {
		Mat4([
			[
				self.0[0][0]*m[0][0] + self.0[1][0]*m[0][1] + self.0[2][0]*m[0][2] + self.0[3][0]*m[0][3],
				self.0[0][1]*m[0][0] + self.0[1][1]*m[0][1] + self.0[2][1]*m[0][2] + self.0[3][1]*m[0][3],
				self.0[0][2]*m[0][0] + self.0[1][2]*m[0][1] + self.0[2][2]*m[0][2] + self.0[3][2]*m[0][3],
				self.0[0][3]*m[0][0] + self.0[1][3]*m[0][1] + self.0[2][3]*m[0][2] + self.0[3][3]*m[0][3],
			], [
				self.0[0][0]*m[1][0] + self.0[1][0]*m[1][1] + self.0[2][0]*m[1][2] + self.0[3][0]*m[1][3],
				self.0[0][1]*m[1][0] + self.0[1][1]*m[1][1] + self.0[2][1]*m[1][2] + self.0[3][1]*m[1][3],
				self.0[0][2]*m[1][0] + self.0[1][2]*m[1][1] + self.0[2][2]*m[1][2] + self.0[3][2]*m[1][3],
				self.0[0][3]*m[1][0] + self.0[1][3]*m[1][1] + self.0[2][3]*m[1][2] + self.0[3][3]*m[1][3],
			], [
				self.0[0][0]*m[2][0] + self.0[1][0]*m[2][1] + self.0[2][0]*m[2][2] + self.0[3][0]*m[2][3],
				self.0[0][1]*m[2][0] + self.0[1][1]*m[2][1] + self.0[2][1]*m[2][2] + self.0[3][1]*m[2][3],
				self.0[0][2]*m[2][0] + self.0[1][2]*m[2][1] + self.0[2][2]*m[2][2] + self.0[3][2]*m[2][3],
				self.0[0][3]*m[2][0] + self.0[1][3]*m[2][1] + self.0[2][3]*m[2][2] + self.0[3][3]*m[2][3],
			], [
				self.0[0][0]*m[3][0] + self.0[1][0]*m[3][1] + self.0[2][0]*m[3][2] + self.0[3][0]*m[3][3],
				self.0[0][1]*m[3][0] + self.0[1][1]*m[3][1] + self.0[2][1]*m[3][2] + self.0[3][1]*m[3][3],
				self.0[0][2]*m[3][0] + self.0[1][2]*m[3][1] + self.0[2][2]*m[3][2] + self.0[3][2]*m[3][3],
				self.0[0][3]*m[3][0] + self.0[1][3]*m[3][1] + self.0[2][3]*m[3][2] + self.0[3][3]*m[3][3],
			]
		])
	}
	
	pub fn identity() -> Mat4 {
		Mat4([
			[1.0, 0.0, 0.0, 0.0],
			[0.0, 1.0, 0.0, 0.0],
			[0.0, 0.0, 1.0, 0.0],
			[0.0, 0.0, 0.0, 1.0],
		])
	}
	
	pub fn rotate_x(&self, angle: f32) -> Mat4 {
		let c = angle.cos();
		let s = angle.sin();
		Mat4([
			[self.0[0][0], self.0[0][1]*c - self.0[0][2]*s, self.0[0][2]*c + self.0[0][1]*s, self.0[0][3]],
			[self.0[1][0], self.0[1][1]*c - self.0[1][2]*s, self.0[1][2]*c + self.0[1][1]*s, self.0[1][3]],
			[self.0[2][0], self.0[2][1]*c - self.0[2][2]*s, self.0[2][2]*c + self.0[2][1]*s, self.0[2][3]],
			[self.0[3][0], self.0[3][1]*c - self.0[3][2]*s, self.0[3][2]*c + self.0[3][1]*s, self.0[3][3]],
		])
	}
	
	pub fn rotate_y(&self, angle: f32) -> Mat4 {
		let c = angle.cos();
		let s = angle.sin();
		Mat4([
			[self.0[0][0]*c + self.0[0][2]*s, self.0[0][1], self.0[0][2]*c - self.0[0][0]*s, self.0[0][3]],
			[self.0[1][0]*c + self.0[1][2]*s, self.0[1][1], self.0[1][2]*c - self.0[1][0]*s, self.0[1][3]],
			[self.0[2][0]*c + self.0[2][2]*s, self.0[2][1], self.0[2][2]*c - self.0[2][0]*s, self.0[2][3]],
			[self.0[3][0]*c + self.0[3][2]*s, self.0[3][1], self.0[3][2]*c - self.0[3][0]*s, self.0[3][3]],
		])
	}
	
	pub fn rotate_z(&self, angle: f32) -> Mat4 {
		let c = angle.cos();
		let s = angle.sin();
		Mat4([
			[self.0[0][0]*c - self.0[0][1]*s, self.0[0][1]*c + self.0[0][0]*s, self.0[0][2], self.0[0][3]],
			[self.0[1][0]*c - self.0[1][1]*s, self.0[1][1]*c + self.0[1][0]*s, self.0[1][2], self.0[1][3]],
			[self.0[2][0]*c - self.0[2][1]*s, self.0[2][1]*c + self.0[2][0]*s, self.0[2][2], self.0[2][3]],
			[self.0[3][0]*c - self.0[3][1]*s, self.0[3][1]*c + self.0[3][0]*s, self.0[3][2], self.0[3][3]],
		])
	}
	
	pub fn get_position(&self) -> Vec3 {
		Vec3(self.0[3][0], self.0[3][1], self.0[3][2])
	}
	
	pub fn set_position(&mut self, v: Vec3) {
		self.0[3][0] = v.0;
		self.0[3][1] = v.1;
		self.0[3][2] = v.2;
	}
	
	pub fn translate(&self, dx: f32, dy: f32, dz: f32) -> Mat4 {
		Mat4([self.0[0], self.0[1], self.0[2], [self.0[3][0] + dx, self.0[3][1] + dy, self.0[3][2] + dz, self.0[3][3]]])
	}
	
	pub fn scale(&self, s: f32) -> Mat4 {
		Mat4([
			[self.0[0][0]*s, self.0[0][1]*s, self.0[0][2]*s, self.0[0][3]],
			[self.0[1][0]*s, self.0[1][1]*s, self.0[1][2]*s, self.0[1][3]],
			[self.0[2][0]*s, self.0[2][1]*s, self.0[2][2]*s, self.0[2][3]],
			[self.0[3][0]*s, self.0[3][1]*s, self.0[3][2]*s, self.0[3][3]],
		])
	}
	
	pub fn scale_xyz(&self, sx: f32, sy: f32, sz: f32) -> Mat4 {
		Mat4([
			[self.0[0][0]*sx, self.0[0][1]*sy, self.0[0][2]*sz, self.0[0][3]],
			[self.0[1][0]*sx, self.0[1][1]*sy, self.0[1][2]*sz, self.0[1][3]],
			[self.0[2][0]*sx, self.0[2][1]*sy, self.0[2][2]*sz, self.0[2][3]],
			[self.0[3][0]*sx, self.0[3][1]*sy, self.0[3][2]*sz, self.0[3][3]],
		])
	}
}


#[allow(dead_code)]
impl Vec3 {
	pub fn length_squared(&self) -> f32 {
		self.0*self.0 + self.1*self.1 + self.2*self.2
	}
	
	pub fn normalize(&self) -> Vec3 {
		let f = 1.0 / (self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt();
		Vec3(self.0*f, self.1*f, self.2*f)
	}

	#[inline]
	pub fn normalize_fast(&self) -> Vec3 {
		let length2: f32 = self.0*self.0 + self.1*self.1 + self.2*self.2;
		let f: f32 = unsafe {
			let i: u32 = *(&length2 as *const f32 as *const u32);
			let i: u32 = 0x5f3759df - (i >> 1);
			let y: f32 = *(&i as *const u32 as *const f32);
			y * (1.5 - 0.5 * length2 * y * y)
		};
		Vec3(self.0*f, self.1*f, self.2*f)
	}
}




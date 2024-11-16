

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec2(pub f32, pub f32);
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Vec3(pub f32, pub f32, pub f32);
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Mat4(pub [[f32; 4]; 4]); // [column][row], [x][y], inner list is a column

#[allow(dead_code)]
impl Mat4 {
	pub fn mult_mat4(&self, m: &Mat4) -> Mat4 {
		Mat4([
			[
				self.0[0][0]*m.0[0][0] + self.0[1][0]*m.0[0][1] + self.0[2][0]*m.0[0][2] + self.0[3][0]*m.0[0][3],
				self.0[0][1]*m.0[0][0] + self.0[1][1]*m.0[0][1] + self.0[2][1]*m.0[0][2] + self.0[3][1]*m.0[0][3],
				self.0[0][2]*m.0[0][0] + self.0[1][2]*m.0[0][1] + self.0[2][2]*m.0[0][2] + self.0[3][2]*m.0[0][3],
				self.0[0][3]*m.0[0][0] + self.0[1][3]*m.0[0][1] + self.0[2][3]*m.0[0][2] + self.0[3][3]*m.0[0][3],
			], [
				self.0[0][0]*m.0[1][0] + self.0[1][0]*m.0[1][1] + self.0[2][0]*m.0[1][2] + self.0[3][0]*m.0[1][3],
				self.0[0][1]*m.0[1][0] + self.0[1][1]*m.0[1][1] + self.0[2][1]*m.0[1][2] + self.0[3][1]*m.0[1][3],
				self.0[0][2]*m.0[1][0] + self.0[1][2]*m.0[1][1] + self.0[2][2]*m.0[1][2] + self.0[3][2]*m.0[1][3],
				self.0[0][3]*m.0[1][0] + self.0[1][3]*m.0[1][1] + self.0[2][3]*m.0[1][2] + self.0[3][3]*m.0[1][3],
			], [
				self.0[0][0]*m.0[2][0] + self.0[1][0]*m.0[2][1] + self.0[2][0]*m.0[2][2] + self.0[3][0]*m.0[2][3],
				self.0[0][1]*m.0[2][0] + self.0[1][1]*m.0[2][1] + self.0[2][1]*m.0[2][2] + self.0[3][1]*m.0[2][3],
				self.0[0][2]*m.0[2][0] + self.0[1][2]*m.0[2][1] + self.0[2][2]*m.0[2][2] + self.0[3][2]*m.0[2][3],
				self.0[0][3]*m.0[2][0] + self.0[1][3]*m.0[2][1] + self.0[2][3]*m.0[2][2] + self.0[3][3]*m.0[2][3],
			], [
				self.0[0][0]*m.0[3][0] + self.0[1][0]*m.0[3][1] + self.0[2][0]*m.0[3][2] + self.0[3][0]*m.0[3][3],
				self.0[0][1]*m.0[3][0] + self.0[1][1]*m.0[3][1] + self.0[2][1]*m.0[3][2] + self.0[3][1]*m.0[3][3],
				self.0[0][2]*m.0[3][0] + self.0[1][2]*m.0[3][1] + self.0[2][2]*m.0[3][2] + self.0[3][2]*m.0[3][3],
				self.0[0][3]*m.0[3][0] + self.0[1][3]*m.0[3][1] + self.0[2][3]*m.0[3][2] + self.0[3][3]*m.0[3][3],
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
	
	pub fn rotate_axis(&self, axis: Vec3, angle: f32) -> Mat4 {
		let c = angle.cos();
		let s = angle.sin();
		Mat4([
			[	c + axis.0*axis.0 * (1.0 - c),			axis.1*axis.0 * (1.0 - c) + axis.2 * s,	axis.2*axis.0 * (1.0 - c) - axis.1 * s,	0.0	],
			[	axis.0*axis.1 * (1.0 - c) - axis.2 * s,	c + axis.1*axis.1 * (1.0 - c),			axis.2*axis.1 * (1.0 - c) + axis.0 * s,	0.0	],
			[	axis.0*axis.2 * (1.0 - c) + axis.1 * s,	axis.1*axis.2 * (1.0 - c) - axis.0 * s,	c + axis.2*axis.2 * (1.0 - c),			0.0	],
			[	0.0,									0.0,									0.0,									1.0	],
		]).mult_mat4(self)
	}
	
	pub fn get_position(&self) -> Vec3 {
		Vec3(self.0[3][0], self.0[3][1], self.0[3][2])
	}
	
	pub fn set_position(&self, v: Vec3) -> Mat4 {
		Mat4([
			self.0[0], self.0[1], self.0[2],
			[v.0, v.1, v.2, self.0[3][3]]
		])
	}
	
	pub fn translate(&self, dp: Vec3) -> Mat4 {
		Mat4([self.0[0], self.0[1], self.0[2], [self.0[3][0] + dp.0, self.0[3][1] + dp.1, self.0[3][2] + dp.2, self.0[3][3]]])
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
impl Vec2 {
	#[inline] pub fn length_squared(self) -> f32 { self.0*self.0 + self.1*self.1 }
	#[inline] pub fn length(self) -> f32 { self.length_squared().sqrt() }
	#[inline] pub fn normalize(self) -> Self { let f = 1.0 / self.length(); Self(self.0*f, self.1*f) }
	#[inline] pub fn dot(self, v: Self) -> f32 { self.0 * v.0 + self.1 * v.1 }
	#[inline] pub fn cross(self, v: Self) -> f32 { self.0 * v.1 - self.1 * v.0 }
}

#[allow(dead_code)]
impl Vec3 {
	#[inline] pub fn length_squared(self) -> f32 { self.0*self.0 + self.1*self.1 + self.2*self.2 }
	#[inline] pub fn length(self) -> f32 { self.length_squared().sqrt() }
	#[inline] pub fn normalize(self) -> Self { let f = 1.0 / self.length(); Self(self.0*f, self.1*f, self.2*f) }
	#[inline] pub fn dot(self, v: Self) -> f32 { self.0 * v.0 + self.1 * v.1 + self.2 * v.2 }
	#[inline] pub fn cross(self, v: Self) -> Vec3 { Vec3(self.1 * v.2 - self.2 * v.1, self.2 * v.0 - self.0 * v.2, self.0 * v.1 - self.1 * v.0) }
	#[inline] pub fn apply_transform(self, m: &Mat4) -> Self { Self(
		m.0[0][0]*self.0 + m.0[1][0]*self.1 + m.0[2][0]*self.2 + m.0[3][0],
		m.0[0][1]*self.0 + m.0[1][1]*self.1 + m.0[2][1]*self.2 + m.0[3][1],
		m.0[0][2]*self.0 + m.0[1][2]*self.1 + m.0[2][2]*self.2 + m.0[3][2],
	) }
}

impl std::ops::Add for Vec2 { type Output = Self; #[inline] fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0, self.1 + rhs.1) } }
impl std::ops::AddAssign for Vec2 { #[inline] fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl std::ops::Sub for Vec2 { type Output = Self; #[inline] fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0, self.1 - rhs.1) } }
impl std::ops::SubAssign for Vec2 { #[inline] fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl std::ops::Neg for Vec2 { type Output = Self; #[inline] fn neg(self) -> Self::Output { Self(-self.0, -self.1) } }
impl std::ops::Mul<f32> for Vec2 { type Output = Self; #[inline] fn mul(self, rhs: f32) -> Self::Output { Self(self.0 * rhs, self.1 * rhs) } }
impl std::ops::MulAssign<f32> for Vec2 { #[inline] fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; } }
impl std::ops::Div<f32> for Vec2 { type Output = Self; #[inline] fn div(self, rhs: f32) -> Self::Output { self * (1.0/rhs) } }
impl std::ops::DivAssign<f32> for Vec2 { #[inline] fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; } }
impl std::ops::Mul<Vec2> for f32 { type Output = Vec2; #[inline] fn mul(self, rhs: Vec2) -> Self::Output { rhs * self } }

impl std::ops::Add for Vec3 { type Output = Self; #[inline] fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2) } }
impl std::ops::AddAssign for Vec3 { #[inline] fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; } }
impl std::ops::Sub for Vec3 { type Output = Self; #[inline] fn sub(self, rhs: Self) -> Self::Output { Self(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2) } }
impl std::ops::SubAssign for Vec3 { #[inline] fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; } }
impl std::ops::Neg for Vec3 { type Output = Self; #[inline] fn neg(self) -> Self::Output { Self(-self.0, -self.1, -self.2) } }
impl std::ops::Mul<f32> for Vec3 { type Output = Self; #[inline] fn mul(self, rhs: f32) -> Self::Output { Self(self.0 * rhs, self.1 * rhs, self.2 * rhs) } }
impl std::ops::MulAssign<f32> for Vec3 { #[inline] fn mul_assign(&mut self, rhs: f32) { *self = *self * rhs; } }
impl std::ops::Div<f32> for Vec3 { type Output = Self; #[inline] fn div(self, rhs: f32) -> Self::Output { self * (1.0/rhs) } }
impl std::ops::DivAssign<f32> for Vec3 { #[inline] fn div_assign(&mut self, rhs: f32) { *self = *self / rhs; } }
impl std::ops::Mul<Vec3> for f32 { type Output = Vec3; #[inline] fn mul(self, rhs: Vec3) -> Self::Output { rhs * self } }




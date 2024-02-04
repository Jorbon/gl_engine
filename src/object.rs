use glium::{index::PrimitiveType, Display, IndexBuffer, VertexBuffer};

use crate::math_structs::{Mat4, Vec3};


pub struct Object {
	pub vertex_buffer: VertexBuffer<Vec3>,
	pub index_buffer: IndexBuffer<u16>,
	pub indices: Box<[(u16, u16, u16)]>,
	pub edges: Box<[(u16, u16)]>,
	pub transform: Mat4,
	/// The velocity of the object in 3D space.
	pub velocity: Vec3,
	pub angular_velocity: Vec3,
}

impl Object {
	pub fn new(display: &Display, vertices: &[Vec3], indices: &[(u16, u16, u16)]) -> Self {
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap(); // might switch to dynamic later
		let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, unsafe {
			core::slice::from_raw_parts(indices.as_ptr() as *const u16, indices.len() * 3)
		}).unwrap();
		
		let mut edges = std::collections::HashSet::new();
		for t in indices {
			let (a, b, c) = match (t.0 < t.1, t.0 < t.2, t.1 < t.2) {
				(true, true, true) => (t.0, t.1, t.2),
				(true, true, false) => (t.0, t.2, t.1),
				(true, false, true) => unreachable!(),
				(true, false, false) => (t.2, t.0, t.1),
				(false, true, true) => (t.1, t.0, t.2),
				(false, true, false) => unreachable!(),
				(false, false, true) => (t.1, t.2, t.0),
				(false, false, false) => (t.2, t.1, t.0)
			};
			edges.insert((a, b));
			edges.insert((a, c));
			edges.insert((b, c));
		}
		
		Self {
			vertex_buffer,
			index_buffer,
			indices: indices.to_vec().into_boxed_slice(),
			edges: edges.into_iter().collect::<Vec<(u16, u16)>>().into_boxed_slice(),
			transform: Mat4::identity(),
			velocity: Vec3(0.0, 0.0, 0.0),
			angular_velocity: Vec3(0.0, 0.0, 0.0)
		}
	}
	pub fn future_transform(&self, dt: f32) -> Mat4 {
		let new_position = self.transform.get_position() + self.velocity * dt;
		let mut new_transform = self.transform;
		if self.angular_velocity.length_squared() > 0.0 {
			new_transform = new_transform.rotate_axis(self.angular_velocity.normalize(), self.angular_velocity.length() * dt);
		}
		new_transform.set_position(new_position)
	}
}
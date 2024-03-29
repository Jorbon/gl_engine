use glium::{Display, IndexBuffer, VertexBuffer};

use crate::{math_structs::Vec3, object::Object};


pub fn initialize_scene(display: &Display) -> (Vec<Object>, Vec<VertexBuffer<Vec3>>, Vec<IndexBuffer<u16>>) {
	let (mut cube, cube_vbuf, cube_ibuf) = Object::new_with_buffers(&display, &[
		Vec3(-1.0, -1.0, -1.0),
		Vec3(-1.0, -1.0,  1.0),
		Vec3(-1.0,  1.0, -1.0),
		Vec3(-1.0,  1.0,  1.0),
		Vec3( 1.0, -1.0, -1.0),
		Vec3( 1.0, -1.0,  1.0),
		Vec3( 1.0,  1.0, -1.0),
		Vec3( 1.0,  1.0,  1.0),
	], &[
		(0, 2, 3),
		(0, 3, 1),
		(0, 1, 5),
		(0, 5, 4),
		(0, 4, 6),
		(0, 6, 2),
		(7, 2, 6),
		(7, 6, 4),
		(7, 4, 5),
		(7, 5, 1),
		(7, 1, 3),
		(7, 3, 2),
	]);
	
	let (floor, floor_vbuf, floor_ibuf) = Object::new_with_buffers(&display, &[
		Vec3(-10.0, 0.0, -10.0),
		Vec3(-10.0, 0.0,  10.0),
		Vec3( 10.0, 0.0, -10.0),
		Vec3( 10.0, 0.0,  10.0),
	], &[
		(0, 2, 3),
		(0, 3, 1)
	]);
	
	cube.transform = cube.transform.rotate_x(0.5).rotate_z(0.5).translate(Vec3(0.0, 10.0, 0.0));

	//o1.angular_velocity = Vec3(0.2, 0.3, 0.5);
	//floor.angular_velocity = Vec3(-0.05, 0.1, -0.02);


	(
		vec![cube, floor],
		vec![cube_vbuf, floor_vbuf],
		vec![cube_ibuf, floor_ibuf]
	)
}



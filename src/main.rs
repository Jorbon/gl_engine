#[macro_use]
extern crate glium;

mod math_structs;
mod object;
mod physics;
mod render;

use math_structs::{Mat4, Vec2, Vec3};

use glium::{glutin::{event::{Event, WindowEvent, ElementState, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}, dpi::{PhysicalPosition, PhysicalSize, LogicalSize}, window::{CursorGrabMode, WindowBuilder}, ContextBuilder}, vertex::Attribute, Display, Vertex, VertexFormat};
use object::Object;
use render::Renderer;




impl Vertex for Vec2 {
	fn build_bindings() -> VertexFormat {
		std::borrow::Cow::Owned(vec![(std::borrow::Cow::Borrowed("position"), 0, -1, <(f32, f32)>::get_type(), false)])
	}
}

impl Vertex for Vec3 {
	fn build_bindings() -> VertexFormat {
		std::borrow::Cow::Owned(vec![(std::borrow::Cow::Borrowed("position"), 0, -1, <(f32, f32, f32)>::get_type(), false)])
	}
}


pub struct Camera {
	pub position: Vec3,
	pub horizontal_angle: f32,
	pub vertical_angle: f32
}

impl Camera {
	pub fn get_transform(&self) -> Mat4 {
		Mat4::identity().translate(-self.position).rotate_y(self.horizontal_angle).rotate_x(self.vertical_angle)
	}
}





fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(1024.0, 768.0));
	let cb = ContextBuilder::new();
	let display = Display::new(wb, cb, &event_loop).unwrap();
	let PhysicalSize { width, height } = display.gl_window().window().inner_size();
	
	
	
	
	let mut o1 = Object::new(&display, &[
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
		]
	);
	
	o1.transform = o1.transform.rotate_x(0.5).rotate_z(0.5).translate(Vec3(0.0, 10.0, 0.0));
	
	
	let floor = Object::new(&display, &[
		Vec3(-10.0, 0.0, -10.0),
		Vec3(-10.0, 0.0,  10.0),
		Vec3( 10.0, 0.0, -10.0),
		Vec3( 10.0, 0.0,  10.0),
	], &[
		(0, 2, 3),
		(0, 3, 1)
	]);
	
	//o1.angular_velocity = Vec3(0.2, 0.3, 0.5);
	//floor.angular_velocity = Vec3(-0.05, 0.1, -0.02);
	
	
	let mut objects = vec![o1, floor];
	
	
	
	
	let mut up = false;
	let mut down = false;
	let mut left = false;
	let mut right = false;
	let mut wkey = false;
	let mut akey = false;
	let mut skey = false;
	let mut dkey = false;
	let mut space = false;
	let mut shift = false;
	
	let mut run = false;
	let mut capture = false;
	let mut previous_mouse_pos = PhysicalPosition::<f64>::new(0.0, 0.0);
	
	
	
	let mut camera = Camera {
		position: Vec3(4.0, 6.0, -11.0),
		horizontal_angle: 0.3,
		vertical_angle: 0.0
	};
	
	let lspeed = 0.00025;
	let speed = 2.0;
	
	let g = 9.8;
	
	
	let mut dummy = 0.0f32;
	
	let do_post_process = true;
	let mut show_shadowmap = false;
	
	
	let mut previous_frame_time = std::time::SystemTime::now();
	let mut avg_frame_time = 0.0;
	
	let mut renderer = Renderer::new(&display, width, height, 75.0, 0.01, 1000.0);
	
	
	let mut count = 0;
	
	
	
	event_loop.run(move |ev, _, control_flow| {
		match ev {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _ } => {
					if let Some(code) = input.virtual_keycode {
						let state = match input.state {
							ElementState::Pressed => true,
							ElementState::Released => false
						};
						match code {
							VirtualKeyCode::Left => left = state,
							VirtualKeyCode::Right => right = state,
							VirtualKeyCode::Up => up = state,
							VirtualKeyCode::Down => down = state,
							VirtualKeyCode::W => wkey = state,
							VirtualKeyCode::S => skey = state,
							VirtualKeyCode::A => akey = state,
							VirtualKeyCode::D => dkey = state,
							VirtualKeyCode::Space => space = state,
							VirtualKeyCode::LShift => shift = state,
							
							VirtualKeyCode::P => if state { run = !run; }
							VirtualKeyCode::M => if state { show_shadowmap = !show_shadowmap; }
							VirtualKeyCode::Comma => if state { dummy -= 0.1; }
							VirtualKeyCode::Period => if state { dummy += 0.1; }
							VirtualKeyCode::Slash => if state { dummy = 0.0; }
							
							VirtualKeyCode::R => if state {
								objects[0].transform = objects[0].transform.set_position(Vec3(0.0, 10.0, 0.0));
								objects[0].velocity = Vec3(0.0, 0.0, 0.0);
								objects[0].angular_velocity = Vec3(0.0, 0.1, 0.2);
							}
							
							VirtualKeyCode::Escape => if state && capture {
								capture = false;
								display.gl_window().window().set_cursor_grab(CursorGrabMode::None).unwrap();
								display.gl_window().window().set_cursor_visible(true);
							}
							
							_ => ()
						}
						
					}
				}
				WindowEvent::MouseInput { device_id: _, state, button: _, .. } => {
					if let ElementState::Pressed = state {
						capture = true;
						display.gl_window().window().set_cursor_grab(CursorGrabMode::Confined).unwrap();
						display.gl_window().window().set_cursor_visible(false);
					}
				}
				WindowEvent::CursorMoved { device_id: _, position, .. } => {
					let _dx = position.x - previous_mouse_pos.x;
					let _dy = position.y - previous_mouse_pos.y;
					
					if capture {
						let size = display.gl_window().window().inner_size();
						let center_x = size.width / 2;
						let center_y = size.height / 2;
						camera.horizontal_angle += (position.x as f32 - center_x as f32) * lspeed;
						camera.vertical_angle -= (position.y as f32 - center_y as f32) * lspeed;
						display.gl_window().window().set_cursor_position(PhysicalPosition::new(center_x, center_y)).unwrap();
					}
					
					previous_mouse_pos = position;
				}
				WindowEvent::Resized(PhysicalSize { width, height }) => {
					renderer.resize(&display, width, height);
				}
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
				}
				_ => ()
			}
			Event::RedrawEventsCleared => {
				display.gl_window().window().request_redraw();
			}
			Event::RedrawRequested(_) => {
				let dt = {
					let now = std::time::SystemTime::now();
					let dt = now.duration_since(previous_frame_time).unwrap().as_secs_f32();
					previous_frame_time = now;
					dt
				};
				
				if dt < 1.0 {
					avg_frame_time += dt * (dt - avg_frame_time);
				} else {
					avg_frame_time = dt;
				}
				
				display.gl_window().window().set_title(&format!("3d things: {} fps", (1.0 / avg_frame_time) as u32));
				
				
				let asin = camera.horizontal_angle.sin();
				let acos = camera.horizontal_angle.cos();
				let mut mov = Vec3(0.0, 0.0, 0.0);
				if wkey { mov.2 += 1.0 }
				if skey { mov.2 -= 1.0 }
				if akey { mov.0 += 1.0 }
				if dkey { mov.0 -= 1.0 }
				if space { mov.1 += 1.0 }
				if shift { mov.1 -= 1.0 }
				if mov.length_squared() > 0.0 {
					mov = mov.normalize();
					let ds = dt * speed;
					camera.position.0 += (mov.0*acos - mov.2*asin)*ds;
					camera.position.1 += mov.1*ds;
					camera.position.2 += (mov.2*acos + mov.0*asin)*ds;
				}
				
				if right { camera.horizontal_angle += lspeed }
				if left { camera.horizontal_angle -= lspeed }
				if up { camera.vertical_angle += lspeed }
				if down { camera.vertical_angle -= lspeed }
				
				
				
				if run {
					let dt = 0.02;//f32::min(dt, 0.02);
					
					for i in 0..objects.len() {
						objects[i].velocity += Vec3(0.0, -g * dt, 0.0);
					}
					objects[1].velocity = Vec3(0.0, 0.0, 0.0);
					
					crate::physics::run(&mut objects, dt, count);
					count += 1;
				}
				
				
				let light_direction = Vec3(f32::cos(dummy), 2.0, f32::sin(dummy)).normalize();
				renderer.shadowmap.set_up_transform(light_direction);
				
				renderer.render(&display, &camera, &objects, light_direction, do_post_process, show_shadowmap, dummy);
				
			}
			_ => ()
		}
	});
}

extern crate glium;

mod math_structs;
mod object;
mod physics;
mod render;
mod scene;

use std::{sync::mpsc::{self, Receiver, TryRecvError}, time::{Duration, Instant}};

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

pub fn get_latest_value<T>(rx: &Receiver<T>) -> Option<T> {
	let mut value = None;
	loop {
		match rx.try_recv() {
			Ok(v) => value = Some(v),
			Err(TryRecvError::Empty) => break,
			Err(TryRecvError::Disconnected) => std::process::exit(0)
		}
	}
	value
}


static TARGET_TPS: f32 = 10.0;







fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(1024.0, 768.0));
	let cb = ContextBuilder::new().with_vsync(true);
	let display = Display::new(wb, cb, &event_loop).unwrap();
	let PhysicalSize { width, height } = display.gl_window().window().inner_size();
	
	// continuous input states
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
	
	
	let mut run = true;
	let mut capture = false;
	let mut do_post_process = true;
	let mut show_shadowmap = false;
	
	let mut previous_mouse_pos = PhysicalPosition::<f64>::new(0.0, 0.0);
	
	
	let mut camera = Camera {
		position: Vec3(4.0, 6.0, -11.0),
		horizontal_angle: 0.3,
		vertical_angle: 0.0
	};
	
	let look_sensitivity = 0.00025;
	let movement_speed = 4.0;
	
	
	let g = 9.8;
	
	
	let mut dummy = 0.0f32;
	
	
	
	let mut previous_frame_time = Instant::now();
	let mut avg_frame_time = 0.0;
	let mut avg_tick_time = 0.0;
	let mut avg_tick_process_time = 0.0;
	
	let mut renderer = Renderer::new(&display, width, height, 75.0, 0.01, 1000.0);
	
	
	
	
	let (physics_tx, main_rx) = mpsc::channel::<Vec<(Mat4, Vec3, Vec3)>>();
	let (main_tx, physics_rx) = mpsc::channel::<Vec<(Mat4, Vec3, Vec3)>>();
	let (tps_tx, tps_rx) = mpsc::channel::<(f32, f32)>();
	let (control_tx, control_rx) = mpsc::channel::<bool>();
	
	let (mut objects, vertex_buffers, index_buffers) = crate::scene::initialize_scene(&display);
	let objects_physics = objects.clone();
	
	let _physics_thread = std::thread::spawn(move || {
		let mut objects = objects_physics;
		let mut previous_tick_time = Instant::now();
		
		loop {
			let start_time = Instant::now();
			let tick_dt = start_time.duration_since(previous_tick_time).as_secs_f32()	;
			previous_tick_time = start_time;
			
			if let Some(dynamic_states) = get_latest_value(&physics_rx) {
				for i in 0..objects.len() {
					objects[i].set_dynamic_state(dynamic_states[i]);
				}
			}
			
			if let Some(control) = get_latest_value(&control_rx) {
				run = control;
			}
			
			if run {
				let dt = 1.0 / TARGET_TPS;
				
				for i in 0..objects.len() {
					objects[i].velocity += Vec3(0.0, -g * dt, 0.0);
				}
				objects[1].velocity = Vec3(0.0, 0.0, 0.0);
				
				
				crate::physics::run(&mut objects, dt);
				
				physics_tx.send(objects.iter().map(Object::get_dynamic_state).collect::<Vec<_>>()).unwrap();
			}
			
			let process_time = Instant::now().duration_since(start_time);
			
			tps_tx.send((process_time.as_secs_f32(), tick_dt)).unwrap();
			
			let sleep_duration = Duration::from_secs_f32(1.0 / TARGET_TPS).checked_sub(process_time).unwrap_or(Duration::ZERO);
			spin_sleep::sleep(sleep_duration);
		}
	});
	
	
	
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
							
							VirtualKeyCode::P => if state { run = !run; control_tx.send(run).unwrap(); }
							VirtualKeyCode::M => if state { show_shadowmap = !show_shadowmap; }
							VirtualKeyCode::N => if state { do_post_process = !do_post_process; }
							VirtualKeyCode::Comma => if state { dummy -= 0.1; }
							VirtualKeyCode::Period => if state { dummy += 0.1; }
							VirtualKeyCode::Slash => if state { dummy = 0.0; }
							
							VirtualKeyCode::R => if state {
								objects = crate::scene::initialize_scene(&display).0;
								main_tx.send(objects.iter().map(Object::get_dynamic_state).collect::<Vec<_>>()).unwrap();
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
						camera.horizontal_angle += (position.x as f32 - center_x as f32) * look_sensitivity;
						camera.vertical_angle -= (position.y as f32 - center_y as f32) * look_sensitivity;
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
					let now = Instant::now();
					let dt = now.duration_since(previous_frame_time).as_secs_f32();
					previous_frame_time = now;
					dt
				};
				
				if dt < 1.0 {
					avg_frame_time += 2.0 * dt * (dt - avg_frame_time);
				} else {
					avg_frame_time = dt;
				}
				
				loop {
					match tps_rx.try_recv() {
						Ok((tick_process_time, tick_dt)) => {
							avg_tick_time += 2.0 * tick_dt * (tick_dt - avg_tick_time);
							avg_tick_process_time += 2.0 * tick_dt * (tick_process_time - avg_tick_process_time);
						}
						Err(TryRecvError::Empty) => break,
						Err(TryRecvError::Disconnected) => panic!()
					}
				}
				
				display.gl_window().window().set_title(&format!("3d things: {} fps, {} tps, {:.3} mspt", (1.0 / avg_frame_time) as u32, (1.0 / avg_tick_time) as u32, 1000.0 * avg_tick_process_time));
				
				
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
					let ds = dt * movement_speed;
					camera.position.0 += (mov.0*acos - mov.2*asin)*ds;
					camera.position.1 += mov.1*ds;
					camera.position.2 += (mov.2*acos + mov.0*asin)*ds;
				}
				
				if right { camera.horizontal_angle += look_sensitivity }
				if left { camera.horizontal_angle -= look_sensitivity }
				if up { camera.vertical_angle += look_sensitivity }
				if down { camera.vertical_angle -= look_sensitivity }
				
				
				let mut dynamic_states = None;
				loop {
					match main_rx.try_recv() {
						Ok(states) => dynamic_states = Some(states),
						Err(TryRecvError::Empty) => break,
						Err(TryRecvError::Disconnected) => panic!()
					}
				}
				
				if let Some(dynamic_states) = dynamic_states {
					for i in 0..objects.len() {
						objects[i].set_dynamic_state(dynamic_states[i]);
					}
				}
				
				
				
				renderer.render(&display, &camera, &objects, &vertex_buffers, &index_buffers, do_post_process, show_shadowmap, dummy);
				
			}
			_ => ()
		}
	});
}

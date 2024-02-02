#[macro_use]
extern crate glium;

mod matfns;

use matfns::{Mat4, Vec2, Vec3};

use glium::{draw_parameters::DepthTest, framebuffer::{SimpleFrameBuffer, MultiOutputFrameBuffer}, glutin::{event::{Event, WindowEvent, ElementState, VirtualKeyCode}, event_loop::{ControlFlow, EventLoop}, dpi::{PhysicalPosition, PhysicalSize, LogicalSize}, window::{CursorGrabMode, WindowBuilder}, ContextBuilder}, index::PrimitiveType, texture::{depth_texture2d::DepthTexture2d, RawImage2d, SrgbTexture2d, Texture2d}, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction}, vertex::Attribute, BackfaceCullingMode, Depth, Display, DrawParameters, IndexBuffer, Program, Surface, Vertex, VertexBuffer, VertexFormat};



fn _load_texture(display: &Display, path: &str) -> SrgbTexture2d {
	let mut osstr = std::env::current_dir().unwrap().as_os_str().to_owned();
	osstr.push("\\textures\\");
	osstr.push(path);
	match std::fs::File::open(&osstr) {
		Ok(file) => {
			let img_buffer = image::load(std::io::BufReader::new(file), image::ImageFormat::Png).unwrap().to_rgba8();
			let dimensions = img_buffer.dimensions();
			let img = RawImage2d::from_raw_rgba_reversed(&img_buffer.into_raw(), dimensions);
			SrgbTexture2d::new(display, img).unwrap()
		}
		Err(e) => panic!("{} - Path: {}", e, osstr.to_str().unwrap())
	}
}



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


struct Object {
	vertex_buffer: VertexBuffer<Vec3>,
	index_buffer: IndexBuffer<u16>,
	indices: Box<[(u16, u16, u16)]>,
	edges: Box<[(u16, u16)]>,
	transform: Mat4,
	velocity: Vec3,
	angular_velocity: Vec3,
}

impl Object {
	fn new(display: &Display, vertices: &[Vec3], indices: &[(u16, u16, u16)]) -> Self {
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
}

struct ShadowMap {
	resolution: (u32, u32),
	size: (f32, f32),
	near_distance: f32,
	far_distance: f32,
	bias_factor: f32,
	transform: Mat4,
	texture: DepthTexture2d
}



static POST_VERTEX_BUFFER: [Vec2; 4] = [Vec2(-1.0, -1.0), Vec2(1.0, -1.0), Vec2(1.0, 1.0), Vec2(-1.0, 1.0)];
static POST_INDEX_BUFFER: [u16; 6] = [0, 1, 2, 0, 2, 3];

static POST_VERTEX_SHADER: &str = "#version 150
in vec2 position;
out vec2 screen_position;
void main() {
	screen_position = (position + 1.0) * 0.5;
	gl_Position = vec4(position, 0.0, 1.0);
}";

static DEFAULT_FRAG_SHADER: &str = "#version 150
in vec2 screen_position;
out vec4 color;
uniform sampler2D main_buffer;
void main() {
	color = texture(main_buffer, screen_position);
}";

static SHADOWMAP_VERTEX_SHADER: &str = "#version 150
in vec3 position;
uniform mat4 shadowmap_transform;
uniform mat4 model_transform;
void main() {
	gl_Position = shadowmap_transform * (model_transform * vec4(position, 1.0));
}";



fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(1024.0, 768.0));
	let cb = ContextBuilder::new();
	let display = Display::new(wb, cb, &event_loop).unwrap();
	let PhysicalSize { width, height } = display.gl_window().window().inner_size();
	
	let bayer16_texture = {
		let img_buffer = image::load_from_memory_with_format(include_bytes!("bayer16.png"), image::ImageFormat::Png).unwrap().to_rgba8();
		let dimensions = img_buffer.dimensions();
		Texture2d::new(&display, RawImage2d::from_raw_rgba_reversed(&img_buffer.into_raw(), dimensions)).unwrap()
	};
	
	
	let program = Program::from_source(&display, include_str!("shaders/main.vert"), include_str!("shaders/main.frag"), None).unwrap();
	let post_program = Program::from_source(&display, &POST_VERTEX_SHADER, include_str!("shaders/post_effects.frag"), None).unwrap();
	let post_program_none = Program::from_source(&display, &POST_VERTEX_SHADER, &DEFAULT_FRAG_SHADER, None).unwrap();
	let shadowmap_program = Program::from_source(&display, &SHADOWMAP_VERTEX_SHADER, "#version 150\nvoid main() {}", None).unwrap();
	let shadowmap_render_program = Program::from_source(&display, &POST_VERTEX_SHADER, include_str!("shaders/shadowmap_render.frag"), None).unwrap();
	
	let post_vertex_buffer = VertexBuffer::new(&display, &POST_VERTEX_BUFFER).unwrap();
	let post_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &POST_INDEX_BUFFER).unwrap();
	
	let mut main_buffer = SrgbTexture2d::empty(&display, width, height).unwrap();
	let mut normals_buffer = SrgbTexture2d::empty(&display, width, height).unwrap();
	let mut depth_buffer = DepthTexture2d::empty(&display, width, height).unwrap();
	
	let mut shadowmap = ShadowMap {
		resolution: (1024, 1024),
		size: (10.0, 10.0),
		near_distance: -10.0,
		far_distance: 10.0,
		bias_factor: 0.01,
		transform: Mat4::identity(),
		texture: DepthTexture2d::empty(&display, 4096, 4096).unwrap()
	};
	
	
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
	
	o1.transform = o1.transform.rotate_x(0.5).rotate_z(0.5).translate(0.0, 10.0, 0.0);
	
	
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
	
	
	let mut a = 0.3f32;
	let mut b = 0.0f32;
	let lspeed = 0.00025;
	
	let mut x = 4.0f32;
	let mut y = 6.0f32;
	let mut z = -11.0f32;
	let speed = 2.0;
	
	let g = 9.8;
	
	
	let mut dummy = 0.0f32;
	
	let do_post_process = true;
	let mut show_shadowmap = false;
	
	
	let mut previous_frame_time = std::time::SystemTime::now();
	let mut avg_frame_time = 0.0;
	
	
	let fov = 75.0;
	let f = 1.0 / f32::tan(fov * 0.5 * std::f32::consts::PI / 180.0);
	
	let z_far = 1000.0;
	let z_near = 0.1;
	
	
	
	
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
						a += (position.x as f32 - center_x as f32) * lspeed;
						b -= (position.y as f32 - center_y as f32) * lspeed;
						display.gl_window().window().set_cursor_position(PhysicalPosition::new(center_x, center_y)).unwrap();
					}
					
					previous_mouse_pos = position;
				}
				WindowEvent::Resized(PhysicalSize { width, height }) => {
					main_buffer = SrgbTexture2d::empty(&display, width, height).unwrap();
					normals_buffer = SrgbTexture2d::empty(&display, width, height).unwrap();
					depth_buffer = DepthTexture2d::empty(&display, width, height).unwrap();
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
				
				
				let asin = a.sin();
				let acos = a.cos();
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
					x += (mov.0*acos - mov.2*asin)*ds;
					y += mov.1*ds;
					z += (mov.2*acos + mov.0*asin)*ds;
				}
				
				
				if right { a += lspeed }
				if left { a -= lspeed }
				if up { b += lspeed }
				if down { b -= lspeed }
				
				
				
				
				if run {
					
					objects[1].velocity = Vec3(0.0, 0.0, 0.0);
					
					
					let new_transforms = (0..objects.len()).map(|i| {
						let new_position = objects[i].transform.get_position() + objects[i].velocity * dt;
						let mut new_transform = objects[i].transform;
						if objects[i].angular_velocity.length_squared() > 0.0 {
							new_transform = new_transform.rotate_axis(objects[i].angular_velocity.normalize(), objects[i].angular_velocity.length() * dt);
						}
						
						objects[i].velocity += Vec3(0.0, -g * dt, 0.0);
						
						new_transform.set_position(new_position)
					}).collect::<Vec<Mat4>>();
					
					
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
								
								
								// 1) find t
								let p0 = this_v - this_a;
								let g0 = this_b - this_a;
								let h0 = this_c - this_a;
								let dp = next_v - next_a - p0;
								let dg = next_b - next_a - g0;
								let dh = next_c - next_a - h0;
								
								/*
								p = p0 + dp * t
								g = g0 + dg * t
								h = h0 + dh * t
								
								g.cross(h).dot(p) == 0.0
								
								(g0 + dg * t).cross(h0 + dh * t).dot(p0 + dp * t) == 0.0
								
								(g0.cross(h0) + (dg.cross(h0) + g0.cross(dh))*t + dg.cross(dh)*t*t).dot(p0 + dp*t) == 0.0
								
								(
									g0.cross(h0).dot(p0) +
									((dg.cross(h0) + g0.cross(dh)).dot(p0) + g0.cross(h0).dot(dp)) *t +
									(dg.cross(dh).dot(p0) + (dg.cross(h0) + g0.cross(dh)).dot(dp)) *t*t +
									dg.cross(dh).dot(dp) *t*t*t
								) == 0.0
								*/
								
								let cubic_a = dg.cross(dh).dot(dp);
								let cubic_b = dg.cross(dh).dot(p0) + (dg.cross(h0) + g0.cross(dh)).dot(dp);
								let cubic_c = (dg.cross(h0) + g0.cross(dh)).dot(p0) + g0.cross(h0).dot(dp);
								let cubic_d = g0.cross(h0).dot(p0);
								
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
										match t >= 0.0 && t <= 1.0 {
											true => Some(t),
											false => None
										}
									} else {
										let dum1 = f32::acos(r / f32::sqrt(-q*q*q));
										let r13 = 2.0 * f32::sqrt(-q);
										let t1 = -b / 3.0 + r13 * f32::cos(dum1 / 3.0);
										let t2 = -b / 3.0 + r13 * f32::cos((dum1 + 2.0*std::f32::consts::PI) / 3.0);
										let t3 = -b / 3.0 + r13 * f32::cos((dum1 + 4.0*std::f32::consts::PI) / 3.0);
										
										match (t1 >= 0.0 && t1 <= 1.0, t2 >= 0.0 && t2 <= 1.0, t3 >= 0.0 && t3 <= 1.0, t1 < t2, t1 < t3, t2 < t3) {
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
										
										match (t1 >= 0.0 && t1 <= 1.0, t2 >= 0.0 && t2 <= 1.0, t1 < t2) {
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
									match t >= 0.0 && t <= 1.0 {
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
									
									if r >= 0.0 && r <= 1.0 && s >= 0.0 && s <= 1.0 && r + s <= 1.0 {
										run = false;
									}
								}
							}
						}
					}}
					
					for i in 0..objects.len() {
						objects[i].transform = new_transforms[i];
					}
				}
				
				
				
				
				
				
				
				let light_direction = Vec3(f32::cos(dummy), 2.0, f32::sin(dummy)).normalize();
				
				shadowmap.transform = Mat4::identity()
					.translate(0.0, 0.0, 0.0)
					.rotate_y(f32::atan2(light_direction.0, -light_direction.2))
					.rotate_x(f32::asin(-light_direction.1))
					.translate(0.0, 0.0, -(shadowmap.far_distance + shadowmap.near_distance))
					.scale_xyz(1.0 / shadowmap.size.0, 1.0 / shadowmap.size.1, 1.0 / (shadowmap.far_distance - shadowmap.near_distance))
					;
				
				let mut target = SimpleFrameBuffer::depth_only(&display, &shadowmap.texture).unwrap();
				target.clear_depth(1.0);
				for object in &objects {
					target.draw(&object.vertex_buffer, &object.index_buffer, &shadowmap_program, &uniform! {
						shadowmap_transform: shadowmap.transform.0,
						model_transform: object.transform.0
					}, &DrawParameters {
						depth: Depth {
							test: DepthTest::IfLess,
							write: true,
							.. Default::default()
						},
						backface_culling: BackfaceCullingMode::CullClockwise,
						.. Default::default()
					}).unwrap();
				}
				
				
				
				let mut target = MultiOutputFrameBuffer::with_depth_buffer(&display, [
					("color", &main_buffer),
					("normal_color", &normals_buffer)
				], &depth_buffer).unwrap();
				
				target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
				
				let (width, height) = target.get_dimensions();
				let aspect_ratio = height as f32 / width as f32;
				
				
				let camera_transform = Mat4::identity().translate(-x, -y, -z).rotate_y(a).rotate_x(b);
				
				for object in &objects {
					let uniforms = uniform! {
						camera_location: (x, y, z),
						camera_transform: camera_transform.0,
						model_transform: object.transform.0,
						perspective_matrix: [
							[-f * aspect_ratio, 0.0, 0.0, 0.0],
							[0.0, f, 0.0, 0.0],
							[0.0, 0.0, (z_far+z_near)/(z_far-z_near), 1.0],
							[0.0, 0.0, -(2.0*z_far*z_near)/(z_far-z_near), 0.0]
						],
						shadowmap_transform: shadowmap.transform.0,
						shadowmap_texture: Sampler(&shadowmap.texture, SamplerBehavior {
							minify_filter: MinifySamplerFilter::Linear,
							magnify_filter: MagnifySamplerFilter::Linear,
							depth_texture_comparison: Some(glium::uniforms::DepthTextureComparison::Greater),
							.. Default::default()
						}),
						shadowmap_resolution: (shadowmap.resolution.0 as f32, shadowmap.resolution.1 as f32),
						shadowmap_tolerance: shadowmap.bias_factor / (shadowmap.far_distance - shadowmap.near_distance),
						light_direction: (light_direction.0, light_direction.1, light_direction.2),
						dummy: dummy
					};
					
					target.draw(&object.vertex_buffer, &object.index_buffer, &program, &uniforms, &DrawParameters {
						depth: Depth {
							test: DepthTest::IfLess,
							write: true,
							.. Default::default()
						},
						backface_culling: BackfaceCullingMode::CullCounterClockwise,
						.. Default::default()
					}).unwrap();
				}
				
				
				let mut target = display.draw();
				target.clear_color(0.0, 0.0, 0.0, 1.0);
				target.draw(
					&post_vertex_buffer,
					&post_index_buffer,
					match (show_shadowmap, do_post_process) {
						(true, _) => &shadowmap_render_program,
						(false, true) => &post_program,
						(false, false) => &post_program_none
					},
					&uniform! {
						resolution: (width as f32, height as f32),
						step_num: 2i32,
						normals_outline_weight: 1.0f32,
						depth_outline_weight: 1.0f32,
						outline_color: (0.0f32, 0.0f32, 0.0f32, 1.0f32),
						z_far: z_far,
						z_near: z_near,
						bayer16_texture: Sampler(&bayer16_texture, SamplerBehavior {
							minify_filter: MinifySamplerFilter::Nearest,
							magnify_filter: MagnifySamplerFilter::Nearest,
							wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
							.. Default::default()
						}),
						main_buffer: &main_buffer,
						normals_buffer: &normals_buffer,
						depth_buffer: &depth_buffer,
						shadowmap_texture: &shadowmap.texture
					},
					&DrawParameters::default()
				).unwrap();
				target.finish().unwrap();
				
				
			}
			_ => ()
		}
	});
}

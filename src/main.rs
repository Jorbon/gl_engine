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
	transform: Mat4,
}

impl Object {
	fn new(display: &Display, vertices: &[Vec3], indices: &[(u16, u16, u16)]) -> Self {
		let vertex_buffer = VertexBuffer::new(display, &vertices).unwrap(); // might switch to dynamic later
		let index_buffer = IndexBuffer::new(display, PrimitiveType::TrianglesList, unsafe {
			core::slice::from_raw_parts(indices.as_ptr() as *const u16, indices.len() * 3)
		}).unwrap();
		
		Self {
			vertex_buffer,
			index_buffer,
			transform: Mat4::identity()
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
	
	o1.transform = o1.transform.translate(0.0, 5.0, 0.0);
	
	
	let floor = Object::new(&display, &[
		Vec3(-10.0, 0.0, -10.0),
		Vec3(-10.0, 0.0,  10.0),
		Vec3( 10.0, 0.0, -10.0),
		Vec3( 10.0, 0.0,  10.0),
	], &[
		(0, 2, 3),
		(0, 3, 1)
	]);
	
	
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
	
	let mut capture = false;
	let mut previous_mouse_pos = PhysicalPosition::<f64>::new(0.0, 0.0);
	
	
	let mut a = 0.0f32;
	let mut b = 0.0f32;
	let lspeed = 0.00025;
	
	let mut x = 0.0f32;
	let mut y = 5.0f32;
	let mut z = -4.0f32;
	let speed = 2.0;
	
	let mut dummy = 0.0f32;
	
	let mut do_post_process = false;
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
							
							VirtualKeyCode::P => if state { do_post_process = !do_post_process; }
							VirtualKeyCode::M => if state { show_shadowmap = !show_shadowmap; }
							VirtualKeyCode::Comma => if state { dummy -= 0.1; }
							VirtualKeyCode::Period => if state { dummy += 0.1; }
							VirtualKeyCode::Slash => if state { dummy = 0.0; }
							
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
				let now = std::time::SystemTime::now();
				let dt = now.duration_since(previous_frame_time).unwrap().as_micros() as f32 * 1.0e-6;
				previous_frame_time = now;
				
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
				
				
				
				objects[0].transform.set_position(Vec3(0.0, dummy, 0.0));
				
				
				
				let light_direction = Vec3(f32::cos(dummy), 2.0, f32::sin(dummy)).normalize();
				
				shadowmap.transform = Mat4::identity()
					.translate(0.0, -5.0, 0.0)
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

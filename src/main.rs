#[macro_use]
extern crate glium;

mod structs;
mod matfns;
mod teapot;

use matfns::{Matrix, normalize_vec3};

use glium::{Surface, glutin::{event::{Event, WindowEvent, ElementState, VirtualKeyCode}, event_loop::ControlFlow, dpi::{PhysicalPosition, PhysicalSize}}};
use structs::Vec2;



fn _load_texture(display: &glium::Display, path: &str) -> glium::texture::SrgbTexture2d {
	let file = std::fs::File::open(path).unwrap();
	let img_buffer = image::load(std::io::BufReader::new(file), image::ImageFormat::Png).unwrap().to_rgba8();
	let dimensions = img_buffer.dimensions();
	let img = glium::texture::RawImage2d::from_raw_rgba_reversed(&img_buffer.into_raw(), dimensions);
	glium::texture::SrgbTexture2d::new(display, img).unwrap()
}

fn main() {
	let event_loop = glium::glutin::event_loop::EventLoop::new();
	
	let wb = glium::glutin::window::WindowBuilder::new()
		.with_inner_size(glium::glutin::dpi::LogicalSize::new(1024.0, 768.0))
		.with_title("Hello world");
	
	let cb = glium::glutin::ContextBuilder::new();
	let display = glium::Display::new(wb, cb, &event_loop).unwrap();
	let glium::glutin::dpi::PhysicalSize { width, height } = display.gl_window().window().inner_size();
	
	let vertex_buffer = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
	let normal_buffer = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
	let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();
	
	let teapot_matrix = Matrix::new().scale(0.01);
	
	
	let post_vertex_buffer = glium::VertexBuffer::new(&display, &[
		Vec2 { pos: [-1.0, -1.0] },
		Vec2 { pos: [ 1.0, -1.0] },
		Vec2 { pos: [ 1.0,  1.0] },
		Vec2 { pos: [-1.0,  1.0] }
	]).unwrap();
	let post_index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &[0u16, 1, 2, 0, 2, 3]).unwrap();
	
	let mut main_texture = glium::texture::SrgbTexture2d::empty(&display, width, height).unwrap();
	let mut aux_texture = glium::texture::SrgbTexture2d::empty(&display, width, height).unwrap();
	let mut depth_texture = glium::texture::depth_texture2d::DepthTexture2d::empty(&display, width, height).unwrap();
	
	let program = glium::Program::from_source(&display, include_str!("shaders/main_vert.glsl"), include_str!("shaders/main_frag.glsl"), None).unwrap();
	let post_program = glium::Program::from_source(&display, include_str!("shaders/post_vert.glsl"), include_str!("shaders/post_effects_frag.glsl"), None).unwrap();
	let _post_program = glium::Program::from_source(&display, include_str!("shaders/post_vert.glsl"), include_str!("shaders/post_frag.glsl"), None).unwrap();
	let shadowmap_program = glium::Program::from_source(&display, include_str!("shaders/shadowmap_vert.glsl"), "#version 150\nvoid main() {}", None).unwrap();
	
	
	let params = glium::DrawParameters {
		depth: glium::Depth {
			test: glium::draw_parameters::DepthTest::IfLess,
			write: true,
			.. Default::default()
		},
		backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
		.. Default::default()
	};
	
	
	let shadowmap_resolution = 4096;
	let shadowmap_size = 4.0f32;
	let shadowmap_range = 10.0f32;
	let shadowmap_tolerance = 0.001f32;
	let shadowmap_texture = glium::texture::depth_texture2d::DepthTexture2d::empty(&display, shadowmap_resolution, shadowmap_resolution).unwrap();
	
	
	
	
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
	
	
	let mut a = std::f32::consts::PI * 0.5;
	let mut b = 0.0f32;
	let lspeed = 0.00025;
	
	let mut x = 2.0f32;
	let mut y = 0.0f32;
	let mut z = 0.0f32;
	let speed = 2.0;
	
	let fov = 75.0f32;
	let f = 1.0 / (fov * 0.5 * std::f32::consts::PI / 180.0).tan();
	
	let zfar = 1024.0;
	let znear = 0.1;
	
	
	let mut previous_frame_time = std::time::SystemTime::now();
	let mut avg_frame_time = 0.0;
	
	
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
							
							VirtualKeyCode::Escape => if state && capture {
								capture = false;
								display.gl_window().window().set_cursor_grab(glium::glutin::window::CursorGrabMode::None).unwrap();
								display.gl_window().window().set_cursor_visible(true);
							}
							_ => ()
						}
						
					}
				}
				WindowEvent::MouseInput { device_id: _, state, button: _, .. } => {
					if let ElementState::Pressed = state {
						capture = true;
						display.gl_window().window().set_cursor_grab(glium::glutin::window::CursorGrabMode::Confined).unwrap();
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
					main_texture = glium::texture::SrgbTexture2d::empty(&display, width, height).unwrap();
					aux_texture = glium::texture::SrgbTexture2d::empty(&display, width, height).unwrap();
					depth_texture = glium::texture::depth_texture2d::DepthTexture2d::empty(&display, width, height).unwrap();
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
				let mut mov = [0.0, 0.0, 0.0];
				if wkey { mov[2] += 1.0 }
				if skey { mov[2] -= 1.0 }
				if akey { mov[0] += 1.0 }
				if dkey { mov[0] -= 1.0 }
				if space { mov[1] += 1.0 }
				if shift { mov[1] -= 1.0 }
				if mov != [0.0, 0.0, 0.0] {
					mov = normalize_vec3(mov);
					let ds = dt * speed;
					x += (mov[0]*acos - mov[2]*asin)*ds;
					y += mov[1]*ds;
					z += (mov[2]*acos + mov[0]*asin)*ds;
				}
				
				
				if right { a += lspeed }
				if left { a -= lspeed }
				if up { b += lspeed }
				if down { b -= lspeed }
				
				
				let light = normalize_vec3([z, y, -x]);
				
				
				let shadowmap_matrix = Matrix::new()
					.scale(1.0 / shadowmap_size)
					.rotate_y(f32::atan2(light[0], -light[2]))
					.rotate_x(f32::asin(-light[1]))
					.scale_xyz(1.0, 1.0, 0.5 / shadowmap_range);
				
				let mut target = glium::framebuffer::SimpleFrameBuffer::depth_only(&display, &shadowmap_texture).unwrap();
				target.clear_depth(1.0);
				target.draw(&vertex_buffer, &index_buffer, &shadowmap_program, &uniform! {
					matrix: shadowmap_matrix.m,
					model_matrix: teapot_matrix.m
				}, &glium::DrawParameters {
					depth: glium::Depth {
						test: glium::draw_parameters::DepthTest::IfLess,
						write: true,
						.. Default::default()
					},
					backface_culling: glium::BackfaceCullingMode::CullClockwise,
					.. Default::default()
				}).unwrap();
				
				
				let mut target = glium::framebuffer::MultiOutputFrameBuffer::with_depth_buffer(&display, [
					("main_color", &main_texture),
					("aux_color", &aux_texture)
				], &depth_texture).unwrap();
				
				target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
				
				let (width, height) = target.get_dimensions();
				let aspect_ratio = height as f32 / width as f32;
				
				
				let cam_matrix = Matrix::new().translate(-x, -y, -z).rotate_y(a).rotate_x(b);
				
				let uniforms = uniform! {
					matrix: cam_matrix.m,
					model_matrix: teapot_matrix.m,
					perspective: [
						[-f * aspect_ratio, 0.0, 0.0, 0.0],
						[0.0, f, 0.0, 0.0],
						[0.0, 0.0, (zfar+znear)/(zfar-znear), 1.0],
						[0.0, 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0]
					],
					sm_matrix: shadowmap_matrix.m,
					sm: glium::uniforms::Sampler(&shadowmap_texture, glium::uniforms::SamplerBehavior {
						minify_filter: glium::uniforms::MinifySamplerFilter::Nearest,
						magnify_filter: glium::uniforms::MagnifySamplerFilter::Nearest,
						.. Default::default()
					}),
					sm_u: 1.0/(shadowmap_resolution as f32),
					sm_tol: 0.5 * shadowmap_tolerance / shadowmap_range,
					cam: [x, y, z],
					light: light
				};
				
				
				
				target.draw((&vertex_buffer, &normal_buffer), &index_buffer, &program, &uniforms, &params).unwrap();
				
				
				
				let mut target = display.draw();
				target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
				target.draw(&post_vertex_buffer, &post_index_buffer, &post_program, &uniform! {
					u: [1.0/(width as f32), 1.0/(height as f32)],
					step_num: 3f32,
					main_tex: &main_texture,
					aux_tex: &aux_texture,
					depth_tex: &depth_texture,
				}, &glium::DrawParameters::default()).unwrap();
				target.finish().unwrap();
				
				
			}
			_ => ()
		}
	});
}

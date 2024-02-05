use glium::{framebuffer::{MultiOutputFrameBuffer, SimpleFrameBuffer}, index::PrimitiveType, texture::{DepthTexture2d, RawImage2d, SrgbTexture2d}, uniform, uniforms::{MagnifySamplerFilter, MinifySamplerFilter, Sampler, SamplerBehavior, SamplerWrapFunction}, BackfaceCullingMode, Depth, DepthTest, Display, DrawParameters, IndexBuffer, Program, Surface, Texture2d, VertexBuffer};

use crate::{math_structs::{Mat4, Vec2, Vec3}, object::Object, Camera};


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



pub struct ShadowMap {
	pub resolution: (u32, u32),
	pub size: (f32, f32),
	pub near_distance: f32,
	pub far_distance: f32,
	pub bias_factor: f32,
	pub transform: Mat4,
	pub texture: DepthTexture2d
}

pub struct Renderer {
	pub main_program: Program,
	pub post_program: Program,
	pub post_program_none: Program,
	pub shadowmap_program: Program,
	pub shadowmap_render_program: Program,
	pub post_vertex_buffer: VertexBuffer<Vec2>,
	pub post_index_buffer: IndexBuffer<u16>,
	pub main_buffer: SrgbTexture2d,
	pub normals_buffer: SrgbTexture2d,
	pub depth_buffer: DepthTexture2d,
	pub shadowmap: ShadowMap,
	pub bayer16_texture: Texture2d,
	pub fov: f32,
	pub f: f32,
	pub z_far: f32,
	pub z_near: f32
}


impl ShadowMap {
	pub fn set_up_transform(&mut self, light_direction: Vec3) {
		self.transform = Mat4::identity()
			.translate(Vec3(0.0, 0.0, 0.0))
			.rotate_y(f32::atan2(light_direction.0, -light_direction.2))
			.rotate_x(f32::asin(-light_direction.1))
			.translate(Vec3(0.0, 0.0, -(self.far_distance + self.near_distance)))
			.scale_xyz(1.0 / self.size.0, 1.0 / self.size.1, 1.0 / (self.far_distance - self.near_distance))
			;
	}
}


impl Renderer {
	pub fn new(display: &Display, width: u32, height: u32, fov: f32, z_near: f32, z_far: f32) -> Self {
		Self {
			main_program: Program::from_source(display, include_str!("shaders/main.vert"), include_str!("shaders/main.frag"), None).unwrap(),
			post_program: Program::from_source(display, &POST_VERTEX_SHADER, include_str!("shaders/post_effects.frag"), None).unwrap(),
			post_program_none: Program::from_source(display, &POST_VERTEX_SHADER, &DEFAULT_FRAG_SHADER, None).unwrap(),
			shadowmap_program: Program::from_source(display, &SHADOWMAP_VERTEX_SHADER, "#version 150\nvoid main() {}", None).unwrap(),
			shadowmap_render_program: Program::from_source(display, &POST_VERTEX_SHADER, include_str!("shaders/shadowmap_render.frag"), None).unwrap(),
			post_vertex_buffer: VertexBuffer::new(display, &POST_VERTEX_BUFFER).unwrap(),
			post_index_buffer: IndexBuffer::new(display, PrimitiveType::TrianglesList, &POST_INDEX_BUFFER).unwrap(),
			main_buffer: SrgbTexture2d::empty(display, width, height).unwrap(),
			normals_buffer: SrgbTexture2d::empty(display, width, height).unwrap(),
			depth_buffer: DepthTexture2d::empty(display, width, height).unwrap(),
			shadowmap: ShadowMap {
				resolution: (1024, 1024),
				size: (10.0, 10.0),
				near_distance: -10.0,
				far_distance: 10.0,
				bias_factor: 0.01,
				transform: Mat4::identity(),
				texture: DepthTexture2d::empty(display, 4096, 4096).unwrap()
			},
			bayer16_texture: {
				let img_buffer = image::load_from_memory_with_format(include_bytes!("bayer16.png"), image::ImageFormat::Png).unwrap().to_rgba8();
				let dimensions = img_buffer.dimensions();
				Texture2d::new(display, RawImage2d::from_raw_rgba_reversed(&img_buffer.into_raw(), dimensions)).unwrap()
			},
			fov,
			f: 1.0 / f32::tan(fov * 0.5 * std::f32::consts::PI / 180.0),
			z_far,
			z_near
		}
	}
	
	pub fn resize(&mut self, display: &Display, width: u32, height: u32) {
		self.main_buffer = SrgbTexture2d::empty(display, width, height).unwrap();
		self.normals_buffer = SrgbTexture2d::empty(display, width, height).unwrap();
		self.depth_buffer = DepthTexture2d::empty(display, width, height).unwrap();
	}
	
	
	pub fn render(&mut self, display: &Display, camera: &Camera, objects: &[Object], vertex_buffers: &[VertexBuffer<Vec3>], index_buffers: &[IndexBuffer<u16>], do_post_process: bool, show_shadowmap: bool, dummy: f32) {
		
		let light_direction = Vec3(f32::cos(dummy), 2.0, f32::sin(dummy)).normalize();
		self.shadowmap.set_up_transform(light_direction);
		
		let mut target = SimpleFrameBuffer::depth_only(display, &self.shadowmap.texture).unwrap();
		target.clear_depth(1.0);
		for i in 0..objects.len() {
			target.draw(&vertex_buffers[i], &index_buffers[i], &self.shadowmap_program, &uniform! {
				shadowmap_transform: self.shadowmap.transform.0,
				model_transform: objects[i].transform.0
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
		
		
		
		let mut target = MultiOutputFrameBuffer::with_depth_buffer(display, [
			("color", &self.main_buffer),
			("normal_color", &self.normals_buffer)
		], &self.depth_buffer).unwrap();
		
		target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
		
		let (width, height) = target.get_dimensions();
		let aspect_ratio = height as f32 / width as f32;
		
		
		
		for i in 0..objects.len() {
			let uniforms = uniform! {
				camera_location: (camera.position.0, camera.position.1, camera.position.2),
				camera_transform: camera.get_transform().0,
				model_transform: objects[i].transform.0,
				perspective_matrix: [
					[-self.f * aspect_ratio, 0.0, 0.0, 0.0],
					[0.0, self.f, 0.0, 0.0],
					[0.0, 0.0, (self.z_far+self.z_near)/(self.z_far-self.z_near), 1.0],
					[0.0, 0.0, -(2.0*self.z_far*self.z_near)/(self.z_far-self.z_near), 0.0]
				],
				shadowmap_transform: self.shadowmap.transform.0,
				shadowmap_texture: Sampler(&self.shadowmap.texture, SamplerBehavior {
					minify_filter: MinifySamplerFilter::Linear,
					magnify_filter: MagnifySamplerFilter::Linear,
					depth_texture_comparison: Some(glium::uniforms::DepthTextureComparison::Greater),
					.. Default::default()
				}),
				shadowmap_resolution: (self.shadowmap.resolution.0 as f32, self.shadowmap.resolution.1 as f32),
				shadowmap_tolerance: self.shadowmap.bias_factor / (self.shadowmap.far_distance - self.shadowmap.near_distance),
				light_direction: (light_direction.0, light_direction.1, light_direction.2),
				dummy: dummy
			};
			
			target.draw(&vertex_buffers[i], &index_buffers[i], &self.main_program, &uniforms, &DrawParameters {
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
			&self.post_vertex_buffer,
			&self.post_index_buffer,
			match (show_shadowmap, do_post_process) {
				(true, _) => &self.shadowmap_render_program,
				(false, true) => &self.post_program,
				(false, false) => &self.post_program_none
			},
			&uniform! {
				resolution: (width as f32, height as f32),
				step_num: 2i32,
				normals_outline_weight: 1.0f32,
				depth_outline_weight: 1.0f32,
				outline_color: (0.0f32, 0.0f32, 0.0f32, 1.0f32),
				z_far: self.z_far,
				z_near: self.z_near,
				bayer16_texture: Sampler(&self.bayer16_texture, SamplerBehavior {
					minify_filter: MinifySamplerFilter::Nearest,
					magnify_filter: MagnifySamplerFilter::Nearest,
					wrap_function: (SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat, SamplerWrapFunction::Repeat),
					.. Default::default()
				}),
				main_buffer: &self.main_buffer,
				normals_buffer: &self.normals_buffer,
				depth_buffer: &self.depth_buffer,
				shadowmap_texture: &self.shadowmap.texture
			},
			&DrawParameters::default()
		).unwrap();
		target.finish().unwrap();
	}
}







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






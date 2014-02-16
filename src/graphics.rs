extern crate sdl2;

use sdl2::rect;
use sdl2::surface;
use sdl2::render;
use sdl2::mouse;
use sdl2::video;


use std::rc::Rc;
use std::hashmap::HashMap;

static SCREEN_WIDTH: 	int 	 	= 640;
static SCREEN_HEIGHT: 	int 	 	= 480;

/// Acts as a buffer to the underlying display
pub struct Graphics {
	priv screen: ~render::Renderer,
	sprite_cache: HashMap<~str, Rc<~render::Texture>>,
}

impl Graphics {
	/// Prepare the display for rendering
	pub fn new() -> Graphics {
		let current_mode = ~video::Window::new(
			"rust-story v0.0",							// title
			video::PosCentered, video::PosCentered,		// position (x,y)
			SCREEN_WIDTH, SCREEN_HEIGHT, 				// width, height
			[video::InputGrabbed]
		);

		let render_context = render::Renderer::from_window(
			current_mode.unwrap(),
			render::DriverAuto,
			[render::Software]
		);

		let graphics: Graphics;
		match render_context {
			Ok(renderer) => {
				graphics = Graphics{
					screen: renderer, 
					sprite_cache: HashMap::<~str, Rc<~render::Texture>>::new()
				};
			}
			Err(_) => {fail!("Could not create a renderer using SDL2.");}
		};
		
		mouse::show_cursor(false);
		return graphics;
	}

	/// Loads a bitmap which resides at `file_path` and returns a handle
	/// This handle can safely be used in any of the graphics subsystem's rendering
	/// contexts.
	pub fn load_image(&mut self, file_path: ~str) -> Rc<~render::Texture> {
		// Retrieve a handle or generate a new one if it exists already.
		let borrowed_display = &self.screen;	
		let sprite_handle = self.sprite_cache.find_or_insert_with(file_path, |key| {
			// Load sprite
			let sprite_path = Path::new((*key).clone());
			let sprite_window = surface::Surface::from_bmp(&sprite_path);

			// Store sprite
			match sprite_window {
				Ok(sprite) => {
					// wrap surface in texture and store it
					let sprite_texture = borrowed_display.create_texture_from_surface(sprite);
					match sprite_texture {
						Ok(texture) => {
							Rc::new(texture)
						}
						Err(msg) => {fail!("sprite could not be rendered: {}", msg)}
					}
				},
				Err(msg) => {fail!("sprite could not be loaded: {}", msg)}
			}
		});

		sprite_handle.clone()
		
	}

	pub fn remove_image(&mut self, file_path: ~str) {
		self.sprite_cache.remove(&file_path);
	}
	

	pub fn blit_surface(
		&self, 
		src: &render::Texture, 
		src_rect: &rect::Rect, 
		dest_rect: &rect::Rect
	) {
		//let src_surface = self.sprite_cache.get(&src.id);
		self.screen.copy(src, Some(*src_rect), Some(*dest_rect));
	}

	pub fn switch_buffers(&self) -> bool {
		self.screen.present();
		true
	}

	pub fn clear_buffer(&self) {
		self.screen.clear();
	}
}

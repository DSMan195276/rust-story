#[crate_id="rust-story#0.0.1"];

extern crate sdl2;
extern crate collections;
extern crate sync;

pub mod game;

pub fn main() {
	let mut story = ::game::Game::new();
	story.start();
}

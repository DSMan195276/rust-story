use game::units;

pub struct Info {
	collided: bool,
	row: units::Tile, 
	col: units::Tile 
}

pub struct Rectangle {
	x: units::Game, 
	y: units::Game,
	width: 	units::Game, 
	height: units::Game
}

impl Rectangle {
	pub fn new(width: units::Game, height: units::Game) -> Rectangle {
		Rectangle {
			x: units::Game(0.0), y: units::Game(0.0),
			width: width, height: height
		}
	}

	pub fn left(&self) 		-> units::Game { self.x }
	pub fn right(&self) 	-> units::Game { self.x + self.width }
	pub fn top(&self) 		-> units::Game { self.y }
	pub fn bottom(&self) 	-> units::Game { self.y + self.height}

	pub fn width(&self) 	-> units::Game { self.width }
	pub fn height(&self) 	-> units::Game { self.height }

	pub fn collides_with(&self, other: &Rectangle) -> bool {
		self.right() >= other.left() &&
		self.left() <= other.right() &&
		self.top() <= other.bottom() &&
		self.bottom() >= other.top()
	}
}

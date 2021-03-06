use std::f64;

static TILE_SIZE: i32 	= 32;
static SCALE: f64	= 1.0; // This is a divider of TILE_SIZE, 1.0 for 32, 2.0 for 16, etc.

pub trait AsGame 	{fn to_game(&self) -> Game;}
pub trait AsTile 	{fn to_tile(&self) -> Tile;}
pub trait AsPixel 	{fn to_pixel(&self) -> Pixel;}

/// A `Game` unit represents a density-independent distance in pixels.
/// Converting a `Game` to pixels will round it to the nearest coordinate,
/// scaled based on the desired tile size & resolution.
#[deriving(Eq,Ord)]
pub struct Game(f64);

impl AsGame for Game {
	#[inline(always)]
	fn to_game(&self) -> Game { *self }
}

impl AsTile for Game {
	#[inline(always)]	
	fn to_tile(&self) -> Tile {
		let Game(a) = *self;	
		Tile((a / TILE_SIZE as f64) as uint)
	}
}

impl AsPixel for Game {
	#[inline(always)]
	fn to_pixel(&self) -> Pixel { let Game(a) = *self; Pixel(f64::round(a / SCALE) as i32) }
}

// Allow `+` operator for anything which can be converted `#to_game()` 
impl<T: AsGame>  Add<T, Game> for Game {
	#[inline(always)]	
	fn add(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a + b)
	}
}

// Allow `-` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Sub<T, Game> for Game {
	#[inline(always)]
	fn sub(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a - b)
	}
}

// Allow `*` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Mul<T, Game> for Game {
	#[inline(always)]
	fn mul(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a * b)
	}
}

// Allow `/` operator for anything which can be converted `#to_game()`
impl <T: AsGame> Div<T, Game> for Game {
	#[inline(always)]
	fn div(&self, rhs: &T) -> Game {
		let (Game(a), Game(b)) = (*self, rhs.to_game());
		Game(a / b)
	}
}

/// A `Pixel` represents an absolute coordinate on a surface.
#[deriving(Eq,Ord)]
pub struct Pixel(i32);

impl AsPixel for Pixel {
	#[inline(always)]
	fn to_pixel(&self) -> Pixel { *self }
}

// Allow `+` operator for anything which can be converted `#to_pixel()`
impl<T: AsPixel> Add<T, Pixel> for Pixel {
	#[inline(always)]
	fn add(&self, rhs: &T) -> Pixel {
		let (Pixel(a), Pixel(b)) = (*self, rhs.to_pixel());
		Pixel(a + b)
	}
}

/// A `Tile` represents a single square drawn in the game's
/// _base tile-size_ (32 pixels).
///
/// This may ultimately be scaled if converted to `Games` or `Pixels`
#[deriving(Eq,Ord)]
pub struct Tile(uint);

impl AsGame for Tile {
	#[inline(always)]	
	fn to_game(&self) -> Game {
		let Tile(a) = *self;	
		Game((a * (TILE_SIZE as uint)) as f64)
	}
}

impl AsTile for Tile {
	#[inline(always)]
	fn to_tile(&self) -> Tile { *self }
}

impl AsPixel for Tile {
	#[inline(always)]	
	fn to_pixel(&self) -> Pixel { self.to_game().to_pixel() }
}

// Allow `+` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Add<T, Tile> for Tile {
	#[inline(always)]	
	fn add(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a + b)
	}
}

// Allow `-` operator for anything which can be converted to `#to_tile()`
impl<T: AsTile> Sub<T, Tile> for Tile {
	#[inline(always)]	
	fn sub(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a - b)
	}
}

// Allow `*` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Mul<T, Tile> for Tile {
	#[inline(always)]	
	fn mul(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a * b)
	}
}

// Allow `/` operator for anything which can be converted `#to_tile()`
impl<T: AsTile> Div<T, Tile> for Tile {
	#[inline(always)]
	fn div(&self, rhs: &T) -> Tile {
		let (Tile(a), Tile(b)) = (*self, rhs.to_tile());
		Tile(a / b)
	}
}

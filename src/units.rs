pub static TILE_SIZE: i32 = 32;

/// Milliseconds expressed as a large positive integer
/// This will be used at module boundaries in place of raw types.
#[deriving(Ord,Eq)]
pub struct Millis(uint);

impl Add<Millis,Millis> for Millis {
	/// The `uint`s inside LHS & RHS will be added together and wrapped 
	/// inside a new `Millis()`
	fn add(&self, rhs: &Millis) -> Millis {
		let Millis(a) = *self;
		let Millis(b) = *rhs;

		Millis(a+b)
	}	
}

impl Mul<f64, f64> for Millis {
	/// The `uint` inside LHS will be cast to f64.
	/// Multiplication will then proceed as normal, returning
	/// an f64 as a result.
	fn mul(&self, rhs: &f64) -> f64 {
		let Millis(a) = *self;

		(*rhs) * (a as f64)
	}
}

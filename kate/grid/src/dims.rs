/// The dimensions of a grid
#[derive(Debug, Clone)]
pub struct Dimensions {
	width: usize,
	height: usize,
}

/// The ways a set of dimensions can be extended
#[derive(Debug, Clone)]
pub struct Extension {
	/// This means extending the height of the grid by some factor.
	/// `2` would mean doubling the grid upwards, increasing the height by a factor of
	/// 2 and multiplying the number of rows by 2
	height_factor: u16,
	/// This means extending the width of the grid by some factor.
	/// `2` would mean doubling the grid sideways, increasing the width by a factor of
	/// 2 and multiplying the number of columns by 2
	width_factor: u16,
}

impl Dimensions {
	pub const fn new(width: usize, height: usize) -> Self {
		Dimensions { width, height }
	}

	pub fn width(&self) -> usize {
		self.width
	}

	pub fn height(&self) -> usize {
		self.width
	}

	pub fn size(&self) -> u32 {
		self.width as u32 * self.height as u32
	}

	pub fn extend(&self, e: Extension) -> Self {
		Self {
			width: e.width_factor as usize * self.width,
			height: e.height_factor as usize * self.height,
		}
	}
}

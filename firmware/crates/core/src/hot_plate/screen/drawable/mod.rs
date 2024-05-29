use micromath::vector::U16x2;

use crate::utils::measurement::color::ColorRGB565;

mod binary_image;
mod line;
mod plot;
mod point;
pub mod special;
mod text;
mod triangle;

pub use binary_image::*;
pub use line::*;
pub use plot::*;
pub use point::*;
pub use text::*;
pub use triangle::*;

// The thickness is in pixels
pub type Thickness = u16;

pub trait Drawable {
	fn size(&self) -> U16x2;
	fn draw(&self, draw_fn: &mut impl FnMut(Pixels));
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Axis {
	#[default]
	Horizontal,
	Vertical,
}

#[derive(Clone)]
pub struct Pixels {
	pub offset_position: U16x2,
	pub repetitions_count: u16,
	pub repetitions_direction: Axis,
	pub color: ColorRGB565,
}

impl Default for Pixels {
	fn default() -> Self {
		Self {
			offset_position: U16x2 { x: 0, y: 0 },
			repetitions_count: 1,
			repetitions_direction: Default::default(),
			color: Default::default(),
		}
	}
}

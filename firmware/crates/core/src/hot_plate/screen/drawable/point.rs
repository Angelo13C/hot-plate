use micromath::vector::{U16x2, Vector2d};

use super::{Drawable, Pixels};

pub struct Point;

impl Drawable for Point {
	fn size(&self) -> Vector2d<u16> {
		U16x2 { x: 1, y: 1 }
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		(draw_fn)(Pixels::default())
	}
}

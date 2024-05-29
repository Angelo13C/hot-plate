use micromath::vector::U16x2;

use crate::hot_plate::screen::drawable::{Axis, Drawable, Pixels};

pub struct Flipped<D: Drawable> {
	pub draw: D,
	pub axis: Axis,
}

impl<D: Drawable> Drawable for Flipped<D> {
	fn size(&self) -> U16x2 {
		self.draw.size()
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		self.draw.draw(&mut |mut pixels| {
			if self.axis == Axis::Horizontal {
				pixels.offset_position.x = self.size().x - 1 - pixels.offset_position.x
			}
			if self.axis == Axis::Vertical {
				pixels.offset_position.y = self.size().y - 1 - pixels.offset_position.y
			}
			(draw_fn)(pixels)
		});
	}
}

use micromath::vector::U16x2;

use crate::hot_plate::screen::drawable::{Drawable, Pixels};

pub struct Scale<D: Drawable> {
	draw: D,
	scale_factor: u16,
}

impl<D: Drawable> Drawable for Scale<D> {
	fn size(&self) -> micromath::vector::U16x2 {
		self.draw.size() * self.scale_factor as u16
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		self.draw.draw(&mut |mut pixels| {
			pixels.offset_position *= self.scale_factor;
			for y in 0..self.scale_factor {
				for x in 0..self.scale_factor {
					let mut copied_pixels = pixels.clone();
					copied_pixels.offset_position += U16x2 { x, y };
					(draw_fn)(copied_pixels)
				}
			}
		});
	}
}

use micromath::vector::U16x2;

use crate::{
	hot_plate::screen::drawable::{Drawable, Pixels},
	utils::measurement::color::ColorRGB565,
};

pub struct Colored<D: Drawable> {
	draw: D,
	color: ColorRGB565,
}

impl<D: Drawable> Drawable for Colored<D> {
	fn size(&self) -> U16x2 {
		self.draw.size()
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		self.draw.draw(&mut |mut pixels| {
			pixels.color = self.color;
			(draw_fn)(pixels)
		});
	}
}

use micromath::vector::Vector2d;

use super::{Axis, Drawable, Pixels, Thickness};

pub struct HorizontalLine {
	pub length: u16,
	pub thickness: Thickness,
}

impl Drawable for HorizontalLine {
	fn size(&self) -> micromath::vector::U16x2 {
		Vector2d {
			x: self.length,
			y: self.thickness,
		}
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		for y in 0..self.thickness {
			(draw_fn)(Pixels {
				offset_position: Vector2d { x: 0, y },
				repetitions_count: self.length,
				repetitions_direction: Axis::Horizontal,
				..Default::default()
			});
		}
	}
}

pub struct VerticalLine {
	length: u16,
	thickness: Thickness,
}

impl Drawable for VerticalLine {
	fn size(&self) -> micromath::vector::U16x2 {
		Vector2d {
			x: self.thickness,
			y: self.length,
		}
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		for x in 0..self.thickness {
			(draw_fn)(Pixels {
				offset_position: Vector2d { x, y: 0 },
				repetitions_count: self.length,
				repetitions_direction: Axis::Vertical,
				..Default::default()
			});
		}
	}
}

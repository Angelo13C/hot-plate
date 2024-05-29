use micromath::vector::U16x2;

use super::{Axis, Drawable, Pixels};

pub struct Triangle {
	pub size: u16,
}

impl Drawable for Triangle {
	fn size(&self) -> U16x2 {
		U16x2 {
			x: self.size,
			y: self.size,
		}
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		for x in 0..self.size {
			(draw_fn)(Pixels {
				offset_position: U16x2 { x, y: x },
				repetitions_count: self.size - 2 * x,
				repetitions_direction: Axis::Vertical,
				..Default::default()
			})
		}
	}
}

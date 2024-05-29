use core::{ops::Index, slice::SliceIndex};

use micromath::vector::Vector2d;

use super::{Axis, Drawable, Pixels, Thickness};

pub struct Plot<const N: usize> {
	pub points: [u16; N],
	pub thickness: Thickness,
}

impl<const N: usize> Plot<N> {
	pub fn get<I>(&self, index: I) -> Option<u16>
	where
		I: SliceIndex<[u16], Output = u16>,
	{
		self.points.get(index).copied()
	}
}

impl<const N: usize> Drawable for Plot<N> {
	fn size(&self) -> micromath::vector::U16x2 {
		Vector2d {
			x: N as u16,
			y: self.points.iter().max().cloned().unwrap_or(0),
		}
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		for (x, &y) in self.points.iter().enumerate() {
			(draw_fn)(Pixels {
				offset_position: Vector2d { x: x as u16, y },
				repetitions_count: self.thickness,
				repetitions_direction: Axis::Vertical,
				..Default::default()
			})
		}
	}
}

impl<const N: usize, T> Index<T> for Plot<N>
where
	[u16; N]: Index<T, Output = u16>,
{
	type Output = u16;

	fn index(&self, index: T) -> &Self::Output {
		&self.points[index]
	}
}

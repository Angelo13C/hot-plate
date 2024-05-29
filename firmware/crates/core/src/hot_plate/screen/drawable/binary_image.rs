use micromath::vector::Vector2d;

use crate::utils::measurement::color::ColorRGB565;

use super::{Drawable, Pixels};

type Chunk = u32;

pub struct BinaryImage<const CHUNKS: usize> {
	pub width: u16,
	pub chunks_data: [Chunk; CHUNKS],
	pub fill_color: ColorRGB565,
	pub background_color: ColorRGB565,
}

impl<const CHUNKS: usize> BinaryImage<CHUNKS> {
	pub const fn height(&self) -> u16 {
		((Chunk::BITS * CHUNKS as u32) / self.width as u32) as u16
	}
}

impl<const CHUNKS: usize> Drawable for BinaryImage<CHUNKS> {
	fn size(&self) -> micromath::vector::U16x2 {
		Vector2d {
			x: self.width,
			y: self.height(),
		}
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		for y in 0..self.height() {
			for x in 0..self.width {
				let index_of_pixel = y as u32 * self.width as u32 + x as u32;
				let index_of_chunk = index_of_pixel / Chunk::BITS;
				let index_in_chunk = index_of_pixel - index_of_chunk * Chunk::BITS;

				let bit = self.chunks_data[index_of_chunk as usize] & 1 << index_in_chunk;
				let color = match bit {
					0 => self.background_color,
					1 => self.fill_color,
					_ => panic!(),
				};

				(draw_fn)(Pixels {
					offset_position: Vector2d { x, y },
					color,
					..Default::default()
				});
			}
		}
	}
}

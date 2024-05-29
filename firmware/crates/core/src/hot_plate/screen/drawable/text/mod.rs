use micromath::vector::Vector2d;

pub use self::font::Font;

use super::{Drawable, Pixels};

mod font;

type Char = u8;

pub struct Text<const N: usize, F: Font> {
	characters: [Char; N],
	font: F,
}

impl<const N: usize, F: Font> Text<N, F> {
	pub fn from_str(text: &str, font: F) -> Self {
		let characters = crate::utils::slice_to_array_filled(text.as_bytes(), 0);
		Self { characters, font }
	}
}

impl<const N: usize, F: Font> Drawable for Text<N, F> {
	fn size(&self) -> micromath::vector::U16x2 {
		let mut x = 0;
		for char in self.characters {
			x += self.font.get_width_of_char(char);
		}

		Vector2d { x, y: todo!() }
	}

	fn draw(&self, draw_fn: &mut impl FnMut(Pixels)) {
		let mut x = 0;
		for char in self.characters {
			self.font.get_image_of_char(char).draw(&mut |mut pixels: Pixels| {
				pixels.offset_position.x += x;
				(draw_fn)(pixels)
			});

			x += self.font.get_width_of_char(char);
		}
	}
}

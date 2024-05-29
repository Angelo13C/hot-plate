#[derive(Clone, Copy)]
/// A color value in [`RGB565`] format.
///
/// RGB565: https://en.wikipedia.org/wiki/List_of_monochrome_and_RGB_color_formats#16-bit_RGB_(also_known_as_RGB565)
pub struct ColorRGB565(u16);

impl ColorRGB565 {
	/// The white color represented as RGB565.
	pub const WHITE: Self = Self(u16::MAX);

	/// Converts this color to its byte representation and returns it.
	pub fn as_bytes(&self) -> [u8; 2] {
		self.0.to_ne_bytes()
	}
}

impl Default for ColorRGB565 {
	/// Returns the default color, which is white.
	fn default() -> Self {
		Self::WHITE
	}
}

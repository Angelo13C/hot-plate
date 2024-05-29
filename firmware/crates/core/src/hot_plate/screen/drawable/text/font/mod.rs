use crate::hot_plate::screen::drawable::binary_image::BinaryImage;

use super::Char;

pub trait Font {
	fn get_image_of_char(&self, char: Char) -> BinaryImage<1>;
	fn get_width_of_char(&self, char: Char) -> u16;
}

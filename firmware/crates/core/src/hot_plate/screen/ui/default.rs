use embedded_hal::{digital::OutputPin, spi::SpiDevice};
use micromath::vector::U16x2;

use crate::hot_plate::{
	drivers::ili9341::SendError,
	screen::{
		drawable::{special::Flipped, Axis, Triangle},
		Screen,
	},
};

use super::Menu;

pub struct DefaultUI {
	current_menu: Menu,
}

impl DefaultUI {
	pub const fn new() -> Self {
		Self {
			current_menu: Menu::Home,
		}
	}

	pub fn draw<DCXPin: OutputPin, ResetPin: OutputPin, Spi: SpiDevice>(
		&self, screen: &mut Screen<DCXPin, ResetPin, Spi>,
	) -> Result<(), SendError<DCXPin, Spi>> {
		match self.current_menu {
			Menu::Home => {
				const TRIANGLE_X: u16 = 10;
				const TRIANGLE_Y: u16 = 140;
				const TRIANGLE_SIZE: u16 = 20;

				screen.draw(
					U16x2 {
						x: TRIANGLE_X,
						y: TRIANGLE_Y,
					},
					&Triangle { size: TRIANGLE_SIZE },
				)?;
				screen.draw(
					U16x2 {
						x: screen.size().x - TRIANGLE_X - TRIANGLE_SIZE,
						y: TRIANGLE_Y,
					},
					&Flipped {
						draw: Triangle { size: TRIANGLE_SIZE },
						axis: Axis::Horizontal,
					},
				)?;
			},
			Menu::Reflowing => todo!(),
		}

		Ok(())
	}
}

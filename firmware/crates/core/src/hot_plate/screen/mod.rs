use embedded_hal::{digital::OutputPin, spi::SpiDevice};
use micromath::vector::U16x2;

use self::drawable::{Axis, Drawable};

use super::drivers::ili9341::{SendError, ILI9341};

pub mod drawable;
pub mod ui;

use ui::default::DefaultUI;

const SCREEN_WIDTH_IN_PIXELS: usize = 320;
const SCREEN_HEIGHT_IN_PIXELS: usize = 240;

pub struct Screen<DCXPin: OutputPin, ResetPin: OutputPin, Spi: SpiDevice> {
	ili9341: ILI9341<DCXPin, ResetPin, Spi>,

	ui: Option<DefaultUI>,
}

impl<DCXPin: OutputPin, ResetPin: OutputPin, Spi: SpiDevice> Screen<DCXPin, ResetPin, Spi> {
	pub const fn new(ili9341: ILI9341<DCXPin, ResetPin, Spi>) -> Self {
		Self {
			ili9341,
			ui: Some(DefaultUI::new()),
		}
	}

	pub const fn size(&self) -> U16x2 {
		U16x2 {
			x: SCREEN_WIDTH_IN_PIXELS as u16,
			y: SCREEN_HEIGHT_IN_PIXELS as u16,
		}
	}

	pub fn tick(&mut self) -> Result<(), SendError<DCXPin, Spi>> {
		if let Some(ui) = self.ui.take() {
			ui.draw(self)?;

			self.ui = Some(ui);
		}

		Ok(())
	}

	pub fn draw(&mut self, position: U16x2, drawable: &impl Drawable) -> Result<(), SendError<DCXPin, Spi>> {
		drawable.draw(&mut |pixels| {
			let start = position + pixels.offset_position;

			let mut end = start;
			match pixels.repetitions_direction {
				Axis::Horizontal => end.x += pixels.repetitions_count,
				Axis::Vertical => end.y += pixels.repetitions_count,
			};

			self.ili9341.set_window(start, end).unwrap();

			self.ili9341.send_color(pixels.color, pixels.repetitions_count).unwrap();
		});

		Ok(())
	}
}

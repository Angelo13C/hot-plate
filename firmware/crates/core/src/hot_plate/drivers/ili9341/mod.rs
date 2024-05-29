mod commands;

use embedded_hal::{digital::OutputPin, spi::SpiDevice};

pub use commands::*;
use micromath::vector::U16x2;

use crate::utils::measurement::color::ColorRGB565;

pub struct ILI9341<DCXPin: OutputPin, ResetPin: OutputPin, Spi: SpiDevice> {
	d_cx_pin: DCXPin,
	reset_pin: ResetPin,
	spi: Spi,
}

impl<DCXPin: OutputPin, ResetPin: OutputPin, Spi: SpiDevice> ILI9341<DCXPin, ResetPin, Spi> {
	pub fn new(d_cx_pin: DCXPin, reset_pin: ResetPin, spi: Spi) -> Result<Self, ResetPin::Error> {
		let mut self_ = Self {
			d_cx_pin,
			reset_pin,
			spi,
		};

		self_.hardware_reset()?;

		Ok(self_)
	}

	pub fn hardware_reset(&mut self) -> Result<(), ResetPin::Error> {
		self.reset_pin.set_low()?;

		todo!("Delay");

		self.reset_pin.set_high()?;

		Ok(())
	}

	pub fn set_window(&mut self, start: U16x2, end: U16x2) -> Result<(), SendError<DCXPin, Spi>> {
		let mut send_axis = |command, start, end| {
			self.send_command(command)?;
			let start = (start as u32) << 16 | end as u32;
			self.send_data(&start.to_be_bytes())?;

			Ok(())
		};

		(send_axis)(Command::ColumnAddressSet, start.x, end.x)?;
		(send_axis)(Command::PageAddressSet, start.y, end.y)?;

		Ok(())
	}

	pub fn send_color(&mut self, color: ColorRGB565, count: u16) -> Result<(), SendError<DCXPin, Spi>> {
		self.send_command(Command::MemoryWrite)?;

		for _ in 0..count {
			self.send_data(&color.as_bytes())?;
		}

		Ok(())
	}

	pub fn send_command(&mut self, command: Command) -> Result<(), SendError<DCXPin, Spi>> {
		self.d_cx_pin.set_low().map_err(SendError::SetDCx)?;

		self.spi.write(&[command as u8]).map_err(SendError::SendOverSPI)?;

		self.d_cx_pin.set_high().map_err(SendError::SetDCx)?;

		Ok(())
	}

	pub fn send_data(&mut self, data: &[u8]) -> Result<(), SendError<DCXPin, Spi>> {
		self.d_cx_pin.set_high().map_err(SendError::SetDCx)?;

		self.spi.write(data).map_err(SendError::SendOverSPI)?;

		Ok(())
	}
}

pub enum SendError<DCXPin: OutputPin, Spi: SpiDevice> {
	SetDCx(DCXPin::Error),
	SendOverSPI(Spi::Error),
}

impl<DCXPin: OutputPin, Spi: SpiDevice> core::fmt::Debug for SendError<DCXPin, Spi> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		match self {
			Self::SetDCx(arg0) => f.debug_tuple("SetDCx").field(arg0).finish(),
			Self::SendOverSPI(arg0) => f.debug_tuple("SendOverSPI").field(arg0).finish(),
		}
	}
}

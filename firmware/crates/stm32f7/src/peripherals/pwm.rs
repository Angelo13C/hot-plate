use core::convert::Infallible;

use firmware_core::{
	hot_plate::hal::pwm::PwmPin as PwmPinTrait,
	utils::{math::Percentage, measurement::frequency::Frequency},
};
use micromath::F32Ext;
use stm32f7xx_hal::timer::{PwmChannel, PwmExt};

pub struct PwmPin<PWM: PwmExt, const CHANNEL: u8> {
	pin: PwmChannel<PWM, CHANNEL>,
	max_duty: f32,
}

impl<PWM: PwmExt, const CHANNEL: u8> PwmPin<PWM, CHANNEL> {
	pub fn new(pin: PwmChannel<PWM, CHANNEL>) -> Self {
		let max_duty = match pin.get_max_duty() {
			0 => u16::MAX as f32 + 1.,
			value => value as f32,
		};

		Self { pin, max_duty }
	}
}

impl<PWM: PwmExt, const CHANNEL: u8> PwmPinTrait for PwmPin<PWM, CHANNEL> {
	type Error = Infallible;

	fn get_duty_cycle(&self) -> Percentage {
		Percentage::from_0_to_1(self.pin.get_duty() as f32 / self.max_duty).unwrap()
	}

	fn set_duty_cycle(&mut self, percentage: Percentage) -> Result<(), Self::Error> {
		let duty = (percentage.into_0_to_1() * self.max_duty).round() as u16;
		self.pin.set_duty(duty);

		Ok(())
	}

	fn set_frequency(&mut self, frequency: Frequency) -> Result<(), Self::Error> {
		todo!()
	}
}

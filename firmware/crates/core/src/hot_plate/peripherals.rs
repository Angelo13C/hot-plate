use embedded_hal::{digital::OutputPin, spi::SpiDevice};

use super::hal::{
	adc::{Adc, AdcPin},
	pwm::PwmPin,
	system_time::SystemTime,
};

pub trait Peripherals {
	type LcdDCXPin: OutputPin;
	type LcdResetPin: OutputPin;
	type LcdSpi: SpiDevice;

	type FanPin: PwmPin;

	type HeaterPin: PwmPin;
	type ADC: Adc;
	type Thermistor1Pin: AdcPin<Self::ADC>;

	type SystemTime: SystemTime;

	fn take_lcd_dcx_pin(&mut self) -> Option<Self::LcdDCXPin>;
	fn take_lcd_reset_pin(&mut self) -> Option<Self::LcdResetPin>;
	fn take_lcd_spi(&mut self) -> Option<Self::LcdSpi>;

	fn take_fan_pin(&mut self) -> Option<Self::FanPin>;

	fn take_heater_pin(&mut self) -> Option<Self::HeaterPin>;
	fn take_adc(&mut self) -> Option<Self::ADC>;
	fn take_thermistor1_pin(&mut self) -> Option<Self::Thermistor1Pin>;

	fn take_system_time(&mut self) -> Option<Self::SystemTime>;
}

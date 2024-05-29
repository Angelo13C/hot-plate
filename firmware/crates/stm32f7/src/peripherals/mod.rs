use cortex_m::Peripherals as CortexPeripherals;
use firmware_core::hot_plate::peripherals::Peripherals as PeripheralsTrait;
use stm32f7xx_hal::{
	gpio::{GpioExt, Output, Pin},
	pac::{Peripherals as Stm32Peripherals, SPI1, TIM1, TIM8},
	prelude::*,
	rcc::{HSEClock, HSEClockMode},
	timer::*,
};

use self::{pwm::PwmPin, system_time::SystemTime};

mod pwm;
mod system_time;

pub struct Peripherals {
	lcd_dcx_pin: Option<<Self as PeripheralsTrait>::LcdDCXPin>,
	lcd_reset_pin: Option<<Self as PeripheralsTrait>::LcdResetPin>,
	lcd_spi: Option<<Self as PeripheralsTrait>::LcdSpi>,

	fan_pin: Option<<Self as PeripheralsTrait>::FanPin>,
	heater_pin: Option<<Self as PeripheralsTrait>::HeaterPin>,
	adc: Option<<Self as PeripheralsTrait>::ADC>,
	thermistor1_pin: Option<<Self as PeripheralsTrait>::Thermistor1Pin>,

	system_time: Option<<Self as PeripheralsTrait>::SystemTime>,
}

impl PeripheralsTrait for Peripherals {
	type LcdDCXPin = Pin<'A', 3, Output>;

	type LcdResetPin = Pin<'A', 2, Output>;

	type LcdSpi = SPI1;

	type FanPin = PwmPin<TIM1, C1>;

	type HeaterPin = PwmPin<TIM8, C1>;

	type ADC;

	type Thermistor1Pin;

	type SystemTime = SystemTime<1_000_000>;

	fn take_lcd_dcx_pin(&mut self) -> Option<Self::LcdDCXPin> {
		self.lcd_dcx_pin.take()
	}

	fn take_lcd_reset_pin(&mut self) -> Option<Self::LcdResetPin> {
		self.lcd_reset_pin.take()
	}

	fn take_lcd_spi(&mut self) -> Option<Self::LcdSpi> {
		self.lcd_spi.take()
	}

	fn take_fan_pin(&mut self) -> Option<Self::FanPin> {
		self.fan_pin.take()
	}

	fn take_heater_pin(&mut self) -> Option<Self::HeaterPin> {
		self.heater_pin.take()
	}

	fn take_adc(&mut self) -> Option<Self::ADC> {
		self.adc.take()
	}

	fn take_thermistor1_pin(&mut self) -> Option<Self::Thermistor1Pin> {
		self.thermistor1_pin.take()
	}

	fn take_system_time(&mut self) -> Option<Self::SystemTime> {
		self.system_time.take()
	}
}

impl Peripherals {
	pub fn from_stm32_peripherals(stm_peripherals: Stm32Peripherals, cortex_peripherals: CortexPeripherals) -> Self {
		let gpio_a = stm_peripherals.GPIOA.split();
		let gpio_c = stm_peripherals.GPIOC.split();

		let rcc = stm_peripherals.RCC.constrain();
		let clocks = rcc
			.cfgr
			.hse(HSEClock::new(8.MHz(), HSEClockMode::Oscillator))
			.sysclk(216.MHz())
			.freeze();

		let heater_pwm = stm_peripherals
			.TIM1
			.pwm_hz(gpio_a.pa1.into_alternate(), 2.kHz(), &clocks)
			.split();

		let system_time = Timer::syst(cortex_peripherals.SYST, &clocks).counter_us();

		Self {
			lcd_dcx_pin: Some(gpio_a.pa3.into_push_pull_output()),
			lcd_reset_pin: Some(gpio_a.pa2.into_push_pull_output()),
			lcd_spi: Some(stm_peripherals.SPI1),
			fan_pin: Some(gpio_c.pc13.into_alternate()),
			heater_pin: Some(PwmPin::new(heater_pwm)),
			adc: Some(),
			thermistor1_pin: Some(),
			system_time: Some(),
		}
	}
}

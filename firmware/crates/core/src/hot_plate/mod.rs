use embedded_hal::{digital::OutputPin, spi::SpiDevice};

use crate::utils::math::Percentage;

use self::{
	config::Configuration,
	drivers::{
		cartridge_heater::CartridgeHeater,
		fan::Fan,
		ili9341::{SendError, ILI9341},
		thermistor::Thermistor,
	},
	hal::{pwm::PwmPin, system_time::Clock},
	peripherals::Peripherals,
	process::DefaultReflowProcess,
	screen::Screen,
	temperature::{safety::TemperatureSafety, TemperaturePidController},
};

pub mod config;
pub mod drivers;
pub mod hal;
pub mod peripherals;
pub mod process;
pub mod screen;
pub mod temperature;

pub struct HotPlate<P: Peripherals> {
	screen: Screen<P::LcdDCXPin, P::LcdResetPin, P::LcdSpi>,
	reflow_process: Option<DefaultReflowProcess>,

	pid_controller: TemperaturePidController<P::HeaterPin, P::ADC, P::Thermistor1Pin>,
	adc: P::ADC,

	fan: Fan<P::FanPin>,

	clock: Clock<P::SystemTime>,
}

impl<P: Peripherals> HotPlate<P> {
	pub fn new(mut peripherals: P, configuration: Configuration) -> Result<Self, CreationError<P::LcdResetPin>> {
		Ok(Self {
			screen: Screen::new(
				ILI9341::new(
					peripherals
						.take_lcd_dcx_pin()
						.ok_or(CreationError::PeripheralMissing { name: "LCD DCX pin" })?,
					peripherals
						.take_lcd_reset_pin()
						.ok_or(CreationError::PeripheralMissing { name: "LCD reset pin" })?,
					peripherals
						.take_lcd_spi()
						.ok_or(CreationError::PeripheralMissing { name: "LCD SPI" })?,
				)
				.map_err(CreationError::ScreenCreation)?,
			),
			reflow_process: None,
			clock: Clock::new(
				peripherals
					.take_system_time()
					.ok_or(CreationError::PeripheralMissing { name: "System time" })?,
			),
			adc: peripherals
				.take_adc()
				.ok_or(CreationError::PeripheralMissing { name: "ADC" })?,
			fan: Fan::new(
				peripherals
					.take_fan_pin()
					.ok_or(CreationError::PeripheralMissing { name: "Fan pin" })?,
				configuration.fan_min_duty_cycle_to_move,
			),
			pid_controller: TemperaturePidController::new(
				Thermistor::new(
					peripherals
						.take_thermistor1_pin()
						.ok_or(CreationError::PeripheralMissing {
							name: "Thermistor 1 pin",
						})?,
					configuration.pid.thermistor.beta,
					configuration.pid.thermistor.resistance_at_t0,
					configuration.pid.thermistor.other_resistance,
				),
				CartridgeHeater::new(
					peripherals
						.take_heater_pin()
						.ok_or(CreationError::PeripheralMissing { name: "Heater pin" })?,
				),
				configuration.pid.pid_gains,
				TemperatureSafety::new(
					configuration.pid.safety.allowed_temperature_range,
					configuration.pid.safety.keep_target_temperature_config,
					configuration.pid.safety.rise_to_target_temperature_config,
					configuration.pid.safety.rise_to_target_temperature_samples_count,
				),
			),
		})
	}

	pub fn tick(&mut self) -> Result<(), TickError<P::LcdDCXPin, P::LcdSpi, P::FanPin>> {
		let delta_time = self.clock.get_delta_time();
		self.clock.tick();

		self.screen.tick().map_err(TickError::Screen)?;

		self.reflow_process = Some(DefaultReflowProcess::start_default());
		if let Some(mut reflow_process) = self.reflow_process.take() {
			if let Some(target_temperature) = reflow_process.tick(delta_time) {
				self.pid_controller.set_target_temperature(target_temperature);

				self.reflow_process = Some(reflow_process);
			} else {
				self.on_reflow_finished().map_err(TickError::SetFanSpeed)?;
			}
		}

		self.pid_controller
			.tick(delta_time.as_secs_f32(), &mut self.adc)
			.map_err(TickError::PidHeater)?;

		Ok(())
	}

	fn on_reflow_finished(&mut self) -> Result<(), <P::FanPin as PwmPin>::Error> {
		self.fan.set_speed(Percentage::FULL)?;

		Ok(())
	}
}

#[derive(Debug)]
/// An error that can occur when you instatiate a [`HotPlate`] struct.
pub enum CreationError<ResetPin: OutputPin> {
	/// A peripheral from the provided ones is missing (`name` is the name of the peripheral that's missing).
	/// This means that `peripherals.take_...()` returned `None` instead of `Some`.
	PeripheralMissing {
		name: &'static str,
	},

	ScreenCreation(ResetPin::Error),
}

/// An error that can occur when you tick a [`Printer3DComponents`] struct.
pub enum TickError<DCXPin: OutputPin, Spi: SpiDevice, FanPin: PwmPin> {
	Screen(SendError<DCXPin, Spi>),
	PidHeater(temperature::PidUpdateError),
	SetFanSpeed(FanPin::Error),
}

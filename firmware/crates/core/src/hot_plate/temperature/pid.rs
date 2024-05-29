use enumset::EnumSet;
use pid::Pid;

use super::safety::{self, TemperatureSafety};
use crate::{
	hot_plate::{
		drivers::{cartridge_heater::CartridgeHeater, thermistor::Thermistor},
		hal::{
			adc::{Adc, AdcPin, ReadPercentageError},
			pwm::PwmPin,
		},
	},
	utils::{
		math::{self, Percentage},
		measurement::temperature::Temperature,
	},
};

/// A [`PID controller`] used to control the temperature of a system in a closed loop.
///
/// To use it, first [`create`] the controller, than whenever you want you can [`choose the target temperature`]
/// and you must continually call [`tick`] to make the controller actually do the work.
///
/// [`PID controller`]: https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller
/// [`create`]: `Self::new`
/// [`choose the target temperature`]: `Self::set_target_temperature`
/// [`tick`]: `Self::tick`
pub struct PidController<CHP: PwmPin, TADC: Adc, TP: AdcPin<TADC>> {
	thermistor: Thermistor<TADC, TP>,
	cartridge_heater: CartridgeHeater<CHP>,
	pid_control: Pid<f32>,
	safety: TemperatureSafety,

	last_current_temperature_sample: Option<Temperature>,
}

impl<CHP: PwmPin, TADC: Adc, TP: AdcPin<TADC>> PidController<CHP, TADC, TP> {
	/// The minimum limit output by the PID control. Take this in consideration when setting the `PidGains`.
	pub const PID_CONTROL_MIN_LIMIT: f32 = 0.;
	/// The maximum limit output by the PID control. Take this in consideration when setting the `PidGains`.
	pub const PID_CONTROL_MAX_LIMIT: f32 = 100.;

	/// Returns a [`PidController`] that will control the `cartridge heater`'s current based on the [`set target temperature`]
	/// and the [`current temperature`] read in the provided `thermistor` using the provided gains.
	///
	/// [`set target temperature`]: `Self::set_target_temperature`
	/// [`current temperature`]: `Self::get_current_temperature`
	pub fn new(
		thermistor: Thermistor<TADC, TP>, cartridge_heater: CartridgeHeater<CHP>, pid_gains: PidGains,
		safety: TemperatureSafety,
	) -> Self {
		let mut pid_control = Pid::new(0., Self::PID_CONTROL_MAX_LIMIT);
		pid_control.p(pid_gains.p, Self::PID_CONTROL_MAX_LIMIT);
		pid_control.i(pid_gains.i, Self::PID_CONTROL_MAX_LIMIT);
		pid_control.d(pid_gains.d, Self::PID_CONTROL_MAX_LIMIT);

		Self {
			thermistor,
			cartridge_heater,
			pid_control,
			safety,
			last_current_temperature_sample: None,
		}
	}

	/// Returns the [`PidGains`] previously set on this PID controller.
	pub fn get_pid_gains(&self) -> PidGains {
		PidGains {
			p: self.pid_control.kp,
			i: self.pid_control.ki,
			d: self.pid_control.kd,
		}
	}

	/// Set the PID gains of this controller. Check [`Self::PID_CONTROL_MIN_LIMIT`] and [`Self::PID_CONTROL_MAX_LIMIT`] to see in what
	/// range the values should be.
	pub fn set_pid_gains(&mut self, pid_gains: &PidGains) {
		self.pid_control.kp = pid_gains.p;
		self.pid_control.ki = pid_gains.i;
		self.pid_control.kd = pid_gains.d;
	}

	/// Reads the current [`Temperature`] of the PID controller.
	///
	/// Returns `Ok(Temperature)` if the read was succesful, otherwise `Err(ReadPercentageError)`.
	pub fn get_current_temperature(&mut self, adc: &mut TADC) -> Result<Temperature, ReadPercentageError<TADC, TP>> {
		match self.thermistor.read_temperature(adc) {
			Ok(temperature) => {
				self.last_current_temperature_sample = Some(temperature);
				Ok(temperature)
			},
			Err(error) => Err(error),
		}
	}

	/// Gets the [`Temperature`] read using [`Self::get_current_temperature`] the last time you called that function.
	///
	/// Returns `None` if [`Self::get_current_temperature`] has never been successfull since the instantation of this
	/// struct.
	pub fn get_last_sample_of_current_temperature(&self) -> Option<Temperature> {
		self.last_current_temperature_sample
	}

	/// Returns the [`Temperature`] the PID controller is trying to reach.
	pub fn get_target_temperature(&self) -> Temperature {
		Temperature::from_kelvin(self.pid_control.setpoint)
	}

	/// Sets the [`Temperature`] the PID controller will try to reach.
	///
	/// # Warning
	/// You need to call [`PidController::tick`] after this to effectively make the PID controller work to reach it.
	pub fn set_target_temperature(&mut self, target_temperature: Temperature) {
		self.pid_control.setpoint(target_temperature.as_kelvin() as f32);
	}

	/// Make the PID controller work to try to reach its [`target temperature`].
	///
	/// [`target temperature`]: `Self::get_target_temperature`
	pub fn tick(&mut self, delta_time: f32, adc: &mut TADC) -> Result<(), TickError> {
		let current_temperature = self
			.get_current_temperature(adc)
			.map_err(|_| TickError::CantReadTemperature)?;

		let safety_errors =
			self.safety
				.is_temperature_safe(current_temperature, self.get_target_temperature(), delta_time);
		if !safety_errors.is_empty() {
			return Err(TickError::ReadTemperatureIsWrong(safety_errors));
		}

		let mut pwm_value = self
			.pid_control
			.next_control_output(current_temperature.as_kelvin() as f32)
			.output;
		pwm_value = math::map(
			pwm_value,
			Self::PID_CONTROL_MIN_LIMIT..=Self::PID_CONTROL_MAX_LIMIT,
			0_f32..=1_f32,
		);

		self.cartridge_heater
			.set_heat_percentage(Percentage::from_0_to_1(pwm_value as f32).unwrap())
			.map_err(|_| TickError::SetCartridgeHeaterPercentage)?;

		Ok(())
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// An error that occurred when calling [`tick`] on a PID controller.
///
/// [`tick`]: PidController::tick
pub enum TickError {
	/// It has been impossible to [`read`] the thermistor's temperature.
	///
	/// [`read`]: `Thermistor::read_temperature`
	CantReadTemperature,

	/// The thermistor's `temperature` has been [`read`], but it's an irregular value.
	///
	/// **It could be that the thermistor is damaged, or its connection to the microcontroller is damaged...**
	/// It could also be a false positive: but it's always better to abort the print and turn off the heaters
	/// to prevent fire hazards. Then if it was a false positive, it means that the parameters passed
	/// to [`Safety::new`] are too strict.
	///
	/// [`read`]: `Thermistor::read_temperature`
	/// [`Safety::new`]: `safety::TemperatureSafety::new`
	ReadTemperatureIsWrong(EnumSet<safety::TemperatureError>),

	/// It has been impossible to [`set`] the cartridge heater's heat percentage.
	///
	/// [`set`]: `CartridgeHeater::set_heat_percentage`
	SetCartridgeHeaterPercentage,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Values of the `proportional`, `integral` and `derivative` gains of a PID controller.
pub struct PidGains {
	/// [`Proportial component`](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller#Proportional).
	pub p: f32,
	/// [`Integral component`](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller#Integral).
	pub i: f32,
	/// [`Derivative component`](https://en.wikipedia.org/wiki/Proportional%E2%80%93integral%E2%80%93derivative_controller#Derivative).
	pub d: f32,
}

use crate::utils::math::Percentage;

pub struct Configuration {
	pub fan_min_duty_cycle_to_move: Percentage,

	pub pid: temperature::PidConfig,
}

pub mod temperature {
	use core::ops::RangeInclusive;

	use crate::{
		hot_plate::temperature::{safety::temperature_change::TemperatureChangeConfig, TemperaturePidGains},
		utils::measurement::temperature::Temperature,
	};

	pub struct PidConfig {
		pub pid_gains: TemperaturePidGains,
		pub thermistor: ThermistorConfig,
		pub safety: SafetyConfig,
	}

	pub struct ThermistorConfig {
		pub beta: u32,
		pub resistance_at_t0: u32,
		pub other_resistance: u32,
	}

	pub struct SafetyConfig {
		pub allowed_temperature_range: RangeInclusive<Temperature>,
		pub keep_target_temperature_config: TemperatureChangeConfig,
		pub rise_to_target_temperature_config: TemperatureChangeConfig,
		pub rise_to_target_temperature_samples_count: usize,
	}
}

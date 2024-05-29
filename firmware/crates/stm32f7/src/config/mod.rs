use firmware_core::{
	hot_plate::{
		config::{temperature::*, Configuration},
		temperature::{safety::temperature_change::TemperatureChangeConfig, TemperaturePidGains},
	},
	utils::{math::Percentage, measurement::temperature::Temperature},
};

pub fn configuration() -> Configuration {
	Configuration {
		fan_min_duty_cycle_to_move: Percentage::from_0_to_100(20.).unwrap(),
		pid: PidConfig {
			pid_gains: TemperaturePidGains { p: 20., i: 2., d: 50. },
			thermistor: ThermistorConfig {
				beta: 3_950,
				resistance_at_t0: 100_000,
				other_resistance: 4_700,
			},
			safety: SafetyConfig {
				allowed_temperature_range: Temperature::from_celsius(0.)..=Temperature::from_celsius(270.),
				keep_target_temperature_config: TemperatureChangeConfig {
					period_in_seconds: 20.,
					hysteresis: 2.,
				},
				rise_to_target_temperature_config: TemperatureChangeConfig {
					period_in_seconds: 90.,
					hysteresis: 2.,
				},
				rise_to_target_temperature_samples_count: 45,
			},
		},
	}
}

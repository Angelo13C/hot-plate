use core::time::Duration;

use crate::utils::measurement::temperature::Temperature;

use self::temperature_reflow_profile::{ReflowProfile, DEFAULT_PROFILE, DEFAULT_PROFILE_POINTS};

use super::screen::drawable::Plot;

mod temperature_reflow_profile;

pub type DefaultReflowProcess = ReflowProcess<DEFAULT_PROFILE_POINTS, 200>;

pub struct ReflowProcess<const N: usize, const PLOT_N: usize> {
	temperature_profile: ReflowProfile<N>,
	plot: Plot<PLOT_N>,
	current_time: Duration,
}

impl<const N: usize, const PLOT_N: usize> ReflowProcess<N, PLOT_N> {
	pub fn start(temperature_profile: ReflowProfile<N>) -> Self {
		assert_ne!(temperature_profile.temperature_points.len(), 0);

		let plot = temperature_profile.to_plot(1);

		Self {
			temperature_profile,
			current_time: Duration::ZERO,
			plot,
		}
	}

	pub fn start_default() -> DefaultReflowProcess {
		DefaultReflowProcess::start(DEFAULT_PROFILE)
	}

	/// Returns the target temperature.
	pub fn tick(&mut self, delta_time: Duration) -> Option<Temperature> {
		self.current_time += delta_time;

		let &(_, last_point_time) = self.temperature_profile.temperature_points.last().unwrap();
		let current_point_index = crate::utils::math::map(
			self.current_time.as_secs() as u32,
			0..=last_point_time as u32,
			0..=(PLOT_N as u32 - 1),
		) as usize;

		self.plot
			.get(current_point_index)
			.map(|celsius| Temperature::from_celsius(celsius as f32))
	}
}

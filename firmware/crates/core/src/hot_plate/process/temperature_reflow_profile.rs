use crate::{
	hot_plate::screen::drawable::{Plot, Thickness},
	utils::measurement::temperature::Temperature,
};

pub const DEFAULT_PROFILE_POINTS: usize = 5;
pub const DEFAULT_PROFILE: ReflowProfile<DEFAULT_PROFILE_POINTS> = ReflowProfile {
	temperature_points: [
		(Temperature::from_kelvin(150. + Temperature::ZERO_CELSIUS_IN_KELVIN), 60),
		(
			Temperature::from_kelvin(180. + Temperature::ZERO_CELSIUS_IN_KELVIN),
			180,
		),
		(
			Temperature::from_kelvin(240. + Temperature::ZERO_CELSIUS_IN_KELVIN),
			240,
		),
		(
			Temperature::from_kelvin(240. + Temperature::ZERO_CELSIUS_IN_KELVIN),
			255,
		),
		(Temperature::from_kelvin(0. + Temperature::ZERO_CELSIUS_IN_KELVIN), 360),
	],
};

type TimeInSeconds = u16;

pub struct ReflowProfile<const N: usize> {
	pub temperature_points: [(Temperature, TimeInSeconds); N],
}

impl<const N: usize> ReflowProfile<N> {
	pub fn to_plot<const P: usize>(&self, thickness: Thickness) -> Plot<P> {
		let mut points = [0; P];
		if let Some(&(_, last_point_time)) = self.temperature_points.last() {
			let step_size = last_point_time as f32 / P as f32;

			let mut current_point_index = 0;
			let mut point_before = (Temperature::from_celsius(0.), 0);
			for i in 0..P {
				let time = i as f32 * step_size;
				if time as u16 > self.temperature_points[current_point_index].1 {
					point_before = self.temperature_points[current_point_index];
					current_point_index += 1;
				}

				let point = crate::utils::math::lerp(
					crate::utils::math::map(
						time,
						point_before.1 as f32..=self.temperature_points[current_point_index].1 as f32,
						0_f32..=1_f32,
					),
					point_before.0.as_celsius()..=self.temperature_points[current_point_index].0.as_celsius(),
				) as u16;

				points[i] = point;
			}
		}

		Plot { points, thickness }
	}
}

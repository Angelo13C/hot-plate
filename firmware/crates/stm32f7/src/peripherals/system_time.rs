use firmware_core::hot_plate::hal::system_time::SystemTime as SystemTimeTrait;
use stm32f7xx_hal::timer::SysCounter;

pub struct SystemTime<const FREQ: u32> {
	system_counter: SysCounter<FREQ>,
}

impl<const FREQ: u32> SystemTimeTrait for SystemTime<FREQ> {
	fn now(&self) -> core::time::Duration {
		self.system_counter.now().duration_since_epoch().to_micros()
	}

	fn delay(&self, duration: core::time::Duration) {
		todo!()
	}
}

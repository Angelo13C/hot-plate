#![no_std]
#![no_main]

pub mod config;
pub mod peripherals;

use firmware_core::hot_plate::HotPlate;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
					 // use panic_abort as _; // requires nightly
					 // use panic_itm as _; // logs messages over ITM; requires ITM support
					 // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use peripherals::Peripherals;

#[entry]
fn main() -> ! {
	let mut hot_plate = create_hot_plate();

	loop {
		hot_plate.tick().unwrap();
	}
}

fn create_hot_plate() -> HotPlate<Peripherals> {
	let peripherals = Peripherals::from_stm32_peripherals(
		stm32f7xx_hal::pac::Peripherals::take().unwrap(),
		cortex_m::Peripherals::take().unwrap(),
	);
	HotPlate::new(peripherals, config::configuration()).unwrap()
}

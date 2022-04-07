#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m::prelude::*;
use cortex_m_rt::entry;
use embedded_hal::{
    digital::v2::{OutputPin, InputPin}
};
use embedded_time::rate::*;
use rp2040_hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio,
    gpio::Pins
};
/// The linker will place this boot block at the start of our program image. We
/// need this to help the ROM bootloader get our code up and running.
// #[link_section = ".boot2"]
// #[used]
// pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_RAM_MEMCPY;
const XTAL_OSC_FREQ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    // initialization code here
    let mut pac = pac::Peripherals::take().unwrap();
    let core = pac::CorePeripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XTAL_OSC_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut timer = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().integer());
    let mut status_pin = pins.gpio25.into_push_pull_output();

    // loop code here
    loop {
        status_pin.set_high().unwrap();
        timer.delay_ms(500);
        status_pin.set_low().unwrap();
        timer.delay_ms(500);
    }
}
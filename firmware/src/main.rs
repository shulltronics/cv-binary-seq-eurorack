#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m::prelude::*;
use cortex_m_rt::entry;
use embedded_hal::{
    digital::v2::{OutputPin, InputPin}
};
// use embedded_time::rate::*;
use fugit::RateExtU32;  // allows the .to_Hz() method
use rp2040_hal as hal;
use hal::{
    clocks::{init_clocks_and_plls, Clock},
    pac,
    watchdog::Watchdog,
    Sio,
    gpio::Pins,
};

use display_interface_spi::SPIInterface;
use st7789::{Orientation, ST7789};
use embedded_graphics::{
    prelude::*,
    primitives::*,
    pixelcolor::Rgb565,
    mono_font::{ascii::FONT_6X9, MonoTextStyle},
    text::Text,
};

mod mcp48fxbxx;
use mcp48fxbxx::MCP48FXBxx;

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

    let mut timer = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());
    
    // general purpose testing pin (goes to unpopulated LED for now)
    let mut status_pin = pins.gpio25.into_push_pull_output();
    let encoder_button = pins.gpio7.into_pull_down_input();

    // pins for use with ST7789 LCD
    let _lcd_cs  = pins.gpio9.into_push_pull_output();
    let _lcd_sclk = pins.gpio10.into_mode::<hal::gpio::FunctionSpi>();
    let _lcd_mosi = pins.gpio11.into_mode::<hal::gpio::FunctionSpi>();
    let _lcd_miso = pins.gpio12.into_mode::<hal::gpio::FunctionSpi>();
    let _lcd_rst = pins.gpio13.into_push_pull_output();
    let _lcd_dc  = pins.gpio14.into_push_pull_output();
    // // initialize the SPI module
    let spi = hal::Spi::<_, _, 8>::new(pac.SPI1);
    let mut spi = spi.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        64_000_000u32.Hz(),
        &embedded_hal::spi::MODE_3,
    );
    // create the SPI "display interface"
    let di = SPIInterface::new(spi, _lcd_dc, _lcd_cs);
    let mut lcd = ST7789::new(di, _lcd_rst, 240, 135);
    // // initialize the display and set its orientation
    lcd.init(&mut timer).unwrap();
    lcd.set_orientation(Orientation::Landscape).unwrap();

    // let line = Line::new(Point::new(0, 0), Point::new(100, 100))
    //     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1));
    lcd.clear(Rgb565::WHITE).unwrap();
    // line.draw(&mut lcd).unwrap();

    let style = MonoTextStyle::new(&FONT_6X9, Rgb565::RED);
    let mut text = Text::new("hello, rust!", Point::new(100, 100), style);
    
    // pins for use with MCP48FXB2X DAC
    let _dac_cs   = pins.gpio17.into_push_pull_output();
    let _dac_sclk = pins.gpio18.into_mode::<hal::gpio::FunctionSpi>();
    let _dac_mosi = pins.gpio19.into_mode::<hal::gpio::FunctionSpi>();
    let _dac_miso = pins.gpio20.into_mode::<hal::gpio::FunctionSpi>();
    // // initialize the SPI module
    let spi0 = hal::Spi::<_, _, 8>::new(pac.SPI0);
    let mut spi0 = spi0.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        64_000_000u32.Hz(),
        &embedded_hal::spi::MODE_3,
    );

    let dac = MCP48FXBxx::new(spi0, _dac_cs);

    // loop code here
    let mut old_button_state: bool = false;
    loop {
        let current_button_state: bool = encoder_button.is_high().unwrap();
        let button_changed: bool = (old_button_state != current_button_state);
        if button_changed {
            old_button_state = current_button_state;
            if current_button_state == true {
                text.draw(&mut lcd).unwrap();
            } else {
                lcd.clear(Rgb565::BLACK).unwrap();
            }
        }
    }
}
#![no_std] //prevent standard lib from linking, useful for non-OS stuff
#![no_main] // use core::ops::End; where did this come from??

// list imports, make aliases
use panic_halt as _; // must be mentioned or the it wont be linked

//don't need to type rp2040-hal every time, this firmware deals with 1 mcu lol

use rp2040_hal as hal;

//use core::fmt::Write;
//use hal::fugit::RateExtU32;
//use hal::Clock;

// list traits
use hal::pac;

//[cfg(feature = "async")]
use hal::gpio::{FunctionI2C, Pin};
use hal::{
    I2C,
    fugit::RateExtU32,
    gpio::bank0::{Gpio22, Gpio23},
    i2c::Controller,
};
use core::fmt::Write;
use ssd1306::{mode::TerminalMode, prelude::*, I2CDisplayInterface, Ssd1306};

//
// future me with keyboard, use W25Q080 bootloader, doesnt matter too much just rember
#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// crystal frequency, 12mhz is standard
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

mod i2c;

// macro entry
// cortex-m start-up code calls this as soon as global vars and spinlocks initialised
#[hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = pac::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .unwrap();

    let mut timer = rp2040_hal::Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio22.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio23.reconfigure();
    // let not_an_scl_pin: Pin<_, FunctionI2C, PullUp> = pins.gpio20.reconfigure(); 
    // Create the I²C drive, using the two pre-configured pins. This will fail
    // at compile time if the pins are in the wrong mode, or if this I²C
    // peripheral isn't available on these pins!
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin, // Try `not_an_scl_pin` here
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    loop{
        //poo
    }
}



/// This is a list of references to our table entries
///
/// They must be in the `.bi_entries` section as we tell picotool the start and
/// end addresses of that section.
#[unsafe(link_section = ".bi_entries")]
#[cfg(all(feature = "binary-info", target_os = "none"))]
#[used]
pub static PICOTOOL_ENTRIES: [rp_binary_info::EntryAddr; 4] = [
    rp_binary_info::rp_program_name!(c"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"),
    rp_binary_info::rp_cargo_version!(),
    rp_binary_info::rp_binary_end!(__flash_binary_end),
    rp_binary_info::int!(
        rp_binary_info::make_tag(b"JP"),
        0x0000_0001,
        0x12345678
    ),
];

unsafe extern "C" {
    unsafe static __flash_binary_end: u32;
}

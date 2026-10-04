//fn main() {
//    println!("Hello, world!");
//}
//
#![no_std] //prevent standard lib from linking, useful for non-OS stuff
#![no_main] // use core::ops::End; where did this come from??

//disabled default 'entry point', i will define this later in .cargo/config.toml
//
// list imports, make aliases
use panic_halt as _; // must be mentioned or the it wont be linked

use rp2040_hal as hal;
//use core::fmt::Write;
//use hal::fugit::RateExtU32;
//use hal::Clock;

// //don't need to type rp2040-hal every time, this firmware deals with 1 mcu lol
// list traits
use hal::pac;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;
//
// future me with keyboard, use W25Q080 bootloader, doesnt matter too much just rember
#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_GENERIC_03H;

// crystal frequency, 12mhz is standard
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

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

    // Configure GPIO25 as an output
    let mut led_pin = pins.gpio25.into_push_pull_output();
    loop {
        led_pin.set_high().unwrap();
        timer.delay_ms(500);
        led_pin.set_low().unwrap();
        timer.delay_ms(500);
    }
}

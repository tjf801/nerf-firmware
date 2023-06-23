#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(abi_avr_interrupt)]

extern crate arduino_hal;

use arduino_hal::simple_pwm::{Prescaler, Timer0Pwm};
use avr_hal_generic::port::{Pin, mode};

mod utils;
mod rev_motors;
mod ssd1306;

fn setup(dp: arduino_hal::Peripherals) -> Pin<mode::Output, arduino_hal::hal::port::PB5> {
    let pins = arduino_hal::pins!(dp);
    
    utils::print::put_console(arduino_hal::default_serial!(dp, pins, 57600));
    
    println!("Setting up firmware...");
    
    let pwm_timer = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    
    // setup all the rev motors
    rev_motors::setup(pins.d2, pins.d5, &pwm_timer, &dp.EXINT);
    
    let i2c = arduino_hal::i2c::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        400_000,
    );
    
    let mut display = ssd1306::SSD1306::new(0x3C, i2c);
    display.initialize().unwrap();
    display.send_command(ssd1306::command::Command::SetAddressMode(ssd1306::command::AddressMode::Horizontal));
    display.send_data(&[0x00; 1024]).unwrap();
    display.send_command(ssd1306::command::Command::SetStartLine(32)).unwrap();
    display.send_command(ssd1306::command::Command::ColumnStart(32)).unwrap();
    display.send_data(&[0x00, 0x66, 0x99, 0x99, 0x7E, 0x24, 0x24, 0x24, 0x24, 0x3C, 0x24, 0x18, 0x00, 0x10, 0x24]);
    
    // SAFETY: this is the only thread running, so it's safe to enable interrupts.
    unsafe { avr_device::interrupt::enable() };
    
    println!("Firmware startup complete!");
    
    return pins.d13.into_output();
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    
    let mut blink_led = setup(dp);
    
    loop {
        blink_led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(abi_avr_interrupt)]

extern crate arduino_hal;

use arduino_hal::simple_pwm::{Prescaler, Timer0Pwm};
use avr_hal_generic::port::{Pin, mode};

mod utils;
mod rev_motors;

fn setup(dp: arduino_hal::Peripherals) -> Pin<mode::Output, arduino_hal::hal::port::PB5> {
    let pins = arduino_hal::pins!(dp);
    
    utils::print::put_console(arduino_hal::default_serial!(dp, pins, 57600));
    
    println!("Setting up firmware...");
    
    let pwm_timer = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    
    // setup all the rev motors
    rev_motors::setup(pins.d2, pins.d5, &pwm_timer, &dp.EXINT);
    
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

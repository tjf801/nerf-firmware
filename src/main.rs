#![no_std]
#![no_main]

#![feature(panic_info_message)]

extern crate arduino_hal;
mod panic;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    
    let mut led = pins.d13.into_output();
    
    loop {
        led.toggle();
        arduino_hal::delay_ms(1000);
    }
}

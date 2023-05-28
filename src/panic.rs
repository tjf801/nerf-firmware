use core::panic::PanicInfo;

#[cfg(debug_assertions)]
use arduino_hal::prelude::*;

#[cfg(debug_assertions)]
#[panic_handler]
fn panic_debug(info: &PanicInfo) -> ! {
    // mostly stolen from https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-panic.rs
    
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();
    
    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: Because main() already has references to the peripherals this is an unsafe
    // operation - but because no other code can run after the panic handler was called,
    // we know it is okay.
    let dp = unsafe { arduino_hal::Peripherals::steal() };
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    
    // Print out panic location
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}\r",
            loc.file(),
            loc.line(),
            loc.column(),
        )
        .void_unwrap();
    }
    
    if let Some(args) = info.message() {
        if let Some(msg) = args.as_str() {
            ufmt::uwriteln!(&mut serial, "  '{}'\r", msg).void_unwrap();
        }
        else {
            ufmt::uwriteln!(&mut serial, "  <error in handling message>\r").void_unwrap();
        }
    } else {
        ufmt::uwriteln!(&mut serial, "  <no message>\r").void_unwrap();
    }
    
    
    // turn off all pins
    pins.d2.into_output().set_low();
    pins.d3.into_output().set_low();
    pins.d4.into_output().set_low();
    pins.d5.into_output().set_low();
    pins.d6.into_output().set_low();
    pins.d7.into_output().set_low();
    pins.d8.into_output().set_low();
    pins.d9.into_output().set_low();
    pins.d10.into_output().set_low();
    pins.d11.into_output().set_low();
    pins.d12.into_output().set_low();
    
    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}



#[cfg(not(debug_assertions))]
#[panic_handler]
fn panic_release(_info: &PanicInfo) -> ! {
    avr_device::interrupt::disable();
    
    // get pins
    let pins = arduino_hal::pins!(unsafe{arduino_hal::Peripherals::steal()});
    
    // turn off all pins
    pins.d2.into_output().set_low();
    pins.d3.into_output().set_low();
    pins.d4.into_output().set_low();
    pins.d5.into_output().set_low();
    pins.d6.into_output().set_low();
    pins.d7.into_output().set_low();
    pins.d8.into_output().set_low();
    pins.d9.into_output().set_low();
    pins.d10.into_output().set_low();
    pins.d11.into_output().set_low();
    pins.d12.into_output().set_low();
    
    let mut led = pins.d13.into_output();
    
    // blink debug LED
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

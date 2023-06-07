use core::panic::PanicInfo;

use arduino_hal::prelude::*;

#[panic_handler]
fn panic_debug(info: &PanicInfo) -> ! {
    // mostly stolen from https://github.com/Rahix/avr-hal/blob/main/examples/arduino-uno/src/bin/uno-panic.rs
    
    // disable interrupts - firmware has panicked so no ISRs should continue running
    avr_device::interrupt::disable();
    
    // get the peripherals so we can access serial and the LED.
    //
    // SAFETY: technically other threads could have been accessing the peripherals, but
    //         because of the panic we know that they have all been stopped
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
        ).void_unwrap();
    }
    
    if let Some(args) = info.message() {
        if let Some(msg) = args.as_str() {
            ufmt::uwriteln!(&mut serial, "  '{}'\r", msg).void_unwrap();
        }
        else {
            ufmt::uwriteln!(&mut serial, "  <error handling message> (most likely dynamically created)").void_unwrap();
        }
    } else {
        ufmt::uwriteln!(&mut serial, "  <no message>\r").void_unwrap();
    }
    
    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

use core::{panic::PanicInfo, fmt::Debug};

use arduino_hal::prelude::*;

struct WriteWrapper<'a, W: ufmt::uWrite>(&'a mut W);

impl<'a, W: ufmt::uWrite> core::fmt::Write for WriteWrapper<'a, W> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.0.write_str(s).map_err(|_| core::fmt::Error)
    }
}

#[inline(always)]
fn print_panic_info(
    mut serial: impl ufmt::uWrite<Error = void::Void>,
    info: &PanicInfo
) {
    ufmt::uwriteln!(&mut serial, "Firmware panic!\r").void_unwrap();
    if let Some(loc) = info.location() {
        ufmt::uwriteln!(
            &mut serial,
            "  At {}:{}:{}:\r",
            loc.file(),
            loc.line(),
            loc.column(),
        ).void_unwrap();
    }
    
    if let Some(args) = info.message().cloned() {
        if let Some(msg) = args.as_str() {
            ufmt::uwriteln!(&mut serial, "    {}\r", msg).void_unwrap();
        }
        else {
            #[cfg(debug_assertions)] {
                ufmt::uwrite!(&mut serial, "    ").void_unwrap();
                match core::fmt::write(&mut WriteWrapper(&mut serial), args) {
                    Err(_) => ufmt::uwriteln!(&mut serial, "<error handling message>\r").void_unwrap(),
                    _ => ufmt::uwriteln!(&mut serial, "\r").void_unwrap(),
                }
            }
            #[cfg(not(debug_assertions))] {
                ufmt::uwriteln!(&mut serial, "    <panic message not available in release mode>\r").void_unwrap()
            }
        }
    }
    else {
        ufmt::uwriteln!(&mut serial, "   <no panic message>\r").void_unwrap()
    }
}

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
    
    // Print out panic location
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);
    print_panic_info(serial, &info);
    
    // Blink LED rapidly
    let mut led = pins.d13.into_output();
    loop {
        led.toggle();
        arduino_hal::delay_ms(100);
    }
}

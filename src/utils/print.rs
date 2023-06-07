use core::cell::RefCell;
use avr_device::interrupt;

type Console = arduino_hal::hal::usart::Usart0<arduino_hal::DefaultClock>;
pub static CONSOLE: interrupt::Mutex<RefCell<Option<Console>>> =
    avr_device::interrupt::Mutex::new(RefCell::new(None));

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = $crate::utils::print::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwrite!(console, $($t)*);
                }
            },
        )
    };
}

#[macro_export]
macro_rules! println {
    ($($t:tt)*) => {
        avr_device::interrupt::free(
            |cs| {
                if let Some(console) = $crate::utils::print::CONSOLE.borrow(cs).borrow_mut().as_mut() {
                    let _ = ufmt::uwriteln!(console, $($t)*);
                }
            },
        )
    };
}


pub fn put_console(console: Console) {
    avr_device::interrupt::free(|cs| {
        *CONSOLE.borrow(cs).borrow_mut() = Some(console);
    })
}


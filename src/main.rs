#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(abi_avr_interrupt)]

extern crate arduino_hal;

use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer0Pwm};
use avr_device::interrupt;
use avr_hal_generic::port::{Pin, mode};
use core::cell::RefCell;

mod utils;


type RevButtonPinType = Pin<mode::Input<mode::Floating>, arduino_hal::hal::port::PD2>;
static REV_BUTTON_PIN: interrupt::Mutex<RefCell<Option<RevButtonPinType>>> = interrupt::Mutex::new(RefCell::new(None));

type RevMotorPinType = Pin<mode::PwmOutput<Timer0Pwm>, arduino_hal::hal::port::PD5>;
static REV_MOTOR_PIN: interrupt::Mutex<RefCell<Option<RevMotorPinType>>> = interrupt::Mutex::new(RefCell::new(None));


/// Interrupt handler for INT0 (pin D2)
/// 
/// NOTE: this is wired to the rev button in the blaster
#[avr_device::interrupt(atmega328p)]
#[allow(non_snake_case)]
fn INT0() {
    interrupt::free(|cs| {
        if let Some(rev_pin) = REV_BUTTON_PIN.borrow(cs).borrow_mut().as_mut() {
            if let Some(motor_pin) = REV_MOTOR_PIN.borrow(cs).borrow_mut().as_mut() {
                set_rev_motors(rev_pin, motor_pin)
            }
            else { panic!("Motor pin not available!") }
        }
        else { panic!("Trigger pin not available!") }
    });
}

#[inline(always)]
fn set_rev_motors(rev_pin: &mut RevButtonPinType, motor_pin: &mut RevMotorPinType) {
    const REV_POWER: u8 = 127;
    
    if rev_pin.is_high() {
        motor_pin.enable();
        motor_pin.set_duty(REV_POWER);
    } else {
        motor_pin.disable();
    }
}


fn setup(dp: arduino_hal::Peripherals) -> Pin<mode::Output, arduino_hal::hal::port::PB5> {
    let pins = arduino_hal::pins!(dp);
    
    utils::print::put_console(arduino_hal::default_serial!(dp, pins, 57600));
    
    println!("Setting up firmware...");
    
    let pwm_timer = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    
    // SAFETY: interrupts are disabled so this is safe
    REV_BUTTON_PIN
        .borrow(unsafe{interrupt::CriticalSection::new()})
        .replace(Some(pins.d2.into_floating_input()));
    REV_MOTOR_PIN
        .borrow(unsafe{interrupt::CriticalSection::new()})
        .replace(Some({
            let motor_pin = pins.d5
                .into_output()
                .into_pwm(&pwm_timer);
            // motor_pin.enable();
            motor_pin
        })
    );
    
    // Configure INT0 to trigger when the pin changes
    dp.EXINT.eicra.modify(|_, w| w.isc0().bits(0b01));
    // Enable INT0
    dp.EXINT.eimsk.modify(|_, w| w.int0().set_bit());
    
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

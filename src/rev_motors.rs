use arduino_hal::simple_pwm::{IntoPwmPin, Timer0Pwm};
use avr_device::interrupt;
use avr_hal_generic::port::{Pin, mode};
use core::cell::RefCell;


const REV_POWER: u8 = 127; // 50% duty cycle

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
    if rev_pin.is_high() {
        motor_pin.enable();
        motor_pin.set_duty(REV_POWER);
    } else {
        motor_pin.disable();
    }
}

#[inline(always)]
pub fn setup(
    d2: Pin<mode::Input<mode::Floating>, arduino_hal::hal::port::PD2>,
    d5: Pin<mode::Input<mode::Floating>, arduino_hal::hal::port::PD5>,
    pwm_timer: &Timer0Pwm,
    exint: &arduino_hal::pac::EXINT,
) {
    // SAFETY: interrupts are disabled so this is safe
    REV_BUTTON_PIN
        .borrow(unsafe{interrupt::CriticalSection::new()})
        .replace(Some(d2.into_floating_input()));
    REV_MOTOR_PIN
        .borrow(unsafe{interrupt::CriticalSection::new()})
        .replace(Some({
            let motor_pin = d5
                .into_output()
                .into_pwm(pwm_timer);
            // motor_pin.enable();
            motor_pin
        })
    );
    
    // Configure INT0 to trigger when the pin changes
    exint.eicra.modify(|_, w| w.isc0().bits(0b01));
    // Enable INT0
    exint.eimsk.modify(|_, w| w.int0().set_bit());
}

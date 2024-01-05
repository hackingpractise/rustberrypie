#![no_std]
#![no_main]
//! This is a flat binary that blinks an led light
//! in the specified port.

mod cpu;
mod gpio_pins;
mod memory;

use gpio_pins::GPIO;

#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    const TIME: usize = 0xC350; // about 1 second
    const PIN: u32 = 0x15; // pin 21
                           // Set pin into output.
    GPIO::set_output(PIN);
    loop {
        // turn pin 21 on
        GPIO::set_high(PIN);

        cpu::spin_for_cycles(TIME);

        // turn pin 21 off
        GPIO::clear(PIN);

        cpu::spin_for_cycles(TIME);
    }
}

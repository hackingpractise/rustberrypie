//! This is a flat binary that blinks an led light
//! in the specified port.

#![no_std]
#![no_main]
use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr::read_volatile as read32;
use core::ptr::write_volatile as write32;
// 0x3F20 0008 fsel2 1<<3 turn pin into an output.
// 0x3F20 001C gpio1_set 1<<21 turn pin 21 on.
// 0x3F20 0028 gpio1_clear 1<<21 turn 21 off.
// Constants
const GPIO_FSEL0: u32 = 0x3F20_0000;
const GPIO_FSEL1: u32 = 0x3F20_0004;
const GPIO_FSEL2: u32 = 0x3F20_0008;

const GPIO_SET0: u32 = 0x3F20_001C;
const GPIO_CLEAR0: u32 = 0x3F20_0028;

/// This GPIO struct is use to interface with the gpio pin in
/// the  Broadcom BCM2835 cpu in the raspberrypi w
#[allow(clippy::upper_case_acronyms)]
struct GPIO;

impl GPIO {
    pub fn set_output(pin: u32) {
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Something went wrong."),
        };
        let mut val: u32;
        unsafe {
            val = read32(register as *mut u32);
        }
        // Create a mask.
        let mut mask: u32 = 0b111;
        // Shift the mask to the right loocation.
        let pinnum = pin % 10;
        mask <<= pinnum * 3;
        // and in the not of the mask.
        val &= !(mask);
        // Set OUR value.
        val |= 1 << (pinnum * 3);

        unsafe {
            write32(register as *mut u32, val);
        }
    }
    pub fn set_high(pin: u32) {
        let bitpos = pin;
        let mut val: u32;
        unsafe {
            val = read32(GPIO_SET0 as *mut u32);
        }
        val |= 1 << bitpos;
        unsafe {
            write32(GPIO_SET0 as *mut u32, val);
        }
    }

    pub fn clear(pin: u32) {
        let bitpos = pin;
        let mut val: u32;
        unsafe {
            val = read32(GPIO_CLEAR0 as *mut u32);
        }
        val |= 1 << bitpos;
        unsafe {
            write32(GPIO_CLEAR0 as *mut u32, val);
        }
    }
}
#[no_mangle]
#[link_section = ".text._start"]
pub extern "C" fn _start() -> ! {
    const TIME: u32 = 0xC350; // about 1 second
    const PIN: u32 = 0x15; // pin 21
                           // Set pin into output.
    GPIO::set_output(PIN);
    loop {
        // turn pin 21 on
        GPIO::set_high(PIN);
        unsafe {
            for _ in 1..TIME {
                asm!("nop");
            }
        }
        // turn pin 21 off
        GPIO::clear(PIN);
        unsafe {
            for _ in 1..TIME {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

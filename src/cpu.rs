use core::{arch::asm, panic::PanicInfo};

pub fn wait_forever() -> ! {
    loop {
        unsafe {
            asm!("wfe", options(nomem, nostack));
        }
    }
}

pub fn spin_for_cycles(n: usize) {
    for _ in 0..n {
        unsafe {
            asm!("nop", options(nomem, nostack));
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    wait_forever()
}

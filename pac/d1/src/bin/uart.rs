#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    "
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top:
"
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let peripherals = d1_pac::Peripherals::take().unwrap();
    // let str = b"Hello, bare metal";
    for byte in 32..=126u8 {
        peripherals
            .UART0
            .thr()
            .write(|w| unsafe { w.thr().bits(byte) });
    }
}

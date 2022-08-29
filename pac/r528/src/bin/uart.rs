#![no_std]
#![no_main]

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    r#"
    .globl _start
    .extern BOOT_STACK_TOP

    .section ".text.entry"
_start:
    ldr r0, =BOOT_STACK_TOP
    mov sp, r0
    bl rust_main
"#
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_main() {
    let peripherals = unsafe { r528_pac::Peripherals::steal() };
    // let str = b"Hello, bare metal";
    for byte in 32..=126u8 {
        peripherals
            .UART0
            .thr()
            .write(|w| unsafe { w.thr().bits(byte) });
    }
    panic!()
}

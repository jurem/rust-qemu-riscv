#![no_std]
#![no_main]

#[panic_handler]
fn on_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod uart;

#[no_mangle]
#[link_section = ".text.init"]
unsafe extern "C" fn _start() -> ! {
    use core::arch::asm;
    asm!(
        ".option push",
        ".option norelax",
        "la gp, _global_pointer",
        // stack pointer
        "la sp, _init_stack_top",
        "tail {entry}",
        entry = sym entry,
        options(noreturn)
    );
}

extern "C" fn entry() -> ! {
    let mut uart = unsafe { uart::UART::new(0x1000_0000) };

      for byte in "Hello, world!\n".bytes() {
        uart.put_ch(byte);
      }


    loop {}
}

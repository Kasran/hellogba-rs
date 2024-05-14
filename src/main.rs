#![no_std]
#![no_main]

use core::arch::global_asm;

// how does no-std programming work

global_asm!(include_str!("entry.s"));

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    // code go hear
    unsafe {
        // let ime: *mut u8 = 0x04000208 as *mut u8;
        // ime.write_volatile(0);

        let ioram: *mut u8 = 0x04000000 as *mut u8;
        ioram.write_volatile(0x03);
        ioram.offset(1).write_volatile(0x04);

        let vram: *mut u16 = 0x06000000 as *mut u16;
        vram.offset(80*240 + 115).write_volatile(0b00000_00000_11111);
        vram.offset(80*240 + 120).write_volatile(0b00000_11111_00000);
        vram.offset(80*240 + 125).write_volatile(0b11111_00000_00000);
    }
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
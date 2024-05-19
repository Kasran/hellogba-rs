#![no_std]
#![no_main]

use core::arch::global_asm;

// stolen from https://github.com/rust-console/gba/blob/main/src/lib.rs#L123
#[derive(Debug)]
#[repr(C, align(4))]
pub struct Align4<T>(pub T);
impl<const N: usize> Align4<[u8; N]> {
    #[inline]
    #[must_use]
    pub fn as_u16_slice(&self) -> &[u16] {
        assert!(self.0.len() % 2 == 0);
        // Safety: our struct is aligned to 4, so the pointer will already be
        // aligned, we only need to check the length
        unsafe {
            let data: *const u8 = self.0.as_ptr();
            let len: usize = self.0.len();
            core::slice::from_raw_parts(data.cast::<u16>(), len / 2)
        }
    }
}

macro_rules! static_include_bytes {
    ($(#[$m:meta])* $name:ident = $path:expr) => {
        $(#[$m])*
        static $name: Align4<[u8; include_bytes!($path).len()]> = Align4(*include_bytes!($path));
    };
}

static_include_bytes!(TESTTILE = "tile.bin");

// how does no-std programming work

global_asm!(include_str!("entry.s"));

#[export_name = "main"]
pub extern "C" fn main() -> ! {
    // code go hear
    tiles_test();
}

fn pixels() -> ! {
    unsafe {
        // let ime: *mut u8 = 0x04000208 as *mut u8;
        // ime.write_volatile(0);

        let ioram: *mut u8 = 0x0400_0000 as *mut u8;
        ioram.write_volatile(0x03);           // BG mode 3 (one bitmap screen)
        ioram.offset(1).write_volatile(0x04); // enable BG layer 2

        let vram: *mut u16 = 0x0600_0000 as *mut u16;
        vram.offset(80*240 + 115).write_volatile(0b00000_00000_11111);  // red
        vram.offset(81*240 + 116).write_volatile(0b00000_11111_00000);  // green
        vram.offset(82*240 + 117).write_volatile(0b11111_00000_00000);  // blue
    }
    loop {}
}

fn tiles_test() -> ! {
    unsafe {
        let ioram: *mut u8 = 0x0400_0000 as *mut u8;
        ioram.write_volatile(0x00);           // BG mode 0 (four layers with tiles n stuff)
        ioram.offset(1).write_volatile(0x01); // enable BG layer 0

        // BG control registers
        let bg0ctrl: *mut u8 = ioram.offset(8);  // 0x04000008
        bg0ctrl.write_volatile(
            0b1_0_00_00_00    // bits layout:
        //    | | || || ^^ 0-1   BG Priority           (0-3, 0=Highest)
        //    | | || ^^    2-3   Character Base Block  (0-3, in units of 16 KBytes) (=BG Tile Data)
        //    | | ^^       4-5   Not used (must be zero)
        //    | ^          6     Mosaic                (0=Disable, 1=Enable)
        //    ^            7     Colors/Palettes       (0=16/16, 1=256/1)
        );
        bg0ctrl.offset(1).write_volatile(
            0b00_0_10000    // bits layout:
        //    || | ^^^^^ 8-12  Screen Base Block     (0-31, in units of 2 KBytes) (=BG Map Data)
        //    || ^       13    Display Area Overflow (0=Transparent, 1=Wraparound; BG2CNT/BG3CNT only)
        //    ^^         14-15 Screen Size (0-3)
        );
        // layout notes taken from https://www.akkit.org/info/gbatek.htm#lcdiobgcontrol
        // in this case, the base chrblock is at 0x0600_0000 (0)
        //           and the base scrblock is at 0x0600_8000 (16)

        // i think normally you'd do some kind of bulk memory copy from rom into palette
        //   ram and vram, but we'll just write some values in to make it work

        let palette: *mut u16 = 0x0500_0000 as *mut u16;
        palette.write_volatile(0);
        palette.offset(1).write_volatile(0b11111_00000_00000);
        palette.offset(2).write_volatile(0b11111_10000_10000);
        palette.offset(3).write_volatile(0b11111_11000_11000);

        // color mode is 256-color so we're using 8bpp
        // so one tile is 8*8 = 64 bytes
        // and one row is 8 bytes
        let chrblock: *mut u16 = 0x0600_0000 as *mut u16;
        // naively writes my test tile thingy to chrblock ram
        // in a real application you'd probably wanna use CpuSet or DMA for this
        for (i, bb) in TESTTILE.as_u16_slice().into_iter().enumerate() {
            chrblock.offset(0x20 + i as isize).write_volatile(*bb);
        }

        // each tile entry is a u16 laid out like this:
        // 0b0000_0_0_0000000000
        //   |||| | | ^^^^^^^^^^ tile index (into chrblock)
        //   |||| | ^            horizontal flip
        //   |||| ^              vertical flip
        //   ^^^^                palette bank (in 16-color mode)
        let scrblock: *mut u16 = 0x0600_8000 as *mut u16;
        // scrblock.offset(1).write_volatile(0b0000_0_0_0000000001);
        for off in [32*1+1, 32*1+4, 32*2+1, 32*2+4, 32*4+1, 32*4+4, 32*5+2, 32*5+3] {
            scrblock.offset(off).write_volatile(0b0000_0_0_0000000001);
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}
#![no_std]
#![no_main]

/// This method is called when a panic occurs
#[panic_handler]
fn panic(_infos: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Entry point of OS
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let message = b"Welcome in StitchOS";
    let vga_buffer = 0xb8000 as *mut u8;

    for (index, &byte) in message.iter().enumerate() {
        let position_buffer: isize = index as isize * 2;

        unsafe {
            *vga_buffer.offset(position_buffer) = byte;
            *vga_buffer.offset(position_buffer + 1) = 0xb;
        }
    }

    loop {}
}

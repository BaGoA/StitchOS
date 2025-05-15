#![no_std]
#![no_main]

mod vga_buffer;

/// This method is called when a panic occurs
#[panic_handler]
fn panic(_infos: &core::panic::PanicInfo) -> ! {
    loop {}
}

/// Entry point of OS
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let mut writer = vga_buffer::Writer::new(vga_buffer::Color::Green, vga_buffer::Color::Black);

    writer.write_byte(b'W');
    writer.write_string("elcome to StitchOS");

    loop {}
}

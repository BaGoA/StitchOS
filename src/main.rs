#![no_std]
#![no_main]

mod vga_buffer;

/// This method is called when a panic occurs
#[panic_handler]
fn panic(infos: &core::panic::PanicInfo) -> ! {
    println!("{}", infos);
    loop {}
}

/// Entry point of OS
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Welcome to StitchOS");
    println!("version {}.{}.{}", 0, 0, 1);
    println!("Let's get started");

    loop {}
}

use core::panic::PanicInfo;
use super::exit;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("\r\nPanic!");
    exit(0);
}
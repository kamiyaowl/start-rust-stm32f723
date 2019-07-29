#![no_std]
#![no_main]

use stm32f7::stm32f7x3;
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO: Show Error Info here
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    main();
    loop{}
}
fn main() {
    let mut peripherals = stm32f7x3::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    gpioa.odr.modify(|_, w| w.odr0().set_bit());
}

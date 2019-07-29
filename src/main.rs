#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;
use stm32f7::stm32f7x3;

#[entry]
fn main() -> ! {
    let mut peripherals = stm32f7x3::Peripherals::take().unwrap();
    let gpioa = &peripherals.GPIOA;
    gpioa.odr.modify(|_, w| w.odr0().set_bit());
    loop {}
}

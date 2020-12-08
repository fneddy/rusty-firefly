#![no_std]
#![feature(lang_items)]
#![feature(llvm_asm)]

// Expose hal & pac crates
pub use atmega328p_hal as hal;
pub use crate::hal::pac;

#[cfg(feature = "rt")]
pub use crate::hal::entry;

pub use crate::hal::pac::Peripherals;
pub mod prelude {
    pub use crate::hal::prelude::*;
    pub use crate::hal::usart::BaudrateExt as _;
}


pub mod pins;
pub use pins::*;

pub type Delay = hal::delay::Delay<hal::clock::MHz8>;

/// Wait (busy spin) for `ms` milliseconds
pub fn delay_ms(ms: u16) {
    use prelude::*;

    Delay::new().delay_ms(ms)
}

/// Wait (busy spin) for `us` microseconds
pub fn delay_us(us: u16) {
    use prelude::*;

    Delay::new().delay_us(us)
}

pub type Serial0<IMODE> = hal::usart::Usart0<hal::clock::MHz8, IMODE>;
pub type Serial1<IMODE> = hal::usart::Usart1<hal::clock::MHz8, IMODE>;

pub mod cpu_prescaler;

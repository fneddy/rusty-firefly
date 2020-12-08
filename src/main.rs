#![no_std]
#![no_main]

use rusty_firefly::prelude::*;
use panic_halt as _;

use rusty_firefly::pins;
use rusty_firefly::cpu_prescaler;


#[rusty_firefly::entry]
fn main() -> ! {
    cpu_prescaler::set();
    let dp = rusty_firefly::Peripherals::take().unwrap();
    let mut pins = rusty_firefly::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD , dp.PORTE);
    
    let mut led = pins.led.into_output(&mut pins.ddr);

    led.set_high().void_unwrap();
    
    let mut serial0 = rusty_firefly::Serial0::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );
    let mut serial1 = rusty_firefly::Serial1::new(
        dp.USART1,
        pins.b4,
        pins.b3.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );

    ufmt::uwriteln!(&mut serial0, "Hello from Firefly RS485!\r").void_unwrap();
    ufmt::uwriteln!(&mut serial1, "Hello from Firefly UART!\r").void_unwrap();
    

    loop{
        led.toggle().void_unwrap();
        rusty_firefly::delay_ms(200);
        led.toggle().void_unwrap();
        rusty_firefly::delay_ms(200);
        led.toggle().void_unwrap();
        rusty_firefly::delay_ms(200);
        led.toggle().void_unwrap();
        rusty_firefly::delay_ms(800);
    }
}

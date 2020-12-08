use crate::hal::port::PortExt;

avr_hal_generic::impl_board_pins! {
    #[port_defs]
    use crate::hal::port;

    /// Generic DDR that works for all ports
    pub struct DDR {
        portb: crate::pac::PORTB,
        portc: crate::pac::PORTC,
        portd: crate::pac::PORTD,
        porte: crate::pac::PORTE,
    }

    pub struct Pins {
		pub led: portb::pb0::PB0,
		pub d0: portd::pd0::PD0,
		pub d1: portd::pd1::PD1,
		pub b4: portb::pb4::PB4,
		pub b3: portb::pb3::PB3,
	}
}
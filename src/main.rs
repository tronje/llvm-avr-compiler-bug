#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal::clock::MHz10;
use atmega_hal::port::mode::{AnyInput, Input, Output};
use atmega_hal::port::{Pin, PD0, PD1};
use atmega_hal::usart::BaudrateExt;
use avr_device::atmega164pa::USART0;
use core::panic::PanicInfo;

type Usart<USART, RX, TX> = atmega_hal::usart::Usart<USART, RX, TX, MHz10>;
type Usart0 = Usart<USART0, Pin<Input<AnyInput>, PD0>, Pin<Output, PD1>>;
type Usart0Writer =
    atmega_hal::usart::UsartWriter<USART0, Pin<Input<AnyInput>, PD0>, Pin<Output, PD1>, MHz10>;

static mut LOGGER: Option<Usart0Writer> = None;

fn logger_init(logger: Usart0Writer) {
    unsafe {
        LOGGER.replace(logger);
    }
}

fn logger_get() -> &'static mut Usart0Writer {
    unsafe { LOGGER.as_mut().unwrap() }
}

macro_rules! log {
    ($($arg:tt)*) => {
        {
            let logger = crate::logger_get();
            ufmt::uwriteln!(logger, $($arg)*).ok();
        }
    };
}

struct State {
    a: u32,
    b: u32,
    c: u32,
    d: u32,

    #[allow(unused)]
    padding: [bool; 49],
}

impl State {
    pub fn new() -> Self {
        Self {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            padding: [true; 49],
        }
    }

    #[inline(never)]
    pub fn run(self) -> ! {
        log!("{}\r", self.a);
        log!("{}\r", self.b);
        log!("{}\r", self.c);
        log!("{}\r", self.d);

        if self.d != 0 {
            log!("BUG\r");
        }

        loop {}
    }
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[avr_device::entry]
fn main() -> ! {
    avr_device::interrupt::disable();

    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let serial = Usart0::new(
        dp.USART0,
        pins.pd0,
        pins.pd1.into_output(),
        BaudrateExt::into_baudrate(1200u32),
    );

    let (_, writer) = serial.split();
    logger_init(writer);

    let state = State::new();
    state.run()
}

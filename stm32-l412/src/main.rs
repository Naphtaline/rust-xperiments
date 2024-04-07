#![no_std]
#![no_main]

use cortex_m::asm::nop;
use cortex_m_rt::entry;
use cortex_m::delay::Delay;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

use hal::{
    self,
    clocks::{Clocks, InputSrc},
    gpio::{Pin, PinMode, Port, PinState},
    pac,
};

#[entry]
fn main() -> ! {
    // Set up CPU peripherals
    let cp = cortex_m::Peripherals::take().unwrap();
    // Set up microcontroller peripherals
    let mut dp = pac::Peripherals::take().unwrap();

    let clock_cfg = Clocks::default();

    // Write the clock configuration to the MCU. If you wish, you can modify `clocks` above
    // in accordance with [its docs](https://docs.rs/stm32-hal2/0.2.0/stm32_hal2/clocks/index.html),
    // and the `clock_cfg` example.
    clock_cfg.setup().unwrap();

    // Setup a delay, based on the Cortex-m systick.
    let mut delay = Delay::new(cp.SYST, clock_cfg.systick());

    let mut led = Pin::new(Port::B, 3, PinMode::Output);
    let mut jumper = Pin::new(Port::A, 12, PinMode::Input);

    led.set_low();
    jumper.set_low();

    // init rtt loger
    rtt_init_print!();

    loop {
        rprintln!("Hello !");

        if jumper.is_high() {
            led.set_high();
        }
        else {
            led.set_low();
        }
        delay.delay_ms(1000);
    }
}

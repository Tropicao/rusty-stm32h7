#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32h7xx_hal as hal;
use crate::hal::{
    pac,
    prelude::*,
    hal::digital::v2::InputPin,
    hal::digital::v2::OutputPin,
    delay::Delay
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(64.mhz()).freeze(pwrcfg, &dp.SYSCFG);

    let gpioe = dp.GPIOE.split(ccdr.peripheral.GPIOE);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let button = gpioe.pe9.into_pull_up_input();
    let mut led_controller_reset = gpioe.pe15.into_push_pull_output();

    let mut led_controller = dp.I2C2.i2c(
        (
            gpiob.pb10.into_alternate_af4().set_open_drain(),
            gpiob.pb11.into_alternate_af4().set_open_drain(),
        ),
        100.khz(),
        ccdr.peripheral.I2C2,
        &ccdr.clocks,
    );

    let mut delay = Delay::new(cp.SYST, ccdr.clocks);
    led_controller_reset.set_low().unwrap();
    delay.delay_ms(5 as u16);
    led_controller_reset.set_high().unwrap();

    // Configure prescaler
    let prescaler_data = [0x01, 0x00];
    led_controller.write(0x60, &prescaler_data).unwrap();
    let prescaler_data = [0x03, 0x00];
    led_controller.write(0x60, &prescaler_data).unwrap();

    // Set green led on
    let led_command = [0x05, 0x00];
    led_controller.write(0x60, &led_command).unwrap();
    let mut count = 0;
    let mut sys_led = false;
    loop {
        let button_state = button.is_low().unwrap();
        let buffer = [0x05, if button_state {1} else {0}];
        led_controller.write(0x60, &buffer).unwrap();
        count += 1;
        if count == 5 {
            count = 0;
            let buffer = [0x06, if sys_led { 0x10 } else { 0x00}];
            led_controller.write(0x60, &buffer).unwrap();
            sys_led = !sys_led;
        }
        delay.delay_ms(100_u16);
    }
}

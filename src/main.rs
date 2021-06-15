#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use crate::hal::{
    delay::DelayFromCountDownTimer, hal::digital::v2::InputPin, hal::digital::v2::OutputPin, pac,
    prelude::*,
};
use cortex_m_rt::entry;
use stm32h7xx_hal as hal;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clock tree with HSI and PLL to reach 200Mhz
    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(200.mhz()).freeze(pwrcfg, &dp.SYSCFG);
    // Configure timer for delays
    let timer2 = dp.TIM2.timer(100.ms(), ccdr.peripheral.TIM2, &ccdr.clocks);
    let mut delay = DelayFromCountDownTimer::new(timer2);
    // Configure I2C for led driver and GPIO for buttons
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
    led_controller_reset.set_low().unwrap();
    delay.delay_ms(5 as u16);
    led_controller_reset.set_high().unwrap();

    // Configure led driver
    let prescaler_data = [0x01, 0x00];
    led_controller.write(0x60, &prescaler_data).unwrap();
    let prescaler_data = [0x03, 0x00];
    led_controller.write(0x60, &prescaler_data).unwrap();

    // Set green led on
    let led_command = [0x05, 0x00];
    led_controller.write(0x60, &led_command).unwrap();
    let mut count = 0;
    let mut sys_led = false;

    // Loop : blink red led each second, set green led to follow button state
    loop {
        let button_state = button.is_low().unwrap();
        let buffer = [0x05, if button_state { 1 } else { 0 }];
        led_controller.write(0x60, &buffer).unwrap();
        count = match count {
            0 => {
                let buffer = [0x06, if sys_led { 0x00 } else { 0x10 }];
                led_controller.write(0x60, &buffer).unwrap();
                sys_led = ! sys_led;
                count + 1
            }
            5 => {
                0
            },
            _ => count + 1
        };
        delay.delay_ms(100_u16);
    }
}

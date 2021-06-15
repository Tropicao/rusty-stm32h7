# rusty-stm32h7
Poking around with Rust on STM32H750

## Build
Run `cargo build`

## Flash
* Convert elf file to raw binary : `arm-none-eabi-objcopy -O binary target/thumbv7em-none-eabihf/debug/inno-rust inno-rust.bin`
* Download STM32CubeProgrammer tool, plug ST-Link to target hardware
* Execute provided flashing script : `./flash_target.sh inno-rust.bin`
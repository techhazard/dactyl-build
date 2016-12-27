# Building my dactyl keyboard

Here I document my progress building my "endgame" keyboard.

[See the wiki.](https://github.com/awesomefireduck/dactyl-build/wiki)

## Requirements

* rustc nightly
* cargo
* arm-none-eabi-gcc
* arm-none-eabi-ar
* arm-none-eabi-objcopy

## Compile and upload

```bash
cargo build --target thumbv7em-none-eabi
arm-none-eabi-objcopy -O ihex -R .eeprom target/thumbv7em-none-eabi/debug/blink dactyl.hex

echo "Reset teensy now"
teensy-loader-cli -w --mcu=mk20dx256 dactyl.hex
```

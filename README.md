# Project summary
UART test on the Adafruit Feather M4. Written in Rust

# Goals
- Write a functioning UART driver for the Adafruit Feather M4 in Rust
- Learn Rust
- Get deeper into embedded programming

# Sources
- [Feather M4 BSP blinky_basic Example](https://github.com/atsamd-rs/atsamd/blob/master/boards/feather_m4/examples/blinky_basic.rs)
- [Feather M4 BSP neopixel_rainbow Example](https://github.com/atsamd-rs/atsamd/blob/master/boards/feather_m4/examples/neopixel_rainbow.rs)

# Prerequisite set up
- Install `cargo-hf2` (`cargo install cargo-hf2`)
- Set up `udev` rules for the Feather M4 bootloader device
    - Add the following to `/etc/udev/rules.d/99-adafruit-boards.rules`:

        ```
        ATTRS{idVendor}=="239a", ENV{ID_MM_DEVICE_IGNORE}="1"
        SUBSYSTEM=="usb", ATTRS{idVendor}=="239a", MODE="0666"
        SUBSYSTEM=="tty", ATTRS{idVendor}=="239a", MODE="0666"
        ```

    - Run:

        ```
        sudo udevadm control --reload-rules
        sudo udevadm trigger
        ```

# Building and flashing the program 
1. Push reset button on the Feather M4 twice in quick succession.
2. Run:

    ```
    cargo hf2 --release
    ```

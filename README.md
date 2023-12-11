# esp-backtrace - backtrace for ESP32 bare-metal

Supports the ESP32, ESP32-C2/C3/C6, ESP32-S2/S3 and ESP32H2. Optional exception and panic handlers are included, both of which can be enabled via their respective features.

Please note that when targeting a RISC-V device, you **need** to force frame pointers (i.e. `"-C", "force-frame-pointers",` in your `.cargo/config.toml`); this is **not** required for Xtensa.

You can get an array of backtrace addresses (currently limited to 10) via `arch::backtrace()` if
you want to create a backtrace yourself (i.e. not using the panic or exception handler).

When using the panic and/or exception handler make sure to include `use esp_backtrace as _;`.

When using this together with `esp-println` make sure to use the same output kind for both dependencies.
(Or don't specify the output for `esp-backtrace`)

## Features

| Feature           | Description                                                          |
| ----------------- | -------------------------------------------------------------------- |
| esp32             | Target ESP32                                                         |
| esp32c2           | Target ESP32-C2                                                      |
| esp32c3           | Target ESP32-C3                                                      |
| esp32c6           | Target ESP32-C6                                                      |
| esp32s2           | Target ESP32-S2                                                      |
| esp32s3           | Target ESP32-S3                                                      |
| esp32h2           | Target ESP32-H2                                                      |
| panic-handler     | Include a panic handler, will add `esp-println` as a dependency      |
| exception-handler | Include an exception handler, will add `esp-println` as a dependency |
| print-uart        | Use UART to print messages\*                                         |
| print-jtag-serial | Use JTAG-Serial to print messages\*                                  |
| print-rtt         | Use RTT to print messages\*                                          |
| colors            | Print messages in red\*                                              |
| halt-cores        | Halt both CPUs on ESP32 / ESP32-S3 in case of a panic or exception   |

\* _only used for panic and exception handlers_

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.

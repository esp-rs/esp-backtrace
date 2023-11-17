fn main() {
    // Ensure that only a single chip is specified.
    let chip_features = [
        cfg!(feature = "esp32"),
        cfg!(feature = "esp32c2"),
        cfg!(feature = "esp32c3"),
        cfg!(feature = "esp32c6"),
        cfg!(feature = "esp32s2"),
        cfg!(feature = "esp32s3"),
        cfg!(feature = "esp32h2"),
    ];

    match chip_features.iter().filter(|&&f| f).count() {
        1 => {}
        n => panic!("Exactly 1 chip must be enabled via its Cargo feature, {n} provided"),
    };

    // Ensure that only a single communication method is specified.
    let method_features = [
        cfg!(feature = "print-jtag-serial"),
        cfg!(feature = "print-rtt"),
        cfg!(feature = "print-uart"),
    ];

    match method_features.iter().filter(|&&f| f).count() {
        1 => {}
        n => panic!(
            "Exactly 1 communication method must be enabled via its Cargo feature, {n} provided"
        ),
    }

    // Ensure that, if the `print-jtag-serial` communication method feature is
    // enabled, either the `esp32c3`, `esp32c6`, or `esp32s3` chip feature is
    // enabled.
    if cfg!(feature = "print-jtag-serial")
        && !(cfg!(feature = "esp32c3")
            || cfg!(feature = "esp32c6")
            || cfg!(feature = "esp32s3")
            || cfg!(feature = "esp32h2"))
    {
        panic!(
            "The `print-jtag-serial` feature is only supported by the ESP32-C3, ESP32-C6, ESP32-S3 and ESP32-H2 chips"
        );
    }

    if cfg!(feature = "semihosting") && cfg!(feature = "halt-cores") {
        panic!("The features semihosting + halt-cores are exclusive");
    }

    if cfg!(feature = "semihosting") && cfg!(target_arch = "xtensa") {
        panic!("Semihosting is not supported on xtensa targets");
    }
}

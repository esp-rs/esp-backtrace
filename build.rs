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
}

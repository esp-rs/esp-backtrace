fn main() {
    // Ensure that only a single chip is specified.
    let chip_features = [
        cfg!(feature = "esp32"),
        cfg!(feature = "esp32c2"),
        cfg!(feature = "esp32c3"),
        cfg!(feature = "esp32c6"),
        cfg!(feature = "esp32h2"),
        cfg!(feature = "esp32p4"),
        cfg!(feature = "esp32s2"),
        cfg!(feature = "esp32s3"),
    ];

    match chip_features.iter().filter(|&&f| f).count() {
        1 => {}
        n => panic!("Exactly 1 chip must be enabled via its Cargo feature, {n} provided"),
    };

    // Ensure that exactly a backend is selected
    let backend = [cfg!(feature = "println"), cfg!(feature = "defmt")];

    if backend.iter().filter(|&&f| f).count() == 0 {
        panic!("A backend needs to be selected");
    }

    if cfg!(feature = "custom-halt") && cfg!(feature = "halt-cores") {
        panic!("Only one of `custom-halt` and `halt-cores` can be enabled");
    }

    if is_nightly() {
        println!("cargo:rustc-cfg=nightly");
    }
}

fn is_nightly() -> bool {
    let version_output = std::process::Command::new(
        std::env::var_os("RUSTC").unwrap_or_else(|| std::ffi::OsString::from("rustc")),
    )
    .arg("-V")
    .output()
    .unwrap()
    .stdout;
    let version_string = String::from_utf8_lossy(&version_output);

    version_string.contains("nightly")
}

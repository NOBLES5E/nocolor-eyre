#[cfg(target_arch = "wasm32")]
#[wasm_bindgen_test::wasm_bindgen_test]
pub fn nocolor_eyre_simple() {
    use nocolor_eyre::eyre::WrapErr;
    use nocolor_eyre::*;

    install().expect("Failed to install nocolor_eyre");
    let err_str = format!(
        "{:?}",
        Err::<(), Report>(eyre::eyre!("Base Error"))
            .note("A note")
            .suggestion("A suggestion")
            .wrap_err("A wrapped error")
            .unwrap_err()
    );
    // Print it out so if people run with `-- --nocapture`, they
    // can see the full message.
    println!("Error String is:\n\n{}", err_str);
    assert!(err_str.contains("A wrapped error"));
    assert!(err_str.contains("A suggestion"));
    assert!(err_str.contains("A note"));
    assert!(err_str.contains("Base Error"));
}

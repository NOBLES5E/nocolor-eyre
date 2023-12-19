#[cfg(feature = "track-caller")]
#[test]
fn disabled() {
    use eyre::eyre;
    use nocolor_eyre::eyre;

    nocolor_eyre::config::HookBuilder::default()
        .display_location_section(false)
        .install()
        .unwrap();

    let report = eyre!("error occured");

    let report = format!("{:?}", report);
    assert!(!report.contains("Location:"));
}

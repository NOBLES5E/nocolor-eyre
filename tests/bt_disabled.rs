use eyre::eyre;
use nocolor_eyre::eyre;

#[test]
fn disabled() {
    nocolor_eyre::config::HookBuilder::default()
        .display_env_section(false)
        .install()
        .unwrap();

    let report = eyre!("error occured");

    let report = format!("{:?}", report);
    assert!(!report.contains("RUST_BACKTRACE"));
}

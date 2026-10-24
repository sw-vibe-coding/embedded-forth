/// Build identity shared by the Forth word and the CLI's detailed version.
pub fn build_version() -> String {
    format!(
        "embedded-forth {}\nBuild host: {}\nBuild commit: {}\nBuild time: {}",
        env!("CARGO_PKG_VERSION"),
        env!("FORTH_BUILD_HOST"),
        env!("FORTH_BUILD_COMMIT"),
        env!("FORTH_BUILD_TIME"),
    )
}

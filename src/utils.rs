#[cfg(feature = "std")]
/// Gets the current timestamp in seconds since the Unix epoch.
pub fn timestamp() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or(std::time::Duration::from_secs(0))
        .as_secs()
}

#[cfg(not(feature = "std"))]
/// If no_std, just return 0
pub fn timestamp() -> u64 {
    0
}

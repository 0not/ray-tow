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

pub fn sqrt(x: f64) -> f64 {
    #[cfg(not(feature = "std"))]
    {
        libm::sqrt(x)
    }
    #[cfg(feature = "std")]
    {
        x.sqrt()
    }
}

pub fn abs(x: f64) -> f64 {
    #[cfg(not(feature = "std"))]
    {
        libm::fabs(x)
    }
    #[cfg(feature = "std")]
    {
        x.abs()
    }
}

pub fn powi(x: f64, n: i32) -> f64 {
    #[cfg(not(feature = "std"))]
    {
        libm::pow(x, n as f64)
    }
    #[cfg(feature = "std")]
    {
        x.powi(n)
    }
}

use crate::result::CrResult;
use log::LevelFilter;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Eq, FromPrimitive)]
pub enum CrLogLevel {
    /// Log nothing
    None = 0,
    /// Log everything that is at least on `error`
    Error,
    /// Log everything that is at least a `warning`
    Warn,
    /// Log everything that is at least an `info`
    Info,
    /// Log everything that is at least a `debug information`
    Debug,
    /// Log everything
    Trace,
}

impl From<CrLogLevel> for LevelFilter {
    fn from(value: CrLogLevel) -> Self {
        match value {
            CrLogLevel::None => LevelFilter::Off,
            CrLogLevel::Error => LevelFilter::Error,
            CrLogLevel::Warn => LevelFilter::Warn,
            CrLogLevel::Info => LevelFilter::Info,
            CrLogLevel::Debug => LevelFilter::Debug,
            CrLogLevel::Trace => LevelFilter::Trace,
        }
    }
}

/// Enables logging for internal events using the given [`CrLogLevel`] as filter level. This
/// operation only succeeds, if the logger is not configured yet. Changing the [`CrLogLevel`] is not
/// possible.
///
/// # Safety
///
/// The given [`CrLogLevel`] must be valid.
#[no_mangle]
pub unsafe extern "C" fn cr_configure_logger(level: CrLogLevel) -> CrResult {
    if common_log_build_info::common_log::logger::init(
        env!("CARGO_PKG_NAME"),
        LevelFilter::from(level).into(),
    )
    .is_err()
    {
        CrResult::ErrLoggerAlreadyConfigured
    } else {
        common_log_build_info::log_start_and_build_info!(env!("CARGO_PKG_NAME"));
        CrResult::Ok
    }
}

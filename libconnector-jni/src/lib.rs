#[cfg(target_os = "android")]
mod android;

macro_rules! cstring_or_utf8_error {
    ($env:expr, $jstring:expr) => {
        if let Some(jstring) = $env
            .get_string($jstring)
            .ok()
            .and_then(|s| s.to_str().ok().and_then(|s| std::ffi::CString::new(s).ok()))
        {
            jstring
        } else {
            return connector::result::CrResult::ErrParameterIsNotValidUtf8 as jlong;
        }
    };
}

mod application_info;
mod connection;
mod connection_config;
mod connection_info;
mod connector;
mod connector_info;
mod conversion;
mod logging;
mod message;
mod message_detailed;
mod result_exception;
mod version;

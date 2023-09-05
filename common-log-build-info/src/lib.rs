pub use common_build_info;
pub use common_log;

#[macro_export]
macro_rules! log_start_and_build_info {
    ($bin_name:expr) => {{
        let info = $crate::common_build_info::env::HostEnv::detect();
        $crate::common_log::info!(
            "Starting {} of BMWi project LUKAS @ IT-Designers GmbH, detected {} logical CPU cores",
            $bin_name,
            info.number_cpus_logical(),
        );
        $crate::common_log::info!("Build {}", $crate::common_build_info::info::simple());
        info
    }};
    () => {{
        $crate::log_start_and_build_info!(env!("CARGO_BIN_NAME"));
    }};
}

#[macro_export]
macro_rules! init_logger_and_log_start {
    ($name:expr, $level:expr) => {{
        $crate::common_log::logger::init($name, Some($level)).unwrap();
        $crate::log_start_and_build_info!($name)
    }};
    ($level:expr) => {{
        $crate::common_log::logger::init(env!("CARGO_BIN_NAME"), Some($level)).unwrap();
        $crate::log_start_and_build_info!()
    }};
    () => {{
        $crate::common_log::logger::init(env!("CARGO_BIN_NAME"), None).unwrap();
        $crate::log_start_and_build_info!()
    }};
}

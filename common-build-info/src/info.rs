use messages::itd_data_protocol::BuildInfo;
use crate::{BUILT_TIME_UTC, CI_PLATFORM, GIT_VERSION, PROFILE, RUSTC_VERSION};

pub fn load() -> BuildInfo {
    BuildInfo {
        git_hash: GIT_VERSION.unwrap_or("<unversioned>").to_string(),
        ci_platform: CI_PLATFORM.unwrap_or("local").to_string(),
        profile: PROFILE.to_string(),
        time: BUILT_TIME_UTC.to_string(),
        compiler: RUSTC_VERSION.to_string(),
    }
}

pub fn simple() -> String {
    let info = load();
    format!(
        "{}, {}, {}, {}, {}",
        info.git_hash, info.ci_platform, info.profile, info.time, info.compiler,
    )
}

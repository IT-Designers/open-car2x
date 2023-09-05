use log::Level;
use ndk::looper::ForeignLooper;
use ndk::looper::ThreadLooper;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::os::unix::prelude::*;
use std::sync::{Arc, Condvar, Mutex};

lazy_static::lazy_static! {
    static ref LOOPER: Mutex<Option<ForeignLooper>> = Default::default();
}

/// https://github.com/rust-windowing/android-ndk-rs/blob/66eb8ca2a2f13713b97442c428c489cf28db6dc1/ndk-glue/src/lib.rs#L170
pub(crate) fn setup_stdio_redirection() {
    let mut pipe: [RawFd; 2] = Default::default();

    unsafe {
        libc::pipe(pipe.as_mut_ptr());
        libc::dup2(pipe[1], libc::STDOUT_FILENO);
        libc::dup2(pipe[1], libc::STDERR_FILENO);
    }

    std::thread::spawn(move || {
        let tag = CStr::from_bytes_with_nul(b"rust-stdio\0").unwrap();
        let file = unsafe { File::from_raw_fd(pipe[0]) };
        let mut reader = BufReader::new(file);
        let mut buffer = String::new();
        loop {
            buffer.clear();
            if let Ok(len) = reader.read_line(&mut buffer) {
                if len == 0 {
                    break;
                } else if let Ok(msg) = CString::new(buffer.clone()) {
                    // match msg.to_str() {
                    //     Ok(str) => info!("{}", str),
                    //     Err(_) => info!("{:?}", msg),
                    // }
                    android_log(Level::Info, tag, &msg);
                }
            }
        }
    });

    let looper_ready = Arc::new(Condvar::new());
    let signal_looper_ready = looper_ready.clone();

    std::thread::spawn(move || {
        let looper = ThreadLooper::prepare();
        let foreign = looper.into_foreign();

        {
            let mut locked_looper = LOOPER.lock().unwrap();
            *locked_looper = Some(foreign);
            signal_looper_ready.notify_one();
        }
    });

    // Don't return from this function (`ANativeActivity_onCreate`) until the thread
    // has created its `ThreadLooper` and assigned it to the static `LOOPER`
    // variable. It will be used from `on_input_queue_created` as soon as this
    // function returns.
    let locked_looper = LOOPER.lock().unwrap();
    let _mutex_guard = looper_ready
        .wait_while(locked_looper, |looper| looper.is_none())
        .unwrap();
}

fn android_log(level: Level, tag: &CStr, msg: &CStr) {
    let prio = match level {
        Level::Error => ndk_sys::android_LogPriority_ANDROID_LOG_ERROR,
        Level::Warn => ndk_sys::android_LogPriority_ANDROID_LOG_WARN,
        Level::Info => ndk_sys::android_LogPriority_ANDROID_LOG_INFO,
        Level::Debug => ndk_sys::android_LogPriority_ANDROID_LOG_DEBUG,
        Level::Trace => ndk_sys::android_LogPriority_ANDROID_LOG_VERBOSE,
    };
    unsafe {
        ndk_sys::__android_log_write(prio as std::os::raw::c_int, tag.as_ptr(), msg.as_ptr());
    }
}

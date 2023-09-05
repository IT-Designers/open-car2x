pub mod env;
pub mod info;

include!(concat!(env!("OUT_DIR"), "/built.rs"));

#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate cstr_derive;

#[macro_use]
extern crate num_derive;

macro_rules! valid_mut_ptr_or {
    ($ptr:ident, $ret:expr) => {
        if $ptr.is_null() {
            return $ret;
        } else {
            &mut *$ptr
        }
    };
}

macro_rules! valid_ptr_or {
    ($ptr:ident, $ret:expr) => {
        if $ptr.is_null() {
            return $ret;
        } else {
            &*$ptr
        }
    };
}

macro_rules! some_or_return {
    ($option:expr, $ret:expr) => {
        match $option {
            Some(value) => value,
            None => return $ret,
        }
    };
}

pub mod connection;
pub mod conversion;
pub mod pods;
pub mod result;
pub mod util;
pub mod worker;
pub mod logging;
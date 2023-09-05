#![allow(clippy::derive_partial_eq_without_eq)]

#[cfg_attr(feature = "derive-serde", macro_use)]
#[cfg(feature = "derive-serde")]
extern crate serde;

#[allow(dead_code, unused, irrefutable_let_patterns)]
pub mod cam_pdu_descriptions;
#[allow(dead_code, unused)]
pub mod cpm_pdu_descriptions;
#[allow(dead_code, unused, irrefutable_let_patterns)]
pub mod denm_pdu_descriptions;
#[allow(dead_code, unused)]
pub mod dsrc;
#[allow(dead_code, unused)]
pub mod itd_ssdm_descriptions;
#[allow(dead_code, unused)]
pub mod its_container;
#[allow(dead_code, unused)]
pub mod mcm_pdu_descriptions;
#[allow(dead_code, unused)]
pub mod itd_data_protocol;
#[allow(dead_code, unused)]
pub mod vam_pdu_descriptions;
#[allow(dead_code, unused)]
pub mod vam_temp_imports;
#[allow(dead_code, unused)]
pub mod vru_motorcyclist_special_container;

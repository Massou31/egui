//! This is a crate that adds some features on top top of [`egui`](https://github.com/emilk/egui).
//!
//! This crate are for experimental features, and features that require big dependencies that does not belong in `egui`.
//!
//! ## Feature flags
#![cfg_attr(feature = "document-features", doc = document_features::document_features!())]
//!

#![allow(clippy::float_cmp)]
#![allow(clippy::manual_range_contains)]
#![forbid(unsafe_code)]

#[cfg(feature = "datepicker")]
mod datepicker;

pub mod image;
mod layout;
mod sizing;
mod strip;
mod table;

#[cfg(feature = "datepicker")]
pub use crate::datepicker::{DateImpl, DatePickerButton, Week};

#[cfg(feature = "chrono")]
pub use crate::datepicker::{chrono::Date as ChronoDate, DatePickerButtonChrono};

#[cfg(feature = "hifitime")]
pub use crate::datepicker::{hifitime::Date as HifiDate, DatePickerButtonHifi};

pub use crate::image::RetainedImage;
pub(crate) use crate::layout::StripLayout;
pub use crate::sizing::Size;
pub use crate::strip::*;
pub use crate::table::*;

/// Log an error with either `log` or `eprintln`
macro_rules! log_err {
    ($fmt: literal, $($arg: tt)*) => {{
        #[cfg(feature = "log")]
        log::error!($fmt, $($arg)*);

        #[cfg(not(feature = "log"))]
        eprintln!(
            concat!("egui_extras: ", $fmt), $($arg)*
        );
    }};
}
pub(crate) use log_err;

/// Panic in debug builds, log otherwise.
macro_rules! log_or_panic {
    ($fmt: literal, $($arg: tt)*) => {{
        if cfg!(debug_assertions) {
            panic!($fmt, $($arg)*);
        } else {
            $crate::log_err!($fmt, $($arg)*);
        }
    }};
}
pub(crate) use log_or_panic;

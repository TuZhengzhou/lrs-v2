#![cfg_attr(not(feature = "std"), no_std)]
#![warn(unused, future_incompatible, nonstandard_style)]
#![allow(clippy::many_single_char_names, clippy::op_ref)]
#![allow(ambiguous_glob_reexports)]
#![forbid(unsafe_code)]

pub mod link;
pub mod cc;
pub mod sma;
pub mod lrs;
pub mod constants;

use crate::cc::error;
pub type Result<T> = core::result::Result<T, error::Error>;
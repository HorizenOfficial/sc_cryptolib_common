#![allow(
    clippy::upper_case_acronyms,
    clippy::too_many_arguments,
    clippy::try_err,
    clippy::map_collect_result_unit,
    clippy::not_unsafe_ptr_arg_deref
)]

pub mod ginger_calls;

#[macro_use]
pub mod rust_jni;

#[macro_use]
pub mod type_mappings;
pub use type_mappings::*;

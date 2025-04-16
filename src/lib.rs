#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/linux_gpib.rs"));

#[cfg(not(feature = "bindgen"))]
include!("./prebind/linux_gpib.rs");

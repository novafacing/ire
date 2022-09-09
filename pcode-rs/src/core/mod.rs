#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub type const_pointer = u64;

include!(concat!(env!("OUT_DIR"), "/ghidra_bindings.rs"));

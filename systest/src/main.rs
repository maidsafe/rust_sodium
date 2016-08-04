#![allow(bad_style)]

extern crate rust_sodium_sys;
extern crate libc;

use libc::*;
use rust_sodium_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));

#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![no_std]

pub mod board;
#[cfg(not(test))]
pub mod debug;
pub mod panic;
pub mod soc;
pub mod timing;
mod util;

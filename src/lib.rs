#![doc = include_str!("./docs.md")]
#![no_std]

extern crate alloc;

mod base;
mod format_num;

pub mod expr;

pub use crate::base::*;

#[cfg(test)]
mod test;

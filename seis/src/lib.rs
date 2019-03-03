#![feature(bufreader_seek_relative)]
#![feature(exclusive_range_pattern)]

extern crate nalgebra as na;

pub mod forward;
pub mod io;

#[cfg(test)]
pub mod tests {}

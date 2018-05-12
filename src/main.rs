#![feature(core)]
#![feature(lang_items)]
#![feature(no_std)]
#![feature(start)]
#![no_std]

#[macro_use]
extern crate core;

use core::result::*;
use core::result::Result::*;

pub mod lang_items;

fn guess(num: i32) -> Result<i32, &'static str> {
    match num {
        42 => Ok(42),
        _ => Err("Not the answer!"),
    }
}

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    if let Ok(num) = guess(69) {
        return num as isize;
    }

    if let Ok(num) = guess(42) {
        return num as isize;
    }

    return -1;  // No idea
}

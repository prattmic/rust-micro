#![feature(core)]
#![feature(lang_items)]
#![feature(no_std)]
#![feature(start)]
#![no_std]

extern crate core;

mod lang_items;

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    42
}

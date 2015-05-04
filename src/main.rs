#![feature(core)]
#![feature(lang_items)]
#![feature(no_std)]
#![feature(start)]
#![no_std]

extern crate core;

#[start]
fn main(argc: isize, argv: *const *const u8) -> isize {
    42
}

#[cfg(not(test))]
use core::fmt::Arguments;

#[cfg(not(test))]
#[lang="stack_exhausted"]
extern fn stack_exhausted() {}

#[cfg(not(test))]
#[lang="eh_personality"]
extern fn eh_personality() {}

#[cfg(not(test))]
#[lang="panic_fmt"]
pub fn panic_fmt(_fmt: &Arguments, _file_line: &(&'static str, usize)) -> ! {
  loop { }
}

#[no_mangle]
pub extern fn __morestack() {
    unsafe { core::intrinsics::abort(); }
}

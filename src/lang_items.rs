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

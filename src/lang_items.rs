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
pub extern fn abort() {
    panic!("abort!")
}

#[no_mangle]
pub unsafe extern fn memcpy(dest: *mut u8, src: *const u8,
                            n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
    return dest;
}

#[no_mangle]
pub unsafe extern fn __aeabi_memcpy(dest: *mut u8, src: *const u8,
                                    n: usize) -> *mut u8 {
    return memcpy(dest, src, n);
}

#[no_mangle]
pub unsafe extern fn memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        *s.offset(i as isize) = c as u8;
        i += 1;
    }
    return s;
}

#[no_mangle]
pub unsafe extern fn __aeabi_memset(s: *mut u8, c: i32, n: usize) -> *mut u8 {
    return memset(s, c, n);
}


#[no_mangle]
pub unsafe extern fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    let mut i = 0;
    while i < n {
        let a = *s1.offset(i as isize);
        let b = *s2.offset(i as isize);
        if a != b {
            return a as i32 - b as i32
        }
        i += 1;
    }
    return 0;
}

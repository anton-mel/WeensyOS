#![no_std]
#![no_main]
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(c_variadic, register_tool)]

use core::ffi::CStr;
use core::fmt::Write;
use core::ptr;

// Type definitions
pub type int8_t = i8;
pub type uint8_t = u8;
pub type int16_t = i16;
pub type uint16_t = u16;
pub type int32_t = i32;
pub type uint32_t = u32;
pub type int64_t = i64;
pub type uint64_t = u64;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = isize;
pub type pid_t = i32;

// Constants
pub const NULL: *const () = ptr::null();
pub const RAND_MAX: i32 = 0x7FFFFFFF;
pub const INT_SYS: i32 = 48;
pub const INT_SYS_PANIC: i32 = INT_SYS + 0;
pub const INT_SYS_GETPID: i32 = INT_SYS + 1;
pub const INT_SYS_YIELD: i32 = INT_SYS + 2;
pub const INT_SYS_PAGE_ALLOC: i32 = INT_SYS + 3;
pub const INT_SYS_FORK: i32 = INT_SYS + 4;
pub const INT_SYS_EXIT: i32 = INT_SYS + 5;
pub const INT_SYS_MAPPING: i32 = INT_SYS + 6;
pub const INT_SYS_MEM_TOG: i32 = INT_SYS + 8;
pub const INT_SYS_BRK: i32 = INT_SYS + 9;
pub const INT_SYS_SBRK: i32 = INT_SYS + 10;
pub const CONSOLE_COLUMNS: usize = 80;
pub const CONSOLE_ROWS: usize = 25;

pub static mut console: [u16; CONSOLE_ROWS * CONSOLE_COLUMNS] = [0; CONSOLE_ROWS * CONSOLE_COLUMNS];
pub static mut cursorpos: i32 = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct console_printer {
    pub p: libc::c_int,
    pub cursor: *mut libc::c_int,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_printer {
    pub p: libc::c_int,
    pub s: *mut libc::c_char,
    pub end: *mut libc::c_char,
}

// Function definitions
pub unsafe fn memcpy(dst: *mut u8, src: *const u8, n: size_t) -> *mut u8 {
    ptr::copy_nonoverlapping(src, dst, n);
    dst
}

pub unsafe fn memmove(dst: *mut u8, src: *const u8, n: size_t) -> *mut u8 {
    ptr::copy(src, dst, n);
    dst
}

pub unsafe fn memset(s: *mut u8, c: i32, n: size_t) -> *mut u8 {
    ptr::write_bytes(s, c as u8, n);
    s
}

pub fn strlen(s: *const i8) -> size_t {
    unsafe { CStr::from_ptr(s).to_bytes().len() }
}

pub fn strnlen(s: *const i8, maxlen: size_t) -> size_t {
    unsafe { CStr::from_ptr(s).to_bytes().len().min(maxlen) }
}

pub unsafe fn strcpy(dst: *mut i8, src: *const i8) -> *mut i8 {
    let mut i = 0;
    while *src.offset(i) != 0 {
        *dst.offset(i) = *src.offset(i);
        i += 1;
    }
    *dst.offset(i) = 0;
    dst
}

pub fn strcmp(a: *const i8, b: *const i8) -> i32 {
    unsafe { CStr::from_ptr(a).cmp(CStr::from_ptr(b)) as i32 }
}

pub unsafe fn strchr(s: *const i8, c: i32) -> *const i8 {
    let s = CStr::from_ptr(s).to_bytes();
    s.iter().position(|&x| x == c as u8).map_or(ptr::null(), |i| s.as_ptr().add(i) as *const i8)
}

pub fn snprintf(s: *mut i8, size: size_t, format: *const i8, args: ...) -> i32 {
    let format = unsafe { CStr::from_ptr(format).to_str().unwrap() };
    let mut buf = Vec::with_capacity(size);
    buf.write_fmt(format_args!(format, args)).unwrap();
    let len = buf.len().min(size);
    unsafe {
        ptr::copy_nonoverlapping(buf.as_ptr(), s as *mut u8, len);
        if len < size {
            *s.add(len) = 0;
        }
    }
    len as i32
}

pub fn vsnprintf(s: *mut i8, size: size_t, format: *const i8, val: va_list) -> i32 {
    unimplemented!()
}

pub fn rand() -> i32 {
    rand::random::<i32>() & RAND_MAX
}

pub fn srand(_seed: u32) {
    unimplemented!()
}

#[macro_export]
macro_rules! offsetof {
    ($ty:ty, $field:ident) => {
        &(*(std::ptr::null::<$ty>())).$field as *const _ as usize
    };
}

#[macro_export]
macro_rules! arraysize {
    ($array:expr) => {
        $array.len()
    };
}

#[macro_export]
macro_rules! assert {
    ($x:expr) => {
        if !$x {
            assert_fail(file!(), line!(), stringify!($x));
        }
    };
}

pub fn assert_fail(file: &str, line: u32, msg: &str) -> ! {
    panic!("Assertion failed at {}:{}: {}", file, line, msg);
}

#[macro_export]
macro_rules! kernel_panic {
    ($($arg:tt)*) => {
        panic!($($arg)*);
    };
}

#[macro_export]
macro_rules! MIN {
    ($a:expr, $b:expr) => {
        if $a <= $b { $a } else { $b }
    };
}

#[macro_export]
macro_rules! MAX {
    ($a:expr, $b:expr) => {
        if $a >= $b { $a } else { $b }
    };
}

#[macro_export]
macro_rules ROUNDDOWN {
    ($a:expr, $n:expr) => {
        $a - ($a % $n)
    };
}

#[macro_export]
macro_rules ROUNDUP {
    ($a:expr, $n:expr) => {
        $a + (($n - ($a % $n)) % $n)
    };
}

pub fn console_clear() {
    unsafe {
        console.fill(0);
        cursorpos = 0;
    }
}

pub fn console_printf(cpos: i32, color: i32, format: *const i8, args: ...) -> i32 {
    unimplemented!()
}

pub fn console_vprintf(cpos: i32, color: i32, format: *const i8, val: va_list) -> i32 {
    unimplemented!()
}

pub struct printer {
    putc: fn(&mut printer, unsigned char, i32),
}

pub fn printer_vprintf(p: &mut printer, color: i32, format: *const i8, val: va_list) {
    unimplemented!()
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(
    mut dst: *mut libc::c_void,
    mut src: *const libc::c_void,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    let mut s: *const libc::c_char = src as *const libc::c_char;
    return dst;
}

#[no_mangle]
pub unsafe extern "C" fn memset(
    mut v: *mut libc::c_void,
    mut c: libc::c_int,
    mut n: libc::c_int,
) -> *mut libc::c_void {
    return v;
}

#[export_name = "strlen"]
pub unsafe extern "C" fn strlen_0(mut s: *const libc::c_char) -> libc::c_int {
    while *s as libc::c_int != '\0' as i32 {
        s = s.offset(1);
    }
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub unsafe extern "C" fn strnlen(
    mut s: *const libc::c_char,
    mut maxlen: libc::c_int,
) -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}

#[no_mangle]
pub unsafe extern "C" fn strcpy(
    mut dst: *mut libc::c_char,
    mut src: *const libc::c_char,
) -> *mut libc::c_char {
    let mut d: *mut libc::c_char = dst;
    while *src != 0 {
        *dst = *src;
        src = src.offset(1);
        dst = dst.offset(1);
    }
    *dst = 0;
    return d;
}

#[no_mangle]
pub unsafe extern "C" fn strchr(
    mut s: *const libc::c_char,
    mut c: libc::c_int,
) -> *mut libc::c_char {
    while *s != 0 {
        if *s as libc::c_int == c {
            return s as *mut libc::c_char;
        }
        s = s.offset(1);
    }
    return 0 as *mut libc::c_char;
}

#[no_mangle]
pub unsafe extern "C" fn rand() -> libc::c_int {
    return 4;
}

#[no_mangle]
pub unsafe extern "C" fn srand(mut seed: libc::c_uint) {}

#[export_name = "snprintf"]
pub unsafe extern "C" fn snprintf_0(
    mut s: *mut libc::c_char,
    mut size: libc::c_int,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}

#[export_name = "vsnprintf"]
pub unsafe extern "C" fn vsnprintf_0(
    mut s: *mut libc::c_char,
    mut size: libc::c_int,
    mut format: *const libc::c_char,
    mut val: ...
) -> libc::c_int {
    panic!("Reached end of non-void function without returning");
}

#![feature(intrinsics, lang_items, no_std)]

#![no_std]

// // Declare some intrinsic functions that are provided to us by the compiler.
// extern "rust-intrinsic" {
//     fn overflowing_add<T>(a: T, b: T) -> T;
//     fn u32_sub_with_overflow(x: u32, y: u32) -> (u32, bool);
// }

#[lang = "panic_fmt"]
pub extern fn panic_fmt() -> ! { loop {} }

#[lang = "stack_exhausted"]
pub extern fn stack_exhausted() -> ! { loop {} }

#[lang = "eh_personality"]
pub extern fn eh_personality() -> ! { loop {} }

// // I'm not 100% sure what this function does, but references to it are compiled
// // into the program by the Rust compiler. I think it would be called in the case
// // of a program panic.
#[no_mangle]
pub fn __aeabi_unwind_cpp_pr0() {
    loop {}
}


#[no_mangle]
pub fn rust_main() -> u32 {
    40 + 2
}

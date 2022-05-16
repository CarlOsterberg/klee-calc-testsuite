#![no_std]
#![no_main]

//cortex-klee-tools --rust-example max --optimize

use klee_sys::klee_make_symbolic;
use panic_klee as _;

#[inline(never)]
#[no_mangle]
fn main() {
    let mut x:i32 = 0;
    let y:i32 = 0;
    klee_make_symbolic!(&mut x, "x");
    max(x,y);
}

#[inline(never)]
fn max(mut x: i32, y:i32) -> i32 {
    if x > y {
        unsafe {
            x = no_std_compat::ptr::read_volatile(&x);
        }
        x
    } else {
        y
    }
}
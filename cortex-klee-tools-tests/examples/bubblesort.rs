#![no_std]
#![no_main]

//cortex-klee-tools --rust-example bubblesort

use klee_sys::klee_make_symbolic;
use panic_klee as _;
use cortex_test_lib::bubble_sort;

#[inline(never)]
#[no_mangle]
fn main() {
    let mut vec = [3,1,4,5];
    klee_make_symbolic!(&mut vec, "vec");
    bubble_sort(&mut vec);
}
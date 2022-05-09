#![no_std]
#![no_main]

//cortex-klee-tools --rust-example bubblesort

use klee_sys::klee_make_symbolic;
use panic_klee as _;

#[inline(never)]
#[no_mangle]
fn main() {
    let mut vec = [3,1,4,5];
    klee_make_symbolic!(&mut vec, "vec");
    bubble_sort(&mut vec);
}

#[inline(never)]
pub fn bubble_sort(vec: &mut [i32]) {
    loop {
        let mut done = true;
        for i in 0..vec.len()-1 {
            if vec[i+1] < vec[i] {
                done = false;
                let temp = vec[i+1];
                vec[i+1] = vec[i];
                vec[i] = temp;
            }
        }
        if done {
            return;
        }
    }
}
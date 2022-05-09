#![no_std]
#![no_main]

//cortex-klee-tools --rust-example primes

use klee_sys::klee_make_symbolic;
use panic_klee as _;

#[inline(never)]
#[no_mangle]
fn main() {
    let mut vec:u64 = 123791329;
    klee_make_symbolic!(&mut vec, "vec");
    count_primes(vec);
}

#[inline(never)]
fn count_primes(max:u64) -> u8 {
    let mut count = 0;
    if max<2 {
        count
    }
    else {
        for n in 2..(max+1) {
            let mut is_prime = true;
            for m in 2..n {
                if n % m == 0 {
                    is_prime = false;
                    break;
                }
            }
            if is_prime {
                count += 1;
                if count == 20 {
                    panic!()
                }
            }
        }
        count
    }
}
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//build
//cargo build --example primes --release --features klee-replay

//run with test tool
//make sure to have the correct .ktest files in /klee-last/
//embedded-rust-debugger -c STM32F411RETx -w . -e target/thumbv7em-none-eabi/release/examples/primes -k cortex-klee-tools-tests/target/thumbv7em-none-eabihf/release/examples/klee-last/

use bkpt_trace::{bkpt_enter, bkpt_end};
use panic_halt as _;
use stm32f4;
pub use cstr_core::CStr;
#[allow(unused_imports)]
use core::arch::asm;
use klee_sys::klee_make_symbolic;

#[rtic::app(device =  stm32f4::stm32f411, dispatchers = [EXTI1])]
mod app {
    use super::*;

    #[shared]
    struct Shared {}

    #[local]
    struct Local {}

    #[init]
    fn init(mut cx: init::Context) -> (Shared, Local, init::Monotonics) {
        cx.core.DCB.enable_trace();
        cx.core.DWT.enable_cycle_counter();

        (Shared {}, Local {}, init::Monotonics())
    }

    #[task()]
    fn foo(_: foo::Context) {
        bkpt_enter();
        caller();
        bkpt_end();
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        cortex_m::asm::bkpt();
        #[allow(unreachable_code)]
        loop {
            foo::spawn().unwrap();
            continue;
        }
    }
}

#[inline(never)]
fn caller() {
    let mut vec = 3;
    klee_make_symbolic!(&mut vec, "vec");
    count_primes(vec);
}

#[inline(never)]
fn count_primes(max:u32) -> u32 {
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
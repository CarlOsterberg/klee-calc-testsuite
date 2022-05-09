#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//run with test tool
//embedded-rust-debugger -c STM32F411RETx -w . -e target/thumbv7em-none-eabi/release/examples/bubblesort -k cortex-klee-tools-tests/target/thumbv7em-none-eabihf/release/examples/klee-last/

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
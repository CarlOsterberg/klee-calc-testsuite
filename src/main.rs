#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//use core::{ffi::{c_void}, arch::asm};
use bkpt_trace::{bkpt_enter, bkpt_end};
use panic_halt as _;
//use panic_rtt_target as _;
use stm32f4;
pub use cstr_core::CStr;
#[allow(unused_imports)]
use core::arch::asm;
use klee_sys::klee_make_symbolic;
//use panic_klee as _;

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

//cargo run -- -c STM32F411RETx -w /home/carlosterberg/testsuite/ -e /home/carlosterberg/testsuite/target/thumbv7em-none-eabi/debug/app

#[inline(never)]
fn get_sign(x: i32) -> i32 {
    if x == 0 {
        return 0;
    }
    if x < 0 {
        return -1;
    } else {
        return 1;
    }
}

#[inline(never)]
fn caller() {
    let mut a: i32 = 5;
    klee_make_symbolic(&mut a);
    get_sign(a);
}
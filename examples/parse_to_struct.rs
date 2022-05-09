#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

//build
//cargo build --example parse_to_struct --release --features klee-replay

//run with test tool
//make sure to have the correct .ktest files in /klee-last/
//embedded-rust-debugger -c STM32F411RETx -w . -e target/thumbv7em-none-eabi/release/examples/parse_to_struct -k cortex-klee-tools-tests/target/thumbv7em-none-eabihf/release/examples/klee-last/

use bkpt_trace::{bkpt_enter, bkpt_end};
use panic_halt as _;
use stm32f4;
pub use cstr_core::CStr;
#[allow(unused_imports)]
use core::arch::asm;
use klee_sys::klee_make_symbolic;

#[allow(dead_code)]
pub enum Stuff {
    OnePayload(OnePayload),
    TwoPayload(TwoPayload),
    ThreePayload(ThreePayload),
    NegatePayload(NegatePayload),
}
#[allow(dead_code)]
pub struct OnePayload {
    payload: u16,
    is_add: bool,
    is_sub: bool,
    is_xor: bool,
}
#[allow(dead_code)]
pub struct TwoPayload {
    payload1: u8,
    payload2: u16,

}
#[allow(dead_code)]
pub struct ThreePayload {
    payload1: u8,
    payload2: u8,
    payload3: u8,
}
#[allow(dead_code)]
pub struct NegatePayload {
    payload:u16,
}

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
    let mut bits:u32 = 5;
    klee_make_symbolic!(&mut bits, "val");
    let result = parse_to_struct(bits);
    match result {
        Stuff::NegatePayload(_value) => panic!(),
        _ => return
    }
}

#[inline(never)]
pub fn parse_to_struct(bits: u32) -> Stuff {
    //match on two msg bits
    match bits >> 30 {
        0 => {
            //mask out 3rd msg bit
            if bits & 0x20000000 >> 29 == 1 {
                //mask out 4th msg bit
                if bits & 0x10000000 >> 28 == 1 {
                    let value = bits & 0xFFF0000 >> 16;
                    Stuff::OnePayload(
                        OnePayload {
                            payload: (bits & 0x0000FFFF + value) as u16,
                            is_add: true,
                            is_sub: false,
                            is_xor: false,
                        }
                    )
                }
                else {
                    let value = bits & 0x0FFF0000 >> 16;
                    Stuff::OnePayload(
                        OnePayload {
                            payload: (bits & 0x0000FFFF - value) as u16,
                            is_add: false,
                            is_sub: true,
                            is_xor: false,
                        }
                    )
                }
            }
            else {
                let value = bits & 0x3FFF0000 >> 16;
                Stuff::OnePayload(
                    OnePayload {
                        payload: (bits & 0x0000FFFF ^ value) as u16,
                        is_add: false,
                        is_sub: false,
                        is_xor: true,
                    }
                )
            }
        },
        1 => {
            let value = bits & 0x3F000000 >> 24;
            Stuff::TwoPayload(
                TwoPayload {
                    payload2: (bits & 0x0000FFFF + value) as u16,
                    payload1: (bits & 0x00FF0000 >> 16) as u8,
                }
            )
        },
        2 => {
            Stuff::ThreePayload(
                ThreePayload {
                    payload3: (bits & 0x00FF0000 >> 16) as u8,
                    payload2: (bits & 0x0000FF00 >> 8) as u8,
                    payload1: (bits & 0x000000FF) as u8,
                }
            )
        },
        3 => {
            Stuff::NegatePayload(
                NegatePayload {
                    payload: !bits as u16,
                }
            )
        }
        _ => panic!()
    }
}
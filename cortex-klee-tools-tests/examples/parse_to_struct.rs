#![no_std]
#![no_main]

//cortex-klee-tools --rust-example parse_to_struct --optimize

use core::panic;

use klee_sys::klee_make_symbolic;
use panic_klee as _;

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

#[inline(never)]
#[no_mangle]
fn main() {
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
    let copy = bits.clone();
    match copy >> 30 {
        0 => {
            let copy = bits.clone();
            //mask out 3rd msg bit
            if copy & 0x20000000 >> 29 == 1 {
                let copy = bits.clone();
                //mask out 4th msg bit
                if copy & 0x10000000 >> 28 == 1 {
                    let copy = bits.clone();
                    let value = copy & 0xFFF0000 >> 16;
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
                let copy = bits.clone();
                let value = copy & 0x3FFF0000 >> 16;
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
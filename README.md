# RTIC on the STM32F4xx Nucleo board

## Rust

We assume Rust to be installed using [rustup](https://www.rust-lang.org/tools/install).

Additionally you need to install the `thumbv7em-none-eabi` target.

```shell
> rustup target add thumbv7em-none-eabi 
```

We assume the following tools are in place:

- [cargo-binutils](https://github.com/rust-embedded/cargo-binutils).

- [probe-run](https://crates.io/crates/probe-run)

```shell
> cargo install cargo-binutils probe-run
```

## Editor

You may use any editor of choice. `vscode` supports Rust using the  `rust-analyzer` plugin.

---

## Exercises

- `src/main.rs`

  Developing embedded applications in Rust is made simle by the RTIC framework. In this exercise you will familiarize yourself with the basics `init` and `idle`, and see how you can trace the output to a terminal using `cargo run`.

  You will also learn about `panic`s and how they can be traced.

- `examples/rtt_timing.rs`

  Here you will learn about (almost) cycle accurate timing measurements:

  - Using instrumentation code (which introduces bloat and overhead).

  - Code generation optimization.

  - Code inspection, `objdump`.

  - Code trimming, RTIC is "A Zero-Cost Abstraction for Memory Safe Concurrency".

---

## A note on embedded programming and debugging

### Stm32f411 Nucleo

The Stm32f411 Nucleo is an entry level development kit for embedded programming based on the ARM Cortex M4 architecture. It features an on-board programmer (st-link-v2) allowing applications to be both programmed onto the MCU (flashed), debugged and traced.

In these examples we use RTT (Real-Time Transfer) for instrumentation/tracing. It is very convenient as we don't need to setup some extra communication link between the host (PC) and the target (MCU), instead tracing is going over the already present debug interface.

### Probe

[probe.rs](https://probe.rs) is a set of libraries and utilities aiming to facilitate embedded programming in general and embedded Rust in particular. You will use `probe-run` a third party utility for flashing and tracing programs based on probe.rs.


### Overall debug scenario

The `probe-run` is configured in the `.cargo/config.toml` (as the "runner" for this project). It will:

- First establish a connection to the target. If you get problems that it fails try:

  - `cargo run --connect-under-reset` and press/hold the reset button on the Nucleo when running the command.

- Then periodically poll a predefined memory region on the target for tracing output and convey this to the host terminal.
  
- Listen to messages from the onboard programmer, e.g., status such as breakpoints hit, or errors occurred.
  
### RTT Pros and Cons

RTT is very easy/convenient and in many cases less intrusive to the embedded application than tracing over some serial link. It is also relatively fast (the majority of overhead on the target side will be to format the data into characters).

The cons is that it requires a host connected initially, so you cannot easily tap-into an already running system in an ad-hoc manner. For the purpose of development debugging/tracing this is not a big issue, but for maintenance its a no go.

Another cons is that the host actively probes the target, so you cannot easily trace systems going in and out of low power states (where the debug unit might be powered down).

Another cons is that the active polling of memory introduces bus load (the MCU and the debug unit cannot access the memory simultaneously, introducing wait states in case of congestion). This is not a big deal in practice but we will see that it matters for cycle accurate measurements.

There are other front-ends for the probe libraries such as `cargo embed`, but they require slightly more configuration. Despite the named drawbacks, the ease of use makes `probe-run` our first choice.

(Using `probe` you can also defer formatting to the host side to further reduce overhead on the target side, we don't need it for our purpose but you might find it useful to developing other applications.)

---

## Learning outcomes

- First exposure to embedded programming in Rust
  
- Tracing and panic handling
  
- You have observed the effect of optimization (going from very very inefficient to very very efficient) 

- You have seen that RTIC applications are easy to trace using RTT

  If more details are required, you may use gdb.

- You have seen how the embedded Rust ecosystem is designed with `cortex-m` dealing with the generic functionality and embedded HAL abstracting hardware from the end user

- You have confirmed that RTIC is extremely light weight (zero-cost in release build)

  Applications can be be less than 1k flash. Ideal for IoT!

- You have setup timing measurements which are

  - (Almost) cycle accurate (0.000 000 0x s at 100MHz)

  - Consistent (down to a few clock cycles)

  - Predictable

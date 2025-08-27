
# Operating Sytem using rust





# Standalone Rust Binary for Embedded ARM

This guide explains how to build a standalone Rust binary targeting an embedded ARM Cortex-M microcontroller.

---

## 2. Linkage errors on native compilation

If you compile directly on your host machine, the Rust compiler assumes the presence of an OS and a C runtime:

- **Windows** → expects `msvcrt`  
- **Linux** → expects `glibc`  

Since bare-metal systems don’t have an operating system or a C runtime, you’ll encounter linkage errors if you try to compile without specifying the correct target.

---

## 3. Compiling for a different environment

To run on microcontrollers, we need to compile for a different environment than our host.  
Rust supports this through **target triples**, which define the architecture, vendor, operating system, and ABI.

---

## 4. Find your environment

To check your current toolchain and default compilation environment, run:


```sh
rustc --version --verbose
```

## Understanding `thumbv7em-none-eabihf` Target Triple

This document explains what each part of the Rust target triple `thumbv7em-none-eabihf` means.

---

## 1. Architecture
- **`thumbv7em`**  
  - Refers to the ARM Cortex-M4 / Cortex-M7 microcontrollers.  
  - The `thumb` prefix indicates the **Thumb instruction set** (a compressed 16/32-bit instruction set).  
  - `v7em` = ARMv7E-M architecture profile.  
  - This profile supports hardware multiply, optional floating-point, DSP instructions, and better interrupt handling.

---

## 2. Vendor
- **`none`**  
  - Indicates that this is a **bare-metal target**.  
  - No operating system or vendor-specific ABI.  
  - You’re directly targeting the hardware without an OS layer.

---

## 3. Operating System
- **`none`** (again implied)  
  - Confirms there’s no operating system like Linux, Windows, etc.  
  - All runtime/OS-like behavior must be provided by you (startup code, panic handling, etc.).

---

## 4. ABI (Application Binary Interface)
- **`eabihf`**  
  - Stands for **Embedded Application Binary Interface, Hard Float**.  
  - `eabi` = ABI used for ARM embedded systems (bare metal).  
  - `hf` = hardware floating-point, meaning floating-point operations are handled directly by the FPU (Floating Point Unit) if available.

---

## 5. Typical Use-Cases
- Writing **firmware** for microcontrollers like Cortex-M4 and Cortex-M7.  
- Applications in:  
  - Embedded systems.  
  - IoT devices.  
  - Robotics.  
  - Real-time systems (RTOS or bare metal).  
- Useful when performance and low-level control are critical.

---

## 6. Tooling & Ecosystem
- Rust cross-compilation uses `rustup target add thumbv7em-none-eabihf`.  
- Requires a cross toolchain like **`arm-none-eabi-gcc`** for linking.  
- Debugging often involves **OpenOCD**, **GDB**, or vendor-specific debuggers.  
- Often paired with crates like:  
  - [`cortex-m`](https://crates.io/crates/cortex-m) – low-level access to Cortex-M features.  
  - [`cortex-m-rt`](https://crates.io/crates/cortex-m-rt) – runtime support.  
  - [`embedded-hal`](https://crates.io/crates/embedded-hal) – hardware abstraction layer.

---

## 7. Key Notes
- Programs built for this target do **not use `std`**, but instead rely on `#![no_std]`.  
- You must define your own **entry point** (via `#[entry]` in `cortex-m-rt`, or `_start` manually).  
- Memory layout and startup sequence are defined in a **linker script**.  
- Debugging/logging often requires semihosting or ITM (Instrumentation Trace Macrocell).

---

## 8. Summary
The target triple `thumbv7em-none-eabihf` means:

- **Architecture:** ARM Cortex-M4/M7 (Thumb, ARMv7E-M).  
- **Vendor:** none (bare metal).  
- **OS:** none (no operating system).  
- **ABI:** Embedded ABI with hardware floating point.  

👉 It is the **go-to Rust target** for ARM Cortex-M4 and Cortex-M7 microcontrollers with FPU support.


targetting the system 
cargo build --target thumbv7em-none-eabihf


## alternatively we can also pass some arguments to the linker to avoid it looking for glib

```javascript
# Linux
cargo rustc -- -C link-arg=-nostartfiles
# Windows
cargo rustc -- -C link-args="/ENTRY:_start /SUBSYSTEM:console"
# macOS
cargo rustc -- -C link-args="-e __start -static -nostartfiles"
```
## Rust analyzer
it comes with vs code and it runs the cargo check
each time u try to save the file
it runs 

```cargo check --all-target ```

### this cmd line checks all the module binary , library,test and bench

#### test comes with std library ,without that it is of no use


# Minimum kernal

1. we wil be creating the kernel now
2. bootloader creation requrie the assembly language henc we do bw using the pre created loader

3. now we do be needing the inline
assmebly code for that i do be using the asm macro

4. this macro is nightly so we do be required to use the nightly verison for ,some reason this
feature has yet not been able to make its way into stable phase depite being three years old 

5. we are tragetting cisc x86 x64 architecture 

6. we are using the custom triple target with some configuration specific to our os


## This is the linux cisc x86 version
```
{
    "llvm-target": "x86_64-unknown-linux-gnu",
    "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": 32,
    "os": "linux",
    "executables": true,
    "linker-flavor": "gcc",
    "pre-link-args": ["-m64"],
    "morestack": false
}
```

6. so data-layout tells about the system  how much data in bytes do each data type will contain

## our custom target configuration

```
{
    "llvm-target": "x86_64-unknown-none",
    "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": 32,
    "os": "none",
    "executables": true,
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
}
```

8. (Note that, in contrast to the Cargo.toml option, this target option also applies when we recompile the core library later in this post. So, even if you prefer to keep the Cargo.toml option, make sure to include this option.)

9. so crate core will be recompiled
accordingly to our configuration

10. disablize the stack ponter optimization,because we are creating the interrupt ,
system interrupt that we do encounter in typical os environment like linux ,windows we aret rying to replicate something like that

11. add "disable-redzone": true, to the configuration file

12. 
```
{
    "llvm-target": "x86_64-unknown-linux-gnu",
    "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": 32,
    "os": "linux",
    "executables": true,
    "linker-flavor": "gcc",
    "pre-link-args": ["-m64"],
    "morestack": false,
    "disable-redzone": true,

}

```
13. there is something called the simd(single instruction multiple data) these are hard to handle when interrupt occur

14. but disabling this simd will cause the float vlaues to not work

15. float are then simulated using the software approach

something is called the soft-float this tells that the target will be using the soft-floats
```
"rustc-abi": "x86-softfloat"

```
16. now the configuration looks like
```
{
    "llvm-target": "x86_64-unknown-none",
    "data-layout": "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128",
    "arch": "x86_64",
    "target-endian": "little",
    "target-pointer-width": "64",
    "target-c-int-width": 32,
    "os": "none",
    "executables": true,
    "linker-flavor": "ld.lld",
    "linker": "rust-lld",
    "panic-strategy": "abort",
    "disable-redzone": true,
    "features": "-mmx,-sse,+soft-float",
    "rustc-abi": "x86-softfloat"
}

```
14. now we want recompile the crate every time we build the crate

```
build-std = ["core","compiler_builtins"]
```

15. another crate is used by the core crate




### 1. Set up nightly Rust
- Added a `rust-toolchain` file with:

16. adding the nightly verison of the code
```
 rustup component add rust-src --toolchain nightly
```

17. now building the crate with the configured target
```
cargo +nightly build -Zbuild-std=core,compiler_builtins --target x86_64-unknown-none.json
```

18. now for displaying the output to the console or the screen 

19. something called vga hardware is used to displaying

20. there is a vga map buffer that is mapped to the vga hardware

21. there are 25 lines

22. each line contains 80 character cells

23. these charcter cell can display ascii and background color 
using ansii code

24. . For printing “Hello World!”, we just need to know that the buffer is located at address 0xb8000 and that each character cell consists of an ASCII byte and a color byte.

25. run the script using
cargo +nightly build -Zbuild-std=core,compiler_builtins --target custom_target.json
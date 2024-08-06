# Possible LLVM compiler bug for AVR targets

The example in `main.rs` is as minimal as we could make it. The `log!` statements
in `State::run` are "load bearing", so they need to be there to trigger the bug
in this example. Basically some kind of repeated access to multiple struct
members appears to trigger the miscompilation.

The 49 bytes of padding appear to be necessary as well; we've only observed this
bug with a total struct size of more than 64 bytes.

## The miscompilation

So we're not sure why this bug happens, but we've looked at the assembly of broken
versions (as in `main.rs`), and working versions (e.g. `main.rs` but with a
smaller `State` struct or fewer prints). We spotted one difference that stands
out, in the code generated for the `State::run` method.

Broken code:

```asm
ldd	r16, Z+61	; 0x3d
ldd	r17, Z+62	; 0x3e
ldd	r14, Z+63	; 0x3f
ld	r15, Y
```

This works fine:

```asm
ldd	r16, Z+60	; 0x3c
ldd	r17, Z+61	; 0x3d
ldd	r14, Z+62	; 0x3e
ldd	r15, Z+63	; 0x3f
```

The special `Y` register is used in the broken version, apparently as an
optimization, but not in the working version.

[gcc.gnu.org](https://gcc.gnu.org/wiki/avr-gcc) says:

> In order to access stack locations, avr-gcc will set up a 16-bit frame pointer in R29:R28 (Y) because the stack pointer (SP) cannot be used to access stack slots.

And [LLVM](https://github.com/rust-lang/llvm-project/blob/rustc-1.80.0/llvm/lib/Target/AVR/AVRRegisterInfo.cpp#L80-L91) says:

```cpp
// We tenatively reserve the frame pointer register r29:r28 because the
// function may require one, but we cannot tell until register allocation
// is complete, which can be too late.
//
// Instead we just unconditionally reserve the Y register.
//
// TODO: Write a pass to enumerate functions which reserved the Y register
//       but didn't end up needing a frame pointer. In these, we can
//       convert one or two of the spills inside to use the Y register.
Reserved.set(AVR::R28);
Reserved.set(AVR::R29);
Reserved.set(AVR::R29R28);
```

But the `Y` register is used, anyway.

## Rust Toolchain

We've reproduced the bug with Rust `1.81.0-nightly`.

## Target

We're using an `atmega164pa`.

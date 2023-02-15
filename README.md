# Furry Emblem

A tactical RPG engine written in Rust for the Game Boy Advance

## Dependencies

- `make` (Yes, I hate this too. Blame grit)
- `grit`

## Building

Compiling Rust code for the GBA requires:

- Nightly Rust
- rust-src
- arm-none-eabi binutils

You can configure Rust with the following commands:
```
rustup default nightly
rustup component add rust-src
```

arm-none-eabi can be found [here](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads) or in your operating system's package manager.

Now just run `cargo build`!

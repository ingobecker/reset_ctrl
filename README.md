<img src="/docs/logo.svg" width="200">

A platform for creating custom midi controllers.

⚠️  **Warning** ⚠️ : I use this project to learn rust. It's not meant to be used in production. Expect things to break and code to undergo heavy refactoring.

## Background

This is the firmware part of a project that aims to let you create your own midi controllers.
Core of this project is an extendible PCB with multiple slots which can accommodate any of a
number of different in- and output modules. Muliple PCBs can be daisy-chained to further extend
the amount of slots available. For now a `YAML` file can be used to describe the arrangement of
in- and output modules. In the future a web based configurator will be added to assist with this
process.

## Hardware

In order to reduce the in- and outputs needed on the used microcontroller
the hardware is based on daisy-chained analog in- and output multiplexers. 

Currently the PCB is designed to support the following in- and outputs:

- Rotary Encoder (EC11 style)
- Analog Fader
- Analog Potentiometer
- WS2812B RGB LEDs

## Features

- In-Memory UI backend can emulate all inputs
- Encoder with Midi CC relative and absolute msg support
- Midi Output support
- Device configuration can be saved/loaded using `YAML`
- All features above are unit- or integration tested

## Building

The project can be built for `thumbv7m-none-eabi` and `x86_64-unknown-linux-gnu`.
The `Containerfile` bundles both toolchains as well as all the dependencies.
To build the container run:

```
$ podman build -t ctrl-reset-rust .
```

Run the following to get an interactive shell inside the container:
```
$ podman container runlabel shell ctrl-reset-rust
```

To build the binary from there run
```
# build the stm32-poc
$ cargo build --target thumbv7m-none-eabi --release --features bare-metal --bin stm32-poc

# build the host-poc
$ cargo build --bin reset_ctrl
```

## Flashing

The binary can be flashed from inside the container. If you are working with an SELinux
enabled OS make sure to load the corresponding `embedded_container.cil` file to allow
the container access to the STLink programmer:

```
$ sudo semodule -i embedded_container.cil /usr/share/udica/templates/{base_container.cil,net_container.cil}
```

Connect your STLink programmer and run the container like this:

```
$ podman container runlabel shellem ctrl-reset-rust
```

Now flashing works as expected by running:

```
$ cargo run --target thumbv7m-none-eabi --release --bin stm32-poc --features bare-metal
```

## Testsing

Most parts of the project are unit- and integration tested. Currently
the tests can only be tested on `x86_64-unknown-linux-gnu` because testing relies on
 the `std` crate. To run the tests, start a shell inside the container and run:

 ```
 $ cargo test
 ```

## Contributing

In order to do automated releases and changelogs [conventional-commits](https://www.conventionalcommits.org/en/v1.0.0/) are used. /
For new features commits are prefix with `feat: ...` and for bugfixes with: `fix: ...`.
Breaking changes are introduced with an `!` after the prefix like `feat!: ...` or `fix!: ...`.

## Roadmap

As this projects purpose is to learn rust, don't expect any of the features
outlined below to be implemented in order or a timely manner.

- Support for linux, wasm and bare metal microcontrollers
- Support for encoder, potentiometer, fader and RGB LEDs
- CLI for configuration


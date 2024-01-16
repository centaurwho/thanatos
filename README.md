# Thanatos

Thanatos is my attempt at a bare-bones operating system. It is written in Rust and targets the x86_64 architecture. Main 
inspiration for this project comes from [Philipp Oppermann's blog](https://os.phil-opp.com/). As of now, I am following
his blog to get a basic understanding of how to write an operating system. I will be using his bootloader for now, but
will eventually (probably never) write my own.

### Goals

The main goal of this project is to learn how to write an operating system. Using Rust is just a bonus.


## Features

- [x] Targeting bare-metal x86_64 architecture
- [ ] Cross-compilation
- [x] Basic build system 
- [ ] Custom build script
- [x] Bootloader (Using a third-party bootloader for now)
- [ ] Custom bootloader 
- [x] VGA Text Mode
- [x] Testing setup
- [ ] Interrupts
- [ ] Memory Management
- [ ] File System Drivers
- [ ] Storage Drivers
- [ ] Keyboard 
- [ ] Mouse
- [ ] Timer
- [ ] User space
- [ ] Shell
- [ ] System Calls
- [ ] Basic Networking
- [ ] Multitasking (Optional)
- [ ] GUI (Optional)
- [ ] Sound (Optional)
- [ ] Security (Optional)
- [ ] Optimizations (Optional)


### Requirements
- Rust toolchain (cargo, rustc, rustup)

Project is written in rust and uses cargo as the build system. Fortunately, you can just install rustup and it will
take care of everything for you. You can install rustup by following the instructions [here](https://rustup.rs/).

- gcc

The bootloader uses gcc during compilation.

- qemu

I am using qemu to run the kernel. You can install qemu by following the instructions [here](https://www.qemu.org/download/).
One of the future goals is to make the build steps customizable, so using qemu will be optional.

### Building

Make sure you have all the requirements installed before building the project.

Before building the project for the first time, there are some things that need to be configured first:

- Set the rust toolchain to nightly. Run the following command in the project directory:

```bash
rustup override set nightly
```

- Install bootimage tool.

```bash
cargo install bootimage
```

- Install rust-src component.

```bash
rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
```

- Install llvm-tools-preview component.

```bash
rustup component add llvm-tools-preview
```

Now you can build the project by running the following command:

```bash
cargo build
```

### Running

You can run the project by running the following command:

```bash
cargo run
```


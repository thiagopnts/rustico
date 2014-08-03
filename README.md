# Rustic

A tiny 32 bit kernel written in Rust.

It prints 'olar mundo' then hangs. That's it:

![](http://f.cl.ly/items/2U2g0i111J3O0E0C1G25/Screen%20Shot%202014-04-07%20at%209.22.52%20PM.png)

## Setup

You need a few things to run rustic:

1. `qemu`
2. a cross-compiler for i386
3. `nasm`
4. Rust's `master` branch.

### OSX

To set things up on OSX, do this:

Install `nasm` and `qemu` from homebrew:

```bash
$ brew install nasm
$ brew install qemu
```

Before install binutils, edit its formula to pass a i386-elf target:

```bash
$ brew edit binutils
```

and add this line to ./configure: 

"--target=i386-elf"

then:

```bash
$ brew install binutils
```

install Rust HEAD:

```bash
brew install rust --HEAD
```

## Running it

To compile, simply

```bash
$ make
```

To run,

```bash
$ make run
```

NOTE: It now uses libcore instead of rust-core for freestanding idiomatic rust. Libcore is now part of the default rust lib,
but in order to use it you will have to compile it to the target architecture(currently i386-intel-linux), to do it you must
compile from rust source and move to your rustlib dir:

first create our architecture specific dir:

`mkdir -p /usr/local/lib/rustlib/i386-intel-linux/lib`

then compile libcore for our new target:

`rustc --target i386-intel-linux -O -Z no-landing-pads src/libcore/lib.rs --out-dir /usr/local/lib/rustlib/i386-intel-linux/lib`

Now you will be able to cross compile rust.

NOTE2: this probably doesnt run on rust's HEAD anymore.

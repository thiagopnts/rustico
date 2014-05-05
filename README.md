# Rustic

A tiny 32 bit kernel written in Rust.

It prints 'olar mundo' then hangs. That's it:

![](http://f.cl.ly/items/2U2g0i111J3O0E0C1G25/Screen%20Shot%202014-04-07%20at%209.22.52%20PM.png)

## Setup

You need a few things to run rustic:

1. `qemu`
2. a cross-compiler for i386
3. `nasm`
4. Rust's `master` branch or 0.10 release.

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

install Rust 0.10:

```bash
brew install rust
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

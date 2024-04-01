# Phish

This is just a simple test to see how small I could make a Rust binary that does something simple. The application just
shows a Windows messagebox alert. The file size optimizations were done by following the wonderful documentation over
at [min-sized-rust](https://github.com/johnthagen/min-sized-rust).

The current binary size is 1604 bytes. Going much smaller would likely require handwritten assembly. If that sounds
interesting to you, take a look at [Alexander Sotirov's Tiny PE writeup](http://www.phreedom.org/research/tinype/).

## Usage

~~This project is useless~~ there are many ways to use this project. Here's how I personally use it:

1. build the project
2. rename the binary to `virus.exe`
3. send it to your friends with a message of "I sent you my malware pls respond 👉👈"
4. see how they react

## Build

```shell
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc --release
```

## Check Bloat

```shell
cargo +nightly bloat -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc --profile bloat
```

The above command gives the following output. Not too shabby!

```
    Analyzing target\i686-pc-windows-msvc\bloat\phish.exe

File  .text Size     Crate Name
1.6% 100.0%  28B [Unknown] _mainCRTStartup@0
0.0%   0.0%   0B           And 0 smaller methods. Use -n N to show more.
1.6% 100.0%  28B           .text section size, the file size is 1.7KiB
```

## Executable Compression

[UPX](https://upx.github.io/) no longer works since phish version 0.2.3, as the uncompressed binary is already too small.

```shell
upx --best --ultra-brute -o phish-compressed.exe phish.exe
```

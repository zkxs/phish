# Phish

This is just a simple test to see how small I could make a Rust binary that does something simple. The application just
shows a Windows messagebox alert.

The current binary size is 503 bytes. Note that presently ~170 bytes (34%) of that are just the strings shown in the
messagebox. With shortened strings, ~333 bytes would be achievable.

Going much smaller than the present binary size would likely require handwritten assembly. If that sounds interesting to
you, take a look at [Alexander Sotirov's Tiny PE writeup](http://www.phreedom.org/research/tinype/).

## Usage

~~This project is useless~~ there are many ways to use this project. Here's how I personally use it:

1. build the project
2. rename the binary to `virus.exe`
3. send it to your friends with a message of "I sent you my malware pls respond 👉👈"
4. see how they react

This may make slightly more sense given a screenshot as context:

![screenshot of a message box claiming to be an Albanian virus](screenshots/0.2.5.png)

Yep, it's just an Albanian Virus clone.

## Build

1. You must install [Crinkler](https://github.com/runestubbe/Crinkler). I recommend doing this via 
   [Chocolatey](https://chocolatey.org/) by running `choco install crinkler`.
2. Run the following:
   ```shell
   cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc --release
   ```

## Appendix

This appendix section includes various interesting things I looked into while making this project.

### Sources

- [min-sized-rust](https://github.com/johnthagen/min-sized-rust)
- [min-sized-rust-windows](https://github.com/mcountryman/min-sized-rust-windows)
- [Making the smallest Windows application](https://davidesnotes.com/articles/1/)

### Executable Compression

[UPX](https://upx.github.io/) no longer works since phish version [0.2.3](CHANGELOG.md#023---2024-04-01), as the uncompressed binary is
already too small. Furthermore, since [0.2.7](CHANGELOG.md#027---2024-04-04) Crinkler is used for link-time compression,
so introducing an additional compressor would be redundant.

### Check Bloat

[cargo-bloat](https://github.com/RazrFalcon/cargo-bloat) can be normally be used to measure where space is being used, but it is unable to parse Crinkler's
binaries.

### Assembly Debugging

It's not really necessary, but you can use [cargo-show-asm](https://github.com/pacak/cargo-show-asm) to see the generated assembly:

```shell
cargo +nightly asm -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target i686-pc-windows-msvc --bin=phish --profile=optimized-debug --intel --simplify --everything
```

There's not very much:

```asm
push 19 ; load the messagebox flags
push offset _anon.e8eee900910a047c66d8ad11464962b0.1 ; load the message title
push offset _anon.e8eee900910a047c66d8ad11464962b0.0 ; load the message text
push 0 ; null HWND pointer
call dword ptr [__imp__MessageBoxA@16] ; show the messagebox
ret ; exit the process cleanly using the above call's return value, which happens to be 2
```

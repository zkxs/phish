# Phish

This is just a simple test to see how small I could make a Rust binary that does something simple. The application just
shows a Windows messagebox alert. The file size optimizations were done by following the wonderful documentation over
at [min-sized-rust](https://github.com/johnthagen/min-sized-rust).

## Usage

~~This project is useless~~ there are many ways to use this project. Here's how I personally use it:

1. build the project, optionally compressing further with [upx](https://upx.github.io/). It should be about 10kB without UPX, or about 7kB with UPX.
2. rename the binary to `virus.exe`
3. send it to your friends with a message of "I sent you my malware pls respond 👉👈"
4. see how they react

## Build

```
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --release
```

## Check Bloat

For some reason `--config strip=false` doesn't work here, so you have to just remove it from the release profile temporarily.

```
cargo +nightly --config strip=false bloat -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-pc-windows-msvc --release
```

## Executable Compression

```
upx --best --ultra-brute -o phish-compressed.exe phish.exe
```

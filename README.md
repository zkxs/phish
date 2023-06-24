# Phish

This program just shows a Windows alert. That's all. To make it easier to send to people I've heavily optimized the build for file size, as per the wonderful documentation over at [min-sized-rust](https://github.com/johnthagen/min-sized-rust).

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

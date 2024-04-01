# Phish

This is just a simple test to see how small I could make a Rust binary that does something simple. The application just
shows a Windows messagebox alert. The file size optimizations were done by following the wonderful documentation over
at [min-sized-rust](https://github.com/johnthagen/min-sized-rust).

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

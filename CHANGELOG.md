# 0.2.7 - 2024-04-04

[Davide Pisanò's post](https://davidesnotes.com/articles/1/?page=4) pointed me at
[Crinkler](https://github.com/runestubbe/Crinkler), a compressing linker. The new binary size is 503 bytes.

## Changed

- Switch linker from link.exe to Crinkler, saving 367 bytes
- Return the result of the MessageBoxA instead of putting 0 in eax, saving a further 2 bytes. This makes the program
  exit with a code of `2` instead of `0`, but that's not really a problem as the exit code is unimportant for this
  application.

# 0.2.6 - 2024-04-04

link.exe tuning, saving 364 bytes and reducing binary size to 872 bytes.

## Changed

- Pass `/EMITPOGOPHASEINFO` to link.exe, removing some debug info
- Pass `/DYNAMICBASE:NO` to link.exe, removing the relocation table
- Pass a custom 64-byte MS-DOS stub in to link.exe. Credit to https://github.com/mcountryman/min-sized-rust-windows/pull/7

# 0.2.5 - 2024-04-03

Removes use of `ExitProcess`, saving 68 bytes and reducing binary size to 1.20 kiB.

## Changed

- Replace call to `ExitProcess(0)` with `return 0`

## Removed

- Removed panic handler, replacing it with undefined behavior
- Remove dependency on `Win32_System_Threading`, as we no longer call `ExitProcess`

# 0.2.4 - 2024-04-02

## Changed

- Switch from `MessageBoxW` to `MessageBoxA`, which makes all the strings take up half the space. 1.27kiB binary size.

# 0.2.3 - 2024-04-01

## Changed

- Reduce section alignment to 4 bytes. 1.56kiB binary size. UPX can no longer compress the binary.

# 0.2.2 - 2024-04-01

## Changed

- Switch from `x86_64-pc-windows-msvc` to `i686-pc-windows-msvc` target. 3.0kiB binary size, 2.5 kB with UPX.

# 0.2.1 - 2024-04-01

## Removed

- Removed error handling. No one was ever going to look at the returned error code. No significant change in binary size.

# 0.2.0 - 2024-04-01

## Changed

- Dropped the dependency on `win-msgbox` and switched to no_std. 3.5kiB binary size, 3.0 kiB with UPX. This has introduced
  a new issue where Windows claims "this program might not have run correctly" and asks if you'd like to turn on
  compatibility mode.

# 0.1.1 - 2024-04-01

## Changed

- Update dependencies. No significant change in binary size.

# 0.1.0 - 2023-06-23

## Added

- Initial release. Simple win32 MessageBox popup. ~10kiB binary size, ~7kiB with UPX.

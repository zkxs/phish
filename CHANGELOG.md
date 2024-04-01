# Changelog

# 0.2.3 - 2024-04-01

## Changed

- Reduce section alignment to 4 bytes. 1.56kB binary size. UPX can no longer compress the binary.

# 0.2.2 - 2024-04-01

## Changed

- Switch from `x86_64-pc-windows-msvc` to `i686-pc-windows-msvc` target. 3.0kB binary size, 2.5 kB with UPX.

# 0.2.1 - 2024-04-01

## Removed

- Removed error handling. No one was ever going to look at the returned error code. No significant change in binary size.

# 0.2.0 - 2024-04-01

## Changed

- Dropped the dependency on `win-msgbox` and switched to no_std. 3.5kB binary size, 3.0 kB with UPX. This has introduced
  a new issue where Windows claims "this program might not have run correctly" and asks if you'd like to turn on
  compatibility mode.

# 0.1.1 - 2024-04-01

## Changed

- Update dependencies. No significant change in binary size.

# 0.1.0 - 2023-06-23

## Added

- Initial release. Simple win32 MessageBox popup. ~10kB binary size, ~7kB with UPX.

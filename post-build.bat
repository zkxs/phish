@echo off
REM normal linker invocation:
REM "C:\\Program Files\\Microsoft Visual Studio\\2022\\Community\\VC\\Tools\\MSVC\\14.38.33130\\bin\\HostX64\\x86\\link.exe" "/NOLOGO" "/LARGEADDRESSAWARE" "/SAFESEH" "C:\\Users\\runtime\\AppData\\Local\\Temp\\rustcK8U5ws\\symbols.o" "F:\\git\\phish\\target\\i686-pc-windows-msvc\\release\\deps\\phish.phish.e86dbaee1f3ebffd-cgu.0.rcgu.o" "/LIBPATH:F:\\git\\phish\\target\\i686-pc-windows-msvc\\release\\deps" "/LIBPATH:F:\\git\\phish\\target\\release\\deps" "/LIBPATH:C:\\Users\\runtime\\.cargo\\registry\\src\\index.crates.io-6f17d22bba15001f\\windows_i686_msvc-0.52.4\\lib" "/LIBPATH:C:\\Users\\runtime\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "F:\\git\\phish\\target\\i686-pc-windows-msvc\\release\\deps\\libcompiler_builtins-be01ecc3f9ce74a0.rlib" "windows.0.52.0.lib" "/NXCOMPAT" "/LIBPATH:C:\\Users\\runtime\\.rustup\\toolchains\\nightly-x86_64-pc-windows-msvc\\lib\\rustlib\\i686-pc-windows-msvc\\lib" "/OUT:F:\\git\\phish\\target\\i686-pc-windows-msvc\\release\\deps\\phish.exe" "/SUBSYSTEM:windows" "/ENTRY:mainCRTStartup" "/OPT:REF,ICF" "/DEBUG:NONE"

cd "target\i686-pc-windows-msvc\release"

crinkler ^
/SUBSYSTEM:WINDOWS ^
/LARGEADDRESSAWARE:NO ^
/OUT:phish.exe ^
/ENTRY:mainCRTStartup ^
/NODEFAULTLIB ^
/LIBPATH:"C:\Program Files (x86)\Windows Kits\10\Lib\10.0.22000.0\um\x86" ^
/LIBPATH:".\deps" ^
/HASHSIZE:512 ^
/HASHTRIES:1024 ^
/TINYHEADER ^
/TINYIMPORT ^
/ORDERTRIES:128 ^
/UNSAFEIMPORT ^
/NOINITIALIZERS ^
/UNALIGNCODE ^
phish.lib kernel32.Lib User32.Lib

pause

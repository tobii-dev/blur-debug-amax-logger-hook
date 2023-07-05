# amax-logger-hook
DLL to display the calls to Blur's original ``amax_logger(...)`` function, used for debugging.

## Build
```
cargo +nightly build --release --target=i686-pc-windows-msvc
COPY "target\i686-pc-windows-msvc\release\amax_logger_hook.dll" "<path to blur>\amax\dlls\amax_logger_hook.dll"
```

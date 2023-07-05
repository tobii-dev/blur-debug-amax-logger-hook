# amax-logger-hook
DLL to display the calls to Blur's original ``amax\_logger(...)`` function, used for debugging Flask loader.

## Build
```
cargo +nightly build --release --target=i686-pc-windows-msvc
COPY "target\i686-pc-windows-msvc\release\amax_logger_hook.dll" "<path to blur>\amax\dlls\amax_logger_hook.dll"
```
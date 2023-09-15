# amax-logger-hook
AMAX Plugin to display the calls to Blur's original ``amax_logger(...)`` function. Used for debugging Flask stuff.

## Build
```
cargo +nightly build --release --target=i686-pc-windows-msvc
COPY \Y "target\i686-pc-windows-msvc\release\amax_logger_hook.dll" "<BLUR_DIR>\amax\dlls\amax_logger_hook.dll"
```

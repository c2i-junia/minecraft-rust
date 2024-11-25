@echo off
rem

python build.py debug
if %ERRORLEVEL% NEQ 0 (
    echo Build failed. Exiting...
    exit /b %ERRORLEVEL%
)

set RUST_LOG=client=debug,server=debug,shared=debug,warn
minecraft-rust\bin\minecraft-rust.exe

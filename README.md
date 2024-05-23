# pyo3_rust_copy_to_clipboard

## crosscompile

Example to `win_x86_64`

```bash
cd pyo3_rust
py_ver="3.10"
rustup target add x86_64-pc-windows-gnu
sudo apt install gcc-mingw-w64
PYO3_CROSS_PYTHON_VERSION=$py_ver cargo build --lib --release --target x86_64-pc-windows-gnu
cp target/x86_64-pc-windows-gnu/release/pyo3_rust.dll ../pyo3_rust.pyd
cd -
```

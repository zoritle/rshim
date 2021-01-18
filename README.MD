# `rshim`

rshim is the [`shim`](https://github.com/71/scoop-better-shimexe) program written in rust.

## Why this exist?
There are several versions of `shim`:
1. the official [`shim.cs`](https://github.com/lukesampson/scoop/blob/master/supporting/shimexe/shim.cs) was written in c# and required an instantiation of a .NET command line app every time it was started therefore is slower than directly executing. And it dose not handle Ctrl+C event correctly.

2. [`shim.c`](https://github.com/71/scoop-better-shimexe) is ok with performance and Ctrl+C event handling,but sometimes suffer memory violation (randomly exited with return code `3221226356`).

3. [`shim.cpp`](https://github.com/kiennq/scoop-better-shimexe) didn't work on my machine, calling it with any executable result in an infinite recursion of creating subprocess until eating all memory for unknown subtle reason.

### `rshim` :
1. less probability with bug and undefined behavior.
2. properly handle variety errors.
3. friendly error message.
4. properly handle Ctrl+C events.
5. properly handle utf-8 with or without bom in shim file.

## Installation
First install [rust](https://rustup.rs/)
```shell
cargo build --release
```
Close any running process with existed shim.
then
```shell
./repshims.bat
```


## Executable Size
Without `crt-static` enabled: around `200kb`

With `crt-static` enabled: around `300kb`

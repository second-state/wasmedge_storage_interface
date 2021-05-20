# Rust Storage Interface Library

A Rust library that provides Rust to WebAssembly developers with syntax for "load" & "store" functionality for their data when their Wasm is being executed on SecondState's WasmEdge.

From a high-level overview here, we are essentially building a storage interface that will allow the native operating system (which WasmEdge is running on) to play a part in the runtime execution. Specifically, play a part in facilitating the storing and loading of data as part of Wasm execution. 

# How to use this library

Please see the [official specification for this storage interface](https://github.com/second-state/specs/blob/master/storage_interface.md) for more information.

# Crates.io

The official crate is available at [crates.io](https://crates.io/crates/wasmedge_storage_interface)
# Rust Storage Interface Library

A Rust library that provides Rust to WebAssembly developers with syntax for "load" & "store" functionality for their data when their Wasm is being executed on SecondState's SSVM.

From a high-level overview here, we are essentially building a storage interface that will allow the native operating system (which SSVM is running on) to play a part in the runtime execution. Specifically, play a part in facilitating the storing and loading of data as part of Wasm execution. 

# How to use this library

SecondState team members who are setting up a runtime execution environment will compile this code to either a `.so` or a `.dylib` by simply typing the following `cargo build --release`. 

Rust developers who are writing code to be compiled and run as WebAssembly will only need to add this as a dependency and then access all of the function call syntax.


SSVM can provide a myriad of key:value storage endpoints such as leveldb, redis etc. However, this rust_storage_interface_library is focussed only on allowing Rust developers to include this library in their Rust -> Wasm application and use the pre-defined load and store functions such as `load_single_i32`, `store_single_i32` etc. This rust_storage_interface_library is also going to provide support for high level data types like Arrays, Strings, Structs etc. This will be incredibly useful in terms of application functionality.


## Stateful vs Stateless

Rust will never leave memory unfree, by default. When Rust code executes, all data values go out of scope as a function completes. 

What this essentially means is that Rust compiled to WebAssembly and then executed on a WebAssembly VM, will always result in stateless operation; when a given function is complete the stack is either left with no values or a single valid Wasm value. Nothing persists for future function calls.

## Wasm data types

As we know, WebAssembly only has 4 data types at present:
- i32
- i64
- f32
- f64

We also know that, at present, WebAssembly can **not** natively work with high level data types such as strings.

## Working with high level data types like strings

This rust_storage_interface_library allows Rust developers to call store and load functions such as `store_string` and `load_string`. This library will ensure that Rust code, using these functions will compile to valid WebAssembly. When the WebAssembly is executed by SecondState's Wasm VM (SSVM) the runtime will recognise these specific function calls and hand them off to native `.so` and/or `.dylib` libraries to be executed by the OS (because Wasm VMs are unable to recieve and return strings).

## Bindgen

This SecondState system is compatible with Rust code that has `wasm_bindgen` annotations. What this means is that the SecondState system is not only able to store and load high level data types but can also recieve and return strings via server-side Node.js implementations. For example where end-users are able to execute WebAssembly functions via HTTP Response/Requests using SecondState's [wasm-joey](https://github.com/second-state/wasm-joey); A lightweight Node.js application for deploying and executing WebAssembly(Wasm) binary-code via HTTP.

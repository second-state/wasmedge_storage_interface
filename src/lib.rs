//! Documentation for rust_storage_interface_library
//!
//! # Add as a dependency
//! Add the following line to the `[dependencies]` section of your applications `Cargo.toml` file (please confirm latest version number at [crates.io](https://crates.io/crates/rust_storage_interface_library))
//! ```
//! [dependencies]
//! rust_storage_interface_library = "^0.1"
//! ```
//! # Bring into scope
//! ```
//! use rust_storage_interface_library::ssvm_storage;
//! ```
//! # Store and load a String
//! This essentially stores and loads high level objects like strings by serializing them into an array of i32 variables. All you need to do is use the following simple syntax
//!```
//! let my_string = String::from("A string to store");
//! let storage_key: i32 = ssvm_storage::store::store_string(&my_string);
//! let new_string: String = ssvm_storage::load::load_string(storage_key);
//!```
//!
//! # Load a single i32
//! ```
//! let my_previously_stored_i32_value: i32 = ssvm_storage::load::loadi32(my_previously_saved_key: i32)
//! ```
//!
//! # Load a single i32
//! ```
//! let my_previously_stored_i32_value: i32 = ssvm_storage::load::loadi32(my_previously_saved_key)
//! ```
//!
//! # Store a single i32
//! Stores a single i32 value and returns a key which can be used to fetch the stored i32 value at a later date
//! ```
//! my_i32_to_store: i32 = 88;
//! my_new_key: i32 = ssvm_storage::store::storei32(my_i32_to_store)
//!
//! ```
//!
//! # Store a single i32
//! Stores a single i32 value and returns a key which can be used to fetch the stored i32 value at a later date
//! ```
//! my_i32_to_store: i32 = 88;
//! my_new_key: i32 = ssvm_storage::store::storei32(my_i32_to_store)
//!
//! ```

mod ssvm_storage {

    pub mod ssvm_native {
        extern "C" {
            pub fn ssvm_storage_createUUID() -> i32;
            pub fn ssvm_storage_beginStoreTx(new_i32_key: i32);
            pub fn ssvm_storage_beginLoadTx(new_i32_key: i32);
            pub fn ssvm_storage_storeI32(_i32_value: i32);
            pub fn ssvm_storage_loadI32() -> i32;
            pub fn ssvm_storage_endStoreTx();
            pub fn ssvm_storage_endLoadTx();
        }
    }

    mod utils {
        use super::ssvm_native;
        pub fn create_key_via_ssvm() -> i32 {
            unsafe {
                let new_i32_key: i32 = ssvm_native::ssvm_storage_createUUID();
                new_i32_key
            }
        }
    }

    pub mod load {
        use super::ssvm_native;
        use super::utils;
        use serialize_deserialize_u8_i32::s_d_u8_i32;
        use std::char;
        // Documentation for load module
        /// Load i32
        /// # Examples
        /// ```
        /// let my_previously_stored_i32_value: i32 = ssvm_storage::load::loadi32(my_previously_saved_key: i32)
        /// ```
        pub fn load_single_i32(_i32_key: i32) -> i32 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }

        /// let new_string: String = ssvm_storage::load::load_string(storage_key);
        pub fn load_string(_i32_key: i32) -> String {
            unsafe {
                let mut the_string = String::from("");
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_chars: i32 = ssvm_native::ssvm_storage_loadI32();
                for i in 0..number_of_chars {
                    let i32_char_representation: i32 = ssvm_native::ssvm_storage_loadI32();
                    let u32_char: u32 = i32_char_representation as u32;
                    let the_char: char = char::from_u32(u32_char).unwrap();
                    the_string.push(the_char);
                }
                the_string
            }
        }
    }

    pub mod store {
        use super::ssvm_native;
        use super::utils;
        use bincode;
        use serde::{Deserialize, Serialize};
        use serialize_deserialize_u8_i32::s_d_u8_i32;
        use std::convert::TryInto;

        pub fn generic_store<V: serde::ser::Serialize>(k: i32, v: V) {
            // Encoding any type to Vec<i32>
            let encoded_as_u8: Vec<u8> = bincode::serialize(&v).unwrap();
            let encoded_as_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(encoded_as_u8);
            // Begin store
            unsafe {
                ssvm_native::ssvm_storage_beginStoreTx(k);
            }
            // Add data length
            unsafe {
                ssvm_native::ssvm_storage_storeI32(encoded_as_i32.len().try_into().unwrap());
            }
            // Add the data
            unsafe {
                for i in encoded_as_i32.iter() {
                    ssvm_native::ssvm_storage_storeI32(*i);
                }
            }
            // End the store
            unsafe {
                ssvm_native::ssvm_storage_endStoreTx();
            }
        }
    }
}

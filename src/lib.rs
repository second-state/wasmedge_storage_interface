//! How to use this crate
//! # Adding this as a dependency
//! ```rust, ignore
//! [dependencies]
//! rust_storage_interface_library = "^0.1"
//! ```
//!
//! # Bringing this into scope
//! ```rust, ignore
//! use rust_storage_interface_library::wasmedge_storage;
//! ```
//! # Tests
//! ```bash, ignore
//! cargo test --lib
//! ```

pub mod wasmedge_storage {

    pub mod wasmedge_native {
        use std::os::raw::c_char;

        #[link(wasm_import_module = "wasmedge_native")]
        extern "C" {
            pub fn wasmedge_storage_createUUID(new_CString_key: *mut c_char);
            pub fn wasmedge_storage_beginStoreTx(new_CString_key: *const c_char);
            pub fn wasmedge_storage_beginLoadTx(new_CString_key: *const c_char);
            pub fn wasmedge_storage_storeI32(_i32_value: i32);
            pub fn wasmedge_storage_loadI32() -> i32;
            pub fn wasmedge_storage_endStoreTx();
            pub fn wasmedge_storage_endLoadTx();
        }
    }

    pub mod load {
        use super::wasmedge_native;
        use bincode;
        use serialize_deserialize_u8_i32::s_d_u8_i32;
        use std::char;
        use std::ffi::CString;
        use std::str;

        pub fn deserialize_vec_i32_to_unknown<'de, T: serde::de::DeserializeOwned>(
            _value: Vec<i32>,
            _t: T,
        ) -> T {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_unknown: T = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_unknown
        }
        pub fn load_as_struct<T: for<'de> serde::de::Deserialize<'de>>(
            _string_key: &str,
            _t: T,
        ) -> T {
            let mut struct_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_i32s {
                    struct_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let the_struct: T = deserialize_vec_i32_to_unknown(struct_vec, _t);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return the_struct;
            }
        }
        pub fn deserialize_vec_i32_to_bool(_value: Vec<i32>) -> bool {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_boolean: bool =
                bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_boolean
        }
        pub fn load_as_bool(_string_key: &str) -> bool {
            let mut bool_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_i32s {
                    bool_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let boolean: bool = deserialize_vec_i32_to_bool(bool_vec);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return boolean;
            }
        }
        pub fn deserialize_vec_i32_to_char(_value: Vec<i32>) -> char {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_char: char = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_char
        }
        pub fn load_as_char(_string_key: &str) -> char {
            let mut char_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_chars: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_chars {
                    char_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let character: char = deserialize_vec_i32_to_char(char_vec);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return character;
            }
        }
        pub fn deserialize_vec_i32_to_i8(_value: Vec<i32>) -> i8 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i8: i8 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i8
        }
        pub fn load_as_i8(_string_key: &str) -> i8 {
            let mut i8_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i8_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let i8_value: i8 = deserialize_vec_i32_to_i8(i8_vec);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return i8_value;
            }
        }
        pub fn deserialize_vec_i32_to_i16(_value: Vec<i32>) -> i16 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i16: i16 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i16
        }
        pub fn load_as_i16(_string_key: &str) -> i16 {
            let mut i16_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i16_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let i16_value: i16 = deserialize_vec_i32_to_i16(i16_vec);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return i16_value;
            }
        }
        pub fn deserialize_vec_i32_to_i32(_value: Vec<i32>) -> i32 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i32: i32 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i32
        }
        pub fn load_as_i32(_string_key: &str) -> i32 {
            let mut i32_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i32_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let i32_value: i32 = deserialize_vec_i32_to_i32(i32_vec);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return i32_value;
            }
        }
        pub fn deserialize_vec_i32_to_i64(_value: Vec<i32>) -> i64 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i64: i64 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i64
        }
        pub fn load_as_i64(_string_key: &str) -> i64 {
            let mut i64_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i64_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let i64_value: i64 = deserialize_vec_i32_to_i64(i64_vec);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return i64_value;
            }
        }
        pub fn deserialize_vec_i32_to_u8(_value: Vec<i32>) -> u8 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_u8: u8 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_u8
        }
        pub fn load_as_u8(_string_key: &str) -> u8 {
            let mut u8_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_i32s {
                    u8_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                let u8_value: u8 = deserialize_vec_i32_to_u8(u8_vec);
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                return u8_value;
            }
        }
        pub fn deserialize_vec_i32_to_string(_value: Vec<i32>) -> String{
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_string: String = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_string
        }
        pub fn load_as_string(_string_key: &str) -> String {
            let mut retrieved_vec = Vec::new();
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_i32s: i32 = wasmedge_native::wasmedge_storage_loadI32();
                // Get all i32s and save them to a vector
                for _i in 0..number_of_i32s {
                    retrieved_vec.push(wasmedge_native::wasmedge_storage_loadI32());
                }
                println!{"Deserialize i32 Vec: {:?}", retrieved_vec};
                // Convert that i32 vector, back into the original u8 vector
                let the_string = deserialize_vec_i32_to_string(retrieved_vec);
                // End load
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                the_string
            }
        }
        pub fn load_as_i32_vector(_string_key: &str) -> Vec<i32> {
            // Update - start (key as string)
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            let ptr_c_string = var_c_string.into_raw();
            // Update - end
            unsafe {
                let mut i32_vector = Vec::new();
                // Update - start (key as string)
                wasmedge_native::wasmedge_storage_beginLoadTx(ptr_c_string);
                // Update - end
                let number_of_items: i32 = wasmedge_native::wasmedge_storage_loadI32();
                for _i in 0..number_of_items {
                    let single_i32: i32 = wasmedge_native::wasmedge_storage_loadI32();
                    i32_vector.push(single_i32);
                }
                wasmedge_native::wasmedge_storage_endLoadTx();
                // Update - start (key as string)
                let var_pointer_released = CString::from_raw(ptr_c_string);
                // Update - end
                i32_vector
            }
        }
    }

    pub mod store {
        use super::wasmedge_native;
        use bincode;
        use serialize_deserialize_u8_i32::s_d_u8_i32;
        use std::any::type_name;
        use std::convert::TryInto;
        use std::ffi::CString;

        pub fn type_of<T>(_: T) -> &'static str {
            type_name::<T>()
        }

        pub fn serialize_unknown_to_vec_i32<V: std::clone::Clone + serde::ser::Serialize>(
            v: V,
        ) -> Vec<i32> {
            let encoded_as_u8: Vec<u8> = bincode::serialize(&v).unwrap();
            println!("Store - orig data is now encoded to u8: {:?}", encoded_as_u8);
            let encoded_as_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(encoded_as_u8);
            println!("Store - u8 data is now encoded to i32: {:?}", encoded_as_i32);
            encoded_as_i32
        }
        ///
        /// # Store String
        /// ```rust, ignore
        /// let my_string = String::from("A string to store");
        /// let storage_key: i32 = wasmedge_storage::store::store(my_string);
        /// ```

        /// # Store Struct
        /// Please note, you must implement the serde features as show below. For example the Default feature is used when re-loading this data back from storage.
        /// ```rust, ignore
        /// #[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
        ///     struct TestStruct {
        ///         a_vec: Vec<u8>,
        ///         a_i32: i32,
        ///         a_u8: u8,
        ///         a_bool: bool,
        ///     }
        /// ```
        pub fn store<V: std::clone::Clone + serde::ser::Serialize>(v: V) -> String {
            //let type_of_value = type_of(v.clone());
            //println!("{:?}", type_of_value);
            // Encode
            let encoded_as_i32: Vec<i32> = serialize_unknown_to_vec_i32(&v);
            // Create key as CString data type
            let var_c_string = CString::new("placeholder-32bytes-aaaaaaaaaaaa").expect("Error: The CString::new constructor has failed");
            // Convert the key to a pointer which can be modified by external C++ code (requires into_raw() as apposed to as_ptr())
            let ptr_c_string = var_c_string.into_raw();
            // Call the createUUID extern C function which allows WasmEdge to modify the CString's (pointer's) contents
            unsafe { wasmedge_native::wasmedge_storage_createUUID(ptr_c_string) }
            // Take ownership of the CString pointer back
            let var_c_string_2: CString= unsafe { CString::from_raw(ptr_c_string) };
            // Create another pointer variable to CString
            let ptr_c_string_2 = var_c_string_2.into_raw();
            // Call the beginStoreTx function using the newest pointer
            unsafe {
                wasmedge_native::wasmedge_storage_beginStoreTx(ptr_c_string_2);
            }
            let var_c_string_3: CString = unsafe { CString::from_raw(ptr_c_string_2) };
            // Add data length
            unsafe {
                wasmedge_native::wasmedge_storage_storeI32(encoded_as_i32.len().try_into().unwrap());
            }
            // Add the data
            unsafe {
                for i in encoded_as_i32.iter() {
                    wasmedge_native::wasmedge_storage_storeI32(*i);
                }
            }
            // End the store
            unsafe {
                wasmedge_native::wasmedge_storage_endStoreTx();
            }
            var_c_string_3.to_str().unwrap().to_string()
        }

       /// This function does not use serde or bincode it just takes Vec<i32> and stores it natively
        /// This is for developers who want to unpack their data ahead of time and make it directly available to the WasmEdge without conversion/serialization overheads at execution time
        pub fn store_as_i32_vector(i32_vector: Vec<i32>) -> String {
            // Create key as CString data type
            // Create key as CString data type
            let var_c_string = CString::new("placeholder")
                .expect("Error: The CString::new constructor has failed");
            // Convert the key to a pointer which can be modified by external C++ code (requires into_raw() as apposed to as_ptr())
            let ptr_c_string = var_c_string.into_raw();
            // Call the createUUID extern C function which allows WasmEdge to modify the CString's (pointer's) contents
            unsafe { wasmedge_native::wasmedge_storage_createUUID(ptr_c_string) }
            // Take ownership of the CString pointer back
            let var_c_string_2: CString = unsafe { CString::from_raw(ptr_c_string) };
            // Create another pointer variable to CString
            let ptr_c_string_2 = var_c_string_2.into_raw();
            // Call the beginStoreTx function using the newest pointer
            unsafe {
                wasmedge_native::wasmedge_storage_beginStoreTx(ptr_c_string_2);
            }
            // Retakes ownership of the newest pointer
            let var_c_string_3: CString = unsafe { CString::from_raw(ptr_c_string_2) };
            // store
            unsafe {
                wasmedge_native::wasmedge_storage_storeI32(i32_vector.len().try_into().unwrap());
            }
            // Add the data
            unsafe {
                for i in i32_vector.iter() {
                    wasmedge_native::wasmedge_storage_storeI32(*i);
                }
            }
            // End the store
            unsafe {
                wasmedge_native::wasmedge_storage_endStoreTx();
            }
            var_c_string_3.to_str().unwrap().to_string()
        }

        pub fn update<V: std::clone::Clone + serde::ser::Serialize>(_string_key: &str, v: V){
            // Create CString out of the string key 
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            // Create a pointer out of the CString
            let ptr_c_string = var_c_string.into_raw();
            // Encode
            let encoded_as_i32: Vec<i32> = serialize_unknown_to_vec_i32(&v);
            // Just go ahead and use the existing pointer, no need to create uuid here because we already have it pased in
            unsafe {
                wasmedge_native::wasmedge_storage_beginStoreTx(ptr_c_string);
            }
            let var_c_string_2: CString = unsafe { CString::from_raw(ptr_c_string) };
            // Add data length
            unsafe {
                wasmedge_native::wasmedge_storage_storeI32(encoded_as_i32.len().try_into().unwrap());
            }
            // Add the data
            unsafe {
                for i in encoded_as_i32.iter() {
                    wasmedge_native::wasmedge_storage_storeI32(*i);
                }
            }
            // End the store
            unsafe {
                wasmedge_native::wasmedge_storage_endStoreTx();
            }
        }

        /// This function does not use serde or bincode it just takes Vec<i32> and stores it natively
        /// This is for developers who want to unpack their data ahead of time and make it directly available to the WasmEdge without conversion/serialization overheads at execution time
        pub fn update_as_i32_vector(_string_key: &str, i32_vector: Vec<i32>){
            // Create CString out of the string key 
            let var_c_string = CString::new(_string_key).expect("CString::new failed");
            // Create a pointer out of the CString
            let ptr_c_string = var_c_string.into_raw();
            // Just go ahead and use the existing pointer, no need to create uuid here because we already have it pased in
            unsafe {
                wasmedge_native::wasmedge_storage_beginStoreTx(ptr_c_string);
            }
            let var_c_string_2: CString = unsafe { CString::from_raw(ptr_c_string) };
            // store
            unsafe {
                wasmedge_native::wasmedge_storage_storeI32(i32_vector.len().try_into().unwrap());
            }
            // Add the data
            unsafe {
                for i in i32_vector.iter() {
                    wasmedge_native::wasmedge_storage_storeI32(*i);
                }
            }
            // End the store
            unsafe {
                wasmedge_native::wasmedge_storage_endStoreTx();
            }
        }
    }
}

// Test
// Please use the following command so that the print statements are shown during testing
// cargo test -- --nocapture
//
#[cfg(test)]
mod tests {
    use super::wasmedge_storage;
    use serde::{Deserialize, Serialize};
    // It is a requirement that the user calling the store and load implements the serde Default etc. as shown below.
    #[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
    struct TestStruct {
        a_vec: Vec<u8>,
        a_i32: i32,
        a_u8: u8,
        a_bool: bool,
    }
    #[test]
    fn test_store_as_struct() {
        let test_struct1 = TestStruct {
            a_vec: vec![134, 122, 131],
            a_i32: 4,
            a_u8: 4,
            a_bool: true,
        };
        let _encoded_as_i32: Vec<i32> =
            wasmedge_storage::store::serialize_unknown_to_vec_i32(&test_struct1);
        println!("Encoded as Vec<i32>: {:?}", test_struct1);
        assert_eq!(
            wasmedge_storage::store::type_of(test_struct1),
            "rust_storage_interface_library::tests::TestStruct"
        );
    }
    #[test]
    fn test_load_as_struct() {
        let test_struct1 = TestStruct {
            a_vec: vec![134, 122, 131],
            a_i32: 4,
            a_u8: 4,
            a_bool: true,
        };
        let encoded_as_i32: Vec<i32> =
            wasmedge_storage::store::serialize_unknown_to_vec_i32(&test_struct1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let test_struct2 = TestStruct::default();
        let test_struct3 =
            wasmedge_storage::load::deserialize_vec_i32_to_unknown(encoded_as_i32, test_struct2);
        println!("Decoded as unknown: {:?}", test_struct3);
        assert_eq!(test_struct3.a_vec[0], 134);
        assert_eq!(test_struct3.a_i32, 4);
        assert_eq!(test_struct3.a_u8, 4);
        assert_eq!(test_struct3.a_bool, true);
    }
    #[test]
    fn test_store_as_bool() {
        let boolean1: bool = true;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&boolean1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(boolean1, true);
        assert_eq!(wasmedge_storage::store::type_of(boolean1), "bool");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 1);
    }
    #[test]
    fn test_load_as_boolean() {
        let boolean1: bool = true;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&boolean1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let boolean2 = wasmedge_storage::load::deserialize_vec_i32_to_bool(encoded_as_i32);
        println!("Decoded as boolean: {:?}", boolean2);
        assert_eq!(boolean2, true);
    }
    #[test]
    fn test_store_as_char() {
        let character1: char = 'a';
        let encoded_as_i32: Vec<i32> =
            wasmedge_storage::store::serialize_unknown_to_vec_i32(&character1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(character1, 'a');
        assert_eq!(wasmedge_storage::store::type_of(character1), "char");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 97);
    }
    #[test]
    fn test_load_as_character() {
        let character1: char = 'a';
        let encoded_as_i32: Vec<i32> =
            wasmedge_storage::store::serialize_unknown_to_vec_i32(&character1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let character2 = wasmedge_storage::load::deserialize_vec_i32_to_char(encoded_as_i32);
        println!("Decoded as character: {:?}", character2);
        assert_eq!(character2, 'a');
    }
    #[test]
    fn test_store_as_i8() {
        let i81: i8 = -2;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i81);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i81, -2);
        assert_eq!(wasmedge_storage::store::type_of(i81), "i8");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 254);
    }
    #[test]
    fn test_load_as_i8() {
        let i81: i8 = -2;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i81);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i82 = wasmedge_storage::load::deserialize_vec_i32_to_i8(encoded_as_i32);
        println!("Decoded as i8: {:?}", i82);
        assert_eq!(i82, -2);
    }
    #[test]
    fn test_store_as_i16() {
        let i161: i16 = -2;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i161);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i161, -2);
        assert_eq!(wasmedge_storage::store::type_of(i161), "i16");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 2000254255);
    }
    #[test]
    fn test_load_as_i16() {
        let i161: i16 = -2;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i161);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i162 = wasmedge_storage::load::deserialize_vec_i32_to_i16(encoded_as_i32);
        println!("Decoded as i16: {:?}", i162);
        assert_eq!(i162, -2);
    }
    #[test]
    fn test_store_as_i32() {
        let i321: i32 = 1234567890;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i321);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i321, 1234567890);
        assert_eq!(wasmedge_storage::store::type_of(i321), "i32");
        assert_eq!(encoded_as_i32.len(), 2);
        assert_eq!(encoded_as_i32[0], 1210002150);
        assert_eq!(encoded_as_i32[1], 73);
    }
    #[test]
    fn test_load_as_i32() {
        let i321: i32 = 1234567890;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i321);
        println!("I32 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i322 = wasmedge_storage::load::deserialize_vec_i32_to_i32(encoded_as_i32);
        println!("Decoded as i32: {:?}", i322);
        assert_eq!(i322, 1234567890);
    }
    #[test]
    fn test_store_as_i64() {
        let i641: i64 = 9223372036854775807;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i641);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i641, 9223372036854775807);
        assert_eq!(wasmedge_storage::store::type_of(i641), "i64");
        assert_eq!(encoded_as_i32.len(), 3);
        assert_eq!(encoded_as_i32[0], 1255255255);
        assert_eq!(encoded_as_i32[1], 1255255255);
        assert_eq!(encoded_as_i32[2], 2000255127);
    }
    #[test]
    fn test_load_as_i64() {
        let i641: i64 = 9223372036854775807;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&i641);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i642 = wasmedge_storage::load::deserialize_vec_i32_to_i64(encoded_as_i32);
        println!("Decoded as i64: {:?}", i642);
        assert_eq!(i642, 9223372036854775807);
    }
    #[test]
    fn test_store_as_u8() {
        let u81: u8 = 100;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&u81);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(u81, 100);
        assert_eq!(wasmedge_storage::store::type_of(u81), "u8");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 100);
    }
    #[test]
    fn test_load_as_u8() {
        let u81: u8 = 100;
        let encoded_as_i32: Vec<i32> = wasmedge_storage::store::serialize_unknown_to_vec_i32(&u81);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let u82 = wasmedge_storage::load::deserialize_vec_i32_to_u8(encoded_as_i32);
        println!("Decoded as u8: {:?}", u82);
        assert_eq!(u82, 100);
    }
}

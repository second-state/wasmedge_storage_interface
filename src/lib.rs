//! How to use this crate
//! # Adding this as a dependency
//! ```rust, ignore
//! [dependencies]
//! rust_storage_interface_library = "^0.1"
//! ```
//!
//! # Bringing this into scope
//! ```rust, ignore
//! use rust_storage_interface_library::ssvm_storage;
//! ```
//! # Tests
//! ```bash, ignore
//! cargo test --lib
//! ```
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub mod ssvm_storage {

    pub mod ssvm_native {
        #[link(wasm_import_module = "ssvm_native")]
        extern "C" {
            pub fn ssvm_storage_createUUID(new_CString_key: *mut c_char);
            pub fn ssvm_storage_beginStoreTx(new_CString_key: *const c_char);
            pub fn ssvm_storage_beginLoadTx(new_CString_key: *const c_char);
            pub fn ssvm_storage_storeI32(_i32_value: *const c_char);
            pub fn ssvm_storage_loadI32() -> i32;
            pub fn ssvm_storage_endStoreTx();
            pub fn ssvm_storage_endLoadTx();
        }
    }

    pub mod load {
        use super::ssvm_native;
        use bincode;
        use serialize_deserialize_u8_i32::s_d_u8_i32;
        use std::char;

        pub fn deserialize_vec_i32_to_unknown<'de, T: serde::de::DeserializeOwned>(
            _value: Vec<i32>,
            _t: T,
        ) -> T {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_unknown: T = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_unknown
        }
        pub fn load_as_struct<T: for<'de> serde::de::Deserialize<'de>>(_i32_key: i32, _t: T) -> T {
            let mut struct_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_i32s: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_i32s {
                    struct_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let the_struct: T = deserialize_vec_i32_to_unknown(struct_vec, _t);
                ssvm_native::ssvm_storage_endLoadTx();
                return the_struct;
            }
        }
        pub fn deserialize_vec_i32_to_bool(_value: Vec<i32>) -> bool {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_boolean: bool =
                bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_boolean
        }
        pub fn load_as_bool(_i32_key: i32) -> bool {
            let mut bool_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_i32s: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_i32s {
                    bool_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let boolean: bool = deserialize_vec_i32_to_bool(bool_vec);
                ssvm_native::ssvm_storage_endLoadTx();
                return boolean;
            }
        }
        pub fn deserialize_vec_i32_to_char(_value: Vec<i32>) -> char {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_char: char = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_char
        }
        pub fn load_as_char(_i32_key: i32) -> char {
            let mut char_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_chars: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_chars {
                    char_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let character: char = deserialize_vec_i32_to_char(char_vec);
                ssvm_native::ssvm_storage_endLoadTx();
                return character;
            }
        }
        pub fn deserialize_vec_i32_to_i8(_value: Vec<i32>) -> i8 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i8: i8 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i8
        }
        pub fn load_as_i8(_i32_key: i32) -> i8 {
            let mut i8_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_i32s: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i8_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let i8_value: i8 = deserialize_vec_i32_to_i8(i8_vec);
                ssvm_native::ssvm_storage_endLoadTx();
                return i8_value;
            }
        }
        pub fn deserialize_vec_i32_to_i16(_value: Vec<i32>) -> i16 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i16: i16 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i16
        }
        pub fn load_as_i16(_i32_key: i32) -> i16 {
            let mut i16_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_i32s: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i16_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let i16_value: i16 = deserialize_vec_i32_to_i16(i16_vec);
                ssvm_native::ssvm_storage_endLoadTx();
                return i16_value;
            }
        }
        pub fn deserialize_vec_i32_to_i32(_value: Vec<i32>) -> i32 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i32: i32 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i32
        }
        pub fn load_as_i32(_i32_key: i32) -> i32 {
            let mut i32_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_i32s: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i32_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let i32_value: i32 = deserialize_vec_i32_to_i32(i32_vec);
                ssvm_native::ssvm_storage_endLoadTx();
                return i32_value;
            }
        }
        pub fn deserialize_vec_i32_to_i64(_value: Vec<i32>) -> i64 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_i64: i64 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_i64
        }
        pub fn load_as_i64(_i32_key: i32) -> i64 {
            let mut i64_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_i32s: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_i32s {
                    i64_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let i64_value: i64 = deserialize_vec_i32_to_i64(i64_vec);
                ssvm_native::ssvm_storage_endLoadTx();
                return i64_value;
            }
        }
        pub fn deserialize_vec_i32_to_u8(_value: Vec<i32>) -> u8 {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_u8: u8 = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_u8
        }
        pub fn load_as_u8(_i32_key: i32) -> u8 {
            let mut u8_vec = Vec::new();
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_i32s: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_i32s {
                    u8_vec.push(ssvm_native::ssvm_storage_loadI32());
                }
                let u8_value: u8 = deserialize_vec_i32_to_u8(u8_vec);
                ssvm_native::ssvm_storage_endLoadTx();
                return u8_value;
            }
        }
        pub fn load_as_string(_i32_key: i32) -> String {
            unsafe {
                let mut the_string = String::from("");
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_chars: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_chars {
                    let i32_char_representation: i32 = ssvm_native::ssvm_storage_loadI32();
                    let u32_char: u32 = i32_char_representation as u32;
                    let the_char: char = char::from_u32(u32_char).unwrap();
                    the_string.push(the_char);
                }
                ssvm_native::ssvm_storage_endLoadTx();
                the_string
            }
        }
        pub fn load_as_i32_vector(_i32_key: i32) -> Vec<i32> {
            unsafe {
                let mut i32_vector = Vec::new();
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let number_of_items: i32 = ssvm_native::ssvm_storage_loadI32();
                for _i in 0..number_of_items {
                    let single_i32: i32 = ssvm_native::ssvm_storage_loadI32();
                    i32_vector.push(single_i32);
                }
                ssvm_native::ssvm_storage_endLoadTx();
                i32_vector
            }
        }
    }

    pub mod store {
        use super::ssvm_native;
        use bincode;
        use serialize_deserialize_u8_i32::s_d_u8_i32;
        use std::any::type_name;
        use std::convert::TryInto;

        pub fn type_of<T>(_: T) -> &'static str {
            type_name::<T>()
        }

        pub fn serialize_unknown_to_vec_i32<V: std::clone::Clone + serde::ser::Serialize>(
            v: V,
        ) -> Vec<i32> {
            let encoded_as_u8: Vec<u8> = bincode::serialize(&v).unwrap();
            let encoded_as_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(encoded_as_u8);
            encoded_as_i32
        }
        ///
        /// # Store String
        /// ```rust, ignore
        /// let my_string = String::from("A string to store");
        /// let storage_key: i32 = ssvm_storage::store::store(my_string);
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
        pub fn store<V: std::clone::Clone + serde::ser::Serialize>(v: V) -> i32 {
            let type_of_value = type_of(v.clone());
            println!("{:?}", type_of_value);
            // Encode
            let encoded_as_i32: Vec<i32> = serialize_unknown_to_vec_i32(&v);
            // Create key as CString data type
            let var_c_string = CString::new("placeholder").expect("Error: The CString::new constructor has failed");
            // Convert the key to a pointer which can be modified by external C++ code (requires into_raw() as apposed to as_ptr())
            let ptr_c_string = var_c_string.into_raw();
            // Call the createUUID extern C function which allows SSVM to modify the CString's (pointer's) contents
            unsafe {
                ssvm_native::ssvm_storage_createUUID(ptr_c_string)
            }
            // Take ownership of the CString pointer back
            let var_c_string_2 = unsafe { 
                CString::from_raw(ptr_c_string) 
            };
            // Create another pointer variable to CString
            let ptr_c_string_2 = var_c_string_2.into_raw();
            // Call the beginStoreTx function using the newest pointer
            unsafe {
                ssvm_native::ssvm_storage_beginStoreTx(ptr_c_string_2);
            }
            // Retakes ownership of the newest pointer
            let _var_c_string_3 = unsafe {
                CString::from_raw(ptr_c_string_2);
            };
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
            new_CStr_key
        }
        /// This function does not use serde or bincode it just takes Vec<i32> and stores it natively
        /// This is for developers who want to unpack their data ahead of time and make it directly available to the SSVM without conversion/serialization overheads at execution time
        pub fn store_as_i32_vector(i32_vector: Vec<i32>) -> i32 {
            // Create key as CString data type
            // Create key as CString data type
            let var_c_string = CString::new("placeholder").expect("Error: The CString::new constructor has failed");
            // Convert the key to a pointer which can be modified by external C++ code (requires into_raw() as apposed to as_ptr())
            let ptr_c_string = var_c_string.into_raw();
            // Call the createUUID extern C function which allows SSVM to modify the CString's (pointer's) contents
            unsafe {
                ssvm_native::ssvm_storage_createUUID(ptr_c_string)
            }
            // Take ownership of the CString pointer back
            let var_c_string_2 = unsafe { 
                CString::from_raw(ptr_c_string) 
            };
            // Create another pointer variable to CString
            let ptr_c_string_2 = var_c_string_2.into_raw();
            // Call the beginStoreTx function using the newest pointer
            unsafe {
                ssvm_native::ssvm_storage_beginStoreTx(ptr_c_string_2);
            }
            // Retakes ownership of the newest pointer
            let _var_c_string_3 = unsafe {
                CString::from_raw(ptr_c_string_2);
            };
            // Add data length
            unsafe {
                ssvm_native::ssvm_storage_storeI32(i32_vector.len().try_into().unwrap());
            }
            // Add the data
            unsafe {
                for i in i32_vector.iter() {
                    ssvm_native::ssvm_storage_storeI32(*i);
                }
            }
            // End the store
            unsafe {
                ssvm_native::ssvm_storage_endStoreTx();
            }
            new_CStr_key
        }
    }
}

// Test
// Please use the following command so that the print statements are shown during testing
// cargo test -- --nocapture
//
#[cfg(test)]
mod tests {
    use super::ssvm_storage;
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
            ssvm_storage::store::serialize_unknown_to_vec_i32(&test_struct1);
        println!("Encoded as Vec<i32>: {:?}", test_struct1);
        assert_eq!(
            ssvm_storage::store::type_of(test_struct1),
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
            ssvm_storage::store::serialize_unknown_to_vec_i32(&test_struct1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let test_struct2 = TestStruct::default();
        let test_struct3 =
            ssvm_storage::load::deserialize_vec_i32_to_unknown(encoded_as_i32, test_struct2);
        println!("Decoded as unknown: {:?}", test_struct3);
        assert_eq!(test_struct3.a_vec[0], 134);
        assert_eq!(test_struct3.a_i32, 4);
        assert_eq!(test_struct3.a_u8, 4);
        assert_eq!(test_struct3.a_bool, true);
    }
    #[test]
    fn test_store_as_bool() {
        let boolean1: bool = true;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&boolean1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(boolean1, true);
        assert_eq!(ssvm_storage::store::type_of(boolean1), "bool");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 1);
    }
    #[test]
    fn test_load_as_boolean() {
        let boolean1: bool = true;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&boolean1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let boolean2 = ssvm_storage::load::deserialize_vec_i32_to_bool(encoded_as_i32);
        println!("Decoded as boolean: {:?}", boolean2);
        assert_eq!(boolean2, true);
    }
    #[test]
    fn test_store_as_char() {
        let character1: char = 'a';
        let encoded_as_i32: Vec<i32> =
            ssvm_storage::store::serialize_unknown_to_vec_i32(&character1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(character1, 'a');
        assert_eq!(ssvm_storage::store::type_of(character1), "char");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 97);
    }
    #[test]
    fn test_load_as_character() {
        let character1: char = 'a';
        let encoded_as_i32: Vec<i32> =
            ssvm_storage::store::serialize_unknown_to_vec_i32(&character1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let character2 = ssvm_storage::load::deserialize_vec_i32_to_char(encoded_as_i32);
        println!("Decoded as character: {:?}", character2);
        assert_eq!(character2, 'a');
    }
    #[test]
    fn test_store_as_i8() {
        let i81: i8 = -2;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i81);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i81, -2);
        assert_eq!(ssvm_storage::store::type_of(i81), "i8");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 254);
    }
    #[test]
    fn test_load_as_i8() {
        let i81: i8 = -2;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i81);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i82 = ssvm_storage::load::deserialize_vec_i32_to_i8(encoded_as_i32);
        println!("Decoded as i8: {:?}", i82);
        assert_eq!(i82, -2);
    }
    #[test]
    fn test_store_as_i16() {
        let i161: i16 = -2;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i161);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i161, -2);
        assert_eq!(ssvm_storage::store::type_of(i161), "i16");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 2000254255);
    }
    #[test]
    fn test_load_as_i16() {
        let i161: i16 = -2;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i161);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i162 = ssvm_storage::load::deserialize_vec_i32_to_i16(encoded_as_i32);
        println!("Decoded as i16: {:?}", i162);
        assert_eq!(i162, -2);
    }
    #[test]
    fn test_store_as_i32() {
        let i321: i32 = 1234567890;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i321);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i321, 1234567890);
        assert_eq!(ssvm_storage::store::type_of(i321), "i32");
        assert_eq!(encoded_as_i32.len(), 2);
        assert_eq!(encoded_as_i32[0], 1210002150);
        assert_eq!(encoded_as_i32[1], 73);
    }
    #[test]
    fn test_load_as_i32() {
        let i321: i32 = 1234567890;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i321);
        println!("I32 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i322 = ssvm_storage::load::deserialize_vec_i32_to_i32(encoded_as_i32);
        println!("Decoded as i32: {:?}", i322);
        assert_eq!(i322, 1234567890);
    }
    #[test]
    fn test_store_as_i64() {
        let i641: i64 = 9223372036854775807;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i641);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(i641, 9223372036854775807);
        assert_eq!(ssvm_storage::store::type_of(i641), "i64");
        assert_eq!(encoded_as_i32.len(), 3);
        assert_eq!(encoded_as_i32[0], 1255255255);
        assert_eq!(encoded_as_i32[1], 1255255255);
        assert_eq!(encoded_as_i32[2], 2000255127);
    }
    #[test]
    fn test_load_as_i64() {
        let i641: i64 = 9223372036854775807;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&i641);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let i642 = ssvm_storage::load::deserialize_vec_i32_to_i64(encoded_as_i32);
        println!("Decoded as i64: {:?}", i642);
        assert_eq!(i642, 9223372036854775807);
    }
    #[test]
    fn test_store_as_u8() {
        let u81: u8 = 100;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&u81);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        assert_eq!(u81, 100);
        assert_eq!(ssvm_storage::store::type_of(u81), "u8");
        assert_eq!(encoded_as_i32.len(), 1);
        assert_eq!(encoded_as_i32[0], 100);
    }
    #[test]
    fn test_load_as_u8() {
        let u81: u8 = 100;
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&u81);
        println!("I64 - Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let u82 = ssvm_storage::load::deserialize_vec_i32_to_u8(encoded_as_i32);
        println!("Decoded as u8: {:?}", u82);
        assert_eq!(u82, 100);
    }
}

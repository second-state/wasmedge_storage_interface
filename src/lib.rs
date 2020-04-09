
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
        use serde::{Deserialize, Serialize};
        use bincode;
        
        pub fn deserialize_vec_i32_to_unknown<'de, T: serde::de::DeserializeOwned>(_value: Vec<i32>, t: T) -> T {
            let deserialized_to_u8: Vec<u8> = s_d_u8_i32::deserialize_i32_to_u8(_value);
            let deserialized_to_unknown: T = bincode::deserialize(&deserialized_to_u8[..]).unwrap();
            deserialized_to_unknown
        }

        /*
        pub fn load_as_bool(_i32_key: i32) -> bool {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }
        
        pub fn load_as_char(_i32_key: i32) -> char {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }
        pub fn load_as_i8(_i32_key: i32) -> i8 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }
        pub fn load_as_i16(_i32_key: i32) -> i16 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }
        pub fn load_as_i32(_i32_key: i32) -> i32 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }
        pub fn load_as_i64(_i32_key: i32) -> i64 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }
        pub fn load_as_u8(_i32_key: i32) -> u8 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }
        pub fn load_as_struct<T: std::clone::Clone + serde::ser::Serialize>(t: T) -> T {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i32_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }

        /// let new_string: String = ssvm_storage::load::load_string(storage_key);
        pub fn load_as_string(_i32_key: i32) -> String {
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
        */
    }

    pub mod store {
        use super::ssvm_native;
        use super::utils;
        use bincode;
        use serde::{Deserialize, Serialize};
        use serialize_deserialize_u8_i32::s_d_u8_i32;
        use std::any::type_name;
        use std::convert::TryInto;

        pub fn type_of<T>(_: T) -> &'static str {
            type_name::<T>()
        }

        pub fn serialize_unknown_to_vec_i32<V: std::clone::Clone + serde::ser::Serialize>(v: V) -> Vec<i32> {
            let encoded_as_u8: Vec<u8> = bincode::serialize(&v).unwrap();
            let encoded_as_i32: Vec<i32> = s_d_u8_i32::serialize_u8_to_i32(encoded_as_u8);
            encoded_as_i32
        }

        pub fn store<V: std::clone::Clone + serde::ser::Serialize>(v: V) -> i32 {
            let type_of_value = type_of(v.clone());
            println!("{:?}", type_of_value);

            // Encode
            let encoded_as_i32: Vec<i32> = serialize_unknown_to_vec_i32(&v);
            // Begin store
            let new_i32_key: i32 = utils::create_key_via_ssvm();
            unsafe {
                ssvm_native::ssvm_storage_beginStoreTx(new_i32_key);
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
            new_i32_key
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

    #[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
    struct TestStruct {
        a_vec: Vec<u8>,
        a_i32: i32,
        a_u8: u8,
        a_bool: bool,
    }
    #[test]
    fn test_store_as_struct() {
        let test_struct1 = TestStruct {a_vec: vec![134, 122, 131], a_i32: 4, a_u8: 4, a_bool: true};
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&test_struct1);
        println!("Encoded as Vec<i32>: {:?}", test_struct1);
        assert_eq!(ssvm_storage::store::type_of(test_struct1), "rust_storage_interface_library::tests::TestStruct");
    }
    #[test]
    fn test_load_as_struct() {
        let test_struct1 = TestStruct {a_vec: vec![134, 122, 131], a_i32: 4, a_u8: 4, a_bool: true};
        let encoded_as_i32: Vec<i32> = ssvm_storage::store::serialize_unknown_to_vec_i32(&test_struct1);
        println!("Encoded as Vec<i32>: {:?}", encoded_as_i32);
        let test_struct2 = TestStruct::default();
        let test_struct3 = ssvm_storage::load::deserialize_vec_i32_to_unknown(encoded_as_i32, test_struct2);
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

    /*
    #[test]
    fn xxxxxx() {
        let num3: i64 = ssvm_storage::store::create_fixed_length_random_number(-100, 1).unwrap();
        assert!(num3 > 0);
    }
    #[test]
    fn xxxxxxx() {
        let num4: i64 = ssvm_storage::store::create_fixed_length_random_number(1, 1000000000).unwrap();
        assert!(num4 < 999999999);
    }
    #[test]
    fn xxxxxxxx() {
        let num5: i64 = ssvm_storage::store::create_unique_key().unwrap();
        assert_eq!(19, num5.to_string().len());
    }
    */
}

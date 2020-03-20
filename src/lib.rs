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
//! let storage_key: i64 = ssvm_storage::store::store_string(&my_string);
//! let new_string: String = ssvm_storage::load::load_string(storage_key);
//!```
//!
//! # Load a single i32
//! ```
//! let my_previously_stored_i32_value: i32 = ssvm_storage::load::loadi32(my_previously_saved_key: i64)
//! ```
//!
//! # Load a single i64
//! ```
//! let my_previously_stored_i64_value: i64 = ssvm_storage::load::loadi64(my_previously_saved_key)
//! ```
//!
//! # Store a single i32
//! Stores a single i32 value and returns a key which can be used to fetch the stored i32 value at a later date
//! ```
//! my_i32_to_store: i32 = 88;
//! my_new_key: i64 = ssvm_storage::store::storei32(my_i32_to_store)
//!
//! ```
//!
//! # Store a single i64
//! Stores a single i64 value and returns a key which can be used to fetch the stored i64 value at a later date
//! ```
//! my_i64_to_store: i64 = 88;
//! my_new_key: i64 = ssvm_storage::store::storei64(my_i64_to_store)
//!
//! ```

mod ssvm_storage {

    pub mod ssvm_native {
        extern "C" {
            pub fn ssvm_storage_createUUID() -> i64;
            pub fn ssvm_storage_beginStoreTx(new_i64_key: i64);
            pub fn ssvm_storage_beginLoadTx(new_i64_key: i64);
            pub fn ssvm_storage_storeI32(_i32_value: i32);
            pub fn ssvm_storage_loadI32() -> i32;
            pub fn ssvm_storage_storeI64(_i64_value: i64);
            pub fn ssvm_storage_loadI64() -> i64;
            pub fn ssvm_storage_endStoreTx();
            pub fn ssvm_storage_endLoadTx();
        }
    }

    mod utils {
        use super::ssvm_native;
        pub fn create_key_via_ssvm() -> i64 {
            unsafe {
                let new_i64_key: i64 = ssvm_native::ssvm_storage_createUUID();
                new_i64_key
            }
        }

    }

    pub mod load {
        use super::ssvm_native;
        use super::utils;
        use std::char;
        // Documentation for load module
        /// Load i32
        /// # Examples
        /// ```
        /// let my_previously_stored_i32_value: i32 = ssvm_storage::load::loadi32(my_previously_saved_key: i64)
        /// ```
        pub fn load_single_i32(_i64_key: i64) -> i32 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i64_key);
                let fetched_i32_value: i32 = ssvm_native::ssvm_storage_loadI32();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i32_value;
            }
        }

        /// Load i64
        /// # Examples
        /// ```
        /// let my_previously_stored_i64_value: i64 = ssvm_storage::load::loadi64(my_previously_saved_key)
        /// ```
        pub fn load_single_i64(_i64_key: i64) -> i64 {
            unsafe {
                ssvm_native::ssvm_storage_beginLoadTx(_i64_key);
                let fetched_i64_value: i64 = ssvm_native::ssvm_storage_loadI64();
                ssvm_native::ssvm_storage_endLoadTx();
                return fetched_i64_value;
            }
        }

        /// let new_string: String = ssvm_storage::load::load_string(storage_key);
        pub fn load_string(_i64_key: i64) -> String {
            unsafe {
                let mut the_string = String::from("");
                ssvm_native::ssvm_storage_beginLoadTx(_i64_key);
                let number_of_chars: i32 = ssvm_native::ssvm_storage_loadI32();
                for i in 1..number_of_chars { // TODO do we start from 1 or zero here ????????????????????????????????????????????????????????????????????????????????????????
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
        use rand::Rng;
        use super::utils;
        extern crate rand;
        use std::convert::TryInto;
        use std::time::SystemTime;

        /// Get the system time in seconds (epoch) which produces a 10 digit number
        pub fn get_time() -> Result<u64, String> {
            match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
                Ok(n) => {
                    println!("Generated 10 digit time epoch: {:?}", n.as_secs());
                    Ok(n.as_secs())
                }
                Err(_) => Err("Unable to get system time".to_string()),
            }
        }

        // Creates an absolute (non negative) random number which is a fixed length. Specifically, one character less than the length of the upper bound
        pub fn create_fixed_length_random_number(
            _lower_bound: i64,
            _upper_bound: i64,
        ) -> Result<i64, rand::Error> {
            let length: u64 = (_upper_bound.to_string().len() - 1).try_into().unwrap();
            let mut rng = rand::thread_rng();
            let mut my_rand: i64 = rng.gen_range(_lower_bound, _upper_bound).abs();
            while my_rand.to_string().len() < length.try_into().unwrap() {
                my_rand = my_rand + rng.gen_range(_lower_bound, _upper_bound - my_rand);
            }
            println!(
                "Generated {:?} digit random number between {:?} and {:?}: {:?}",
                (_upper_bound.to_string().len() - 1),
                _lower_bound,
                _upper_bound - 1,
                my_rand
            );
            Ok(my_rand)
        }

        /// Join the time and random numbers from above to form a unique 19 digit key (to completely fill an i64 variable)
        /// Also make sure that the number is absolute (not negative) and does not exceed the max value of i64 (which is 9223372036854775807)
        pub fn create_unique_key() -> Result<i64, String> {
            let a = format!(
                "{:?}{:?}",
                get_time().unwrap(),
                create_fixed_length_random_number(1, 1000000000).unwrap()
            );
            let my_int: i64 = a.parse().unwrap();
            assert!(my_int.is_positive());
            assert!(my_int <= i64::max_value());
            Ok(my_int)
        }

        /// Store i32
        /// Stores a single i32 value and returns a key which can be used to fetch the stored i32 value at a later date
        /// # Examples
        /// ```
        /// my_i32_to_store: i32 = 88;
        /// my_new_key: i64 = ssvm_storage::store::storei32(my_i32_to_store)
        ///
        /// ```
        pub fn store_single_i32(_i32_value: i32) -> i64 {
            let new_i64_key: i64 = utils::create_key_via_ssvm();
            unsafe {
                ssvm_native::ssvm_storage_beginStoreTx(new_i64_key);
                ssvm_native::ssvm_storage_storeI32(_i32_value);
                ssvm_native::ssvm_storage_endStoreTx();
            }
            new_i64_key
        }

        /// Store i64
        /// Stores a single i64 value and returns a key which can be used to fetch the stored i64 value at a later date
        /// # Examples
        /// ```
        /// my_i64_to_store: i64 = 88;
        /// my_new_key: i64 = ssvm_storage::store::storei64(my_i64_to_store)
        ///
        /// ```
        pub fn store_single_i64(_i64_value: i64) -> i64 {
            let new_i64_key: i64 = utils::create_key_via_ssvm();
            unsafe {
                ssvm_native::ssvm_storage_beginStoreTx(new_i64_key);
                ssvm_native::ssvm_storage_storeI64(_i64_value);
                ssvm_native::ssvm_storage_endStoreTx();
            }
            new_i64_key
        }

        /// let my_string = String::from("A string to store");
        /// let storage_key: i64 = ssvm_storage::store::store_string(&my_string);
        pub fn store_string(_string_value: &str) -> i64 {
            let new_i64_key: i64 = utils::create_key_via_ssvm();
            // Take an example string
            let raw_string = String::from(_string_value);
            // Start the storage transaction
            println!("Starting the storage transaction");
            unsafe {
                //let new_i64_key: i64 = ssvm_native::ssvm_storage_createUUID();
                ssvm_native::ssvm_storage_beginStoreTx(new_i64_key);
            }
            // Find the length of the raw string in terms of unicode scalar values
            let raw_string_length: i32 = raw_string.chars().count().try_into().unwrap();
            unsafe {
                ssvm_native::ssvm_storage_storeI32(raw_string_length as i32);
            }
            // Storing each of the unicode scalar values to slots
            const MAX: i32 = i32::max_value();
            let mut live_char_count: i64 = 0;
            for single_char in raw_string.chars() {
                live_char_count = live_char_count + 1;
                if live_char_count < MAX.into() {
                    println!("Adding {:?}", single_char);
                    // A char can always be safely cast to a u32
                    let u32_single_char: u32 = single_char as u32;
                    unsafe {
                        // We then store this as i32 because this is WebAssembly compatible data type
                        ssvm_native::ssvm_storage_storeI32(u32_single_char as i32);
                    }
                } else {
                    panic!("The current size of characters in this string({:?}), has reached the limit({:?}) of what we can store in an i32 variable.");
                }
            }
            unsafe {
                ssvm_native::ssvm_storage_endStoreTx();
            }
            // Return the new unique key to the calling code
            new_i64_key
        }
    }
}

// Test
#[cfg(test)]
mod tests {
    use super::ssvm_storage;
    #[test]
    fn test_get_time_is_ten_digits() {
        let num1: u64 = ssvm_storage::store::get_time().unwrap();
        assert_eq!(10, num1.to_string().len());
    }
    #[test]
    fn test_create_fixed_length_random_number_is_nine_digits() {
        let num2: i64 =
            ssvm_storage::store::create_fixed_length_random_number(1, 1000000000).unwrap();
        assert_eq!(9, num2.to_string().len());
    }
    #[test]
    fn test_create_fixed_length_random_number_is_gt_0() {
        let num3: i64 = ssvm_storage::store::create_fixed_length_random_number(-100, 1).unwrap();
        assert!(num3 > 0);
    }
    #[test]
    fn test_create_fixed_length_random_number_is_lt_1000000000() {
        let num4: i64 =
            ssvm_storage::store::create_fixed_length_random_number(1, 1000000000).unwrap();
        assert!(num4 < 999999999);
    }
    #[test]
    fn test_create_unique_key_in_nineteen_digits() {
        let num5: i64 = ssvm_storage::store::create_unique_key().unwrap();
        assert_eq!(19, num5.to_string().len());
    }
}

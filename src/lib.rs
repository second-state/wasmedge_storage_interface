//! Documentation for rust_storage_interface_library
//!
//! Load i32
//! # Examples
//! ```
//! let my_previously_stored_i32_value: i32 = rust_storage_interface_library::load::loadi32(my_previously_saved_key: i64)
//! ```
//!
//! Load i64
//! # Examples
//! ```
//! let my_previously_stored_i64_value: i64 = rust_storage_interface_library::load::loadi64(my_previously_saved_key)
//! ```
//!
//! Store i32
//! Stores a single i32 value and returns a key which can be used to fetch the stored i32 value at a later date
//! # Examples
//! ```
//! my_i32_to_store: i32 = 88;
//! my_new_key: i64 = rust_storage_interface_library::store::storei32(my_i32_to_store)
//!
//! ```
//!
//! Store i64
//! Stores a single i64 value and returns a key which can be used to fetch the stored i64 value at a later date
//! # Examples
//! ```
//! my_i64_to_store: i64 = 88;
//! my_new_key: i64 = rust_storage_interface_library::store::storei64(my_i64_to_store)
//!
//! ```

mod load {
    // Documentation for load module
    /// Load i32
    /// # Examples
    /// ```
    /// let my_previously_stored_i32_value: i32 = rust_storage_interface_library::load::loadi32(my_previously_saved_key: i64)
    /// ```
    pub fn load_single_i32(_key: i64) -> i32 {
        // TODO - will require the syntax to interact with SecondState's other native library for SSVM which provides the database interaction
        // placeholder for now is 1
        1
    }

    /// Load i64
    /// # Examples
    /// ```
    /// let my_previously_stored_i64_value: i64 = rust_storage_interface_library::load::loadi64(my_previously_saved_key)
    /// ```
    pub fn load_single_i64(_key: i64) -> i64 {
        // TODO - will require the syntax to interact with SecondState's other native library for SSVM which provides the database interaction
        // placeholder for now is 1
        1
    }

    pub fn load_string(_key: i64) -> String {
        // The first thing we do is call SSVM database using the key and this will return (a list of i32s)
        // Just for now though, the test i32 is currently 7170388 which is equivalent to the String "Tim". Obviouisly longer strings will be stored in many i32s
        // For strings which are stored across many i32s we will be calling the database many times using the master key using its sequential suffix
        let _call_database_using_key: i32 = 7170388;
        // We need to convert each i32 to little endian bytes like this
        let value_as_le_bytes = _call_database_using_key.to_le_bytes();
        // TODO use the i32 that we fetched instead of this concrete value 7170388 above
        println!(
            "The i32 integer as little endian bytes: {:?}",
            value_as_le_bytes
        );
        // Output
        // The i32 integer as little endian bytes: [84, 105, 109, 0]

        // We now get the bytes into a vector and clean out the zeros that we may have used to fill the batch
        let mut vec_for_string = value_as_le_bytes.to_vec();
        while vec_for_string.get(vec_for_string.len() - 1) == Some(&0) {
            println!("Found a zero at position: {:?}", vec_for_string.len() - 1);
            vec_for_string.remove(vec_for_string.len() - 1);
        }
        // Output
        // Found a zero at position: 3

        // Lastly we pass the vec into the String::from_utf8
        let re_string = String::from_utf8(vec_for_string).unwrap();
        println!("String of bytes as string again: {:?}", re_string);
        // Output
        // String of bytes as string again: "Tim"
        String::from("Tim") // Placeholder
    }
}

mod store {

    use rand::Rng;
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
    /// my_new_key: i64 = rust_storage_interface_library::store::storei32(my_i32_to_store)
    ///
    /// ```
    pub fn store_single_i32(_value: i32) -> i64 {
        let new_key: i64 = create_unique_key().unwrap();
        // TODO - will require the syntax to interact with SecondState's other native library for SSVM which provides the database interaction
        // placeholder for now is just returning a key via create_random_i64
        new_key
    }

    /// Store i64
    /// Stores a single i64 value and returns a key which can be used to fetch the stored i64 value at a later date
    /// # Examples
    /// ```
    /// my_i64_to_store: i64 = 88;
    /// my_new_key: i64 = rust_storage_interface_library::store::storei64(my_i64_to_store)
    ///
    /// ```
    pub fn store_single_i64(_value: i64) -> i64 {
        let new_key: i64 = create_unique_key().unwrap();
        // TODO - will require the syntax to interact with SecondState's other native library for SSVM which provides the database interaction
        // placeholder for now is just returning a key via create_random_i64
        new_key
    }

    // Just FYI, this will store Strings which consist of many i32 integers, not just "Tim". The user will still only get a single key and use that (internally we will have sequential suffix abstraction)
    pub fn store_string(_value: &str) -> i64 {
        let new_key: i64 = create_unique_key().unwrap();
        // Take an example string
        let raw_string = String::from(_value);
        // Start the storage transaction
        println!("Starting the storage transaction");
        // TODO SSVM::beginStoreTx()

        // Find the length of the raw string in terms of unicode scalar values
        let raw_string_length: i32 = raw_string.chars().count().try_into().unwrap();
        // Ensure that the number that represents the length of the string is going to fit inside a i32 variable
        //assert!(raw_string_length <= i32::max_value());
        // Also make sure that the number that represents the length of the string has not overflowed i.e. is greater than 2147483647

        // Place the length of the raw string into the first slot of the sequence
        //println!("Adding the length ( {:?} ) of the raw string to the first slot.", raw_string_length);
        // TODO SSVM::storei32(char as i32);

        // Storing each of the unicode scalar values to slots
        const MAX: i32 = i32::max_value();
        let mut live_char_count: i64 = 0;
        for single_char in raw_string.chars() {
            live_char_count = live_char_count + 1;
            if live_char_count < MAX.into() {
                println!("Adding {:?}", single_char);
            // TODO SSVM:: storei32(char as i32);
            } else {
                panic!("The current size of characters in this string({:?}), has reached the limit({:?}) of what we can store in an i32 variable.");
            }
        }
        // Return the new unique key to the calling code
        new_key
    }
}

// Test
#[cfg(test)]
mod tests {
    use super::store;
    #[test]
    fn test_get_time_is_ten_digits() {
        let num1: u64 = store::get_time().unwrap();
        assert_eq!(10, num1.to_string().len());
    }
    #[test]
    fn test_create_fixed_length_random_number_is_nine_digits() {
        let num2: i64 = store::create_fixed_length_random_number(1, 1000000000).unwrap();
        assert_eq!(9, num2.to_string().len());
    }
    #[test]
    fn test_create_fixed_length_random_number_is_gt_0() {
        let num3: i64 = store::create_fixed_length_random_number(-100, 1).unwrap();
        assert!(num3 > 0);
    }
    #[test]
    fn test_create_fixed_length_random_number_is_lt_1000000000() {
        let num4: i64 = store::create_fixed_length_random_number(1, 1000000000).unwrap();
        assert!(num4 < 999999999);
    }
    #[test]
    fn test_create_unique_key_in_nineteen_digits() {
        let num5: i64 = store::create_unique_key().unwrap();
        assert_eq!(19, num5.to_string().len());
    }
}

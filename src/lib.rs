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
        // The test i32 is currently 7170388 which is equivalent to the String "Tim"
        // First we convert the i32 to little endian bytes
        let value_as_le_bytes = 7170388i32.to_le_bytes();
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
    extern crate rand;
    use rand::Rng;

    /// Create Random i64 Integer (which is positive i.e. non-negative)
    /// Generates random i64 number and then converts it to an absolute value
    /// Asserts that the value being returned is indeed absoulte i.e. is_positive
    fn create_random_i64() -> i64 {
        let mut rng = rand::thread_rng();
        let my_rand: i64 = rng.gen::<i64>().abs();
        assert!(my_rand.is_positive());
        my_rand
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
        let new_key: i64 = create_random_i64();
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
        let new_key: i64 = create_random_i64();
        // TODO - will require the syntax to interact with SecondState's other native library for SSVM which provides the database interaction
        // placeholder for now is just returning a key via create_random_i64
        new_key
    }

    // Just FYI, this will store Strings which consist of many i32 integers, not just "Tim". The user will still only get a single key and use that (internally we will have sequential suffix abstraction)
    pub fn store_string(_value: &str) -> i64 {
        // Take an example string
        let raw_string = String::from(_value);

        // Convert it into a byte array
        let string_as_bytes = raw_string.as_bytes();
        println!("String as bytes: {:?}", string_as_bytes);
        // Output
        // String as bytes: [84, 105, 109]

        // See how long the entire array is
        let string_as_bytes_length = raw_string.as_bytes().len();
        println!(
            "We have {:?} individual bytes to process ...",
            string_as_bytes_length
        );
        // Output
        // We have 3 individual bytes to process ...

        // Need to split this example string into batches of 4 bytes
        // Why? Because 8 of these 4 byte batches will make 32 bits which will fill a single i32

        // If the number of individual bytes is not exactly divisible by 4 then ...
        // We add as many zeros as required (topping up the byte array to be exactly 4 individual 8 bit parts) to the end of the byte array
        // [84, 105, 109, 0]

        // Need to generate a new i64 key for each batch (i.e. 1234001, 1234002, 1234003, 1234004)
        // We can tweak our existing create_random_i64() function, from line 90 above, to achieve this easily

        // We now convert the batch of 4 bytes to a full i32 integer
        let string_as_i32 = i32::from_le_bytes([84, 105, 109, 0]); // Obviously we get this from a variable (after topping up code exists) and not concrete values as shown here
        println!("String of bytes as i32: {:?}", string_as_i32);
        // Output
        // String of bytes as i32: 7170388

        // We now call the store_single_i32 as many times as necessary and then return the i64 key
        // We may need to create an additional store_single_i32 function which allows us to pass in the key
        // This way, we can mint the base key, also append the suffix each time but only send back the base key (knowing that we can handle the iteration here)
        1 // Placeholder return type
    }
}

// Background information

// WebAssembly - reference https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format

// Native WebAssembly Types
// i32
// i64
// f32
// f64

// Rust - reference https://doc.rust-lang.org/book/ch03-02-data-types.html#scalar-types

// Native Rust Integer Types (signed)
// i8
// i16
// i32 - Primitive Type i32 reference https://doc.rust-lang.org/std/primitive.i32.html
// i64 - Primitive Type i64 reference https://doc.rust-lang.org/std/primitive.i64.html
// i128

// Native Rust Integer Types (unsigned)
// u8
// u16
// u32
// u64
// u128

// High-level Rust Types which we are planning on catering for
// Array
// String
// Struct

// Test
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

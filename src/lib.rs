//! Documentation for rust_storage_interface_library

mod load {
    //! Documentation for load module
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

use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::mem;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct PhotonImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}

fn exceeding_max_i32_threshold(_num: i64) -> bool {
    const MAX: i32 = i32::max_value();
    if _num >= i32::max_value().into() {
        //println!("Sorry, {:?}, exceeds the maximum allowable data length which can be saved as an i32 number", _num);
        true
    } else {
        //println!("Data length, of {:?} is within the i32 threshold, continue ... ",_num);
        false
    }
}

fn count_vec_items_left(_vec: &Vec<u8>) -> i32 {
    let items_left: i32 = _vec.len().try_into().unwrap();
    //println!("{:?}, items left to process", items_left);
    items_left
}

fn flush_value_to_zero(_value: &mut i32, _position: i32, _size: i32) {
    //println!("Flushing Value ... \n Original value: {:?}", _value);
    *_value = *_value
        - ((*_value % (10_i32.pow(_position.try_into().unwrap())))
            - (*_value % (10_i32.pow((_position - _size).try_into().unwrap()))));
    //println!("Updated value: {:?}", _value);
    //_value
}

fn insert_value_at_position(_value: &mut i32, _single_value: i32, _position: i32, _size: i32) {
    let mut string_single_value = _single_value.to_string();
    while string_single_value.len() < _size.try_into().unwrap() {
        string_single_value = "0".to_owned() + &string_single_value;
    }
    let new_single_value = string_single_value.parse::<i32>();
    //println!("Inserting new value ... \n Original value: {:?}", _value);
    *_value = *_value + new_single_value.unwrap() * (10_i32.pow((_position - _size).try_into().unwrap()));
    //println!("Updated value: {:?}", _value);
    //_value
}

fn main() {
    let photon_image = PhotonImage {
        raw_pixels: vec![
            134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126,
            125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125,
            132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110,
            255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255,
        ],
        width: 4,
        height: 4,
    };

    let mut encoded: Vec<u8> = bincode::serialize(&photon_image).unwrap();
    //let mut encoded = vec![64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255];
    //for itemu8 in &encoded {
        //println!("{:?}", *itemu8);
    //}
    println!( "This is the completely serialized object as a byte array: {:?} \n",encoded);

    // Serialisation
    //

    // Create vector to hold the i32's
    let mut vec_of_i32s: Vec<i32> = Vec::new();
    //println!("Vector of i32s: {:?}", vec_of_i32s);

    // Test to see if there are too many i32s to store (we need to store the number of i32s in the first i32 so this can not exceed 2147483647)
    if exceeding_max_i32_threshold(count_vec_items_left(&encoded).into()) == false {
        let mut items_left: i32 = count_vec_items_left(&encoded);
        // Begin processing all of the data into i32s
        while items_left > 0 {
            // Create a placeholder i32
            let mut single_i32: i32 = 1000000000;
            // See how many items we have left in the serialised Vec<u8>
            if items_left == 1 {
                let one = &mut encoded.remove(0);
                //println!("One: {:?}", one);
                flush_value_to_zero(&mut single_i32, 3, 3);
                insert_value_at_position(&mut single_i32, *one as i32, 3, 3);
                // Set the indicator to 3
                flush_value_to_zero(&mut single_i32, 10, 1);
                // A single u8 stored in a single i32 will have a prefix of 3 - this is a code used in encoding/decoding
                insert_value_at_position(&mut single_i32, 3, 10, 1);
            }
            if items_left == 2 {
                let one = &mut encoded.remove(0);
                //println!("One: {:?}", one);
                let two = &mut encoded.remove(0);
                //println!("Two: {:?}", two);
                flush_value_to_zero(&mut single_i32, 6, 3);
                insert_value_at_position(&mut single_i32, *one as i32, 6, 3);
                flush_value_to_zero(&mut single_i32, 3, 3);
                insert_value_at_position(&mut single_i32, *two as i32, 3, 3);
                // Set the indicator to 2
                flush_value_to_zero(&mut single_i32, 10, 1);
                // When two u8s are stored in a single i32 it will have a prefix of 2 - this is a code used in encoding/decoding
                insert_value_at_position(&mut single_i32, 2, 10, 1);
            }
            if items_left >= 3 {
                let one = &mut encoded.remove(0);
                //println!("One: {:?}", one);
                let two = &mut encoded.remove(0);
                //println!("Two: {:?}", two);
                let three = &mut encoded.remove(0);
                //println!("Three: {:?}", three);
                flush_value_to_zero(&mut single_i32, 9, 3);
                insert_value_at_position(&mut single_i32, *one as i32, 9, 3);
                flush_value_to_zero(&mut single_i32, 6, 3);
                insert_value_at_position(&mut single_i32, *two as i32, 6, 3);
                flush_value_to_zero(&mut single_i32, 3, 3);
                insert_value_at_position(&mut single_i32, *three as i32, 3, 3);
                // Set the indicator to 2
                flush_value_to_zero(&mut single_i32, 10, 1);
                // When 3 u8s are stored in a single i32 it will have a prefix of 1 - this is a code used in encoding/decoding
                insert_value_at_position(&mut single_i32, 1, 10, 1);
            }
            // Calculate the remaining items left to process
            items_left = count_vec_items_left(&encoded);
            // Push this new i32 to the vec_of_i32s
            vec_of_i32s.push(single_i32);
            //println!("Vector u8s left to process -> {:?}", encoded);
        }
    }
    //println!("Finished processing");
    println!("Final encoded i32's ready to be stored in Wasm -> {:?}", vec_of_i32s);
}

/*
    let s = String::from_utf8(encoded);
    //println!("Serialised data as string: {:?}", s);

// 2147483647 is the max - 10 digits


    let decoded: PhotonImage = bincode::deserialize(&encoded[..]).unwrap();
    //println!(
        "Here is the high level Rust representation of the object: {:?} \n",
        decoded
    );
    */

// encode this [0, 34, 2, 131, 255]
//Step 1 - count the number of available items left to process
// If there are 3 or more available numbers then put a one upfront (this is so we can store byte arrays that start with 0)
// 1
// 1 always signals that we have a full set of 3 items each ranging from 0 to 255

// Step 2 - take the first 3 items and full the 9 remaining digits
// Convert the number to 3 digits if it is not already i.e. 0 becomes 000 and 14 becomes 014 etc.
// 1 000 034 002

//Step 3 - make that an i32 and store it
// 1000034002

// Step 4 - count the number of leftovers
// if there are 2 left overs, place a number 2 upfront, if there is only one left over place a number 3 upfront
// convert numbers to 3 digits as above, create i32 and store
// [131, 55]
// 2 131 055
// 2131055

// [55]
// 3 055
// 3055

// decode
// Step 1 - break into leading number (1, 2 or 3) followed by the sets of 3
// 1 000 034 002

// 2 131 055

// 3 055

// Step 2 - drop the leading zeros
// [1, 0, 34, 2]

// [2, 131, 55]

// [3, 55]

// Step 3 - drop the 1,2or3 at the start
// [0, 34, 2]

// [131, 55]

// [55]


// Unit tests for later on when this becomes a library
    /*

    ////println!("{:?}");
    //
    let mut _test_single_i32: i32 = 1000000000;
    //println!("{:?}", _test_single_i32);
    //
    flush_value_to_zero(&mut _test_single_i32, 3, 3);
    assert_eq!(_test_single_i32, 1000000000);
    //println!("{:?}", _test_single_i32);
    //
    insert_value_at_position(&mut _test_single_i32, 333, 9, 3);
    assert_eq!(_test_single_i32, 1333000000);
    //println!("{:?}", _test_single_i32);
    //
    flush_value_to_zero(&mut _test_single_i32, 9, 3);
    assert_eq!(_test_single_i32, 1000000000);
    //println!("{:?}", _test_single_i32);
    //
    insert_value_at_position(&mut _test_single_i32, 333, 3, 3);
    assert_eq!(_test_single_i32, 1000000333);
    //println!("{:?}", _test_single_i32);
    //
    //
    insert_value_at_position(&mut _test_single_i32, 333, 6, 3);
    assert_eq!(_test_single_i32, 1000333333);
    //println!("{:?}", _test_single_i32);
    //
    insert_value_at_position(&mut _test_single_i32, 8, 9, 3);
    assert_eq!(_test_single_i32, 1008333333);
    //println!("{:?}", _test_single_i32);
    //

    */

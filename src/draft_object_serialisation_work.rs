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

fn access_value(_value: i64, _position: i64, _size: i64) -> i64 {
	//               ((_value % (10 ** _position                          )) - (_value % (10 **      (_position - _size)                     ))) / (10 **      (_position - _size)                     )
    let _mode: i64 = ((_value % (10_i64.pow(_position.try_into().unwrap()))) - (_value % (10_i64.pow((_position - _size).try_into().unwrap())))) / (10_i64.pow((_position - _size).try_into().unwrap()));
    return _mode
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

    println!("Consider the following high level struct; in this case custom image data in the form of raw pixels, as well as, width and height values");
    println!("");
    println!("{:?}",photon_image);
    println!("");
    println!("Let's serialize the image struct to a byte array of u8 values (numbers between 0 and 255 only)");
    println!("");
    let mut encoded: Vec<u8> = bincode::serialize(&photon_image).unwrap();
    println!( "{:?}",encoded);
    println!("");
    let num_bytes = encoded.len();
    // Serialisation
    //

    // Create vector to hold the i32's
    let mut vec_of_i32s: Vec<i32> = Vec::new();
    println!("Let's now fill up a bunch of i32 variables with those u8s");
    println!("");

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
    println!("{:?}", vec_of_i32s);
    println!("");
    println!("We have essentially packed {:?} u8's into {:?} i32's", num_bytes, vec_of_i32s.len());
    println!("");

    // Decode
    println!("");
    println!("These can be decoded back to u8 at any time, as required.");
    println!("");
    let mut vec_of_u8s: Vec<u8> = Vec::new();
    for single_i32_from_vec in vec_of_i32s {
    	//println!("Processing: {:?}", single_i32_from_vec);
    	let mode: i64 = access_value(single_i32_from_vec.into(), 10, 1);
    	if mode == 1 {
    		//println!("Full: {:?}", single_i32_from_vec);
    		vec_of_u8s.push(access_value(single_i32_from_vec.into(), 9, 3).try_into().unwrap());
    		vec_of_u8s.push(access_value(single_i32_from_vec.into(), 6, 3).try_into().unwrap());
    		vec_of_u8s.push(access_value(single_i32_from_vec.into(), 3, 3).try_into().unwrap());
    	} 
    	if mode == 2 {
    		//println!("Full: {:?}", single_i32_from_vec);
    		vec_of_u8s.push(access_value(single_i32_from_vec.into(), 6, 3).try_into().unwrap());
    		vec_of_u8s.push(access_value(single_i32_from_vec.into(), 3, 3).try_into().unwrap());
    	} 
    	if mode == 3 {
    		vec_of_u8s.push(access_value(single_i32_from_vec.into(), 3, 3).try_into().unwrap());
    	} 
    }
        println!("{:?}", vec_of_u8s);
}



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

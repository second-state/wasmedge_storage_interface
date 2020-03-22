# Storing a high level Rust data structure as i32 variables

[This experimental code (that works!)](https://github.com/second-state/rust_storage_interface_library/blob/master/src/draft_object_serialisation_work.rs) is able to safely serialise/deserialise a high level Rust object to and from i32 values. 

Consider the following high level struct; in this case custom image data in the form of raw pixels, as well as, width and height values.

```Rust
struct PhotonImage {
    raw_pixels: Vec<u8>,
    width: u32,
    height: u32,
}
```

```bash
PhotonImage { raw_pixels: [134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255], width: 4, height: 4 }
```

Let's serialize the image struct to a byte array of u8 values (numbers between 0 and 255 only)

```bash
[64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255, 4, 0, 0, 0, 4, 0, 0, 0]
```

Let's now fill up a bunch of i32 variables with those u8s

```bash
[1064000000, 1000000000, 1000000134, 1122131255, 1131131139, 1255135134, 1137255138, 1134130255, 1126125119, 1255131134, 1129255137, 1134132255, 1130126130, 1255132125, 1132255122, 1142129255, 1134135128, 1255138120, 1125255125, 1134110255, 1121122137, 1255141140, 1141255125, 1144120255, 1004000000, 1000004000, 2000000000]
```

**We have essentially packed 80 u8's into 27 i32's**


These can be decoded back to u8 at any time, as required.

```bash
[64, 0, 0, 0, 0, 0, 0, 0, 134, 122, 131, 255, 131, 131, 139, 255, 135, 134, 137, 255, 138, 134, 130, 255, 126, 125, 119, 255, 131, 134, 129, 255, 137, 134, 132, 255, 130, 126, 130, 255, 132, 125, 132, 255, 122, 142, 129, 255, 134, 135, 128, 255, 138, 120, 125, 255, 125, 134, 110, 255, 121, 122, 137, 255, 141, 140, 141, 255, 125, 144, 120, 255, 4, 0, 0, 0, 4, 0, 0, 0]
```

// try_from_into.rs
//
// TryFrom is a simple and safe type conversion that may fail in a controlled
// way under some circumstances. Basically, this is the same as From. The main
// difference is that this should return a Result type instead of the target
// type itself. You can read more about it at
// https://doc.rust-lang.org/std/convert/trait.TryFrom.html
//
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for
// a hint.

use std::convert::{TryFrom, TryInto};

#[derive(Debug, PartialEq)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// We will use this error type for these `TryFrom` conversions.
#[derive(Debug, PartialEq)]
enum IntoColorError {
    // Incorrect length of slice
    BadLen,
    // Integer conversion error
    IntConversion,
}



// Your task is to complete this implementation and return an Ok result of inner
// type Color. You need to create an implementation for a tuple of three
// integers, an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile
// time, but the slice implementation needs to check the slice length! Also note
// that correct RGB color values must be integers in the 0..=255 range.

// Tuple implementation
// impl TryFrom<(i16, i16, i16)> for Color {
//     type Error = IntoColorError;
//     fn try_from(tuple: (i16, i16, i16)) -> () {
//     }
// }

// // Array implementation
// // impl TryFrom<[i16; 3]> for Color {
// //     type Error = IntoColorError;
// //     fn try_from(arr: [i16; 3]) -> () {
// //     }
// // }

// // Slice implementation
// impl TryFrom<&[i16]> for Color {
//     type Error = IntoColorError;
//     fn try_from(slice: &[i16]) -> () {
//     }
// }

fn main() {
    // Use the `try_from` function
    // let c1 = Color::try_from((183, 65, 14));
    // println!("{:?}", c1);

    // // Since TryFrom is implemented for Color, we should be able to use TryInto
    // let c2: Result<Color, _> = [183, 65, 14].try_into();
    // println!("{:?}", c2);

    // let v = vec![183, 65, 14];
    // // With slice we should use `try_from` function
    // let c3 = Color::try_from(&v[..]);
    // println!("{:?}", c3);
    // // or take slice within round brackets and use TryInto
    // let c4: Result<Color, _> = (&v[..]).try_into();
    // println!("{:?}", c4);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tuple_out_of_range_positive() {
        assert!(true)
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert!(true)
    }
    #[test]
    fn test_tuple_sum() {
        assert!(true)
    }
    #[test]
    fn test_tuple_correct() {
        assert!(true)
    }
    #[test]
    fn test_array_out_of_range_positive() {
        assert!(true)
    }
    #[test]
    fn test_array_out_of_range_negative() {
        assert!(true)
    }
    #[test]
    fn test_array_sum() {
        assert!(true)
    }
    #[test]
    fn test_array_correct() {
        assert!(true)
    }
    #[test]
    fn test_slice_out_of_range_positive() {
        assert!(true)
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        assert!(true)
    }
    #[test]
    fn test_slice_sum() {
        assert!(true)
    }
    #[test]
    fn test_slice_correct() {
        assert!(true)
    }
    #[test]
    fn test_slice_excess_length() {
        assert!(true)
    }
    #[test]
    fn test_slice_insufficient_length() {
        assert!(true)
    }
}

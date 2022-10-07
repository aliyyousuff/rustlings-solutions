// try_from_into.rs
// TryFrom is a simple and safe type conversion that may fail in a controlled way under some circumstances.
// Basically, this is the same as From. The main difference is that this should return a Result type
// instead of the target type itself.
// You can read more about it at https://doc.rust-lang.org/std/convert/trait.TryFrom.html
// Execute `rustlings hint try_from_into` or use the `hint` watch subcommand for a hint.

use std::convert::{TryFrom, TryInto};
use std::num::TryFromIntError;

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


// Your task is to complete this implementation
// and return an Ok result of inner type Color.
// You need to create an implementation for a tuple of three integers,
// an array of three integers, and a slice of integers.
//
// Note that the implementation for tuple and array will be checked at compile time,
// but the slice implementation needs to check the slice length!
// Also note that correct RGB color values must be integers in the 0..=255 range.

impl TryFrom<(i16, i16, i16)> for Color 
{
    type Error = IntoColorError;
    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> 
    {
        //todo!()
        //tuple_range(tuple);

        match tuple_range(tuple)
        {
            Ok(v) => 
            {
                return Ok( Color{ red: v[0], green: v[1], blue: v[2]} );
            }

            Err(_e) => { return Err(IntoColorError::IntConversion); }
        }
    }
}

// Array implementation
impl TryFrom<[i16; 3]> for Color 
{
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> 
    {
        match array_range(arr)
        {
            Ok(v) => 
            {
                return Ok( Color{ red: v[0], green: v[1], blue: v[2]} );
            }

            Err(_e) => { return Err(IntoColorError::IntConversion); }
        }
    }
}

impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> 
    {
        //todo!();

        if slice.len() != 3
        {
            return Err(Self::Error::BadLen);
        }

        match slice_range(slice)
        {
            Ok(v) => 
            {
                return Ok( Color{ red: v[0], green: v[1], blue: v[2]} );
            }

            Err(_e) => { return Err(IntoColorError::IntConversion); }
        }
    }
}

fn main()
{
    let a = [1, 2, 3, 4];
    println!("{}", a[0..2].len());

    
}

fn tuple_range(t: (i16, i16, i16)) -> Result<Vec<u8>, TryFromIntError>
{

    let temp_vec = vec![t.0, t.1, t.2];
    let mut temp_vec2 = vec![];

    for num in temp_vec.into_iter()
    {
        match u8::try_from(num)
        {
            Ok(n) => temp_vec2.push(n),
            Err(e)    => return Err(e)
        };
    }
    Ok(temp_vec2)
}

fn array_range(a: [i16; 3]) -> Result<[u8; 3], TryFromIntError>
{

    let mut c = [0, 0, 0];
    let mut index: usize = 0;

    for n in a
    {
         
        match u8::try_from(n)
        {
            Ok(n) =>  { c[index] = n; },
            Err(e) => return Err(e),
        }
        index += 1;
    }

    Ok(c)

}

fn slice_range(slice: &[i16]) -> Result<[u8; 3], TryFromIntError>
{
    let mut c = [0, 0, 0];
    let mut index: usize = 0;

    for n in slice
    {
         
        match u8::try_from(*n)
        {
            Ok(n) =>  { c[index] = n; },
            Err(e) => return Err(e),
        }
        index += 1;
    }

    Ok(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tuple_out_of_range_positive() {
        assert_eq!(
            Color::try_from((256, 1000, 10000)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_out_of_range_negative() {
        assert_eq!(
            Color::try_from((-1, -10, -256)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_sum() {
        assert_eq!(
            Color::try_from((-1, 255, 255)),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_tuple_correct() {
        let c: Result<Color, _> = (183, 65, 14).try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_array_out_of_range_positive() {
        let c: Result<Color, _> = [1000, 10000, 256].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_out_of_range_negative() {
        let c: Result<Color, _> = [-10, -256, -1].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_sum() {
        let c: Result<Color, _> = [-1, 255, 255].try_into();
        assert_eq!(c, Err(IntoColorError::IntConversion));
    }
    #[test]
    fn test_array_correct() {
        let c: Result<Color, _> = [183, 65, 14].try_into();
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }

    #[test]
    fn test_slice_out_of_range_positive() {
        let arr = [10000, 256, 1000];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_out_of_range_negative() {
        let arr = [-256, -1, -10];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_sum() {
        let arr = [-1, 255, 255];
        assert_eq!(
            Color::try_from(&arr[..]),
            Err(IntoColorError::IntConversion)
        );
    }
    #[test]
    fn test_slice_correct() {
        let v = vec![183, 65, 14];
        let c: Result<Color, _> = Color::try_from(&v[..]);
        assert!(c.is_ok());
        assert_eq!(
            c.unwrap(),
            Color {
                red: 183,
                green: 65,
                blue: 14
            }
        );
    }
    #[test]
    fn test_slice_excess_length() {
        let v = vec![0, 0, 0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }
    #[test]
    fn test_slice_insufficient_length() {
        let v = vec![0, 0];
        assert_eq!(Color::try_from(&v[..]), Err(IntoColorError::BadLen));
    }

}
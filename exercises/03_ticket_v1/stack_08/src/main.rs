// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        assert_eq!(size_of::<u16>(), 2);
    }

    #[test]
    fn i32_size() {
        assert_eq!(size_of::<i32>(), 4);
    }

    #[test]
    fn bool_size() {
        assert_eq!(size_of::<bool>(), 1);
    }
}


// Integer types
// std::mem::size_of::<i8>()   // 1 byte
// std::mem::size_of::<i16>()  // 2 bytes
// std::mem::size_of::<i32>()  // 4 bytes
// std::mem::size_of::<i64>()  // 8 bytes
// std::mem::size_of::<i128>() // 16 bytes/



// Unsigned integers
// std::mem::size_of::<u8>()   // 1 byte
// std::mem::size_of::<u32>()  // 4 bytes



// Boolean
// std::mem::size_of::<bool>() // 1 byte

// Character (Unicode scalar)
// std::mem::size_of::<char>() // 4 bytes

// Floating point
// std::mem::size_of::<f32>()  // 4 bytes
// std::mem::size_of::<f64>()  // 8 bytes

// Arrays (compile-time fixed size)
// std::mem::size_of::<[i32; 5]>()  // 20 bytes (4 * 5)

// Tuples
// std::mem::size_of::<(i32, i32)>()  // 8 bytes

// References (pointers)
// std::mem::size_of::<&i32>()    // 8 bytes on 64-bit, 4 on 32-bit
// std::mem::size_of::<&mut i32>() // 8 bytes on 64-bit
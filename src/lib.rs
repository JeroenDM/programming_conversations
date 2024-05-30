use std::mem::size_of;

pub fn high_byte(x: u32) -> u8 {
    (x >> (size_of::<u32>() * 8)) as u8
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(high_byte(0), 0);
//         // assert_eq!(high_byte(999), 0);

        
//     }
// }

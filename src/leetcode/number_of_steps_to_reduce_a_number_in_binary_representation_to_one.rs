// First iteration: BigInt Usage
// use std::ops::{Add, Rem, ShrAssign};

// use num_bigint::BigUint;
// use num_traits::{Num, One, Zero};

// pub fn num_steps(s: String) -> i32 {
//     let mut val = BigUint::from_str_radix(&s, 2).unwrap();
//     let mut steps = 0;

//     let one = BigUint::one();
//     let two = BigUint::from(2_u8);

//     while val != one {
//         if (&val).rem(&two).is_zero() {
//             val.shr_assign(1);
//         } else {
//             val = val.add(&one);
//         }
//         steps += 1;
//     }
//     steps
// }

// Second iteration: Manual bit movements
pub fn num_steps(s: String) -> i32 {
    let mut bits = s.into_bytes();
    let mut steps = 0;

    while bits.len() > 1 {
        steps += 1;

        if bits[bits.len() - 1] == b'0' {
            bits.pop();
        } else {
            let mut i = (bits.len() - 1) as i32;

            while i >= 0 && bits[i as usize] == b'1' {
                bits[i as usize] = b'0';
                i -= 1;
            }

            if i >= 0 {
                bits[i as usize] = b'1';
            } else {
                bits.insert(0, b'1');
            }
        }
    }
    steps
}

// Third Pass: Fully optimized carry utilization
// pub fn num_steps(s: String) -> i32 {
//     let bytes = s.as_bytes();
//     let mut steps = 0;
//     let mut carry = 0;

//     for i in (1..bytes.len()).rev() {
//         let bit = (bytes[i] - b'0') + carry;

//         if bit == 1 {
//             steps += 2;
//             carry = 1;
//         } else {
//             steps += 1;
//         }
//     }

//     steps + (carry as i32)
// }

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::leetcode::number_of_steps_to_reduce_a_number_in_binary_representation_to_one::num_steps;

    #[test_case("1101", 6)]
    #[test_case("10", 1)]
    #[test_case("1", 0)]
    #[test_case("1111011110000011100000110001011011110010111001010111110001", 85)]
    fn test_num_steps(s: &str, expected: i32) {
        assert_eq!(expected, num_steps(s.to_string()));
    }
}

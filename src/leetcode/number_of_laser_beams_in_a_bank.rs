// pub fn number_of_beams(bank: Vec<String>) -> i32 {
//     fn count_devices(row: &str) -> usize {
//         row.chars().filter(|b| *b == '1').count()
//     }

//     let mut row = 0;
//     let mut laser_count = 0;
//     while row < bank.len() {
//         let curr_row_devices = count_devices(&bank[row]);
//         row += 1;
//         if curr_row_devices > 0 {
//             while row < bank.len() {
//                 let next_row_devices = count_devices(&bank[row]);
//                 if next_row_devices > 0 {
//                     laser_count += curr_row_devices * next_row_devices;
//                     break;
//                 }
//                 row += 1;
//             }
//         }
//     }

//     laser_count as i32
// }

pub fn number_of_beams(bank: Vec<String>) -> i32 {
    let mut laser_count = 0;
    let mut previous = 0;
    for row in bank {
        let device_count = row.chars().filter(|b| *b == '1').count();
        if device_count > 0 {
            laser_count += previous * device_count;
            previous = device_count;
        }
    }
    laser_count as i32
}

#[cfg(test)]
mod tests {
    use crate::{leetcode::number_of_laser_beams_in_a_bank::number_of_beams, string_vec};

    #[test]
    fn test_number_of_beams() {
        assert_eq!(
            8,
            number_of_beams(string_vec!["011001", "000000", "010100", "001000"])
        );

        assert_eq!(0, number_of_beams(string_vec!["000", "111", "000"]));
    }
}

use crate::bit::{Bit16, Bit3};
use crate::chips::alu::two_complement::two_complement16;
use crate::{bit16string, bit3string};
use std::env;
use std::fs;

pub(crate) struct TestReader {}

impl TestReader {
    pub(crate) fn read(test_file_name: &'static str) -> Vec<String> {
        let current_directory = env::current_dir().unwrap();
        let file_path = current_directory
            .join("src/test_files")
            .join(test_file_name);
        let file_content = fs::read_to_string(file_path).unwrap();

        let tokens: Vec<String> = file_content
            .lines()
            .flat_map(|line| line.split("|"))
            .map(str::trim)
            .map(str::to_string)
            .filter(|a| !a.is_empty())
            .collect();

        tokens
    }

    pub(crate) fn from_16_bit_int_string(int_string: String) -> Bit16 {
        let is_negative = int_string.starts_with("-");
        let magnitude = int_string.parse::<i64>().unwrap().abs();

        let magnitude_as_binary = format!("{:b}", magnitude);
        let padding = "0".to_string().repeat(16 - magnitude_as_binary.len());
        let mut as_bit_16 = bit16string!(padding + &magnitude_as_binary);

        if is_negative {
            as_bit_16 = two_complement16(as_bit_16);
        }

        as_bit_16
    }

    // TODO: remove duplication
    //  to properly do this, would need to make all function generic over BitN
    //  tall order for now, so going to push to some future date
    pub(crate) fn from_3_bit_int_string(int_string: String) -> Bit3 {
        let is_negative = int_string.starts_with("-");
        let magnitude = int_string.parse::<i64>().unwrap().abs();

        let magnitude_as_binary = format!("{:b}", magnitude);
        let padding = "0".to_string().repeat(16 - magnitude_as_binary.len());
        let mut as_bit_16 = bit16string!(padding + &magnitude_as_binary);

        if is_negative {
            as_bit_16 = two_complement16(as_bit_16);
        }

        Bit3::from([as_bit_16[13], as_bit_16[14], as_bit_16[15]])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit16string;
    use crate::testing::TestReader;

    #[test]
    fn test_from_16bit_int_string() {
        assert_eq!(
            TestReader::from_16_bit_int_string("7".to_string()),
            bit16string!("0000000000000111")
        );
        assert_eq!(
            TestReader::from_16_bit_int_string("-7".to_string()),
            bit16string!("1111111111111001")
        );
        assert_eq!(
            TestReader::from_16_bit_int_string("32767".to_string()),
            bit16string!("0111111111111111")
        );
        assert_eq!(
            TestReader::from_16_bit_int_string("-32767".to_string()),
            bit16string!("1000000000000001")
        );
        assert_eq!(
            TestReader::from_16_bit_int_string("-32768".to_string()),
            bit16string!("1000000000000000")
        );
    }

    #[test]
    fn test_from_3bit_int_string() {
        assert_eq!(
            TestReader::from_3_bit_int_string("7".to_string()),
            bit3string!("111")
        );
        assert_eq!(
            TestReader::from_3_bit_int_string("-7".to_string()),
            bit3string!("001")
        );
    }
}

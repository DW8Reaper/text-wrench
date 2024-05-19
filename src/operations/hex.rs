use std::num::ParseIntError;

use super::text_operation::{TextOperation, TextOperationError, TextOperationResult};

pub struct OperationToHex {}

impl TextOperation for OperationToHex {
    fn get_id(&self) -> &'static str {
        "TO_HEX"
    }

    fn get_name(&self) -> &'static str {
        "UTF-8 to Hexadecimal"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let hex_string = String::from_iter(input.as_bytes().iter().map(|b| format!("{:x}", b)));

        Ok(TextOperationResult::with_string(hex_string))
    }
}

pub struct OperationToLongHex {}

impl TextOperation for OperationToLongHex {
    fn get_id(&self) -> &'static str {
        "TO_LONG_HEX"
    }

    fn get_name(&self) -> &'static str {
        "UTF-8 to Hexadecimal Long"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let hex_string =
            String::from_iter(input.as_bytes().iter().enumerate().map(|(i, b)| match i {
                0 => format!("{:#x}", b),
                _ => format!(" {:#x}", b),
            }));

        Ok(TextOperationResult::with_string(hex_string))
    }
}

pub struct OperationFromHex {}

impl TextOperation for OperationFromHex {
    fn get_id(&self) -> &'static str {
        "FROM_HEX"
    }

    fn get_name(&self) -> &'static str {
        "Hexadecimal to UTF-8"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let mut code_point: Vec<char> = Vec::with_capacity(4);
        let mut bytes: Vec<u8> = vec![];

        for (index, value) in input.chars().filter(|c| !c.is_whitespace()).enumerate() {
            if !"ABCDEFabcdef0123456789 x".contains(value) {
                return Err(TextOperationError::InvalidInputAtOffset(index));
            } else if value == ' ' {
                continue;
            }

            code_point.push(value);

            if code_point.len() == 4 || (code_point.len() == 2 && code_point[1] != 'x') {
                let mut parsed_bytes = parse_bytes(&code_point)
                    .map_err(|e| TextOperationError::InvalidInputError(e.to_string()))?;
                bytes.append(&mut parsed_bytes);
                code_point.clear();
            }
        }

        if code_point.len() > 0 {
            let mut parsed_bytes = parse_bytes(&code_point)
                .map_err(|e| TextOperationError::InvalidInputError(e.to_string()))?;
            bytes.append(&mut parsed_bytes);
            code_point.clear();
        }

        let value = String::from_utf8(bytes)
            .map_err(|e| TextOperationError::InvalidInputError(e.to_string()))?;
        Ok(TextOperationResult::with_string(value))
    }
}

fn parse_bytes(chars: &Vec<char>) -> Result<Vec<u8>, ParseIntError> {
    if chars.len() < 2 || chars.len() > 4 {
        Ok(Vec::<u8>::new())
    } else if chars.len() < 4 {
        // always only use 2 bytes if there is a third dangler ignore it
        let byte1 = u8::from_str_radix(String::from_iter(chars[0..2].iter()).as_str(), 16)?;
        Ok(vec![byte1])
    } else if chars.len() == 4 && chars[0] == '0' && chars[1] == 'x' {
        let byte1 = u8::from_str_radix(String::from_iter(chars[2..4].iter()).as_str(), 16)?;
        Ok(vec![byte1])
    } else {
        let byte1 = u8::from_str_radix(String::from_iter(chars[0..2].iter()).as_str(), 16)?;
        let byte2 = u8::from_str_radix(String::from_iter(chars[2..4].iter()).as_str(), 16)?;
        Ok(vec![byte1, byte2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_converts_bytes_to_string_to_utf8() {
        let bytes: Vec<u8> = String::from("414243").bytes().collect();

        let operation = OperationFromHex {};
        let result = operation.convert_bytes(&bytes);

        assert_eq!(result.unwrap().text_value.unwrap(), "ABC");
    }

    #[test]
    fn it_converts_hex_string_to_utf8() {
        let operation = OperationFromHex {};

        let result = operation.convert("414243");
        assert_eq!(result.unwrap().text_value.unwrap(), "ABC");
    }

    #[test]
    fn it_converts_hex_string_with_0x_to_utf8() {
        let operation = OperationFromHex {};

        let result = operation.convert("0x410x420x43");
        assert_eq!(result.unwrap().text_value.unwrap(), "ABC");
    }

    #[test]
    fn it_converts_hex_string_with_whitespace_to_utf8() {
        let operation = OperationFromHex {};

        let result = operation.convert("0x41 42 \n 0x43");
        assert_eq!(result.unwrap().text_value.unwrap(), "ABC");
    }

    #[test]
    fn it_ignores_additional_digits_converting_to_utf8() {
        let operation = OperationFromHex {};

        let result = operation.convert("0x41 424");
        assert_eq!(result.unwrap().text_value.unwrap(), "AB");
    }

    #[test]
    fn it_converts_utf8_string_to_hex() {
        let operation = OperationToHex {};

        let result = operation.convert("ABC");
        assert_eq!(result.unwrap().text_value.unwrap(), "414243");
    }

    #[test]
    fn it_converts_utf8_string_to_long_form_hex() {
        let operation = OperationToLongHex {};

        let result = operation.convert("ABC");
        assert_eq!(result.unwrap().text_value.unwrap(), "0x41 0x42 0x43");
    }
}

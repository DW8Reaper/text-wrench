use super::text_operation::{TextOperation, TextOperationError, TextOperationResult};

pub struct OperationUpperCase {}

impl TextOperation for OperationUpperCase {
    fn get_id(&self) -> &'static str {
        "UPPER_CASE"
    }

    fn get_name(&self) -> &'static str {
        "To upper case"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        Ok(TextOperationResult::with_string(input.to_uppercase()))
    }
}

pub struct OperationLowerCase {}

impl TextOperation for OperationLowerCase {
    fn get_id(&self) -> &'static str {
        "LOWER_CASE"
    }

    fn get_name(&self) -> &'static str {
        "To lower case"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        Ok(TextOperationResult::with_string(input.to_lowercase()))
    }
}

#[cfg(test)]
mod test {
    use super::OperationLowerCase;
    use super::OperationUpperCase;
    use super::TextOperation;

    #[test]
    fn it_converts_to_upper_case() {
        let operation = OperationUpperCase {};

        let result = operation.convert("aBcD");
        assert_eq!(result.unwrap().text_value.unwrap(), "ABCD");
    }

    #[test]
    fn it_converts_to_lower_case() {
        let operation = OperationLowerCase {};

        let result = operation.convert("aBcD");
        assert_eq!(result.unwrap().text_value.unwrap(), "abcd");
    }
}

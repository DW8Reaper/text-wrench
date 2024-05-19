use super::text_operation::{TextOperation, TextOperationError, TextOperationResult};

pub struct OperationNone {}

impl TextOperation for OperationNone {
    fn get_id(&self) -> &'static str {
        "NONE"
    }

    fn get_name(&self) -> &'static str {
        "None"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        Ok(TextOperationResult::with_string(String::from(input)))
    }
}

use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum TextOperationError {
    InvalidInput(),
    InvalidInputError(String),
    InvalidInputAtOffset(usize),
}

impl Display for TextOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for TextOperationError {}

pub struct TextOperationResult {
    pub text_value: Option<String>,
    pub byte_value: Option<Vec<u8>>,
}

impl TextOperationResult {
    pub fn with_string(string: String) -> Self {
        TextOperationResult {
            text_value: Some(string),
            byte_value: None,
        }
    }

    pub fn with_bytes(bytes: Vec<u8>) -> Self {
        TextOperationResult {
            text_value: None,
            byte_value: Some(bytes),
        }
    }
}

impl Into<String> for TextOperationResult {
    fn into(self) -> String {
        match self.text_value {
            Some(text) => text,
            None => match self.byte_value {
                Some(bytes) => bytes.iter().map(|b| format!("{:x}", b)).collect(),
                None => String::from(""),
            },
        }
    }
}

pub trait TextOperation {
    fn get_id(&self) -> &'static str;

    fn get_name(&self) -> &'static str;

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError>;

    fn convert_bytes(&self, input: &Vec<u8>) -> Result<TextOperationResult, TextOperationError> {
        let maybe_input = String::from_utf8(input.clone());
        match maybe_input {
            Ok(input) => self.convert(input.as_str()),
            Err(_) => Err(TextOperationError::InvalidInput()),
        }
    }

    fn get_inverse(&self) -> Option<String> {
        None
    }
}

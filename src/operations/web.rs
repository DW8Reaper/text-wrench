use super::text_operation::TextOperation;
use super::text_operation::TextOperationError;
use super::text_operation::TextOperationResult;

use base64::Engine;

pub struct OperationEncodeBase64 {}

impl TextOperation for OperationEncodeBase64 {
    fn get_id(&self) -> &'static str {
        "TO_BASE64"
    }

    fn get_name(&self) -> &'static str {
        "Base64 Encode"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let encoded = base64::engine::general_purpose::STANDARD.encode(input);
        Ok(TextOperationResult::with_string(encoded))
    }
}

pub struct OperationEncodeBase64UrlSafe {}

impl TextOperation for OperationEncodeBase64UrlSafe {
    fn get_id(&self) -> &'static str {
        "TO_URL_BASE64"
    }

    fn get_name(&self) -> &'static str {
        "Base64 (URL Safe) Encode"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let encoded = base64::engine::general_purpose::URL_SAFE.encode(input);
        Ok(TextOperationResult::with_string(encoded))
    }
}

pub struct OperationDecodeBase64 {}

impl TextOperation for OperationDecodeBase64 {
    fn get_id(&self) -> &'static str {
        "FROM_BASE64"
    }

    fn get_name(&self) -> &'static str {
        "Base64 Decode"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let decoded = match base64::engine::general_purpose::STANDARD.decode(input) {
            Ok(v) => String::from_utf8(v).unwrap_or_default(),
            Err(_) => match base64::engine::general_purpose::URL_SAFE.decode(input) {
                Ok(v) => String::from_utf8(v).unwrap_or_default(),
                Err(e) => e.to_string(),
            },
        };

        Ok(TextOperationResult::with_string(decoded))
    }
}

pub struct OperationEncodeURL {}

impl TextOperation for OperationEncodeURL {
    fn get_id(&self) -> &'static str {
        "URL_ENCODE"
    }

    fn get_name(&self) -> &'static str {
        "URL Encode"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let encoded = String::from(urlencoding::encode(input));
        Ok(TextOperationResult::with_string(encoded))
    }
}

pub struct OperationDecodeURL {}

impl TextOperation for OperationDecodeURL {
    fn get_id(&self) -> &'static str {
        "URL_DECODE"
    }

    fn get_name(&self) -> &'static str {
        "URL Decode"
    }

    fn convert(&self, input: &str) -> Result<TextOperationResult, TextOperationError> {
        let decoded = urlencoding::decode(input)
            .map_err(|e| TextOperationError::InvalidInputError(e.to_string()))?;
        Ok(TextOperationResult::with_string(String::from(decoded)))
    }
}

#[cfg(test)]
mod test {
    use crate::operations::web::OperationDecodeBase64;
    use crate::operations::web::OperationEncodeBase64UrlSafe;

    use super::TextOperation;

    use super::OperationEncodeBase64;

    #[test]
    fn it_encodes_utf8_to_base64_string() {
        let operation = OperationEncodeBase64 {};

        let result = operation.convert("av===> 1");
        assert_eq!(result.unwrap().text_value.unwrap(), "YXY9PT0+IDE=")
    }

    #[test]
    fn it_encodes_utf8_to_url_safe_base64_string() {
        let operation = OperationEncodeBase64UrlSafe {};

        let result = operation.convert("av===> 1");
        assert_eq!(result.unwrap().text_value.unwrap(), "YXY9PT0-IDE=")
    }

    #[test]
    fn it_decodes_base64_string_to_utf8() {
        let operation = OperationDecodeBase64 {};

        let result = operation.convert("YXY9PT0+IDE=");
        assert_eq!(result.unwrap().text_value.unwrap(), "av===> 1")
    }

    #[test]
    fn it_decodes_url_safe_base64_string_to_utf8() {
        let operation = OperationDecodeBase64 {};

        let result = operation.convert("YXY9PT0-IDE=");
        assert_eq!(result.unwrap().text_value.unwrap(), "av===> 1")
    }
}

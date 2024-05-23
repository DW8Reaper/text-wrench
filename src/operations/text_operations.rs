use super::hex::OperationFromHex;
use super::hex::{OperationToHex, OperationToLongHex};
use super::none::OperationNone;
use super::text::{OperationLowerCase, OperationUpperCase};
pub use super::text_operation::{TextOperation, TextOperationError, TextOperationResult};
use super::web::{
    OperationDecodeBase64, OperationDecodeURL, OperationEncodeBase64, OperationEncodeBase64UrlSafe,
    OperationEncodeURL,
};
use std::sync::OnceLock;
use std::{collections::HashMap, sync::Arc};

static TEXT_OPERATIONS: OnceLock<Arc<TextOperations>> = OnceLock::new();

pub struct TextOperations {
    noop_id: String,
    operations: HashMap<String, Box<dyn TextOperation + Sync + Send>>,
    all: Vec<String>,
}

impl TextOperations {
    fn new() -> Self {
        let none = OperationNone {};
        let noop_id = String::from(none.get_id());

        let mut operations: HashMap<String, Box<dyn TextOperation + Sync + Send>> = HashMap::new();

        add_operation(Box::new(none), &mut operations);
        add_operation(Box::new(OperationEncodeBase64 {}), &mut operations);
        add_operation(Box::new(OperationEncodeBase64UrlSafe {}), &mut operations);
        add_operation(Box::new(OperationDecodeBase64 {}), &mut operations);
        add_operation(Box::new(OperationEncodeURL {}), &mut operations);
        add_operation(Box::new(OperationDecodeURL {}), &mut operations);
        add_operation(Box::new(OperationUpperCase {}), &mut operations);
        add_operation(Box::new(OperationLowerCase {}), &mut operations);
        add_operation(Box::new(OperationFromHex {}), &mut operations);
        add_operation(Box::new(OperationToHex {}), &mut operations);
        add_operation(Box::new(OperationToLongHex {}), &mut operations);

        let all: Vec<String> = operations
            .iter()
            .map(|operation| operation.0.clone())
            .collect();

        TextOperations {
            noop_id,
            operations,
            all,
        }
    }

    pub fn get_instance() -> &'static TextOperations {
        let text_operations = TEXT_OPERATIONS.get_or_init(|| Arc::new(TextOperations::new()));
        text_operations
    }

    pub fn get_operation(&self, id: &str) -> Option<&Box<dyn TextOperation + Send + Sync>> {
        self.operations.get(id)
    }

    pub fn get_operation_or_noop(&self, name: &str) -> &Box<dyn TextOperation + Send + Sync> {
        self.operations.get(name).unwrap_or_else(|| self.get_noop())
    }

    pub fn get_noop(&self) -> &Box<dyn TextOperation + Send + Sync> {
        self.get_operation(self.noop_id.as_str()).unwrap()
    }

    pub fn get_operations(&self) -> Vec<&str> {
        self.all.iter().map(|value| value.as_str()).collect()
    }
}

fn add_operation(
    operation: Box<dyn TextOperation + Sync + Send>,
    set: &mut HashMap<String, Box<dyn TextOperation + Sync + Send>>,
) {
    set.insert(String::from(operation.get_id()), operation);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_returns_registered_oprations() {
        let text_operations = TextOperations::get_instance();
        let operations = text_operations.get_operations();

        assert_eq!(operations.len(), 10);
    }

    #[test]
    fn it_finds_requested_operation() {
        let text_operations = TextOperations::get_instance();
        let operations = text_operations.get_operations();
        let operation = text_operations.get_operation(operations[0]);
        assert_eq!(operation.is_some(), true);
    }

    #[test]
    fn it_returns_none_for_unknown_operation() {
        let text_operations = TextOperations::get_instance();
        let operation = text_operations.get_operation("some unknown operation type");
        assert_eq!(operation.is_some(), false);
    }

    #[test]
    fn it_provides_an_operation_with_no_transformation() {
        let text_operations = TextOperations::get_instance();
        let operation = text_operations.get_noop();
        let result = operation.convert("abc");
        assert_eq!(result.unwrap().text_value.unwrap(), "abc");
    }
}

use thiserror::Error;

#[derive(Debug, Clone, Error, Eq, PartialEq)]
#[error("field `{field_name}` is missing")]
pub struct MissingFieldError {
    field_name: &'static str,
}

impl MissingFieldError {
    pub fn new(field_name: &'static str) -> MissingFieldError {
        MissingFieldError { field_name }
    }
}

#[cfg(test)]
mod tests {
    use crate::aws::errors::MissingFieldError;

    #[test]
    fn test_message() {
        assert_eq!(
            "field `field-name` is missing".to_string(),
            format!("{}", MissingFieldError::new("field-name"))
        );
    }
}

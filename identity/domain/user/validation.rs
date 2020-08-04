use uuid::Uuid;

use common::result::Result;

pub type ValidationCode = String;

#[derive(Debug, Clone)]
pub struct Validation {
    code: ValidationCode,
    validated: bool,
}

impl Validation {
    pub fn new() -> Result<Validation> {
        let code = Uuid::new_v4();
        Ok(Validation {
            code: code.to_string(),
            validated: false,
        })
    }

    pub fn code(&self) -> &ValidationCode {
        &self.code
    }

    pub fn validated(&self) -> bool {
        self.validated
    }

    pub fn validate(&self, code: &ValidationCode) -> bool {
        &self.code == code
    }
}

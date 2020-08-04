use std::cmp;
use std::collections::HashMap;
use std::error;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    Internal,
    Application,
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
    code: Option<String>,
    path: Option<String>,
    status: Option<i32>,
    message: Option<String>,
    context: HashMap<String, String>,
    cause: Option<Box<Error>>,
}

impl Error {
    fn new(kind: ErrorKind) -> Error {
        Error {
            kind,
            code: None,
            path: None,
            status: None,
            message: None,
            context: HashMap::new(),
            cause: None,
        }
    }

    // TODO: create app errors with code and message
    // pub fn new(k: &str, v: &str) -> Error {
    //     Error::pair(k, v)
    // }

    pub fn pair(k: &str, v: &str) -> Error {
        Error::application().add_context(k, v).build()
    }

    pub fn internal() -> Error {
        Error::new(ErrorKind::Internal)
    }

    pub fn application() -> Error {
        Error::new(ErrorKind::Application)
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn code(&self) -> Option<&String> {
        self.code.as_ref()
    }

    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    pub fn status(&self) -> Option<&i32> {
        self.status.as_ref()
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    pub fn context(&self) -> &HashMap<String, String> {
        &self.context
    }

    pub fn has_context(&self) -> bool {
        !self.context.is_empty()
    }

    pub fn cause(&self) -> Option<&Error> {
        match &self.cause {
            Some(boxed_err) => Some(boxed_err.as_ref()),
            None => None,
        }
    }

    pub fn set_code(&mut self, code: &str) -> &mut Error {
        self.code = Some(String::from(code));
        self
    }

    pub fn set_path(&mut self, path: &str) -> &mut Error {
        self.path = Some(String::from(path));
        self
    }

    pub fn set_status(&mut self, status: i32) -> &mut Error {
        self.status = Some(status);
        self
    }

    pub fn set_message(&mut self, message: &str) -> &mut Error {
        self.message = Some(String::from(message));
        self
    }

    pub fn add_context(&mut self, k: &str, v: &str) -> &mut Error {
        self.context.insert(String::from(k), String::from(v));
        self
    }

    pub fn wrap(&mut self, err: Error) -> &mut Error {
        self.cause = Some(Box::new(err));
        self
    }

    pub fn wrap_raw<E: error::Error>(&mut self, err: E) -> &mut Error {
        let err = Error::internal().set_message(&err.to_string()).build();
        self.cause = Some(Box::new(err));
        self
    }

    pub fn merge(&mut self, err: Error) -> &mut Error {
        self.context.extend(err.context);
        self
    }

    pub fn build(&self) -> Error {
        self.clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for Error {}

impl cmp::PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.code == other.code && self.path == other.path && self.status == other.status
    }
}

#[cfg(test)]
mod tests {
    use std::error;
    use std::fmt;

    use super::*;

    #[test]
    fn basic() {
        let err = Error::internal()
            .set_code("code")
            .set_message("message")
            .set_path("my.path")
            .set_status(404)
            .add_context("k1", "v1")
            .add_context("k2", "v2")
            .add_context("k2", "v3")
            .build();
        assert_eq!(err.code().unwrap(), "code");
        assert_eq!(err.message().unwrap(), "message");
        assert_eq!(err.path().unwrap(), "my.path");
        assert_eq!(err.status().unwrap(), &404);
        assert_eq!(err.context().len(), 2);
        assert_eq!(err.context().get("k1").unwrap(), "v1");
        assert_eq!(err.context().get("k2").unwrap(), "v3");
    }

    #[test]
    fn pair() {
        let err = Error::pair("property", "error_code");
        assert_eq!(err.context().len(), 1);
        assert_eq!(err.context().get("property").unwrap(), "error_code");
    }

    #[derive(Debug, Clone)]
    struct StringError {
        error: String,
    }

    impl fmt::Display for StringError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.error)
        }
    }

    impl error::Error for StringError {}

    #[test]
    fn wrap() {
        let raw_err = StringError {
            error: "raw_err".to_owned(),
        };
        let inner_err = Error::internal()
            .set_code("inner")
            .wrap_raw(raw_err.clone())
            .build();
        let outer_err = Error::application()
            .set_code("outer")
            .wrap(inner_err.build())
            .build();

        assert_eq!(
            inner_err.cause().unwrap().message().unwrap(),
            &raw_err.error
        );
        assert_eq!(outer_err.cause().unwrap(), &inner_err);
    }

    #[test]
    fn merge() {
        let mut err1 = Error::internal().set_code("err1").build();
        err1.add_context("e1-key1", "value1");
        err1.add_context("e1-key2", "value2");

        let mut err2 = Error::internal().set_code("err2").build();
        err2.add_context("e2-key", "value");
        err2.merge(err1);

        let mut err3 = Error::application().set_code("err3").build();
        err3.add_context("e1-key1", "value");
        err3.add_context("e3-key", "value");
        err3.merge(err2);

        assert_eq!(err3.context().len(), 4);
        assert_eq!(err3.context().get("e1-key1"), Some(&"value1".to_owned()));
        assert_eq!(err3.context().get("e1-key2"), Some(&"value2".to_owned()));
        assert_eq!(err3.context().get("e2-key"), Some(&"value".to_owned()));
        assert_eq!(err3.context().get("e3-key"), Some(&"value".to_owned()));
    }
}

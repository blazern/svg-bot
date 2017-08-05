use std;

#[derive(Debug)]
pub struct MyError {
    message: String,
}

impl MyError {
    pub fn new(message: String) -> MyError {
        MyError{ message: message }
    }
}

impl From<std::io::Error> for MyError {
    fn from(error: std::io::Error) -> Self {
        MyError::new(format!("{:?}", error))
    }
}

impl From<std::str::Utf8Error> for MyError {
    fn from(error: std::str::Utf8Error) -> Self {
        MyError::new(format!("{:?}", error))
    }
}

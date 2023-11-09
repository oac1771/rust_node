
#[derive(Debug)]
pub struct Error {
    pub body: String,
}

impl Error {
    pub fn new(err: String) -> Self {
        let response = Error { body: err };

        return response;
    }
}


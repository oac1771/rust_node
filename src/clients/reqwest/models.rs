use rocket::serde::Serialize;

#[derive(Debug)]
#[derive(Serialize)]
pub struct Error {
    pub body: String
}

impl Error {
    pub fn new(err: reqwest::Error) -> Error {

        let response = Error {
            body: err.to_string()
        };

        return response
    }
}
use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub status_code: String,
    pub body: String,
}

impl Response {
    pub async fn new(r: reqwest::Response) -> Response {

        let response = Response {
            status_code: r.status().to_string(),
            body: r.text().await.unwrap(),
        };

        return response
    }
}

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
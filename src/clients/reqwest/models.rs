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
// use std::collections::HashMap;
// use futures::{future::BoxFuture, FutureExt};

use reqwest;
use rocket::serde::Serialize;

// look into having http methods in client take a struct as deserializer?
// 

// pub struct RequestClient {
//     client: reqwest::Client

// }

// impl RequestClient {
//     pub async fn post(&self, data: HashMap<&str, &str>, url: &str) -> Response{
//         let request =  || async move {self.client.post(url).json(&data).send().await}.boxed();
//         let response = self.call(request).await;

//         return response

//     }

//     // see if there is a way to make this impl thing more compact, maybe using where?
//     async fn call<'a>(&self, request: impl FnOnce() -> BoxFuture<'a, Result<reqwest::Response, reqwest::Error>>) -> Response {
//         let r = request().await.unwrap();

//         let response = Response::new(r);

//         return response

//             // todo: verify that response from /receive endpoint is being matched to match thing inside this
//         // you have to wrap response from reqwest in custom struct/enum then create service to be able
//         // to process. In forward something that handles and sends reqwuest post; in receive something that handles
//         // and reqwuest and returns correct response

//         // match response {
//         //     Ok(response) => {
//         //         match response.status() {
//         //             reqwest::StatusCode::OK => {
//         //                 println!("status code: {}", response.status());
//         //             },
//         //             _ => {
//         //                 println!("foo")
//         //             }
//         //         }

//         //     },
//         //     Err(_error) => {
//         //         println!("Uh oh! Something unexpected happened.");
//         //     },
//         // }
//     }


//     pub fn new() -> RequestClient {
//         let client = reqwest::Client::new();
//         let request_client = RequestClient {
//             client
//         };

//         return request_client
//     }
// }

#[derive(Serialize)]
pub struct Response {
    pub status_code: String,
}

impl Response {
    fn new(r: reqwest::Response) -> Response {
        let response = Response {
            status_code: r.status().to_string()
        };

        return response
    }
}
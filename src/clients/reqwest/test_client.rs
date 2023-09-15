#[cfg(test)]
mod tests {

    use http;
    use futures::FutureExt;
    use std::any::type_name;
    use serde::{Deserialize, Serialize};

    use crate::clients::reqwest::client::ReqwestClient;

    #[derive(Deserialize, Serialize)]
    pub struct TestResponse {
        pub body: String,
    }

    async fn build_response(body: String, status_code: reqwest::StatusCode) -> Result<reqwest::Response, reqwest::Error> {
        let http_response = http::response::Response::builder()
                    .status(status_code)
                    .body(body)
                    .unwrap();
        return Ok(reqwest::Response::from(http_response));
    }

    async fn build_error(body: String, status_code: reqwest::StatusCode) -> Result<reqwest::Response, reqwest::Error> {
        let http_response = http::response::Response::builder()
                    .status(status_code)
                    .body(body)
                    .unwrap();
        let reqwest_response = reqwest::Response::from(http_response);
        let error_response = reqwest_response.error_for_status().err().unwrap();

        return Err(error_response);
    }
    fn type_of<T>(_: T) -> &'static str {
        type_name::<T>()
    }

    #[tokio::test]
    async fn should_return_body_status_code() {

        let client = ReqwestClient::new();
        let body = "{\"body\": \"success!\"}";
        let status_code = reqwest::StatusCode::OK;

        let request = || async move {build_response(body.to_string(), status_code).await}.boxed();
        let response = client.call(request).await.unwrap();

        assert_eq!(response, "success!");
    }

    #[tokio::test]
    async fn should_return_error_struct_when_request_returns_non_200() {

        let client = ReqwestClient::new();
        let body = "404 not found";
        let status_code = reqwest::StatusCode::NOT_FOUND;

        let request = || async move {build_response(body.to_string(), status_code).await}.boxed();
        let error = client.call(request).await.err().unwrap();

        assert_eq!(type_of(error), "rust_node::clients::reqwest::models::Error");
    }

    #[tokio::test]
    async fn should_return_error_struct_when_request_cant_connect() {

        let client = ReqwestClient::new();
        let body = "failure :(";
        let status_code = reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED;

        let request = || async move {build_error(body.to_string(), status_code).await}.boxed();
        let error = client.call(request).await.err().unwrap();

        assert_eq!(type_of(error), "rust_node::clients::reqwest::models::Error");
    }
    
}
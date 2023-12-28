#[cfg(test)]
mod tests {

    use futures::FutureExt;
    use http;
    use serde::Deserialize;

    use crate::clients::reqwest::client::ReqwestClient;

    #[derive(Deserialize, Debug)]
    pub struct TestResponse {
        pub body: String,
    }

    #[derive(Debug)]

    pub struct TestError {
        pub err: String
    }
    
    impl From<reqwest::Error> for TestError {
        fn from(error: reqwest::Error) -> Self {
            Self {
                err: error.to_string(),
            }
        }
    }

    impl From<serde_json::Error> for TestError {
        fn from(error: serde_json::Error) -> Self {
            Self {
                err: error.to_string(),
            }
        }
    }

    async fn build_response(
        body: String,
        status_code: reqwest::StatusCode,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let http_response = http::response::Response::builder()
            .status(status_code)
            .body(body)
            .unwrap();
        return Ok(reqwest::Response::from(http_response));
    }

    async fn build_error(
        body: String,
        status_code: reqwest::StatusCode,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let http_response = http::response::Response::builder()
            .status(status_code)
            .body(body)
            .unwrap();
        let reqwest_response = reqwest::Response::from(http_response);
        let error_response = reqwest_response.error_for_status().err().unwrap();

        return Err(error_response);
    }

    #[tokio::test]
    async fn should_return_body_status_code() {
        let client = ReqwestClient::new();
        let body = "{\"body\": \"success!\"}";
        let status_code = reqwest::StatusCode::OK;

        let request = || async move { build_response(body.to_string(), status_code).await }.boxed();
        let response = client.call::<TestResponse, TestError>(request).await.unwrap();

        assert_eq!(response.body, "success!");
    }

    #[tokio::test]
    async fn should_return_error_struct_when_request_returns_non_200() {
        let client = ReqwestClient::new();
        let body = "error";
        let status_code = reqwest::StatusCode::NOT_FOUND;

        let request = || async move { build_response(body.to_string(), status_code).await }.boxed();
        let error = client.call::<TestResponse, TestError>(request).await.err().unwrap();

        assert_eq!(
            error.err,
            "HTTP status client error (404 Not Found) for url (http://no.url.provided.local/)"
        );
    }

    #[tokio::test]
    async fn should_return_error_struct_when_request_cant_connect() {
        let client = ReqwestClient::new();
        let body = "failure :(";
        let status_code = reqwest::StatusCode::GATEWAY_TIMEOUT;

        let request = || async move { build_error(body.to_string(), status_code).await }.boxed();
        let error = client.call::<TestResponse, TestError>(request).await.err().unwrap();

        assert_eq!(error.err, "HTTP status server error (504 Gateway Timeout) for url (http://no.url.provided.local/)");
    }
}

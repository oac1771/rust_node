#[cfg(test)]
mod tests {

    use http;
    use futures::FutureExt;

    use crate::clients::reqwest::client::ReqwestClient;

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

    #[tokio::test]
    async fn should_return_body_and_200_status_code() {

        let client = ReqwestClient::new();
        let body = "success!";
        let status_code = reqwest::StatusCode::OK;

        let request = || async move {build_response(body.to_string(), status_code).await}.boxed();
        let response = client.call(request).await.unwrap();

        assert_eq!(response.body, body);
        assert_eq!(response.status_code, status_code.to_string());
    }

    #[tokio::test]
    async fn should_return_error_struct() {

        let client = ReqwestClient::new();
        let body = "failure :(";
        let status_code = reqwest::StatusCode::NETWORK_AUTHENTICATION_REQUIRED;

        let request = || async move {build_error(body.to_string(), status_code).await}.boxed();
        let error = client.call(request).await.err().unwrap();

        assert_eq!(error.status, status_code.to_string());
    }
    
}
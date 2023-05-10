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

    #[tokio::test]
    async fn foo() {

        let client = ReqwestClient::new();
        let body = "hi".to_string();
        let status_code = reqwest::StatusCode::OK;

        let request = || async move {build_response(body, status_code).await}.boxed();
        let foo = client.call(request).await;
        assert_eq!(foo.unwrap().body, "hi")
    }
    
}
#[cfg(test)]
mod tests {

    use serde_json::json;

    use super::super::hash::HashService;
    use super::super::super::controllers::models::Data;

    #[test]
    fn test_hash_should_return_hash_of_data() {

        let hash_service = HashService::new();
        let data = Data {
            meta_data: "foo".to_string(),
            data: json!({
                "bar": "hi"
            })
        };

        let file_content = hash_service.hash(data);

        assert_eq!(file_content.content, "{\"meta_data\":\"foo\",\"data\":{\"bar\":\"hi\"}}");
        assert_eq!(file_content.hash, "42d17f6ffe28ab0a8303e9aff0f016b0da9e45866717099d0f4d5b7505e28e2a");

    }
    
}
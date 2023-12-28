#[cfg(test)]
mod tests {

    use super::super::identity::{IdentityService, IdService};
    use super::super::super::clients::ipfs::models::IpfsGetResponse;
    use std::path::Path;

    #[test]
    fn create_identity_should_return_temp_file_and_identity() {
        let identity_service = IdentityService::new();
        let data = "test data";

        let (file, identity) = identity_service.create_identity(data).unwrap();

        assert_ne!(identity.encryption_key, "");
        assert_ne!(identity.data, "");
        assert_eq!(identity.hash, identity_service.hash_service.hash(data));
        assert_eq!(Path::new(file.path()).exists(), true);
    }

    #[test]
    fn regenerate_identity_should_return_identity() {
        let identity_service = IdentityService::new();
        let data = "more test data";

        let (_, identity_1) = identity_service.create_identity(data).unwrap();
        let transformed_data = serde_json::from_str::<IpfsGetResponse>(&identity_1.data).unwrap();
        let identity_2 = identity_service
            .regenerate_identity(&identity_1.encryption_key, transformed_data.data)
            .unwrap();

        assert_eq!(identity_1.encryption_key, identity_2.encryption_key);
        assert_eq!(identity_1.hash, identity_2.hash);
        assert_eq!(data, identity_2.data);
    }
}

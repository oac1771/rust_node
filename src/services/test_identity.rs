#[cfg(test)]
mod tests {

    use super::super::identity::IdentityService;
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
        let identity_2 = identity_service
            .regenerate_identity(&identity_1.encryption_key, &identity_1.data)
            .unwrap();

        assert_eq!(identity_1.encryption_key, identity_2.encryption_key);
        assert_eq!(identity_1.hash, identity_2.hash);
        assert_eq!(data, identity_2.data);
    }

    #[test]
    fn regenerate_identity_should_return_error_if_cannot_serialize_into_bytes() {
        let identity_service = IdentityService::new();

        let err = identity_service
            .regenerate_identity("key that does not matter", "string not in correct form")
            .err()
            .unwrap();

        assert_eq!(err.err, "Unable to transform string literal to bytes");
    }

    #[test]
    fn regenerate_identity_should_return_error_private_key_and_data_do_not_match() {
        let identity_service = IdentityService::new();

        let err = identity_service
            .regenerate_identity(
                "-----BEGIN RSA PRIVATE KEY-----\nMIIEpq9g==\n-----END RSA PRIVATE KEY-----\n",
                "[1,2,3,4,5]",
            )
            .err()
            .unwrap();

        assert_eq!(err.err, "error:09091064:PEM routines:PEM_read_bio_ex:bad base64 decode:crypto/pem/pem_lib.c:949:");
    }
}

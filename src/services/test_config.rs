#[cfg(test)]
mod tests {

    use std::{
        any::Any,
        marker::Send,
        path::Path,
        env,
        fs
    };
    use futures::FutureExt;

    use super::super::config::*;

    fn run_test(result: Result<(), Box<dyn Any + Send>>, teardown: impl FnOnce() -> ()) {
        teardown();
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn should_create_config_file() {
        let test = async {

            env::set_var("IPFS_BASE_URL", "foo");
            env::set_var("PRIVATE_KEY", "foo");
            env::set_var("ZKSYNC_API_URL", "foo");
            env::set_var("ZKSYNC_WS_URL", "foo");

            create_config().await;

            let path = Path::new(CONFIG_PATH);
            assert_eq!(path.exists(), true);
        };
        let teardown = || {
            fs::remove_file(CONFIG_PATH);
        };

        let result = test.catch_unwind().await;

        run_test(result, teardown);
    }
}
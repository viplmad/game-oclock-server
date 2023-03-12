use futures::future::BoxFuture;

use crate::errors::ImageClientError;

#[async_trait::async_trait]
pub trait ImageClient: 'static + Send + Sync {
    /// Checks if a connection is still valid.
    fn ping(&self) -> BoxFuture<'_, Result<(), ImageClientError>>;

    async fn upload_image(
        &self,
        file_path: &str,
        folder: &str,
        filename: &str,
    ) -> Result<String, ImageClientError>;

    async fn rename_image(
        &self,
        folder: &str,
        old_filename: &str,
        new_filename: &str,
    ) -> Result<String, ImageClientError>;

    async fn delete_image(&self, folder: &str, filename: &str) -> Result<(), ImageClientError>;

    fn get_image_uri(&self, folder: &str, filename: &str) -> String;
}

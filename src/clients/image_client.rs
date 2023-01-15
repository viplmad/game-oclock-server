use std::fs::File;

use futures::future::BoxFuture;

use crate::errors::ImageClientError;

pub trait ImageClient: 'static + Send + Sync {
    /// Checks if a connection is still valid.
    fn ping(&self) -> BoxFuture<'_, Result<(), ImageClientError>>;

    fn upload_image(
        &self,
        file: File,
        folder: &str,
        filename: &str,
    ) -> Result<String, ImageClientError>;

    fn rename_image(
        &self,
        folder: &str,
        old_filename: &str,
        new_filename: &str,
    ) -> Result<String, ImageClientError>;

    fn delete_image(&self, folder: &str, filename: &str) -> Result<String, ImageClientError>;

    fn get_image_uri(&self, folder: &str, filename: &str) -> String;
}

use std::fs::File;

use chrono::Utc;
use cloudinary::result::CloudinaryResult;
use cloudinary::upload::UploadOptions;
use cloudinary::Cloudinary;
use futures::future::BoxFuture;

use crate::clients::image_client::ImageClient;
use crate::errors::ImageClientError;

const ASSET_URL: &str = "https://res.cloudinary.com";

/// A Cloudinary client.
#[derive(Clone, Default)]
pub struct CloudinaryClient {
    client: Cloudinary,
}

impl CloudinaryClient {
    pub fn connect_with(mut self, client: Cloudinary) -> Self {
        self.client = client;
        self
    }
}

#[async_trait::async_trait]
impl ImageClient for CloudinaryClient {
    fn ping(&self) -> BoxFuture<'_, Result<(), ImageClientError>> {
        todo!(); // TODO access sample
    }

    async fn upload_image(
        &self,
        file: File,
        folder: &str,
        filename: &str,
    ) -> Result<String, ImageClientError> {
        let timestamp = Utc::now().timestamp_millis().to_string();
        let public_id = format!("{filename}_{timestamp}");

        let options = UploadOptions::new()
            .set_public_id(public_id)
            .set_folder(String::from(folder));

        let result = self
            .client
            .upload_image(file, filename, &options)
            .await
            .map_err(|err| {
                log::error!("{}", err.0);
                ImageClientError()
            })?;

        match result {
            CloudinaryResult::Succes(res) => get_filename(res),
            CloudinaryResult::Error(err) => {
                log::info!("{}", err.error.message);
                Err(ImageClientError())
            }
        }
    }

    async fn rename_image(
        &self,
        folder: &str,
        old_filename: &str,
        new_filename: &str,
    ) -> Result<String, ImageClientError> {
        let public_id = format!("{folder}/{old_filename}");

        let timestamp = Utc::now().timestamp_millis().to_string();
        let new_public_id = format!("{folder}/{new_filename}_{timestamp}");

        let result = self
            .client
            .rename_image(&public_id, &new_public_id)
            .await
            .map_err(|err| {
                log::error!("{}", err.0);
                ImageClientError()
            })?;

        match result {
            CloudinaryResult::Succes(res) => get_filename(res),
            CloudinaryResult::Error(err) => {
                log::info!("{}", err.error.message);
                Err(ImageClientError())
            }
        }
    }

    async fn delete_image(&self, folder: &str, filename: &str) -> Result<String, ImageClientError> {
        let public_id = format!("{folder}/{filename}");

        let result = self.client.delete_image(&public_id).await.map_err(|err| {
            log::error!("{}", err.0);
            ImageClientError()
        })?;

        match result {
            CloudinaryResult::Succes(res) => get_filename(res),
            CloudinaryResult::Error(err) => {
                log::info!("{}", err.error.message);
                Err(ImageClientError())
            }
        }
    }

    fn get_image_uri(&self, folder: &str, filename: &str) -> String {
        let cloud_name = &self.client.cloud_name;
        format!("{ASSET_URL}/{cloud_name}/image/upload/{folder}/{filename}")
    }
}

/// Connection options to Cloudinary.
#[derive(Debug)]
pub struct CloudinaryClientBuilder;

impl CloudinaryClientBuilder {
    pub fn try_from_env() -> Option<Cloudinary> {
        let cloud_name = match std::env::var("CLOUDINARY_CLOUD_NAME") {
            Ok(val) => Some(val),
            Err(_) => {
                log::info!("Cloudinary cloud name not set. -> Image disabled");
                None
            }
        }?;
        let api_key = match std::env::var("CLOUDINARY_API_KEY") {
            Ok(val) => match val.parse::<i64>() {
                Ok(int_val) => Some(int_val),
                Err(_) => {
                    log::info!("Cloudinary api key is not a number. -> Image disabled");
                    None
                }
            },
            Err(_) => {
                log::info!("Cloudinary api key not set. -> Image disabled");
                None
            }
        }?;
        let api_secret = match std::env::var("CLOUDINARY_API_SECRET") {
            Ok(val) => Some(val),
            Err(_) => {
                log::info!("Cloudinary api key not set. -> Image disabled");
                None
            }
        }?;

        log::info!(
            "Cloudinary connected to <redacted>:<redacted>@{}",
            // Hide api key and secret from info log
            cloud_name
        );

        Some(Cloudinary::new(&cloud_name, api_key, &api_secret))
    }
}

fn get_filename(result: Box<cloudinary::result::Response>) -> Result<String, ImageClientError> {
    if let Some(value) = result.public_id.split('/').last() {
        let format = result.format;
        return Ok(format!("{value}{format}"));
    }

    Err(ImageClientError())
}

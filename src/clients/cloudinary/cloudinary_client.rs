use std::str::FromStr;

use futures::future::BoxFuture;

use crate::clients::image_client::ImageClient;
use crate::errors::ImageClientError;

const DEFAULT_API_URL: &str = "https://api.cloudinary.com/v1_1";
const DEFAULT_ASSET_URL: &str = "https://res.cloudinary.com";

/// A Cloudinary client.
#[derive(Clone, Default)]
pub struct CloudinaryClient {
    options: CloudinaryClientOptions,
}

impl CloudinaryClient {
    pub fn connect_with(mut self, options: CloudinaryClientOptions) -> Self {
        self.options = options;
        self
    }
}

impl ImageClient for CloudinaryClient {
    fn ping(&self) -> BoxFuture<'_, Result<(), ImageClientError>> {
        todo!(); // TODO access sample
    }

    fn upload_image(
        &self,
        file: std::fs::File,
        folder: &str,
        filename: &str,
    ) -> Result<String, ImageClientError> {
        todo!()
    }

    fn rename_image(
        &self,
        folder: &str,
        old_filename: &str,
        new_filename: &str,
    ) -> Result<String, ImageClientError> {
        todo!()
    }

    fn delete_image(&self, folder: &str, filename: &str) -> Result<String, ImageClientError> {
        todo!()
    }

    fn get_image_uri(&self, folder: &str, filename: &str) -> String {
        let options = &self.options;
        let base_asset_url = &options.asset_url;
        let cloud_name = &options.cloud_name;
        format!("{base_asset_url}/{cloud_name}/image/upload/{folder}/{filename}")
    }
}

/// Connection options to Cloudinary.
#[derive(Debug, Clone)]
pub struct CloudinaryClientOptions {
    pub(crate) api_url: String,
    pub(crate) asset_url: String,
    pub(crate) port: u16,
    pub(crate) cloud_name: String,
    pub(crate) api_key: i32,
    pub(crate) api_secret: String,
}

impl CloudinaryClientOptions {
    pub fn try_from_env() -> Option<Self> {
        let cloud_name = match std::env::var("CLOUDINARY_CLOUD_NAME") {
            Ok(val) => Some(val),
            Err(_) => {
                log::info!("Cloudinary cloud name not set. -> Image disabled");
                None
            }
        }?;
        let api_key = match std::env::var("CLOUDINARY_API_KEY") {
            Ok(val) => match val.parse() {
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

        Some(
            Self::default()
                .cloud_name(&cloud_name)
                .api_key(api_key)
                .api_secret(&api_secret),
        )
    }

    pub fn api_url(mut self, api_url: &str) -> Self {
        self.api_url = String::from(api_url);
        self
    }

    pub fn asset_url(mut self, asset_url: &str) -> Self {
        self.asset_url = String::from(asset_url);
        self
    }

    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn cloud_name(mut self, cloud_name: &str) -> Self {
        self.cloud_name = String::from(cloud_name);
        self
    }

    pub fn api_key(mut self, api_key: i32) -> Self {
        self.api_key = api_key;
        self
    }

    pub fn api_secret(mut self, api_secret: &str) -> Self {
        self.api_secret = String::from(api_secret);
        self
    }
}

impl Default for CloudinaryClientOptions {
    fn default() -> Self {
        Self {
            api_url: String::from(DEFAULT_API_URL),
            asset_url: String::from(DEFAULT_ASSET_URL),
            port: 80,
            cloud_name: String::default(),
            api_key: i32::default(),
            api_secret: String::default(),
        }
    }
}

/// Create connection options from URI cloudinary://<apiKey>:<apiSecret>@<cloudName>
impl FromStr for CloudinaryClientOptions {
    type Err = ImageClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url: url::Url = s.parse().map_err(|_| ImageClientError())?;

        let mut options = Self::default();

        if let Some(cloud_name) = url.host_str() {
            options = options.cloud_name(cloud_name);
        } else {
            return Err(ImageClientError());
        }

        let api_key_string = url.username();
        if !api_key_string.is_empty() {
            let api_key = api_key_string.parse().map_err(|_| ImageClientError())?;
            options = options.api_key(api_key);
        } else {
            return Err(ImageClientError());
        }

        if let Some(api_secret) = url.password() {
            options = options.api_secret(api_secret);
        } else {
            return Err(ImageClientError());
        }

        Ok(options)
    }
}

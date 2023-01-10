use std::str::FromStr;

use futures::future::BoxFuture;

use crate::errors::ImageClientError;

use super::image_client::{ImageConnectOptions, ImageConnection};

const DEFAULT_API_URL: &str = "https://api.cloudinary.com/v1_1";
const DEFAULT_ASSET_URL: &str = "https://res.cloudinary.com";

/// A connection to Cloudinary.
#[derive(Clone, Default)]
pub struct CloudinaryConnection {
    pub(super) connect_options: <Self as ImageConnection>::Options,
}

impl ImageConnection for CloudinaryConnection {
    type Options = CloudinaryConnectOptions;

    fn connect_with(mut self, options: Self::Options) -> Self {
        self.connect_options = options;
        self
    }

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
        let options = &self.connect_options;
        let base_asset_url = &options.asset_url;
        let cloud_name = &options.cloud_name;
        format!("{base_asset_url}/{cloud_name}/image/upload/{folder}/{filename}")
    }
}

/// Connection options to Cloudinary.
#[derive(Debug, Clone)]
pub struct CloudinaryConnectOptions {
    pub(crate) api_url: String,
    pub(crate) asset_url: String,
    pub(crate) port: u16,
    pub(crate) cloud_name: String,
    pub(crate) api_key: i32,
    pub(crate) api_secret: String,
}

impl CloudinaryConnectOptions {
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

impl Default for CloudinaryConnectOptions {
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

impl ImageConnectOptions for CloudinaryConnectOptions {
    type Connection = CloudinaryConnection;

    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, ImageClientError>>
    where
        Self::Connection: Sized,
    {
        todo!()
    }
}

/// Create connection options from URI cloudinary://<apiKey>:<apiSecret>@<cloudName>
impl FromStr for CloudinaryConnectOptions {
    type Err = ImageClientError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let url: url::Url = s.parse().map_err(|_| ImageClientError())?; //.map_err(Error::config)?;

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
            //.map_err(Error::config)?;
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

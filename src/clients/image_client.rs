use std::fmt::Debug;
use std::fs::File;
use std::str::FromStr;

use futures::future::BoxFuture;

use crate::errors::ImageClientError;

pub trait ImageConnection: Send {
    type Options: ImageConnectOptions<Connection = Self>;

    fn connect_with(self, options: Self::Options) -> Self;

    /// Checks if a connection to the database is still valid.
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

pub trait ImageConnectOptions:
    'static + Send + Sync + FromStr<Err = ImageClientError> + Debug + Clone
{
    type Connection: ImageConnection + ?Sized;

    /// Establish a new client connection with the options specified by `self`.
    fn connect(&self) -> BoxFuture<'_, Result<Self::Connection, ImageClientError>>
    where
        Self::Connection: Sized;

    /*/// Log executed statements with the specified `level`
    fn log_statements(&mut self, level: LevelFilter) -> &mut Self;

    /// Log executed statements with a duration above the specified `duration`
    /// at the specified `level`.
    fn log_slow_statements(&mut self, level: LevelFilter, duration: Duration) -> &mut Self;

    /// Entirely disables statement logging (both slow and regular).
    fn disable_statement_logging(&mut self) -> &mut Self {
        self.log_statements(LevelFilter::Off)
            .log_slow_statements(LevelFilter::Off, Duration::default())
    }*/
}

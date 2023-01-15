use std::sync::Arc;

use crate::clients::image_client::ImageClient;

pub struct ImageClientProvider(Arc<Option<Box<dyn ImageClient>>>);

impl ImageClientProvider {
    pub fn new(client: impl ImageClient) -> Self {
        Self(Arc::new(Some(Box::new(client))))
    }

    pub fn empty() -> Self {
        Self(Arc::new(None))
    }

    pub fn get_client(&self) -> Option<&dyn ImageClient> {
        match self.0.as_ref() {
            Some(client_ref) => Some(client_ref.as_ref()),
            None => None,
        }
    }
}

impl Clone for ImageClientProvider {
    fn clone(&self) -> Self {
        let repo = self.0.clone();
        Self(repo)
    }
}

use crate::clients::image_client::ImageClient;

pub struct ImageClientProvider(Option<Box<dyn ImageClient>>);

impl ImageClientProvider {
    pub fn new(client: impl ImageClient) -> Self {
        Self(Some(Box::new(client)))
    }

    pub fn empty() -> Self {
        Self(None)
    }

    pub fn get_client(&self) -> Option<&dyn ImageClient> {
        match &self.0 {
            Some(client_ref) => Some(client_ref.as_ref()),
            None => None,
        }
    }
}

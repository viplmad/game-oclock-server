use serde::Serialize;
use utoipa::ToSchema;

use super::{DeviceDTO, ModelInfo, SessionDTO};

#[derive(Serialize, ToSchema)]
pub struct DeviceSessionDTO {
    pub device: DeviceDTO,
    pub session: SessionDTO,
}

impl ModelInfo for DeviceSessionDTO {
    const MODEL_NAME: &'static str = "Relation of Media and Device"; // TODO
    const ID_FIELDS: &'static [&'static str] = &["media id", "device id"];
    const UNIQUE_FIELDS: &'static [&'static str] = DeviceSessionDTO::ID_FIELDS;
}

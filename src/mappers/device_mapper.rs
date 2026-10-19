use uuid::Uuid;

use crate::entities::Device;
use crate::models::DeviceDTO;

impl From<Device> for DeviceDTO {
    fn from(device: Device) -> Self {
        Self {
            id: device.id,
            name: device.name,
            image_url: device.image_url,
            added_datetime: device.added_datetime,
            updated_datetime: device.updated_datetime,
        }
    }
}

impl From<DeviceDTO> for Device {
    fn from(device: DeviceDTO) -> Self {
        Self {
            id: Uuid::default(),
            user_id: Uuid::default(),
            name: device.name,
            image_url: device.image_url,
            added_datetime: device.added_datetime,
            updated_datetime: device.updated_datetime,
        }
    }
}

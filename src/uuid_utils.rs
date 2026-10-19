use uuid::{Error, Uuid};

pub fn new_model_uuid() -> Uuid {
    Uuid::now_v7()
}

pub fn new_random_uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn parse_uuid(uuid: &str) -> Result<Uuid, Error> {
    Uuid::parse_str(uuid)
}

pub fn to_string(uuid: &Uuid) -> String {
    uuid.to_string()
}

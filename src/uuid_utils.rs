use uuid::Uuid;

pub fn new_model_uuid() -> String {
    Uuid::now_v7().to_string()
}

pub fn new_random_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn parse_uuid(uuid: &str) -> Uuid {
    Uuid::parse_str(uuid).expect("Id was not valid Uuid")
}

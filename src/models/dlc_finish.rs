use super::ModelName;

pub struct DLCFinish();

impl ModelName for DLCFinish {
    const MODEL_NAME: &'static str = "DLC finish";
    const ID_FIELDS: &'static [&'static str] = &["dlc id", "date"];
    const UNIQUE_FIELDS: &'static [&'static str] = DLCFinish::ID_FIELDS;
}

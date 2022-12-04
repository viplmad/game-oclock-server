use super::ModelInfo;

pub struct DLCFinish();

impl ModelInfo for DLCFinish {
    const MODEL_NAME: &'static str = "DLC finish";
    const ID_FIELDS: &'static [&'static str] = &["dlc id", "date"];
    const UNIQUE_FIELDS: &'static [&'static str] = DLCFinish::ID_FIELDS;
}

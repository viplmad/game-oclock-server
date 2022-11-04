pub trait ModelName {
    /// Obtain name of model
    const MODEL_NAME: &'static str;

    /// Obtain the id fields
    const ID_FIELDS: &'static [&'static str];

    /// Obtain the unique fields
    const UNIQUE_FIELDS: &'static [&'static str];
}
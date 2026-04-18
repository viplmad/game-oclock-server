#[derive(Default, Clone)]
pub struct DurationDef {
    pub micros: i64,
}

impl DurationDef {
    pub fn microseconds(microseconds: i64) -> Self {
        Self {
            micros: microseconds,
        }
    }
}

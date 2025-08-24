pub struct Body(pub String);

impl Body {
    pub fn is_valid_json(&self) -> bool {
        serde_json::from_str::<serde_json::Value>(self.0.as_str()).is_ok()
    }
}

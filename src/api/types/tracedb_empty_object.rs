pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct EmptyObject(pub HashMap<String, serde_json::Value>);

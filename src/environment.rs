use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Environment {
    #[serde(rename = "default")]
    Default,
}
impl Environment {
    pub fn url(&self) -> &'static str {
        match self {
            Self::Default => "http://127.0.0.1:8090",
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Default
    }
}

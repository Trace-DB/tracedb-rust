pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum RecordPutBody {
    RecordInput(RecordInput),

    RecordPutRequest(RecordPutRequest),
}

impl RecordPutBody {
    pub fn is_record_input(&self) -> bool {
        matches!(self, Self::RecordInput(_))
    }

    pub fn is_record_put_request(&self) -> bool {
        matches!(self, Self::RecordPutRequest(_))
    }

    pub fn as_record_input(&self) -> Option<&RecordInput> {
        match self {
            Self::RecordInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_record_input(self) -> Option<RecordInput> {
        match self {
            Self::RecordInput(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_record_put_request(&self) -> Option<&RecordPutRequest> {
        match self {
            Self::RecordPutRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_record_put_request(self) -> Option<RecordPutRequest> {
        match self {
            Self::RecordPutRequest(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for RecordPutBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RecordInput(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::RecordPutRequest(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}

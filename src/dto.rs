use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeneralResponse {
    msg: String,
    code: i64,
    data: serde_json::Value,
}

impl GeneralResponse {
    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }

    pub fn code(&self) -> i64 {
        self.code
    }

    pub fn data(&self) -> &serde_json::Value {
        &self.data
    }
}

pub struct AccountInfo;
pub struct CaptchaInfo;
pub struct ErrorReport;

//! 用户相关接口

use async_trait::async_trait;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::Client;

const USER_INFO_API_URL: &str = "https://www.jfbym.com/api/YmServer/getUserInfoApi";
const ERROR_REPORT_URL: &str = "https://www.jfbym.com/api/YmServer/refundApi";

lazy_static! {
    static ref ACCOUNT_QUERIER: reqwest::Client = reqwest::Client::new();
}

#[async_trait]
pub trait Account {
    fn token(&self) -> String;
    async fn query_balance(&self) -> String;
    async fn report_error(&self, unique_code: String) -> String;
}

#[async_trait]
impl Account for Client {
    fn token(&self) -> String {
        self.token.to_owned()
    }

    async fn query_balance(&self) -> String {
        let token = self.token.as_str();
        let params = [("token", token), ("type", "score")];
        ACCOUNT_QUERIER
            .post(USER_INFO_API_URL)
            .form(&params)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }

    async fn report_error(&self, unique_code: String) -> String {
        let params = [
            ("token", self.token()),
            ("uniqueCode", unique_code.to_string()),
        ];
        ACCOUNT_QUERIER
            .post(ERROR_REPORT_URL)
            .form(&params)
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountInfoQueryResponse {
    msg: String,
    code: i64,
    data: Data,
}

impl AccountInfoQueryResponse {
    #[inline]
    pub fn msg(&self) -> &str {
        self.msg.as_ref()
    }

    #[inline]
    pub fn code(&self) -> i64 {
        self.code
    }

    #[inline]
    pub fn data(&self) -> Option<&Data> {
        if self.data.score.is_empty() {
            None
        } else {
            Some(&self.data)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    #[serde(default)]
    pub score: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorReportResponse {
    msg: String,
    code: i64,
    data: Vec<Option<serde_json::Value>>,
}

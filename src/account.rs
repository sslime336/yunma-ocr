//! 用户相关接口

use async_trait::async_trait;
use reqwest::Response;
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
    async fn query_balance_of(&self, type_id: i32) -> String;
    async fn report_error(&self, unique_code: i32) -> String;
}

#[async_trait]
impl Account for Client<'_> {
    fn token(&self) -> String {
        self.token.to_owned()
    }

    async fn query_balance_of(&self, type_id: i32) -> String {
        let params = [("token", self.token()), ("type", type_id.to_string())];
        send_params(USER_INFO_API_URL, params)
            .await
            .text()
            .await
            .unwrap()
    }

    async fn report_error(&self, unique_code: i32) -> String {
        let params = [
            ("token", self.token()),
            ("uniqueCode", unique_code.to_string()),
        ];
        send_params(ERROR_REPORT_URL, params)
            .await
            .text()
            .await
            .unwrap()
    }
}

#[inline]
async fn send_params(url: &str, params: [(&str, String); 2]) -> Response {
    ACCOUNT_QUERIER
        .post(url)
        .form(&params)
        .send()
        .await
        .unwrap()
}

#[derive(Serialize, Deserialize)]
pub struct S {
    msg: String,
    code: i64,
    data: Data,
}

#[derive(Serialize, Deserialize)]
pub struct Data {
    score: String,
}

#[cfg(test)]
mod test {
    use crate::{account::Account, Client};
    use std::env;

    #[tokio::test]
    async fn test_query_balance() {
        dotenv::dotenv().ok();
        let token = env::var("TOKEN").unwrap();
        let client = Client::init(&token);
        let response_text = client.query_balance_of(1).await;
        println!("response = {}", response_text);
    }
}

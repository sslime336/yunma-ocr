#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
pub mod account;
pub mod captchas;

use account::{Account, AccountInfoQueryResult};
use captchas::Captcha;
use serde::{de::DeserializeOwned, Serialize};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref OCR_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct Client {
    token: String,
}

impl Client {
    pub fn init(token: String) -> Self {
        Client { token }
    }

    pub async fn parse(&self, mut base64encoded_captcha: impl Captcha + Serialize) -> String {
        base64encoded_captcha.check_type_id();
        base64encoded_captcha.set_token(self.token.clone());
        let url = base64encoded_captcha.query_url();
        let resp = OCR_CLIENT
            .post(url)
            .json(&base64encoded_captcha)
            .send()
            .await
            .unwrap();
        resp.text().await.unwrap()
    }

    pub async fn parse_marshaled<T>(&self, base64encoded_captcha: impl Captcha + Serialize) -> T
    where
        T: DeserializeOwned,
    {
        let response_text = self.parse(base64encoded_captcha).await;
        serde_json::from_str(&response_text).unwrap()
    }

    pub async fn query_balance_marshaled(&self) -> AccountInfoQueryResult {
        let response_text = self.query_balance().await;
        serde_json::from_str(&response_text).unwrap()
    }
}

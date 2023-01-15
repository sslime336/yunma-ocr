#![allow(incomplete_features)]
#![feature(async_fn_in_trait)]
pub mod account;
pub mod captchas;

use captchas::Captcha;
use serde::{de::DeserializeOwned, Serialize};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref OCR_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct Client<'a> {
    token: &'a str,
}

impl<'a> Client<'a> {
    pub fn init(token: &'a str) -> Self {
        Client { token }
    }

    pub async fn parse(&self, mut base64encoded_captcha: impl Captcha + Serialize) -> String {
        base64encoded_captcha.set_token(self.token);
        let url = base64encoded_captcha.query_url();
        let resp = OCR_CLIENT
            .post(url)
            .json(&base64encoded_captcha)
            .send()
            .await
            .unwrap();
        resp.text().await.unwrap()
    }

    #[inline]
    pub async fn parse_into<T>(&self, base64encoded_captcha: impl Captcha + Serialize) -> T
    where
        T: DeserializeOwned,
    {
        let json_data = self.parse(base64encoded_captcha).await;
        serde_json::from_str::<T>(&json_data).unwrap()
    }
}

#[cfg(test)]
mod test {
    use crate::captchas::common_captcha::{CommonCaptcha, CommonResponse};

    use super::*;
    use dotenv::dotenv;
    use std::{env, str::FromStr};

    #[tokio::test]
    async fn test_ocr_common_type() {
        dotenv().ok();

        let url = reqwest::Url::from_str(&env::var("URL").unwrap()).unwrap();
        let token = env::var("TOKEN").unwrap();
        let client = Client::init(&token);

        let common_captcha = CommonCaptcha::from_url(url).await.set_type_id(10110);
        let res = client.parse(common_captcha).await;
        println!("result = {}", res);
    }

    #[tokio::test]
    async fn test_ocr_common_type_into_response_struct() {
        dotenv().ok();

        let url = reqwest::Url::from_str(&env::var("URL").unwrap()).unwrap();
        let token = env::var("TOKEN").unwrap();
        let client = Client::init(&token);

        let common_captcha = CommonCaptcha::from_url(url).await.set_type_id(10110);
        let res = client.parse_into::<CommonResponse>(common_captcha).await;
        println!("resp = {:?}", res);
        println!("captcha = {}", res.data.data);
    }
}

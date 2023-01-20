use lazy_static::lazy_static;
use serde::Serialize;

use crate::captchas::Captcha;
use crate::error::Result;

lazy_static! {
    pub(crate) static ref OCR_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct Client {
    pub(crate) token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        Client { token }
    }

    pub async fn parse(&self, mut base64encoded_captcha: impl Captcha + Serialize) {
        base64encoded_captcha.set_token(self.token.clone());
        let url = base64encoded_captcha.query_url();
        let resp = OCR_CLIENT
            .post(url)
            .json(&base64encoded_captcha)
            .send()
            .await
            .unwrap();
        resp.text().await.unwrap();
        todo!("finish me")
    }

    pub async fn query_balance(&self) -> Result<String> {
        let token = self.token.as_str();
        let params = [("token", token), ("type", "score")];
        Ok(OCR_CLIENT
            .post(USER_INFO_API_URL)
            .form(&params)
            .send()
            .await?
            .text()
            .await?)
    }

    pub async fn report_error(&self, unique_code: String) -> Result<String> {
        let params = [
            ("token", self.token.clone()),
            ("uniqueCode", unique_code.to_string()),
        ];
        Ok(OCR_CLIENT
            .post(ERROR_REPORT_URL)
            .form(&params)
            .send()
            .await?
            .text()
            .await?)
    }
}

const USER_INFO_API_URL: &str = "https://www.jfbym.com/api/YmServer/getUserInfoApi";
const ERROR_REPORT_URL: &str = "https://www.jfbym.com/api/YmServer/refundApi";

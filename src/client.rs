use lazy_static::lazy_static;
use serde::Serialize;

use crate::{
    account::{Account, AccountInfoQueryResponse, ErrorReportResponse},
    captchas::{Captcha, SimpleCaptchaResponse},
};

lazy_static! {
    static ref OCR_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct Client {
    pub(crate) token: String,
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

    pub async fn parse_marshaled(
        &self,
        base64encoded_captcha: impl Captcha + Serialize,
    ) -> SimpleCaptchaResponse {
        let response_text = self.parse(base64encoded_captcha).await;
        serde_json::from_str(&response_text).unwrap()
    }

    pub async fn query_balance_marshaled(&self) -> AccountInfoQueryResponse {
        let response_text = self.query_balance().await;
        serde_json::from_str(&response_text).unwrap()
    }

    #[inline]
    pub async fn report(&self, unique_code: String) -> String {
        self.report_error(unique_code).await
    }

    pub async fn report_marshaled(&self, unique_code: String) -> ErrorReportResponse {
        let response_text = self.report_error(unique_code).await;
        serde_json::from_str(&response_text).unwrap()
    }
}

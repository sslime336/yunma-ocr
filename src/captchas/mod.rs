#![allow(dead_code)]
use base64::Engine;
use lazy_static::lazy_static;
use reqwest::Url;
use serde::{Deserialize, Serialize};

pub mod click_captcha;
pub mod common_captcha;
pub mod google_captchas;
pub mod slide_captcha;

lazy_static! {
    static ref CAPTCHAGETTER: reqwest::Client = reqwest::Client::new();
}

const SIMPLE_CAPTCHA_QUERY_URL: &str = "https://www.jfbym.com/api/YmServer/customApi";
const GOOGLE_CAPTCHA_FUNNEL_API: &str = "https://www.jfbym.com/api/YmServer/funnelApi";
const GOOGLE_CAPTCHA_FUNNEL_API_RESULT: &str = "https://www.jfbym.com/api/YmServer/funnelApiResult";

pub trait Captcha {
    fn query_url(&self) -> String;
    fn to_json(&self) -> String;
    fn set_token(&mut self, token: String);
    fn check_type_id(&self);
}

#[inline]
async fn get_base64_image_from_url(url: Url) -> String {
    let raw_captcha = CAPTCHAGETTER
        .get(url)
        .send()
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();
    encode_captcha(raw_captcha)
}

#[inline]
fn encode_captcha(input: impl AsRef<[u8]>) -> String {
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(input)
}

/// SimpleCaptchaResponse is used to represent common typed, slide typed and
/// click typed captcha query response entities.
#[derive(Serialize, Deserialize, Debug)]
pub struct SimpleCaptchaResponse {
    msg: String,
    code: i64,
    data: Data,
}

impl SimpleCaptchaResponse {
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
        if self.data.data.is_empty() {
            None
        } else {
            Some(&self.data)
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    #[serde(default)]
    code: i64,

    #[serde(default)]
    data: String,

    #[serde(default)]
    time: f64,

    #[serde(default)]
    unique_code: String,
}

impl Data {
    #[inline]
    pub fn code(&self) -> i64 {
        self.code
    }

    #[inline]
    pub fn data(&self) -> &str {
        self.data.as_ref()
    }

    #[inline]
    pub fn time(&self) -> f64 {
        self.time
    }

    #[inline]
    pub fn unique_code(&self) -> &str {
        self.unique_code.as_ref()
    }
}

//! 通用类验证码
//!
//! | 数英汉字类型 | 类型编号 |
//! | ------------ | -------- |
//! | 通用数英1-4位 | 10110 |
//! | 通用数英5-8位 | 10111 |
//! | 通用数英9-11位 | 10112 |
//! | 通用数英12位及以上 | 10113 |
//! | 通用数英1-6位plus | 10103 |
//! | 定制-数英5位-qcs | 9001 |
//! | 定制-纯数字4位 | 193 |
//!
//! | 中文类型 | 类型编号 |
//! | ------------ | -------- |
//! | 通用中文字符1~2位 | 10114 |
//! | 通用中文字符 3~5位 | 10115 |
//! | 通用中文字符6~8位 | 10116 |
//! | 通用中文字符9位及以上 | 10117 |
//! | 定制-XX西游苦行中文字符 | 10107 |
//!
//! | 计算类型 | 类型编号 |
//! | ------------ | -------- |
//! | 通用数字计算题 | 50100 |
//! | 通用中文计算题 | 50101 |
//! | 定制-计算题 cni | 452 |
use reqwest::Url;
use serde::{Deserialize, Serialize};

use super::{get_base64_image_from_url, yunma_captcha_query_urls, Captcha};

/// 通用类型
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommonCaptcha {
    /// 需要识别图片的base64字符串
    image: String,
    /// 用户中心密钥
    token: String,
    #[serde(rename = "type")]
    type_id: i32,
}

impl CommonCaptcha {
    pub fn set_type_id(&mut self, id: i32) {
        self.type_id = id;
    }

    pub async fn from_url(url: Url) -> Self {
        CommonCaptcha {
            image: get_base64_image_from_url(url).await,
            token: String::new(),
            type_id: -1,
        }
    }
}

impl Captcha for CommonCaptcha {
    fn query_url(&self) -> String {
        String::from(yunma_captcha_query_urls::COMMON_CAPTCHA_QUERY_URL)
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn set_token(&mut self, token: String) {
        self.token = token;
    }

    fn check_type_id(&self) {
        if self.type_id < 0 {
            panic!("type id unset")
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonCaptchaQueryResult {
    pub msg: String,
    pub code: i64,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Data {
    #[serde(default)]
    pub code: i64,
    #[serde(default)]
    pub data: String,
    #[serde(default)]
    pub time: f64,
    #[serde(default)]
    pub unique_code: String,
}

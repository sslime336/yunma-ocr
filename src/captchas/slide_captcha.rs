//! 通用双图滑块  20111
//! 通用单图滑块(截图) 20110
//! 定制-滑块协议slide_traffic 900010
use reqwest::Url;
use serde::{Deserialize, Serialize};

use super::{get_base64_image_from_url, yunma_captcha_query_urls, Captcha};

/// 滑块缺口类验证
#[derive(Serialize, Deserialize, Debug)]
pub struct SlideCaptcha {
    /// 需要识别图片的小图片的base64字符串
    slide_image: String,
    /// 需要识别图片的背景图片的base64字符串(背景图需还原)
    background_image: String,
    /// 用户中心密钥
    token: String,
    #[serde(rename = "type")]
    type_id: i32,
}

impl SlideCaptcha {
    pub fn new() -> Self {
        SlideCaptcha {
            slide_image: String::new(),
            background_image: String::new(),
            token: String::new(),
            type_id: -1,
        }
    }

    pub fn set_type_id(&mut self, id: i32) {
        self.type_id = id;
    }

    pub async fn set_slide_image_from_url(&mut self, url: Url) {
        self.slide_image = get_base64_image_from_url(url).await;
    }

    pub async fn set_background_image_from_url(&mut self, url: Url) {
        self.background_image = get_base64_image_from_url(url).await;
    }

    pub async fn set_single_image_from_url(&mut self, url: Url) {}
}

impl Captcha for SlideCaptcha {
    fn query_url(&self) -> String {
        String::from(yunma_captcha_query_urls::SLIDE_CAPTCHA_QUERY_URL)
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

//! 通用双图滑块  20111
//! 通用单图滑块(截图) 20110
//! 定制-滑块协议slide_traffic 900010
use reqwest::Url;
use serde::{Deserialize, Serialize};

use super::{get_base64_image_from_url, Captcha, SIMPLE_CAPTCHA_QUERY_URL};

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
    /// slide_image, background_image 需要正确 base64 编码
    pub fn new(slide_image: String, background_image: String) -> Self {
        SlideCaptcha {
            slide_image,
            background_image,
            token: String::new(),
            type_id: 20111,
        }
    }

    pub async fn from_urls(slide_image_url: Url, background_image_url: Url) -> Self {
        SlideCaptcha {
            slide_image: get_base64_image_from_url(slide_image_url).await,
            background_image: get_base64_image_from_url(background_image_url).await,
            token: String::new(),
            type_id: 20111,
        }
    }
}

impl Captcha for SlideCaptcha {
    fn query_url(&self) -> String {
        String::from(SIMPLE_CAPTCHA_QUERY_URL)
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn set_token(&mut self, token: String) {
        self.token = token;
    }

    fn check_type_id(&self) {}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SingleSlideCaptcha {
    image: String,

    /// 用户中心密钥
    token: String,

    #[serde(rename = "type")]
    type_id: i32,
}

impl SingleSlideCaptcha {
    pub fn new(image: String) -> Self {
        Self {
            image,
            type_id: 20110,
            token: String::new(),
        }
    }

    pub async fn from_url(url: Url) -> Self {
        SingleSlideCaptcha {
            image: get_base64_image_from_url(url).await,
            token: String::new(),
            type_id: 20110,
        }
    }
}

impl Captcha for SingleSlideCaptcha {
    fn query_url(&self) -> String {
        String::from(SIMPLE_CAPTCHA_QUERY_URL)
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn set_token(&mut self, token: String) {
        self.token = token;
    }

    fn check_type_id(&self) {}
}

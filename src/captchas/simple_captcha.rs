use serde::{Deserialize, Serialize};

use crate::error::Result;

use super::Captcha;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SimpleCaptcha {
    /// 一般验证码，数字，字符及其组合
    Common {
        /// 需要识别图片的 base64 字符串
        image: String,

        /// 用户中心密钥
        token: String,

        #[serde(rename = "type")]
        type_id: i64,
    },

    /// 单滑块验证码
    SingleSlide {
        image: String,

        /// 用户中心密钥
        token: String,

        #[serde(rename = "type")]
        type_id: i64,
    },

    // 滑块验证码
    Slide {
        /// 需要识别图片的小图片的base64字符串
        slide_image: String,

        /// 需要识别图片的背景图片的base64字符串(背景图需还原)
        background_image: String,

        /// 用户中心密钥
        token: String,

        #[serde(rename = "type")]
        type_id: i64,
    },

    /// 点选类验证码
    Click {
        /// 需要识别图片的base64字符串
        image: String,

        /// 需要按某种语义点选的汉字
        extra: String,

        /// 用户中心密钥
        // #[serde(serialize_with = "fns")] //TODO
        token: String,

        #[serde(rename = "type")]
        type_id: i64,
    },
}

impl Captcha for SimpleCaptcha {
    fn query_url(&self) -> &str {
        "https://www.jfbym.com/api/YmServer/customApi"
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn set_token(&mut self, _token: String) {
        todo!()
    }

    fn check_type_id(&self) -> crate::error::Result<()> {
        todo!()
    }
}

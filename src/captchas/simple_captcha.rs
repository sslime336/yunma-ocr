use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};

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
        token: String,

        #[serde(rename = "type")]
        type_id: i64,
    },
}

impl SimpleCaptcha {
    #[allow(unused)]
    fn set_token(&mut self, client_token: String) {
        match self {
            SimpleCaptcha::Common {
                image,
                token,
                type_id,
            } => *token = client_token,
            SimpleCaptcha::SingleSlide {
                image,
                token,
                type_id,
            } => *token = client_token,
            SimpleCaptcha::Slide {
                slide_image,
                background_image,
                token,
                type_id,
            } => *token = client_token,
            SimpleCaptcha::Click {
                image,
                extra,
                token,
                type_id,
            } => *token = client_token,
        }
    }

    #[allow(unused)]
    fn check_type_id(&self) -> Result<()> {
        match self {
            SimpleCaptcha::Common {
                image,
                token,
                type_id,
            } => check_common_captcha_type_id(*type_id),
            SimpleCaptcha::SingleSlide {
                image,
                token,
                type_id,
            } => check_single_slide_captcha_type_id(*type_id),
            SimpleCaptcha::Slide {
                slide_image,
                background_image,
                token,
                type_id,
            } => check_slide_captcha_type_id(*type_id),
            SimpleCaptcha::Click {
                image,
                extra,
                token,
                type_id,
            } => check_click_captcha_type_id(*type_id),
        }
    }

    #[allow(unused)]
    fn check_click_captcha_extra_info(&self) -> Result<()> {
        match self {
            _ => Ok(()),
            SimpleCaptcha::Click {
                image,
                extra,
                token,
                type_id,
            } => {
                if extra.is_empty() {
                    Err(Error::FormatError(String::from(
                        "extra should not be empty",
                    )))
                } else {
                    //TODO: check type id with specific extra, here should add another if branch
                    Ok(())
                }
            }
        }
    }
}

impl Captcha for SimpleCaptcha {
    fn query_url(&self) -> &str {
        "https://www.jfbym.com/api/YmServer/customApi"
    }

    fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string(self)?)
    }

    fn set_token(&mut self, _token: String) {
        self.set_token(_token);
    }

    #[allow(unused)]
    fn check_vaild(&self) -> Result<()> {
        match self {
            _ => self.check_type_id(),
            SimpleCaptcha::Click {
                image,
                extra,
                token,
                type_id,
            } => self
                .check_type_id()
                .and(self.check_click_captcha_extra_info()),
        }
    }
}

#[inline]
fn check_common_captcha_type_id(type_id: i64) -> Result<()> {
    let supported_type_ids = [
        10110, 10111, 10112, 10113, 10103, 9001, 193, 10114, 10115, 10116, 10117, 10107, 50100,
        50101, 452,
    ];
    if supported_type_ids.contains(&type_id) {
        Ok(())
    } else {
        Err(Error::FormatError(String::from(
            "common captcha type unsupported",
        )))
    }
}
#[inline]
fn check_slide_captcha_type_id(type_id: i64) -> Result<()> {
    if type_id != 20111 {
        Err(Error::FormatError(String::from(
            "slide captcha type unsupported",
        )))
    } else {
        Ok(())
    }
}
#[inline]
fn check_single_slide_captcha_type_id(type_id: i64) -> Result<()> {
    if type_id != 20110 {
        Err(Error::FormatError(String::from(
            "single slide captcha type unsupported",
        )))
    } else {
        Ok(())
    }
}
#[inline]
fn check_click_captcha_type_id(type_id: i64) -> Result<()> {
    let supported_type_ids = [
        30009, 30100, 30103, 30102, 30104, 30105, 30106, 30107, 30109, 30110, 50009, 30101, 30108,
        30008, 30112, 30113, 30114,
    ];
    if supported_type_ids.contains(&type_id) {
        Ok(())
    } else {
        Err(Error::FormatError(String::from(
            "common captcha type unsupported",
        )))
    }
}

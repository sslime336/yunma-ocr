use serde::{Deserialize, Serialize};

use super::Captcha;

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

impl Captcha for SlideCaptcha {
    fn query_url(&self) -> String {
        String::from("https://www.jfbym.com/api/YmServer/customApi")
    }

    fn to_json(&self) -> String {
        todo!()
    }

    fn set_token(&mut self, token: &str) {
        self.token = token.to_owned();
    }
}

use serde::{Deserialize, Serialize};

use super::Captcha;

/// 点选类验证码
#[derive(Serialize, Deserialize, Debug)]
pub struct ClickCaptcha {
    /// 需要识别图片的base64字符串
    image: String,
    /// 需要按某种语义点选的汉字
    extra: String,
    /// 用户中心密钥
    token: String,
    #[serde(rename = "type")]
    type_id: i32,
}

impl Captcha for ClickCaptcha {
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

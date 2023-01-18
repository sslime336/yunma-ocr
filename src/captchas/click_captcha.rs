use super::{Captcha, SIMPLE_CAPTCHA_QUERY_URL};
use serde::{Deserialize, Serialize};

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

impl ClickCaptcha {
    pub fn new(image: String, extra: String, type_id: i32) -> Self {
        Self {
            image,
            extra,
            token: String::new(),
            type_id,
        }
    }

    pub fn set_type_id(&mut self, id: i32) {
        self.type_id = id;
    }
}

impl Captcha for ClickCaptcha {
    fn query_url(&self) -> String {
        String::from(SIMPLE_CAPTCHA_QUERY_URL)
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    fn set_token(&mut self, token: String) {
        self.token = token;
    }

    fn check_type_id(&self) {
        todo!()
    }
}

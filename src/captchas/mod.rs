use base64::Engine;

pub mod click_captcha;
pub mod common_captcha;
pub mod slide_captcha;

lazy_static! {
    static ref CAPTCHAGETTER: reqwest::Client = reqwest::Client::new();
}

pub trait Captcha {
    fn query_url(&self) -> String;
    fn to_json(&self) -> String;
    fn set_token(&mut self, token: &str);
}

#[inline]
fn encode_captcha(input: impl AsRef<[u8]>) -> String {
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(input)
}

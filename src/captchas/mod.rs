use base64::Engine;
use lazy_static::lazy_static;
use reqwest::Url;

pub mod click_captcha;
pub mod common_captcha;
pub mod slide_captcha;
mod yunma_captcha_query_urls;

lazy_static! {
    static ref CAPTCHAGETTER: reqwest::Client = reqwest::Client::new();
}

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

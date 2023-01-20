use crate::client::OCR_CLIENT;
use crate::error::Result;
use base64::Engine;
use reqwest::IntoUrl;

pub mod google_captchas;
pub mod simple_captcha;

pub trait Captcha {
    fn query_url(&self) -> &str;
    fn to_json(&self) -> Result<String>;
    fn set_token(&mut self, token: String);
    fn check_type_id(&self) -> Result<()>;
}

#[inline]
pub async fn get_base64ed_image_from_url(url: impl IntoUrl) -> Result<String> {
    let raw_captcha = OCR_CLIENT.get(url).send().await?.bytes().await?;

    Ok(encode_captcha(raw_captcha))
}

#[inline]
fn encode_captcha(input: impl AsRef<[u8]>) -> String {
    base64::engine::general_purpose::STANDARD_NO_PAD.encode(input)
}

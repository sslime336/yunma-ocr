#![allow(dead_code)]
mod settings;

use reqwest::Url;
use std::str::FromStr;
use yunma_ocr::Client;

use self::settings::CONFIG;

pub fn get_inited_test_client() -> Client {
    Client::init(user_token())
}

pub fn get_common_captcha_url() -> Url {
    Url::from_str(&CONFIG.captchas.common_captcha.url).unwrap()
}

pub fn get_common_captcha_result_expected() -> String {
    CONFIG.captchas.common_captcha.expected.clone()
}

pub fn get_slide_captcha_urls() -> (Url, Url, Url) {
    (
        Url::from_str(&CONFIG.captchas.slide_captcha.background_image_url).unwrap(),
        Url::from_str(&CONFIG.captchas.slide_captcha.slide_image_url).unwrap(),
        Url::from_str(&CONFIG.captchas.slide_captcha.single_image_url).unwrap(),
    )
}

#[inline]
pub fn user_token() -> String {
    CONFIG.account.token.clone()
}

#![allow(dead_code)]
mod settings;

use reqwest::{IntoUrl, Url};
use yunma_ocr::Client;

use self::settings::CONFIG;

pub fn get_inited_test_client() -> Client {
    Client::new(user_token())
}

pub fn get_common_captcha_url() -> Url {
    Url::parse(&CONFIG.captchas.common_captcha.url).unwrap()
}

pub fn get_common_captcha_result_expected() -> String {
    CONFIG.captchas.common_captcha.expected.clone()
}

pub fn get_slide_captcha_urls() -> (impl IntoUrl, impl IntoUrl, impl IntoUrl) {
    let placeholder_url = &CONFIG.captchas.slide_captcha.placeholder_url;

    let mut background_image_url = &CONFIG.captchas.slide_captcha.background_image_url;
    if background_image_url.is_empty() {
        background_image_url = placeholder_url;
    }
    let mut slide_image_url = &CONFIG.captchas.slide_captcha.slide_image_url;
    if slide_image_url.is_empty() {
        slide_image_url = placeholder_url;
    }
    let mut single_image_url = &CONFIG.captchas.slide_captcha.single_image_url;
    if single_image_url.is_empty() {
        single_image_url = placeholder_url;
    }

    (background_image_url, slide_image_url, single_image_url)
}

#[inline]
pub fn user_token() -> String {
    CONFIG.account.token.clone()
}

use std::fs::File;

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

const CONFIG_FILE: &str = "settings.yaml";

lazy_static! {
    pub static ref CONFIG: Config = init_config();
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub account: Account,
    pub captchas: Captchas,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Account {
    pub token: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(default)]
pub struct Captchas {
    pub common_captcha: CommonCaptcha,
    pub slide_captcha: SlideCaptcha,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "common_captcha")]
pub struct CommonCaptcha {
    pub url: String,
    #[serde(rename = "expect")]
    pub expected: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename = "slide_captcha")]
pub struct SlideCaptcha {
    pub slide_image_url: String,
    pub background_image_url: String,
    pub single_image_url: String,
    pub placeholder_url: String,
}

#[inline]
fn init_config() -> Config {
    serde_yaml::from_reader(File::open(CONFIG_FILE).unwrap()).unwrap()
}

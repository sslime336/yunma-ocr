use yunma_ocr::captchas::common_captcha::{CommonCaptcha, CommonCaptchaQueryResult};

mod helper;

#[tokio::test]
async fn test_parse_common_captcha() {
    let url = helper::get_common_captcha_url();
    let mut common_captcha = CommonCaptcha::from_url(url).await;
    common_captcha.set_type_id(10110);
    let client = helper::get_inited_test_client();
    let result = client.parse(common_captcha).await;

    dbg!(result);
}

#[tokio::test]
async fn test_parse_common_captcha_marshaled() {
    let url = helper::get_common_captcha_url();
    let mut common_captcha = CommonCaptcha::from_url(url).await;
    common_captcha.set_type_id(10110);
    let client = helper::get_inited_test_client();
    let result = client
        .parse_marshaled::<CommonCaptchaQueryResult>(common_captcha)
        .await;

    dbg!(result);
}

use yunma_ocr::captchas::slide_captcha::{SingleSlideCaptcha, SlideCaptcha};

mod helper;

#[tokio::test]
async fn test_get_token() {
    let (_background_image_url, _slide_image_url, _single_image_url) =
        helper::get_slide_captcha_urls();

    let _client = helper::get_inited_test_client();
}

#[tokio::test]
async fn test_single_image_slide_captcha() {
    let (_, _, single_image_url) = helper::get_slide_captcha_urls();
    let client = helper::get_inited_test_client();
    let base64encoded_captcha = SingleSlideCaptcha::from_url(single_image_url).await;
    let res = client.parse(base64encoded_captcha).await;

    dbg!(res);
}

#[tokio::test]
async fn test_double_image_slide_captcha() {
    let (background_image_url, slide_image_url, _) = helper::get_slide_captcha_urls();
    let client = helper::get_inited_test_client();
    let base64encoded_captcha =
        SlideCaptcha::from_urls(slide_image_url, background_image_url).await;
    let res = client.parse(base64encoded_captcha).await;

    dbg!(res);
}

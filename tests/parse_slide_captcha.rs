use reqwest::Url;
use yunma_ocr::captchas::slide_captcha::SlideCaptcha;

mod helper;

#[tokio::test]
async fn test_get_token() {
    let (background_image_url, slide_image_url, single_image_url) =
        helper::get_slide_captcha_urls();

    let _slide_captcha = build_slide_captcha(
        background_image_url,
        slide_image_url,
        single_image_url,
        20111,
    );
    let _client = helper::get_inited_test_client();
}

#[inline]
async fn build_slide_captcha(
    background_image_url: Url,
    slide_image_url: Url,
    single_image_url: Url,
    type_id: i32,
) -> SlideCaptcha {
    let mut sc = SlideCaptcha::new();
    sc.set_background_image_from_url(background_image_url).await;
    sc.set_slide_image_from_url(slide_image_url).await;
    sc.set_single_image_from_url(single_image_url).await;
    sc.set_type_id(type_id);

    sc
}

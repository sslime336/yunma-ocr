use yunma_ocr::{
    captchas::{self, simple_captcha::SimpleCaptcha},
    dto::GeneralResponse,
};

mod helper;

#[tokio::test]
async fn parse_common() {
    let client = helper::get_inited_test_client();
    let token = helper::user_token();
    let common_captcha = SimpleCaptcha::Common {
        image: captchas::get_base64ed_image_from_url(helper::get_common_captcha_url())
            .await
            .unwrap(),
        token,
        type_id: 10110,
    };
    let resp: GeneralResponse = client.parse(common_captcha).await.unwrap().into();

    dbg!(resp);
}

#[tokio::test]
async fn parse_single_slide() {
    let client = helper::get_inited_test_client();
    let token = helper::user_token();
    let (_, _, single_slide_url) = helper::get_slide_captcha_urls();
    let common_captcha = SimpleCaptcha::SingleSlide {
        image: captchas::get_base64ed_image_from_url(single_slide_url)
            .await
            .unwrap(),
        token,
        type_id: 20110,
    };
    let resp: GeneralResponse = client.parse(common_captcha).await.unwrap().into();

    dbg!(resp);
}

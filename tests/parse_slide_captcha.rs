mod helper;

#[tokio::test]
async fn test_get_token() {
    let (_background_image_url, _slide_image_url, _single_image_url) =
        helper::get_slide_captcha_urls();

    let _client = helper::get_inited_test_client();
}

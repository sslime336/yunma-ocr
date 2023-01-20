use yunma_ocr::{captchas::simple_captcha::SimpleCaptcha, dto::GeneralResponse};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let client = yunma_ocr::Client::new(std::env::var("TOKEN").unwrap());
    let slide_captcha = SimpleCaptcha::SingleSlide {
        image: yunma_ocr::captchas::get_base64ed_image_from_url(std::env::var("URL").unwrap())
            .await
            .unwrap(),
        token: String::new(), // 可不设置
        type_id: 20110,
    };
    let resp: GeneralResponse = client.parse(slide_captcha).await.unwrap().into();
    dbg!(resp);

    // 查询用户余额
    let account_score: GeneralResponse = client.query_balance().await.unwrap().into();
    dbg!(account_score);
}

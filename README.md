# ![云码](https://www.jfbym.com/static/img/logo.png) yunma-ocr rust-sdk[WIP]

**[点我看官方文档](https://www.jfbym.com/demo/)**

## 目前支持的请求

验证码

- [x] 通用类识别
- [x] 滑块缺口类验证码(双图未测试)
- [x] 点选类验证码(未测试)
- [ ] 谷歌v2，v3验证码 获取凭证
- [ ] 谷歌v2，v3验证码 根据凭证获取结果

用户相关

- [x] 查询积分余额
- [x] 报错接口(未测试)

## 食用方法

```rust
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
```

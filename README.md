# ![云码](https://www.jfbym.com/static/img/logo.png) yunma-ocr [WIP]

*云码 OCR 的 rust-sdk （非官方）*

---

## 目前支持的请求

- [x] 通用类识别
- [ ] 滑块缺口类验证码
- [ ] 点选类验证码
- [ ] 谷歌v2，v3验证码 获取凭证
- [ ] 谷歌v2，v3验证码 获取凭证
- [ ] 获取余额积分接口, post传参;报错接口

## 食用方法

### 通用类识别

```rust
dotenv().ok();

// 从 url 获取验证码
let url = reqwest::Url::from_str(&env::var("URL").unwrap()).unwrap();
let common_captcha = CommonCaptcha::from_url(url).await.set_type_id(10110);

let token = env::var("TOKEN").unwrap();
let client = Client::init(&token);
let res = client.parse(common_captcha.clone()).await;
println!("result = {}", res);

let res = client.parse_into::<CommonResponse>(common_captcha).await;
println!("captcha = {}", res.data.data);
```

### 滑块缺口类 *TODO*

## 速查

官网开发文档: <https://www.jfbym.com/demo/>

### 返回值参数

| 参数名称 | 传值类型 | 说明             |
| -------- | -------- | ---------------- |
| code     | int      | 状态值           |
| msg      | string   | 请求说明         |
| data     | array    | 打码数据         |
| --code   | int      | 打码服务状态     |
| --data   | string   | 打码服务识别内容 |
| --time   | string   | 打码服务时长     |

### code 列表

| code  | 说明                   |
| ----- | ---------------------- |
| 10000 | 识别成功               |
| 10001 | 参数错误               |
| 10002 | 余额不足               |
| 10003 | 无此访问权限           |
| 10004 | 无此验证类型           |
| 10005 | 网络拥塞               |
| 10006 | 数据包过载             |
| 10007 | 服务繁忙               |
| 10008 | 网络错误，请稍后重试   |
| 10009 | 结果准备中，请稍后再试 |
| 10010 | 请求结束               |

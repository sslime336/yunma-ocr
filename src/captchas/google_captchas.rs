use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetVoucherRequest {
    /// 用户中心密钥
    token: String,

    /// 40010或40011
    #[serde(rename = "type")]
    type_id: String,

    /// 验证码页面搜索data-sitekey的值
    googlekey: String,

    /// 验证码页面的所在地址
    pageurl: String,

    /// 如果不传默为非隐形版本,是否可见类型,0,1,default 1
    invisible: String,

    /// 可选参数，代理（示例： proxy=123.123.123.123:3128或proxy=proxyuser:strongPassword@123.123.123.123:3128）
    proxy: String,

    /// 可选参数，代理类型（HTTP、HTTPS、SOCKS4、SOCKS5）
    proxytype: String,

    /// 是否为企业版,如果为企业版本必须 default 0
    enterprise: String,

    /// reCaptcha v3 需要action值，找不到填空字符串
    action: String,

    ///	v2 企业版本必须参数，找不到填空字符串
    #[serde(rename = "data-s")]
    data_s: String,

    ///	V3才支持的可选参数，0.1~0.9之间
    min_score: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetResultRequest {
    /// 用户中心密钥
    token: String,

    /// 41接口返回的captchaId凭证 	是
    #[serde(rename = "captchaId")]
    captcha_id: String,

    /// 41接口返回的recordId记录号
    #[serde(rename = "recordId")]
    record_id: String,
}

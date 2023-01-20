use yunma_ocr::dto::GeneralResponse;

use crate::helper::get_inited_test_client;

mod helper;

#[tokio::test]
async fn test_get_user_score() {
    let client = get_inited_test_client();
    let resp_text: GeneralResponse = client.query_balance().await.unwrap().into();

    dbg!(resp_text);
}

#[ignore = "unsafe"]
#[tokio::test]
async fn test_report_error() {}

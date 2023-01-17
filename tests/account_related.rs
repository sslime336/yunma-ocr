use yunma_ocr::account::Account;

mod helper;

#[tokio::test]
async fn test_query_balance() {
    let client = helper::get_inited_test_client();
    let response_text = client.query_balance().await;

    dbg!(response_text);
}

#[tokio::test]
async fn test_query_balance_marshaled() {
    let client = helper::get_inited_test_client();
    let response_text = client.query_balance_marshaled().await;

    dbg!(response_text);
}

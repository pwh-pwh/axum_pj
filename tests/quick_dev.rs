use anyhow::Result;
use axum::Json;
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:8080")?;
    hc.do_get("/hello2/coderpwh1").await?.print().await?;
    hc.do_get("/src/error.rs").await?.print().await?;
    let req_login = hc.do_post(
        "/api/login",
        json!({
            "username": "root",
            "pwd": "root"
        }),
    );
    req_login.await?.print().await?;
    let req_ticket_create = hc.do_post(
        "/api/tickets",
        json!({
            "title": "ticket aaa"
        }),
    );
    req_ticket_create.await?.print().await?;
    let req_ticket_list = hc.do_get("/api/tickets").await?.print().await?;
    let req_ticket_del = hc.do_delete("/api/tickets/0").await?.print().await?;
    Ok(())
}

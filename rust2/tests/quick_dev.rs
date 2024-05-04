#![allow(unused)]

use std::{ thread, time::Duration };
use anyhow::{ Ok, Result };
use serde_json::json;

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let base_url = "http://localhost:8080";
    let hc = httpc_test::new_client(base_url)?;

    hc.do_get("/hello?name=Zaeem").await?.print().await?;
    hc.do_get("/hello2/Zaeem").await?.print().await?;

    let req_login = hc.do_post(
        "/api/login",
        json!({
        "username": "demo1",
        "pwd": "welcome"
    })
    );
    req_login.await?.print().await?;

    Ok(())
}

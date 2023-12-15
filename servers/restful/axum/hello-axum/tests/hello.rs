use reqwest::StatusCode;
use std::error::Error;

#[tokio::test]
async fn get_hello() -> Result<(), Box<dyn Error>> {
    let client = httpc_test::new_client("http://localhost:8080")?;
    let r = client.do_get("/hello").await?;

    r.print().await?;

    if r.status() != StatusCode::OK {Err("Http status != 200 Ok".into())}
    else {Ok(())}
}

#[tokio::test]
async fn get_hello2() -> Result<(), Box<dyn Error>> {
    let client = httpc_test::new_client("http://localhost:8080")?;
    let r = client.do_get("/hello2?name=Anton").await?;

    r.print().await?;

    if r.status() != StatusCode::OK {Err("Http status != 200 Ok".into())}
    else {Ok(())}
}

#[tokio::test]
async fn get_hello3() -> Result<(), Box<dyn Error>> {
    let client = httpc_test::new_client("http://localhost:8080")?;
    let r = client.do_get("/hello3/Romanov").await?;

    r.print().await?;

    if r.status() != StatusCode::OK {Err("Http status != 200 Ok".into())}
    else {Ok(())}
}
#![allow(unused)]

use eyre::Result;
use httpc_test::new_client;
// for runing tests with output, use 
// cargo test -- --nocapture
// ********************************
#[tokio::test]
async fn default_get() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    
    let res = hc.do_get("/").await?; // httpc_test::Response 
	let status = res.status();
	// Pretty print the result (status, headers, response cookies, client cookies, body)
	res.print().await?;

    Ok(())
}

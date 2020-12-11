use skyvein_rust_api::{module::pacs_deposit::{
    pacs_deposit_register_report
}};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // 影像存证
    pacs_deposit_register_report(
        "//Alice",
        "{\"id\":\"11113\",\"props\":[{\"name\":\"xxx\",\"value\":\"222\"}]}"
    ).await?;
    // 影像列表
    
    // 影像详情

    Ok(())
}
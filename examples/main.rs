use std::sync::Arc;
use skyvein_rust_api::{
    Client,
    hander::{
        PacsDeposit
    },
    model::{
        pacs_deposit::RegisterReportArgs
    }
};


#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // 创建连接
    let client:Arc<Client> = Arc::new(Client::new(
        "ws://127.0.0.1:9944".to_string(),
        "near rotate couch giant neck diary token roast come embark doll chaos".to_string()
    ));

    // 影像存证
    let pacs = PacsDeposit::new(Arc::clone(&client));
    let res = pacs.register_report(
        "{\"id\":\"11113\",\"props\":[{\"name\":\"xxx\",\"value\":\"222\"}]}"
    ).await;
    match res {
        Ok(s) => println!("Error reading file: {:?}", s),
        Err(e) => println!("Error reading file: {:?}", e)
    }

    // 影像存证
    let pacs2 = PacsDeposit::new(Arc::clone(&client));
    let res2 = pacs2.register_report(
        "{\"id\":\"11113\",\"props\":[{\"name\":\"xxx\",\"value\":\"222\"}]}"
    ).await;
    match res2 {
        Ok(s) => println!("Error reading file: {:?}", s),
        Err(e) => println!("Error reading file: {:?}", e)
    }

    // 影像列表
    
    
    // 影像详情

    Ok(())
}
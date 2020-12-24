use std::sync::Arc;
use skyvein_rust_api::{
    Client,
    hander::{
        PacsDeposit
    },
    model::{
        pacs_deposit::*,
    }
};


#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // 创建连接
    let client:Arc<Client> = Arc::new(Client::new(
        "ws://122.224.183.34:32186".to_string(),
        "near rotate couch giant neck diary token roast come embark doll chaos".to_string()
    ));

    // 影像存证
    let pacs = PacsDeposit::new(Arc::clone(&client));
    let call_args: ReportData = serde_json::from_str("{\"id\":\"11114\",\"props\":[{\"name\":\"xxx33333333333333333\",\"value\":\"222\"}]}")?;
    let res = pacs.register_report(call_args.clone()).await;
    match res {
        Ok(s) => println!("{:?}", s.as_str()),
        Err(e) => println!("{:?}", e)
    }

    // // 影像存证
    // let pacs2 = PacsDeposit::new(Arc::clone(&client));
    // let res2 = pacs2.register_report(
    //     "{\"id\":\"11113\",\"props\":[{\"name\":\"8888888888888xxx\",\"value\":\"222\"}]}"
    // ).await;
    // match res2 {
    //     Ok(s) => println!("{:?}", s),
    //     Err(e) => println!("{:?}", e)
    // }

    // // 影像列表
    // let res3 = pacs.report_list(10,None).await;
    // match res3 {
    //     Ok(s) => println!("{:?}", s),
    //     Err(e) => println!("{:?}", e)
    // }

    // // 影像详情
    // let res4 = pacs.report_detail_hash(
    //     "4a02cb5579ba624d1cb4d7f6bf911561b262e9238fa402540c250bc3f5d6188decd267edcb4ffed9d5c66510f6e3f593143131313133",
    // ).await;
    // match res4 {
    //     Ok(s) => println!("{:?}", s),
    //     Err(e) => println!("{:?}", e)
    // }

    // 影像详情
    let res4 = pacs.report_detail(
        "11114",
    ).await;
    match res4 {
        Ok(s) => println!("{:?}", s),
        Err(e) => println!("{:?}", e)
    }

    Ok(())
}
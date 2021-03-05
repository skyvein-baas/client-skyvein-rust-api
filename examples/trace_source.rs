use std::sync::Arc;
use skyvein_rust_api::{
    Client,
    hander::{
        TraceSource
    },
    model::{
        trace_source::*,
    }
};


#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // 创建连接
    let client:Arc<Client> = Arc::new(Client::new(
        "ws://localhost:9944".to_string(),
        "melt draft shy egg tomorrow there below flash patient code butter blind".to_string(),
        // "afford soft various cabbage census ship wild sample cute split soap sad".to_string(),
    ));

    // 注册产品
    let tsrc = TraceSource::new(Arc::clone(&client)).feeless();
    let call_args: ProductData = serde_json::from_str("{\"id\":\"11120\",\"props\":[{\"name\":\"name1\",\"value\":\"222\"}]}")?;
    let res = tsrc.register_product(call_args.clone()).await;
    match res {
        Ok(s) => println!("{:?}", s.as_str()),
        Err(e) => println!("{:?}", e)
    }
    // 注册批次
    let call_args2: ShipmentData = serde_json::from_str("{\"id\":\"21120\",\"products\":[\"11120\"]}")?;
    let res = tsrc.register_shipment(call_args2.clone()).await;
    match res {
        Ok(s) => println!("{:?}", s.as_str()),
        Err(e) => println!("{:?}", e)
    }
    // 提交批次
    let call_args3: TrackData = serde_json::from_str("{\"id\":\"21120\",\"shipping_operation\":\"Pickup\",\"location\":{\"latitude\":\"1\", \"longitude\":\"2\"}}")?;
    let res = tsrc.track_shipment(call_args3.clone()).await;
    match res {
        Ok(s) => println!("{:?}", s.as_str()),
        Err(e) => println!("{:?}", e)
    }
    // 提交批次
    let call_args4: TrackData = serde_json::from_str("{\"id\":\"21120\",\"shipping_operation\":\"Scan\",\"location\":{\"latitude\":\"3\", \"longitude\":\"4\"}, \"readings\":[{\"device_id\":\"11\", \"reading_type\":\"Humidity\", \"timestamp\":233, \"sensor_value\":\"213\"}]}")?;
    let res = tsrc.track_shipment(call_args4.clone()).await;
    match res {
        Ok(s) => println!("{:?}", s.as_str()),
        Err(e) => println!("{:?}", e)
    }
    // 提交批次
    let call_args5: TrackData = serde_json::from_str("{\"id\":\"21120\",\"shipping_operation\":\"Deliver\",\"location\":{\"latitude\":\"6\", \"longitude\":\"7\"}, \"readings\":[{\"device_id\":\"11\", \"reading_type\":\"Shock\", \"timestamp\":23333, \"sensor_value\":\"213\"}]}")?;
    let res = tsrc.track_shipment(call_args5.clone()).await;
    match res {
        Ok(s) => println!("{:?}", s.as_str()),
        Err(e) => println!("{:?}", e)
    }
    
    Ok(())
}
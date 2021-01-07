use std::sync::Arc;
use subxt::{
  PairSigner, DefaultNodeRuntime,
};

use sp_core::{sr25519::Pair, Pair as TraitPair};
use core::marker::PhantomData;
use std::error::Error;
use std::time;

use super::super::error_types::Error as RuntimeError;
use super::super::model::{
  trace_source::*,
};
use super::super::client::Client;

/// 溯源
pub struct TraceSource {
  // 连接
  client: Arc<Client>,
}

impl TraceSource {
	pub fn new(c: Arc<Client>) -> Self {
    TraceSource {
      client: c,
    }
  }

  // 注册产品
  pub async fn register_product(&self, call_args: ProductData) -> Result<String, Box<dyn Error>> {
  	let signer_key = Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
  	let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer_key);

  	// 创建连接
    let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(self.client.uri.as_str()).build().await?;
    // 构造请求参数
    let mut props:Vec<ProductProperty> = Vec::new();
    for v in call_args.props {
      props.push(ProductProperty::new(v.name.clone().into_bytes(),v.value.clone().into_bytes()));
    }

    // 构造请求
    let product_call = RegisterProductCall::<DefaultNodeRuntime> {
      id: call_args.id.clone().into_bytes(),
      props: Some(props),
      _runtime: PhantomData,
    };

    // 签名
    let extrinsic = client.create_signed(product_call, &signer).await?;

    // 构造错误接受
    let mut decoder = client.events_decoder::<RegisterProductCall<DefaultNodeRuntime>>();
    // decoder.register_type_size::<ProductId>("ProductId");
    decoder.with_trace_source();

    // 提交请求
    let event_result = client.submit_and_watch_extrinsic(extrinsic, decoder).await;
    #[allow(unused_assignments)]
    let mut block_hash = String::from("");
    match event_result {
      Ok(s) => {
        block_hash = "0x".to_string()+&hex::encode(&s.block[..].to_vec());
      },
      Err(e) => return Err(("调用错误:".to_owned()+&(e.to_string())).into())
    };
    Ok(block_hash)
  }

  // 注册批次
  pub async fn register_shipment(&self, call_args: ShipmentData) -> Result<String, Box<dyn Error>> {
    let signer_key = Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
    let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer_key);

    // 创建连接
    let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(self.client.uri.as_str()).build().await?;
    // 构造请求参数
    let mut products:Vec<ProductId> = Vec::new();
    for v in call_args.products {
      products.push(v.clone().into_bytes() as ProductId);
    }

    // 构造请求
    let shipment_call = RegisterShipmentCall::<DefaultNodeRuntime> {
      id: call_args.id.clone().into_bytes(),
      products: products,
      _runtime: PhantomData,
    };

    // 签名
    let extrinsic = client.create_signed(shipment_call, &signer).await?;

    // 构造错误接受
    let mut decoder = client.events_decoder::<RegisterShipmentCall<DefaultNodeRuntime>>();
    decoder.register_type_size::<u128>("u128");
    decoder.with_trace_source();

    // 提交请求
    let event_result = client.submit_and_watch_extrinsic(extrinsic, decoder).await;
    #[allow(unused_assignments)]
    let mut block_hash = String::from("");
    match event_result {
      Ok(s) => {
        block_hash = "0x".to_string()+&hex::encode(&s.block[..].to_vec());
      },
      Err(e) => return Err(("调用错误:".to_owned()+&(e.to_string())).into())
    };
    Ok(block_hash)
  }

  // 批次追踪
  pub async fn track_shipment(&self, call_args: TrackData) -> Result<String, Box<dyn Error>> {
    let signer_key = Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
    let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer_key);

    // 创建连接
    let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(self.client.uri.as_str()).build().await?;
    // 构造请求参数


    // 构造请求
    let shipment_call = TrackShipmentCall::<DefaultNodeRuntime> {
      id: call_args.id.clone().into_bytes(),
      operation: match &call_args.shipping_operation[..] {
        "Pickup" => ShippingOperation::Pickup,
        "Scan" => ShippingOperation::Scan,
        "Deliver" => ShippingOperation::Deliver,
        _ => ShippingOperation::Scan,
      },
      timestamp: time::SystemTime::now().elapsed().unwrap().as_secs(),
      location: match call_args.location {
        Some(loc) => Some(ReadPoint::new(loc.latitude.into_bytes(), loc.longitude.into_bytes())),
        None => None,
      },
      readings: match call_args.readings {
        Some(readings_data) => Some(readings_data.iter().map(|r| Reading::from_data(r)).collect()),
        None => None,
      },
    };

    // 签名
    let extrinsic = client.create_signed(shipment_call, &signer).await?;

    // 构造错误接受
    let mut decoder = client.events_decoder::<RegisterShipmentCall<DefaultNodeRuntime>>();
    decoder.register_type_size::<u128>("u128");
    decoder.with_trace_source();

    // 提交请求
    let event_result = client.submit_and_watch_extrinsic(extrinsic, decoder).await;
    #[allow(unused_assignments)]
    let mut block_hash = String::from("");
    match event_result {
      Ok(s) => {
        block_hash = "0x".to_string()+&hex::encode(&s.block[..].to_vec());
      },
      Err(e) => return Err(("调用错误:".to_owned()+&(e.to_string())).into())
    };
    Ok(block_hash)
  }
}
// use sp_keyring::sr25519::sr25519::Pair;

use sp_core::{sr25519::Pair, Pair as TraitPair};
use core::marker::PhantomData;
use subxt::{
	PairSigner, DefaultNodeRuntime,
	Error, 
};
use skyvein_rust_api::error_types::Error as RuntimeError;
use std::sync::Arc;
use skyvein_rust_api::{
    Client,
};
use skyvein_rust_api::model::{
  trace_source::*,
  feeless::*
};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
	// 创建连接
    let clientctx:Arc<Client> = Arc::new(Client::new(
        "ws://localhost:9944".to_string(),
        "melt draft shy egg tomorrow there below flash patient code butter blind".to_string()
    ));
    let call_args: ProductData = serde_json::from_str("{\"id\":\"11120\",\"props\":[{\"name\":\"name1\",\"value\":\"222\"}]}")?;

    let signer_key = Pair::from_string(&clientctx.seed_get(), None).map_err(|_| RuntimeError::WrongAcount)?;
  	let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer_key);

  	// 创建连接
    let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().set_url(clientctx.uri.as_str()).
      skip_type_sizes_check().
      build().await?;
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

    let product_proposal = client.encode(product_call)?;

    let feeless_call = FeelessCall::<DefaultNodeRuntime> {
    	call: &product_proposal,
    	_runtime: PhantomData,
    };

    // 签名
    let extrinsic = client.create_signed(feeless_call, &signer).await?;
	// 提交请求
    let event_result = client.submit_and_watch_extrinsic(extrinsic).await;
    // let event_result = client.feeless_and_watch(&signer, &product_proposal).await;
     #[allow(unused_assignments)]
    let mut block_hash = String::from("");
    match event_result {
      Ok(s) => {
        block_hash = "0x".to_string()+&hex::encode(&s.block[..].to_vec());
        println!("{:?}", block_hash.as_str());
      },
      Err(Error::Runtime(e)) => {
      	// match e {
      	// 	RuntimeError::Runtime::Module(me) => {
      	// 		println!("{:?}", e);		
      	// 	}
      	// }
      	let emain = e.to_string();
      	let estr: Vec<&str> = emain.trim_start_matches("Runtime module error:").split(" from ").collect();
      	println!("{:?}", estr[0].trim_start_matches(" "));
      }
      Err(e) => {
      	// e = Err(("调用错误:".to_owned()+&(e.to_string())).into());
      	println!("{:?}", e.to_string());
      }
    };
    Ok(())
}

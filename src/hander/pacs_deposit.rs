use std::sync::Arc;
use subxt::{
  PairSigner, DefaultNodeRuntime
};
use sp_core::{sr25519::Pair, Pair as TraitPair};
use core::marker::PhantomData;
use std::error::Error;

use super::super::error_types::Error as RuntimeError;
use super::super::model::pacs_deposit::*;
use super::super::client::Client;


/// 医疗影像存证
pub struct PacsDeposit {
  // 连接
  client: Arc<Client>,
}

impl PacsDeposit {
  pub fn new(c: Arc<Client>) -> Self {
    PacsDeposit {
      client: c,
    }
  }

  // 生成报告
  pub async fn register_report(self, val: &str) -> Result<String, Box<dyn Error>>{
    // let s = self.client.seed_get();
    let signer = Pair::from_string(&self.client.seed_get(), None).map_err(|_| RuntimeError::WrongSudoSeed)?;
    let signer = PairSigner::<DefaultNodeRuntime, Pair>::new(signer);
    
    // 创建连接
    let client = subxt::ClientBuilder::<DefaultNodeRuntime>::new().build().await?;
    
    // 构造请求参数
    let call_args: RegisterReportArgs = serde_json::from_str(val)?;
    let mut props:Vec<ReportProperty> = Vec::new();
    for v in call_args.props {
      props.push(ReportProperty::new(v.name.clone().into_bytes(),v.value.clone().into_bytes()));
    }

    // 构造请求
    let report_call = RegisterReportCall::<DefaultNodeRuntime> {
      id: call_args.id.clone().into_bytes(),
      props: Some(props),
      _runtime: PhantomData,
    };

    // 签名
    let extrinsic = client.create_signed(report_call, &signer).await?;

    // 构造错误接受
    let mut decoder = client.events_decoder::<RegisterReportCall<DefaultNodeRuntime>>();
    decoder.with_pacs_deposit();

    // 提交请求
    let events = client.submit_and_watch_extrinsic(extrinsic, decoder).await?;
    events.find_event::<ReportRegisteredEvent::<DefaultNodeRuntime>>()?.ok_or("No Event found or decoded.")?;
    let block_hash = events.block;

    Ok(block_hash.to_string())
  }
}


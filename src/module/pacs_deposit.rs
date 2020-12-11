use subxt::{
  system::{System, SystemEventsDecoder},PairSigner,
  Call, Event,
  DefaultNodeRuntime, 
};
use super::super::error_types::Error as RuntimeError;
use sp_core::{sr25519::Pair, Pair as TraitPair};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
use std::error::Error;

/// 模块定义
#[subxt::module]
pub trait PacsDeposit: System {}
impl PacsDeposit for DefaultNodeRuntime {}

pub type ReportId = Vec<u8>;
pub type PropName = Vec<u8>;
pub type PropValue = Vec<u8>;

// 存证信息
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct Report<AccountId, Moment> {
  // 存证id
  id: ReportId,
  // 存证企业
  com: AccountId,
  // 操作人员
  operator: AccountId,
  // 用户手机
  // 用户身份证
  // 属性列表
  props: Option<Vec<ReportProperty>>,
  // 存证时间
  registered: Moment,
}

// 存证属性
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportProperty {
  // 属性名
  name: PropName,
  // 属性值
  value: PropValue,
}

impl ReportProperty {
  pub fn new(name: PropName, value: PropValue) -> ReportProperty {
    ReportProperty { name: name, value: value }
  }
}

/// Execute a transaction with sudo permissions.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RegisterReportCall<T: PacsDeposit> {
  pub id: ReportId,
  pub props: Option<Vec<ReportProperty>>,
  pub _runtime: PhantomData<T>,
}

// 存证属性
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportPropertyArgs {
  // 属性名
  name: String,
  // 属性值
  value: String,
}
#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct RegisterReportArgs {
  // 报告id
  pub id: String,
  // 属性
  pub props: Vec<ReportPropertyArgs>,
}

// 成功生成报告
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ReportRegistered<T: PacsDeposit> {
	pub _runtime: PhantomData<T>,
}

// 生成报告
pub async fn pacs_deposit_register_report(signer: &str,val: &str) -> Result<String, Box<dyn Error>>{
  let signer = Pair::from_string(signer.as_ref(), None).map_err(|_| RuntimeError::WrongSudoSeed)?;
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

  // 提交请求 let batch_events = 
	client.submit_and_watch_extrinsic(extrinsic, decoder).await?;
	// let event = batch_events.find_event::<BatchCompletedEvent::<DefaultNodeRuntime>>()?.ok_or("No Event found or decoded.")?;
  // let block_hash = batch_events.block;
  // return true
  
  Ok("ok".to_string())
}
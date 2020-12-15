use subxt::{
  system::{System, SystemEventsDecoder},
  Call, Event,
  DefaultNodeRuntime, 
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};

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
  pub id: ReportId,
  // 存证企业
  pub com: AccountId,
  // 操作人员
  pub operator: AccountId,
  // 用户手机
  // 用户身份证
  // 属性列表
  pub props: Option<Vec<ReportProperty>>,
  // 存证时间
  pub registered: Moment,
}

// 存证属性
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportProperty {
  // 属性名
  pub name: PropName,
  // 属性值
  pub value: PropValue,
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
  pub name: String,
  // 属性值
  pub value: String,
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
pub struct ReportRegisteredEvent<T: PacsDeposit> {
  pub who: <T as System>::AccountId,
  pub com: <T as System>::AccountId,
  pub id: ReportId,
}
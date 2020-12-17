use subxt::{
  system::{System, SystemEventsDecoder},
  Call, Event, Store,
  DefaultNodeRuntime,
  balances::{Balances, BalancesEventsDecoder},
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use super::timestamp::{Timestamp, TimestampEventsDecoder};

// 模块定义
#[subxt::module]
pub trait PacsDeposit: System + Balances + Timestamp {
  // type ReportId: Encode + Decode + PartialEq + Eq + Default + Send + Sync + 'static;
}
impl PacsDeposit for DefaultNodeRuntime {
  // type ReportId = ReportId;
}

// 类型
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

// 报告注册参数
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RegisterReportCall<T: PacsDeposit> {
  pub id: ReportId,
  pub props: Option<Vec<ReportProperty>>,
  pub _runtime: PhantomData<T>,
}

// 报告属性参数
#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportData {
  // 报告id
  pub id: String,
  // 存证企业
  pub com: Option<String>,
  // 操作人员
  pub operator: Option<String>,
  // 属性
  pub props: Vec<ReportPropertyData>,
}
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ReportPropertyData {
  // 属性名
  pub name: String,
  // 属性值
  pub value: String,
}

// 成功生成报告
#[derive(Clone, Debug, Eq, PartialEq, Event, Decode)]
pub struct ReportRegisteredEvent<T: PacsDeposit> {
  pub who: <T as System>::AccountId,
  pub com: <T as System>::AccountId,
  pub id: Vec<u8>,
}

// 报告列表
#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ComOfReportStore<'a, T: PacsDeposit> {
  #[store(returns = T::AccountId)]
  pub report_id: &'a ReportId,
  pub _phantom: PhantomData<T>,
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct ReportsStore<'a, T: PacsDeposit> {
  #[store(returns = Option<Report<T::AccountId, <T as Timestamp>::Moment>>)]
  pub report_id: &'a ReportId,
  pub _phantom: PhantomData<T>,
}

// 报告提交记录
#[derive(Debug, Clone, Decode, Encode, Deserialize, Serialize)]
pub struct ReportDataH{
  pub hash: String,
  pub value: ReportData,
}

// 报告详情
#[derive(Debug, Clone, Decode, Encode, Deserialize, Serialize)]
pub struct ReportDetail{
  // 数据唯一值
  pub key: String,
  // 当前值
  pub curent_value: Option<ReportDataH>,
  // 历史记录
  pub history: Vec<ReportDataH>,
}

// 报告列表
#[derive(Debug, Clone, Decode, Encode, Deserialize, Serialize)]
pub struct ReportList{
  pub list: Vec<ReportDetail>,
}
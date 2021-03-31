use super::timestamp::Timestamp;
use codec::{Decode, Encode};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use subxt::{balances::Balances, system::System, Call, DefaultNodeRuntime, Event, Store};

// 模块定义
#[subxt::module]
pub trait Art: System + Balances + Timestamp {
  // type ReportId: Encode + Decode + PartialEq + Eq + Default + Send + Sync + 'static;
}
impl Art for DefaultNodeRuntime {
  // type ReportId = ReportId;
}

// 类型
pub type ArtStr = Vec<u8>;

// 存证属性
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ArtProperty {
  // 属性名
  pub name: ArtStr,
  // 属性值
  pub value: ArtStr,
}

// 存证请求参数
#[derive(Clone, Debug, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct RegisterArtData {
  pub id: String,
  pub owner: String,
  pub entity_ids: Vec<String>,
  pub props: Vec<ArtPropertyData>,
  pub hash_method: String,
  pub hash: String,
  pub dna_method: String,
  pub dna: String,
}
#[derive(Clone, Debug, Eq, PartialEq, Decode, Encode, Deserialize, Serialize)]
pub struct ArtPropertyData {
  // 属性名
  pub name: String,
  // 属性值
  pub value: String,
}

// 存证请求函数
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct RegisterArtCall<T: Art> {
  pub id: ArtStr,
  pub owner: T::AccountId,
  pub entity_ids: Vec<ArtStr>,
  pub props: Vec<ArtProperty>,
  pub hash_method: ArtStr,
  pub hash: ArtStr,
  pub dna_method: ArtStr,
  pub dna: ArtStr,
  pub _runtime: PhantomData<T>,
}

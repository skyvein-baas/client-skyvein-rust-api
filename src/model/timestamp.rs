use subxt::{
  system::{System, SystemEventsDecoder},
  Call, Event, Store,
  DefaultNodeRuntime,
};
use sp_runtime::{
	RuntimeString,
	traits::{
		AtLeast32Bit, Zero, SaturatedConversion, Scale, Member
	}
};
use frame_support::{
  Parameter,
};
use codec::{Decode, Encode};
use core::marker::PhantomData;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

// 模块定义
#[subxt::module]
pub trait Timestamp: System {
	type Moment: Parameter + Default + AtLeast32Bit + Member
		+ Scale<Self::BlockNumber, Output = Self::Moment> + Copy;
}

pub type Moment = u64;
impl Timestamp for DefaultNodeRuntime {
  type Moment = Moment;
}

#[derive(Clone, Debug, Eq, PartialEq, Store, Encode)]
pub struct NowStore<T: Timestamp> {
    #[store(returns = T::Moment)]
    pub _runtime: PhantomData<T>,
}
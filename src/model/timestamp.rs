use subxt::{
  system::{System, SystemEventsDecoder}, Store,
  DefaultNodeRuntime,
};
use sp_runtime::{
	traits::{
		AtLeast32Bit, Scale, Member
	}
};
use frame_support::{
  Parameter,
};
use codec::{Encode};
use core::marker::PhantomData;
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
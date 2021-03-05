use subxt::{
  system::{System},
  Call, Encoded,
  DefaultNodeRuntime,
};
use codec::{Encode};
use core::marker::PhantomData;

/// The subset of the `frame_sudo::Trait` that a client must implement.
#[subxt::module]
pub trait Feeless: System {}

/// Execute a transaction with sudo permissions.
#[derive(Clone, Debug, Eq, PartialEq, Call, Encode)]
pub struct FeelessCall<'a, T: Feeless> {
    /// Runtime marker.
    pub _runtime: PhantomData<T>,
    /// Encoded transaction.
    pub call: &'a Encoded,
}

impl Feeless for DefaultNodeRuntime {}
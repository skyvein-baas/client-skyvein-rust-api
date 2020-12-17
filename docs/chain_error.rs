use jsonrpsee::{
  client::RequestError,
  transport::ws::WsNewDnsError,
};
use sp_core::crypto::SecretStringError;
use sp_runtime::{
  transaction_validity::TransactionValidityError,
  DispatchError,
};
use thiserror::Error;

use crate::metadata::{
  Metadata,
  MetadataError,
};

/// Error enum.
#[derive(Debug, Error)]
pub enum Error {
  /// Io error.
  #[error("Io error: {0}")]
  Io(#[from] std::io::Error),
  /// Codec error.
  #[error("Scale codec error: {0}")]
  Codec(#[from] codec::Error),
  /// Rpc error.
  #[error("Rpc error: {0}")]
  Rpc(#[from] RequestError),
  /// Error that can happen during the initial websocket handshake
  #[error("Rpc error: {0}")]
  WsHandshake(#[from] WsNewDnsError),
  /// Serde serialization error
  #[error("Serde json error: {0}")]
  Serialization(#[from] serde_json::error::Error),
  /// Secret string error.
  #[error("Secret String Error")]
  SecretString(SecretStringError),
  /// Extrinsic validity error
  #[error("Transaction Validity Error: {0:?}")]
  Invalid(TransactionValidityError),
  /// Metadata error.
  #[error("Metadata error: {0}")]
  Metadata(#[from] MetadataError),
  /// Type size unavailable.
  #[error("Type size unavailable while decoding event: {0:?}")]
  TypeSizeUnavailable(String),
  /// Runtime error.
  #[error("Runtime error: {0}")]
  Runtime(#[from] RuntimeError),
  /// Other error.
  #[error("Other error: {0}")]
  Other(String),
}

impl From<SecretStringError> for Error {
  fn from(error: SecretStringError) -> Self {
      Error::SecretString(error)
  }
}

impl From<TransactionValidityError> for Error {
  fn from(error: TransactionValidityError) -> Self {
      Error::Invalid(error)
  }
}

impl From<&str> for Error {
  fn from(error: &str) -> Self {
      Error::Other(error.into())
  }
}

impl From<String> for Error {
  fn from(error: String) -> Self {
      Error::Other(error)
  }
}

/// Runtime error.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RuntimeError {
  /// Module error.
  #[error("Runtime module error: {0}")]
  Module(ModuleError),
  /// Bad origin.
  #[error("Bad origin: throw by ensure_signed, ensure_root or ensure_none.")]
  BadOrigin,
  /// Cannot lookup.
  #[error("Cannot lookup some information required to validate the transaction.")]
  CannotLookup,
  /// Other error.
  #[error("Other error: {0}")]
  Other(String),
}

impl RuntimeError {
  /// Converts a `DispatchError` into a subxt error.
  pub fn from_dispatch(
      metadata: &Metadata,
      error: DispatchError,
  ) -> Result<Self, Error> {
      match error {
          DispatchError::Module {
              index,
              error,
              message: _,
          } => {
              let module = metadata.module_with_errors(index)?;
              let error = module.error(error)?;
              Ok(Self::Module(ModuleError {
                  module: module.name().to_string(),
                  error: error.to_string(),
              }))
          }
          DispatchError::BadOrigin => Ok(Self::BadOrigin),
          DispatchError::CannotLookup => Ok(Self::CannotLookup),
          DispatchError::Other(msg) => Ok(Self::Other(msg.into())),
      }
  }
}

/// Module error.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("{error} from {module}")]
pub struct ModuleError {
  pub module: String,
  pub error: String,
}
use std::{
	fmt::{self, Display},
	error,
};

#[derive(Clone, Debug)]
pub enum Error {
	WrongSudoSeed,
	SubxtError(&'static str),
}

impl Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match *self {
			Self::WrongSudoSeed => write!(f, "Wrong sudo seed, failed to sign transaction."),
			Self::SubxtError(e) => write!(f, "Error from subxt crate: {}", e),
		}
	}
}

impl error::Error for Error {
	fn description(&self) -> &str {
		match *self {
			Self::WrongSudoSeed => "Wrong sudo seed, failed to sign transaction.",
			Self::SubxtError(e) => e,
		}
	}
}

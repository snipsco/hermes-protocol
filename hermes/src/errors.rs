use failure;
use std::result;
use std::sync::PoisonError;

pub type Result<T> = result::Result<T, failure::Error>;

#[derive(Debug, Fail)]
#[fail(display = "Can't lock thread")]
pub struct PoisonLock;

impl<T> From<PoisonError<T>> for PoisonLock {
    fn from(_: PoisonError<T>) -> Self {
        Self {}
    }
}

use std::sync::PoisonError;

#[derive(Debug, Fail)]
#[fail(display = "Can't lock thread")]
pub struct PoisonLock;

impl<T> From<PoisonError<T>> for PoisonLock {
    fn from(_: PoisonError<T>) -> Self {
        Self {}
    }
}
